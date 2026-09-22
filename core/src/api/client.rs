use std::{future::pending, time::Duration};

use tokio::{
    sync::{broadcast, watch},
    time::{sleep_until, Instant},
};

use crate::bloop::{ClientConfiguration, Progress, Request, Response};

pub const DEFAULT_PROGRESS_UPDATES_PER_SECOND: u32 = 10;
pub const MAX_PROGRESS_UPDATES_PER_SECOND: u32 = 60;

#[derive(Clone)]
pub struct ClientConfigurationHandle {
    progress_rate_tx: watch::Sender<u32>,
}

impl ClientConfigurationHandle {
    pub fn progress_updates_per_second(&self) -> u32 {
        *self.progress_rate_tx.borrow()
    }

    fn set_progress_updates_per_second(&self, rate: u32) {
        self.progress_rate_tx.send_replace(rate);
    }
}

pub struct ClientResponses {
    response_rx: broadcast::Receiver<Response>,
    progress_rate_rx: watch::Receiver<u32>,
    pending_progress: Option<Progress>,
    next_progress_at: Option<Instant>,
    configuration_closed: bool,
}

impl ClientResponses {
    pub async fn recv(&mut self) -> Result<Response, broadcast::error::RecvError> {
        loop {
            let progress_deadline = self.next_progress_at;

            tokio::select! {
                result = self.response_rx.recv() => {
                    let response = result?;
                    if let Some(response) = self.on_response(response) {
                        return Ok(response);
                    }
                }
                changed = self.progress_rate_rx.changed(), if !self.configuration_closed => {
                    if changed.is_err() {
                        self.configuration_closed = true;
                    } else {
                        self.reset_progress_deadline();
                    }
                }
                _ = wait_for_deadline(progress_deadline) => {
                    self.advance_progress_deadline();
                    if let Some(progress) = self.pending_progress.take() {
                        return Ok(Response::default().with_progress(&progress));
                    }
                }
            }
        }
    }

    fn on_response(&mut self, mut response: Response) -> Option<Response> {
        let progress = response.progress.take();
        let Some(progress) = progress else {
            return Some(response);
        };

        if is_terminal_progress(&progress) {
            self.pending_progress = None;
            response.progress = Some(progress).into();
            return Some(response);
        }

        self.pending_progress = Some(progress);

        if response == Response::default() {
            None
        } else {
            Some(response)
        }
    }

    fn progress_rate(&self) -> u32 {
        *self.progress_rate_rx.borrow()
    }

    fn reset_progress_deadline(&mut self) {
        let rate = self.progress_rate();
        self.next_progress_at = progress_period(rate).map(|period| Instant::now() + period);
    }

    fn advance_progress_deadline(&mut self) {
        let rate = self.progress_rate();
        self.next_progress_at = progress_period(rate).map(|period| Instant::now() + period);
    }
}

pub fn create_client_responses(
    response_rx: broadcast::Receiver<Response>,
) -> (ClientConfigurationHandle, ClientResponses) {
    let (progress_rate_tx, progress_rate_rx) = watch::channel(DEFAULT_PROGRESS_UPDATES_PER_SECOND);
    let next_progress_at = progress_period(DEFAULT_PROGRESS_UPDATES_PER_SECOND).map(|period| Instant::now() + period);

    (
        ClientConfigurationHandle { progress_rate_tx },
        ClientResponses {
            response_rx,
            progress_rate_rx,
            pending_progress: None,
            next_progress_at,
            configuration_closed: false,
        },
    )
}

pub fn handle_client_configuration(request: &Request, configuration: &ClientConfigurationHandle) -> Option<Response> {
    let requested = request.configure_client.as_ref()?;

    let mut remaining_request = request.clone();
    remaining_request.configure_client = None.into();
    if remaining_request != Request::default() {
        return Some(Response::default().with_error("Client configuration must be a standalone request"));
    }

    let rate = requested.progress_updates_per_second;
    if rate > MAX_PROGRESS_UPDATES_PER_SECOND {
        return Some(Response::default().with_error("Progress update rate must be between 0 and 60 Hz"));
    }

    configuration.set_progress_updates_per_second(rate);
    let effective_rate = configuration.progress_updates_per_second();
    Some(Response::default().with_client_configuration(&ClientConfiguration {
        progress_updates_per_second: effective_rate,
        ..Default::default()
    }))
}

fn progress_period(rate: u32) -> Option<Duration> {
    if rate == 0 {
        None
    } else {
        Some(Duration::from_secs_f64(1.0 / f64::from(rate)))
    }
}

fn is_terminal_progress(progress: &Progress) -> bool {
    progress.song_progress == 0.0 && progress.section_progress == 0.0 && progress.section_beat == 0.0
}

async fn wait_for_deadline(deadline: Option<Instant>) {
    match deadline {
        Some(deadline) => sleep_until(deadline).await,
        None => pending::<()>().await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::timeout;

    fn progress(value: f64) -> Response {
        Response::default().with_progress(&Progress {
            song_progress: value,
            section_progress: value,
            section_beat: value,
            ..Default::default()
        })
    }

    fn configure(configuration: &ClientConfigurationHandle, rate: u32) -> Response {
        handle_client_configuration(
            &Request {
                configure_client: Some(ClientConfiguration {
                    progress_updates_per_second: rate,
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            },
            configuration,
        )
        .unwrap()
    }

    #[test]
    fn client_configuration_defaults_to_ten_hz() {
        let (response_tx, _) = broadcast::channel(8);
        let (configuration, _) = create_client_responses(response_tx.subscribe());
        assert_eq!(
            configuration.progress_updates_per_second(),
            DEFAULT_PROGRESS_UPDATES_PER_SECOND
        );
    }

    #[test]
    fn client_configuration_is_local_and_validated() {
        let (response_tx, _) = broadcast::channel(8);
        let (first, _) = create_client_responses(response_tx.subscribe());
        let (second, _) = create_client_responses(response_tx.subscribe());

        let request = Request {
            configure_client: Some(ClientConfiguration {
                progress_updates_per_second: 60,
                ..Default::default()
            })
            .into(),
            ..Default::default()
        };
        let response = handle_client_configuration(&request, &first).unwrap();

        assert_eq!(first.progress_updates_per_second(), 60);
        assert_eq!(second.progress_updates_per_second(), 10);
        assert_eq!(
            response
                .client_configuration
                .as_ref()
                .unwrap()
                .progress_updates_per_second,
            60
        );
    }

    #[test]
    fn invalid_rate_does_not_change_configuration() {
        let (response_tx, _) = broadcast::channel(8);
        let (configuration, _) = create_client_responses(response_tx.subscribe());
        let request = Request {
            configure_client: Some(ClientConfiguration {
                progress_updates_per_second: 61,
                ..Default::default()
            })
            .into(),
            ..Default::default()
        };

        let response = handle_client_configuration(&request, &configuration).unwrap();

        assert!(!response.error.is_empty());
        assert_eq!(configuration.progress_updates_per_second(), 10);
    }

    #[test]
    fn accepts_supported_progress_rates() {
        let (response_tx, _) = broadcast::channel(8);
        let (configuration, _) = create_client_responses(response_tx.subscribe());

        for rate in [0, 1, 10, 30, 60] {
            let request = Request {
                configure_client: Some(ClientConfiguration {
                    progress_updates_per_second: rate,
                    ..Default::default()
                })
                .into(),
                ..Default::default()
            };

            let response = handle_client_configuration(&request, &configuration).unwrap();
            assert_eq!(
                response
                    .client_configuration
                    .as_ref()
                    .unwrap()
                    .progress_updates_per_second,
                rate
            );
        }
    }

    #[test]
    fn rejects_configuration_combined_with_a_core_request() {
        let (response_tx, _) = broadcast::channel(8);
        let (configuration, _) = create_client_responses(response_tx.subscribe());
        let mut request = Request::get_request(crate::bloop::Entity::ALL, 0);
        request.configure_client = Some(ClientConfiguration {
            progress_updates_per_second: 30,
            ..Default::default()
        })
        .into();

        let response = handle_client_configuration(&request, &configuration).unwrap();

        assert!(!response.error.is_empty());
        assert_eq!(configuration.progress_updates_per_second(), 10);
    }

    #[tokio::test]
    async fn zero_rate_still_delivers_terminal_progress() {
        let (response_tx, _) = broadcast::channel(8);
        let (configuration, mut responses) = create_client_responses(response_tx.subscribe());
        configure(&configuration, 0);

        response_tx.send(progress(0.5)).unwrap();
        assert!(timeout(Duration::from_millis(50), responses.recv()).await.is_err());
        response_tx.send(progress(0.0)).unwrap();

        let response = responses.recv().await.unwrap();
        assert_eq!(response.progress.as_ref().unwrap().song_progress, 0.0);
    }

    #[tokio::test]
    async fn samples_supported_rates_and_keeps_only_the_newest_progress() {
        for rate in [1, 10, 30, 60] {
            let (response_tx, _) = broadcast::channel(8);
            let (configuration, mut responses) = create_client_responses(response_tx.subscribe());
            configure(&configuration, rate);

            response_tx.send(progress(0.25)).unwrap();
            response_tx.send(progress(0.75)).unwrap();

            let period = progress_period(rate).unwrap();
            let started = Instant::now();
            let response = timeout(period * 2 + Duration::from_millis(100), responses.recv())
                .await
                .expect("sampled progress was not delivered")
                .unwrap();

            assert!(started.elapsed() >= period / 2);
            assert_eq!(response.progress.as_ref().unwrap().song_progress, 0.75);
        }
    }

    #[tokio::test]
    async fn applies_runtime_rate_changes_without_reconnecting() {
        let (response_tx, _) = broadcast::channel(8);
        let (configuration, mut responses) = create_client_responses(response_tx.subscribe());
        configure(&configuration, 0);
        response_tx.send(progress(0.25)).unwrap();
        assert!(timeout(Duration::from_millis(50), responses.recv()).await.is_err());

        configure(&configuration, 60);
        response_tx.send(progress(0.75)).unwrap();
        let response = timeout(Duration::from_millis(100), responses.recv())
            .await
            .expect("progress was not delivered after enabling updates")
            .unwrap();

        assert_eq!(response.progress.as_ref().unwrap().song_progress, 0.75);
    }

    #[tokio::test]
    async fn forwards_non_progress_fields_immediately_and_samples_progress_separately() {
        let (response_tx, _) = broadcast::channel(8);
        let (_, mut responses) = create_client_responses(response_tx.subscribe());
        let mut combined = progress(0.5);
        combined.error = "immediate".to_string();
        response_tx.send(combined).unwrap();

        let immediate = timeout(Duration::from_millis(20), responses.recv())
            .await
            .expect("non-progress fields were not forwarded immediately")
            .unwrap();
        assert_eq!(immediate.error, "immediate");
        assert!(immediate.progress.is_none());

        let sampled = timeout(Duration::from_millis(150), responses.recv())
            .await
            .expect("separated progress was not sampled")
            .unwrap();
        assert_eq!(sampled.progress.as_ref().unwrap().song_progress, 0.5);
    }
}
