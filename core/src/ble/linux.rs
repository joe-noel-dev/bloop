use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::{bail, Result};
use bluer::{
    adv::Advertisement,
    gatt::{
        local::{
            characteristic_control, service_control, Application, Characteristic, CharacteristicControlEvent,
            CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicWrite, CharacteristicWriteMethod, ReqError,
            Service,
        },
        CharacteristicWriter,
    },
    Address, Uuid,
};
use futures::StreamExt;
use log::{error, info, warn};
use tokio::{
    sync::{broadcast, mpsc, Mutex, Notify},
    time::{sleep, timeout},
};

use super::{
    framing::{encode_message, Reassembler},
    REQUEST_CHARACTERISTIC_UUID, RESPONSE_CHARACTERISTIC_UUID, SERVICE_UUID,
};
use crate::{
    api::{
        client::{create_client_responses, handle_client_configuration, ClientConfigurationHandle, ClientResponses},
        wire::{decode_request, encode_response},
    },
    bloop::{Request, Response},
};

const RETRY_DELAYS: [Duration; 5] = [
    Duration::from_secs(1),
    Duration::from_secs(2),
    Duration::from_secs(5),
    Duration::from_secs(10),
    Duration::from_secs(30),
];

#[derive(Clone)]
struct ActiveClient {
    address: Address,
    session_id: u64,
    configuration: ClientConfigurationHandle,
    direct_response_tx: mpsc::Sender<Response>,
}

type SharedActiveClient = Arc<Mutex<Option<ActiveClient>>>;

struct IncomingChunk {
    value: Vec<u8>,
    client: ActiveClient,
}

pub async fn run(enabled: bool, request_tx: mpsc::Sender<Request>, response_tx: broadcast::Sender<Response>) {
    if !enabled {
        return;
    }

    let mut retry_index = 0;
    loop {
        match serve_once(request_tx.clone(), response_tx.clone()).await {
            Ok(()) => warn!("BLE peripheral stopped unexpectedly"),
            Err(error) => warn!("BLE peripheral unavailable: {error:#}"),
        }

        let delay = RETRY_DELAYS[retry_index.min(RETRY_DELAYS.len() - 1)];
        retry_index = (retry_index + 1).min(RETRY_DELAYS.len() - 1);
        warn!("Retrying BLE peripheral registration in {}s", delay.as_secs());
        sleep(delay).await;
    }
}

async fn serve_once(request_tx: mpsc::Sender<Request>, response_tx: broadcast::Sender<Response>) -> Result<()> {
    let service_uuid = Uuid::parse_str(SERVICE_UUID)?;
    let request_uuid = Uuid::parse_str(REQUEST_CHARACTERISTIC_UUID)?;
    let response_uuid = Uuid::parse_str(RESPONSE_CHARACTERISTIC_UUID)?;

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let advertisement = Advertisement {
        service_uuids: [service_uuid].into_iter().collect(),
        discoverable: Some(true),
        local_name: Some("Bloop".to_string()),
        ..Default::default()
    };
    let _advertisement_handle = adapter.advertise(advertisement).await?;

    let active_client: SharedActiveClient = Arc::new(Mutex::new(None));
    let active_client_changed = Arc::new(Notify::new());
    let (incoming_chunk_tx, mut incoming_chunk_rx) = mpsc::channel::<IncomingChunk>(128);
    let (_service_control, service_handle) = service_control();
    let (response_control, response_handle) = characteristic_control();
    let request_active_client = active_client.clone();
    let request_active_client_changed = active_client_changed.clone();
    let application = Application {
        services: vec![Service {
            uuid: service_uuid,
            primary: true,
            characteristics: vec![
                Characteristic {
                    uuid: request_uuid,
                    write: Some(CharacteristicWrite {
                        write: true,
                        write_without_response: false,
                        method: CharacteristicWriteMethod::Fun(Box::new(move |value, request| {
                            let incoming_chunk_tx = incoming_chunk_tx.clone();
                            let active_client = request_active_client.clone();
                            let active_client_changed = request_active_client_changed.clone();
                            Box::pin(async move {
                                if request.offset != 0 {
                                    return Err(ReqError::InvalidOffset);
                                }

                                let client = wait_for_active_client(
                                    &active_client,
                                    &active_client_changed,
                                    request.device_address,
                                )
                                .await
                                .ok_or(ReqError::NotPermitted)?;
                                incoming_chunk_tx
                                    .send(IncomingChunk { value, client })
                                    .await
                                    .map_err(|_| ReqError::Failed)
                            })
                        })),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                Characteristic {
                    uuid: response_uuid,
                    notify: Some(CharacteristicNotify {
                        notify: true,
                        method: CharacteristicNotifyMethod::Io,
                        ..Default::default()
                    }),
                    control_handle: response_handle,
                    ..Default::default()
                },
            ],
            control_handle: service_handle,
            ..Default::default()
        }],
        ..Default::default()
    };
    let _application_handle = adapter.serve_gatt_application(application).await?;

    info!(
        "BLE peripheral advertising on {} ({})",
        adapter.name(),
        adapter.address().await?
    );

    let next_session_id = Arc::new(AtomicU64::new(1));
    futures::pin_mut!(response_control);
    let mut reassembler = Reassembler::default();
    let mut reassembly_session_id = None;

    loop {
        tokio::select! {
            chunk = incoming_chunk_rx.recv() => {
                let Some(chunk) = chunk else {
                    bail!("BLE request characteristic input stream closed")
                };
                if reassembly_session_id != Some(chunk.client.session_id) {
                    reassembler.reset();
                    reassembly_session_id = Some(chunk.client.session_id);
                }
                handle_incoming_chunk(
                    chunk,
                    &mut reassembler,
                    &active_client,
                    &request_tx,
                ).await;
            }
            event = response_control.next() => {
                match event {
                    Some(CharacteristicControlEvent::Notify(writer)) => {
                        register_notification_client(
                            writer,
                            active_client.clone(),
                            active_client_changed.clone(),
                            response_tx.clone(),
                            next_session_id.fetch_add(1, Ordering::Relaxed),
                        ).await;
                    }
                    Some(_) => {}
                    None => bail!("BLE response characteristic control stream closed"),
                }
            }
        }
    }
}

async fn register_notification_client(
    writer: CharacteristicWriter,
    active_client: SharedActiveClient,
    active_client_changed: Arc<Notify>,
    response_tx: broadcast::Sender<Response>,
    session_id: u64,
) {
    let address = writer.device_address();
    let mut active = active_client.lock().await;
    if active.is_some() {
        warn!("Rejecting additional BLE notification client {address}");
        return;
    }

    let (configuration, responses) = create_client_responses(response_tx.subscribe());
    let (direct_response_tx, direct_response_rx) = mpsc::channel(32);
    *active = Some(ActiveClient {
        address,
        session_id,
        configuration,
        direct_response_tx,
    });
    drop(active);
    active_client_changed.notify_waiters();

    info!("BLE client connected: {address}");
    tokio::spawn(run_response_writer(
        writer,
        responses,
        direct_response_rx,
        active_client,
        address,
        session_id,
    ));
}

async fn wait_for_active_client(
    active_client: &SharedActiveClient,
    active_client_changed: &Notify,
    address: Address,
) -> Option<ActiveClient> {
    let changed = active_client_changed.notified();
    if let Some(client) = active_client
        .lock()
        .await
        .as_ref()
        .filter(|client| client.address == address)
        .cloned()
    {
        return Some(client);
    }

    timeout(Duration::from_secs(1), changed).await.ok()?;
    active_client
        .lock()
        .await
        .as_ref()
        .filter(|client| client.address == address)
        .cloned()
}

async fn handle_incoming_chunk(
    chunk: IncomingChunk,
    reassembler: &mut Reassembler,
    active_client: &SharedActiveClient,
    request_tx: &mpsc::Sender<Request>,
) {
    let client = chunk.client;
    let address = client.address;
    if !is_active_client(active_client, address, client.session_id).await {
        return;
    }

    let bytes = match reassembler.push(&chunk.value) {
        Ok(Some(bytes)) => bytes,
        Ok(None) => return,
        Err(error) => {
            reassembler.reset();
            send_direct_error(&client.direct_response_tx, &format!("Invalid BLE frame: {error}")).await;
            return;
        }
    };

    let request = match decode_request(&bytes) {
        Ok(request) => request,
        Err(error) => {
            send_direct_error(&client.direct_response_tx, &format!("Error parsing request: {error}")).await;
            return;
        }
    };

    if let Some(response) = handle_client_configuration(&request, &client.configuration) {
        let _ = client.direct_response_tx.send(response).await;
        return;
    }

    if request_tx.send(request).await.is_err() {
        error!("Core request channel closed while handling BLE client {address}");
    }
}

async fn run_response_writer(
    writer: CharacteristicWriter,
    mut responses: ClientResponses,
    mut direct_response_rx: mpsc::Receiver<Response>,
    active_client: SharedActiveClient,
    address: Address,
    session_id: u64,
) {
    let mut message_id = 0_u16;

    loop {
        enum Next {
            Closed,
            Response(Response),
            Lagged(u64),
        }

        let next = tokio::select! {
            _ = writer.closed() => Next::Closed,
            response = direct_response_rx.recv() => match response {
                Some(response) => Next::Response(response),
                None => Next::Closed,
            },
            response = responses.recv() => match response {
                Ok(response) => Next::Response(response),
                Err(broadcast::error::RecvError::Lagged(count)) => Next::Lagged(count),
                Err(broadcast::error::RecvError::Closed) => Next::Closed,
            },
        };

        let response = match next {
            Next::Closed => break,
            Next::Response(response) => response,
            Next::Lagged(count) => Response::default().with_error(&format!(
                "BLE response stream skipped {count} updates; request ALL to resynchronise"
            )),
        };

        if let Err(error) = send_response(&writer, &response, message_id).await {
            warn!("BLE response stream to {address} failed: {error:#}");
            break;
        }
        message_id = message_id.wrapping_add(1);
    }

    let mut active = active_client.lock().await;
    if active
        .as_ref()
        .is_some_and(|client| client.address == address && client.session_id == session_id)
    {
        *active = None;
    }
    info!("BLE client disconnected: {address}");
}

async fn send_response(writer: &CharacteristicWriter, response: &Response, message_id: u16) -> Result<()> {
    let bytes = encode_response(response)?;
    let frames = encode_message(&bytes, message_id, writer.mtu())?;
    for frame in frames {
        writer.send(&frame).await?;
    }
    Ok(())
}

async fn send_direct_error(direct_response_tx: &mpsc::Sender<Response>, message: &str) {
    let _ = direct_response_tx.send(Response::default().with_error(message)).await;
}

async fn is_active_client(active_client: &SharedActiveClient, address: Address, session_id: u64) -> bool {
    active_client
        .lock()
        .await
        .as_ref()
        .is_some_and(|client| client.address == address && client.session_id == session_id)
}
