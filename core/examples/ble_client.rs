#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("The BLE verification client is only available on Linux");
}

#[cfg(target_os = "linux")]
mod linux {
    use std::{
        env, fs,
        io::{self, Write},
        path::PathBuf,
        time::Duration,
    };

    use anyhow::{anyhow, bail, Context, Result};
    use bloop::{
        ble::{
            framing::{encode_message, Reassembler},
            REQUEST_CHARACTERISTIC_UUID, RESPONSE_CHARACTERISTIC_UUID, SERVICE_UUID,
        },
        bloop::{ClientConfiguration, Entity, Request, Response},
    };
    use bluer::{
        gatt::remote::Characteristic, Adapter, AdapterEvent, Device, DiscoveryFilter, DiscoveryTransport, Uuid,
    };
    use futures::{pin_mut, StreamExt};
    use protobuf::Message;
    use tokio::time::timeout;

    struct Options {
        progress_rate: Option<u32>,
        request_path: Option<PathBuf>,
        output_path: Option<PathBuf>,
    }

    pub async fn run() -> Result<()> {
        let options = parse_options()?;
        let service_uuid = Uuid::parse_str(SERVICE_UUID)?;
        let request_uuid = Uuid::parse_str(REQUEST_CHARACTERISTIC_UUID)?;
        let response_uuid = Uuid::parse_str(RESPONSE_CHARACTERISTIC_UUID)?;

        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;
        adapter
            .set_discovery_filter(DiscoveryFilter {
                transport: DiscoveryTransport::Le,
                uuids: [service_uuid].into_iter().collect(),
                ..Default::default()
            })
            .await?;

        println!("Scanning for Bloop on {}…", adapter.name());
        let device = timeout(Duration::from_secs(30), find_device(&adapter, service_uuid))
            .await
            .context("Timed out while scanning for Bloop")??;
        if !device.is_connected().await? {
            device.connect().await?;
        }
        println!("Connected to {}", device.address());

        let (request_characteristic, response_characteristic) =
            find_characteristics(&device, service_uuid, request_uuid, response_uuid).await?;
        let response_reader = response_characteristic.notify_io().await?;
        println!("GATT ready (symmetric MTU {})", response_reader.mtu());

        let mut message_id = 0_u16;
        if let Some(progress_rate) = options.progress_rate {
            if progress_rate > 60 {
                bail!("--progress-hz must be between 0 and 60")
            }
            let request = Request {
                configure_client: Some(ClientConfiguration {
                    progress_updates_per_second: progress_rate,
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            };
            send_request(
                &request_characteristic,
                response_reader.mtu(),
                &request.write_to_bytes()?,
                message_id,
            )
            .await?;
            message_id = message_id.wrapping_add(1);
        }

        let request_bytes = match options.request_path {
            Some(path) => fs::read(&path).with_context(|| format!("Unable to read {}", path.display()))?,
            None => Request::get_request(Entity::ALL, 0).write_to_bytes()?,
        };
        Request::parse_from_bytes(&request_bytes).context("Request file does not contain a valid Bloop Request")?;
        send_request(
            &request_characteristic,
            response_reader.mtu(),
            &request_bytes,
            message_id,
        )
        .await?;

        let mut output = options
            .output_path
            .map(fs::File::create)
            .transpose()
            .context("Unable to create response output file")?;
        let mut reassembler = Reassembler::default();
        loop {
            let value = match timeout(Duration::from_secs(5), response_reader.recv()).await {
                Ok(Ok(value)) => value,
                Ok(Err(error)) => return Err(error.into()),
                Err(_) => break,
            };

            if let Some(bytes) = reassembler.push(&value)? {
                let response = Response::parse_from_bytes(&bytes)?;
                println!("{response:#?}");
                if let Some(output) = output.as_mut() {
                    output.write_all(&(bytes.len() as u32).to_le_bytes())?;
                    output.write_all(&bytes)?;
                }
            }
        }

        Ok(())
    }

    async fn find_device(adapter: &Adapter, service_uuid: Uuid) -> Result<Device> {
        for address in adapter.device_addresses().await? {
            let device = adapter.device(address)?;
            if device.uuids().await?.unwrap_or_default().contains(&service_uuid) {
                return Ok(device);
            }
        }

        let events = adapter.discover_devices().await?;
        pin_mut!(events);
        while let Some(event) = events.next().await {
            if let AdapterEvent::DeviceAdded(address) = event {
                let device = adapter.device(address)?;
                if device.uuids().await?.unwrap_or_default().contains(&service_uuid) {
                    return Ok(device);
                }
            }
        }
        Err(anyhow!("Bluetooth discovery ended before Bloop was found"))
    }

    async fn find_characteristics(
        device: &Device,
        service_uuid: Uuid,
        request_uuid: Uuid,
        response_uuid: Uuid,
    ) -> Result<(Characteristic, Characteristic)> {
        for service in device.services().await? {
            if service.uuid().await? != service_uuid {
                continue;
            }

            let mut request_characteristic = None;
            let mut response_characteristic = None;
            for characteristic in service.characteristics().await? {
                match characteristic.uuid().await? {
                    uuid if uuid == request_uuid => request_characteristic = Some(characteristic),
                    uuid if uuid == response_uuid => response_characteristic = Some(characteristic),
                    _ => {}
                }
            }
            return Ok((
                request_characteristic.context("Bloop request characteristic was not found")?,
                response_characteristic.context("Bloop response characteristic was not found")?,
            ));
        }
        Err(anyhow!("Bloop GATT service was not found after connecting"))
    }

    async fn send_request(
        characteristic: &Characteristic,
        maximum_value_length: usize,
        request: &[u8],
        message_id: u16,
    ) -> Result<()> {
        for frame in encode_message(request, message_id, maximum_value_length)? {
            characteristic.write(&frame).await?;
        }
        Ok(())
    }

    fn parse_options() -> Result<Options> {
        let mut progress_rate = None;
        let mut request_path = None;
        let mut output_path = None;
        let mut args = env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "get-all" => {}
                "--progress-hz" => {
                    let value = args.next().context("--progress-hz requires a value")?;
                    progress_rate = Some(value.parse().context("--progress-hz must be an integer")?);
                }
                "--request" => {
                    request_path = Some(PathBuf::from(args.next().context("--request requires a path")?));
                }
                "--output" => {
                    output_path = Some(PathBuf::from(args.next().context("--output requires a path")?));
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                _ => bail!("Unknown argument {arg}; use --help for usage"),
            }
        }

        Ok(Options {
            progress_rate,
            request_path,
            output_path,
        })
    }

    fn print_help() {
        let _ = writeln!(
            io::stdout(),
            "Usage: cargo run --example ble_client -- [get-all] [--progress-hz 0..60] [--request FILE] [--output FILE]"
        );
    }
}

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    linux::run().await
}
