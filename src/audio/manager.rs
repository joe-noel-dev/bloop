use super::{command::Command, notification::Notification, process::Process};
use crate::{
    api::response::Response,
    audio::command::QueueCommand,
    model::{id::ID, project::Project},
};
use futures::StreamExt;
use futures_channel::mpsc;
use tokio::sync::broadcast;

pub trait Audio {
    fn play(&mut self);
    fn stop(&mut self);
    fn enter_loop(&mut self);
    fn exit_loop(&mut self);
    fn queue(&mut self, song_id: &ID, section_id: &ID);
    fn queue_selected(&mut self);
}

pub struct AudioManager {
    _process: Process,
    command_tx: mpsc::Sender<Command>,
    notification_rx: mpsc::Receiver<Notification>,
    response_tx: broadcast::Sender<Response>,
}

impl AudioManager {
    pub fn new(response_tx: broadcast::Sender<Response>) -> Self {
        let (command_tx, command_rx) = mpsc::channel(128);
        let (notification_tx, notification_rx) = futures_channel::mpsc::channel(128);

        Self {
            _process: Process::new(command_rx, notification_tx),
            command_tx,
            notification_rx,
            response_tx,
        }
    }

    pub async fn run(&mut self) {
        loop {
            let notification = match self.notification_rx.next().await {
                Some(notification) => notification,
                None => return,
            };

            self.on_notification(notification);
        }
    }

    pub fn on_notification(&self, notification: Notification) {
        match notification {
            Notification::ReturnProject(_) => (/* Project is dropped here */),
            Notification::ReturnSample(_) => (/* Sample is dropped here */),
            Notification::Transport(playback_state) => {
                self.response_tx
                    .send(Response::new().with_playback_state(playback_state))
                    .unwrap();
            }
        }
    }

    pub fn on_project_updated(&mut self, project: &Project) {
        self.send(Command::UpdateProject(Box::new(project.clone())));
    }

    fn send(&mut self, command: Command) {
        self.command_tx.try_send(command).unwrap();
    }
}

impl Audio for AudioManager {
    fn play(&mut self) {
        self.send(Command::Play);
    }

    fn stop(&mut self) {
        self.send(Command::Stop);
    }

    fn enter_loop(&mut self) {
        self.send(Command::EnterLoop);
    }

    fn exit_loop(&mut self) {
        self.send(Command::ExitLoop);
    }

    fn queue(&mut self, song_id: &ID, section_id: &ID) {
        self.send(Command::Queue(QueueCommand {
            song_id: *song_id,
            section_id: *section_id,
        }));
    }

    fn queue_selected(&mut self) {}
}
