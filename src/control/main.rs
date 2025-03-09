use super::{directories::Directories, project_store::ProjectStore, waveform_store::WaveformStore};

use crate::{
    api::*,
    audio::AudioController,
    midi::MidiController,
    model::{Action, Project, Sample, Section, Tempo},
    preferences::{read_preferences, Preferences},
    samples::SamplesCache,
    switch,
};

use anyhow::anyhow;
use log::{error, info, warn};
use std::time::Duration;
use tokio::{
    sync::{broadcast, mpsc},
    time,
};

pub async fn run_main_controller(request_rx: mpsc::Receiver<Request>, response_tx: broadcast::Sender<Response>) {
    let mut main_controller = MainController::new(request_rx, response_tx.clone());
    main_controller.load_last_project().await;
    main_controller.run().await;
}

struct MainController {
    samples_cache: SamplesCache,
    project_store: ProjectStore,
    request_rx: mpsc::Receiver<Request>,
    response_tx: broadcast::Sender<Response>,
    project: Project,
    audio_controller: AudioController,
    waveform_store: WaveformStore,
    _midi_controller: MidiController,
    action_rx: mpsc::Receiver<Action>,
    action_tx: mpsc::Sender<Action>,
    should_save: bool,
    preferences: Preferences,
}

impl ResponseBroadcaster for MainController {
    fn broadcast(&self, response: Response) {
        self.send_response(response);
    }
}

impl MainController {
    pub fn new(request_rx: mpsc::Receiver<Request>, response_tx: broadcast::Sender<Response>) -> Self {
        let directories = Directories::new();

        let (action_tx, action_rx) = mpsc::channel(128);

        let preferences = match read_preferences(&directories.root) {
            Ok(preferences) => {
                info!("Preferences loaded: {preferences:#?}");
                preferences
            }
            Err(error) => {
                warn!("Unable to read preferences, using default: {error}");
                Default::default()
            }
        };

        let audio_preferences = preferences.clone().audio.unwrap_or_default();
        let midi_preferences = preferences.clone().midi.unwrap_or_default();

        Self {
            samples_cache: SamplesCache::new(&directories.samples),
            project_store: ProjectStore::new(&directories.projects),
            request_rx,
            response_tx: response_tx.clone(),
            project: Project::new(),
            audio_controller: AudioController::new(response_tx.clone(), audio_preferences),
            waveform_store: WaveformStore::new(response_tx),
            _midi_controller: MidiController::new(action_tx.clone(), midi_preferences),
            action_rx,
            action_tx,
            should_save: false,
            preferences,
        }
    }

    pub async fn load_last_project(&mut self) {
        match self.project_store.load_last_project(&mut self.samples_cache).await {
            Ok(project) => self.set_project(project),
            Err(error) => error!("Unable to open last project: {error}"),
        };
    }

    async fn handle_request(&mut self, request: Request) {
        let project = self.project.clone();
        let result = match request {
            Request::Add(add_request) => self.handle_add(project, &add_request),
            Request::Get(get_request) => self.handle_get(&get_request).await.map(|_| project),
            Request::Load(load_request) => self.handle_load(&load_request).await,
            Request::Remove(remove_request) => self.handle_remove(project, &remove_request).await,
            Request::Duplicate(duplicate_request) => self.handle_duplicate(project, &duplicate_request).await,
            Request::RemoveSample(remove_request) => project.remove_sample_from_song(&remove_request.song_id),
            Request::Rename(rename_request) => self.handle_rename(project, &rename_request),
            Request::Save => self.save_project(project).await,
            Request::Select(select_request) => self.handle_select(project, &select_request),
            Request::Transport(transport_method) => {
                self.handle_transport_request(&transport_method);
                Ok(project)
            }
            Request::Update(update_request) => self.handle_update(project, &update_request),
            Request::BeginUpload(begin_upload_request) => {
                self.handle_begin_upload(begin_upload_request).map(|_| project)
            }
            Request::Upload(upload_request) => self.handle_upload(upload_request).await.map(|_| project),
            Request::CompleteUpload(complete_upload_request) => {
                self.handle_complete_upload(complete_upload_request).map(|_| project)
            }
            Request::AddSample(add_sample_request) => self.handle_add_sample(add_sample_request, project),
            Request::AddSection(add_section_request) => {
                self.handle_add_section_with_params(add_section_request, project)
            }
        };

        match result {
            Ok(project) => self.set_project(project),
            Err(error) => self.send_error_response(&error.to_string()),
        }
    }

    async fn save_project(&mut self, project: Project) -> anyhow::Result<Project> {
        self.project_store
            .save(project.clone(), &self.samples_cache)
            .await
            .map(|_| project)
    }

    fn set_project(&mut self, project: Project) {
        if self.project != project {
            self.should_save = project.info.id == self.project.info.id;
            self.project = project;
            self.send_project_response(&self.project);
            self.audio_controller
                .on_project_updated(&self.project, &self.samples_cache);
        }
    }

    async fn handle_get(&mut self, get_request: &GetRequest) -> anyhow::Result<()> {
        match get_request.entity {
            Entity::All => self.send_response(
                Response::default()
                    .with_project(&self.project)
                    .with_playback_state(self.audio_controller.get_playback_state()),
            ),
            Entity::Projects => {
                let projects = self.project_store.projects().await?;
                self.send_response(Response::default().with_projects(&projects));
            }
            Entity::Waveform => {
                let sample_id = get_request.id.expect("Missing sample ID in waveform request");
                self.waveform_store.get_waveform(&sample_id, &self.samples_cache)?;
            }
            _ => (),
        };

        Ok(())
    }

    fn send_project_response(&self, project: &Project) {
        self.send_response(Response::default().with_project(project));
    }

    fn send_error_response(&self, message: &str) {
        self.send_response(Response::default().with_error(message));
    }

    fn send_response(&self, response: Response) {
        let _ = self.response_tx.send(response);
    }

    async fn auto_save_project(&mut self) {
        if self.should_save && self.save_project(self.project.clone()).await.is_ok() {
            self.should_save = false
        }
    }

    pub async fn run(&mut self) {
        let switch_preferences = self.preferences.clone().switch.unwrap_or_default();
        let switch_task = switch::run(self.action_tx.clone(), switch_preferences);

        let mut save_interval = time::interval(Duration::from_secs(2));

        loop {
            tokio::select! {
                Some(request) = self.request_rx.recv() => self.handle_request(request).await,
                _ = self.audio_controller.run() => (),
                Some(action) = self.action_rx.recv() => self.handle_action(action),
                _ = save_interval.tick() => self.auto_save_project().await,
                else => break,
            }
        }

        drop(switch_task);
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::PreviousSong => self.previous_song(),
            Action::NextSong => self.next_song(),
            Action::PreviousSection => self.previous_section(),
            Action::NextSection => self.next_section(),
            Action::QueueSelected => self.queue_selected(),
            Action::ToggleLoop => self.audio_controller.toggle_loop(),
            Action::TogglePlay => self.audio_controller.toggle_play(),
        }
    }

    fn previous_song(&mut self) {
        let mut project = self.project.clone();
        project = project.select_previous_song();
        self.set_project(project);
    }

    fn next_song(&mut self) {
        let mut project = self.project.clone();
        project = project.select_next_song();
        self.set_project(project);
    }

    fn previous_section(&mut self) {
        let project = self.project.clone();
        if let Ok(project) = project.select_previous_section() {
            self.set_project(project);
        }
    }

    fn next_section(&mut self) {
        let project = self.project.clone();
        if let Ok(project) = project.select_next_section() {
            self.set_project(project);
        }
    }

    fn queue_selected(&mut self) {
        if let Some(song_id) = self.project.selections.song {
            if let Some(section_id) = self.project.selections.section {
                self.audio_controller.queue(&song_id, &section_id);
            }
        }
    }

    fn handle_add(&self, project: Project, request: &AddRequest) -> anyhow::Result<Project> {
        match request.entity {
            Entity::Section => self.handle_add_section(project, request),
            Entity::Song => Ok(project.add_song(1)),
            Entity::Project => Ok(Project::new()),
            _ => Ok(project),
        }
    }

    fn handle_add_section(&self, project: Project, request: &AddRequest) -> anyhow::Result<Project> {
        let song_id = request.id.ok_or_else(|| anyhow!("Missing parent ID"))?;
        project.add_section_to_song(&song_id)
    }

    fn handle_select(&self, project: Project, select_request: &SelectRequest) -> anyhow::Result<Project> {
        match select_request.entity {
            Entity::Song => Ok(project.select_song_with_id(&select_request.id)),
            Entity::Section => project.select_section(&select_request.id),
            _ => Ok(project),
        }
    }

    async fn handle_remove(&self, project: Project, remove_request: &RemoveRequest) -> anyhow::Result<Project> {
        match remove_request.entity {
            Entity::Song => project.remove_song(&remove_request.id),
            Entity::Section => project.remove_section(&remove_request.id),
            Entity::Project => {
                self.project_store.remove_project(&remove_request.id).await?;
                let projects = self.project_store.projects().await?;
                self.send_response(Response::default().with_projects(&projects));
                Ok(project)
            }
            _ => Ok(project),
        }
    }

    async fn handle_duplicate(
        &mut self,
        project: Project,
        duplicate_request: &DuplicateRequest,
    ) -> anyhow::Result<Project> {
        match duplicate_request.entity {
            Entity::Project => {
                let project = self
                    .project_store
                    .load(&duplicate_request.id, &mut self.samples_cache)
                    .await?;
                let project = project.replace_ids();
                Ok(project)
            }
            _ => Ok(project),
        }
    }

    fn handle_update(&self, project: Project, update_request: &UpdateRequest) -> anyhow::Result<Project> {
        match update_request {
            UpdateRequest::Song(song) => project.replace_song(song),
            UpdateRequest::Section(section) => project.replace_section(section),
            UpdateRequest::Sample(sample) => project.replace_sample(sample),
            UpdateRequest::Project(new_project) => {
                if !new_project.is_valid() {
                    return Err(anyhow!("Invalid project"));
                }

                Ok(new_project.clone())
            }
        }
    }

    fn handle_rename(&self, project: Project, rename_request: &RenameRequest) -> anyhow::Result<Project> {
        match rename_request.entity {
            Entity::Project => Ok(project.with_name(&rename_request.name)),
            _ => Ok(project),
        }
    }

    async fn handle_load(&mut self, request: &LoadRequest) -> anyhow::Result<Project> {
        self.project_store.load(&request.id, &mut self.samples_cache).await
    }

    fn handle_begin_upload(&mut self, request: BeginUploadRequest) -> anyhow::Result<()> {
        info!("Upload started {}", request.upload_id);
        self.samples_cache
            .begin_upload(&request.upload_id, &request.format, &request.filename);
        self.send_response(Response::default().with_upload_ack(UploadAck {
            upload_id: request.upload_id,
        }));
        Ok(())
    }

    fn handle_complete_upload(&mut self, request: CompleteUploadRequest) -> anyhow::Result<()> {
        info!("Upload complete {}", request.upload_id);
        self.samples_cache.complete_upload(&request.upload_id)?;
        self.send_response(Response::default().with_upload_ack(UploadAck {
            upload_id: request.upload_id,
        }));
        Ok(())
    }

    async fn handle_upload(&mut self, request: UploadRequest) -> anyhow::Result<()> {
        self.samples_cache.upload(&request.upload_id, &request.data).await?;
        self.send_response(Response::default().with_upload_ack(UploadAck {
            upload_id: request.upload_id,
        }));
        Ok(())
    }

    fn handle_add_sample(&mut self, request: AddSampleRequest, mut project: Project) -> anyhow::Result<Project> {
        let sample_metadata = self.samples_cache.get_sample_metadata(&request.upload_id)?;

        let mut sample = Sample::new_with_id(&request.upload_id);
        sample.name = sample_metadata.name;
        sample.sample_rate = sample_metadata.sample_rate as i32;
        sample.channel_count = sample_metadata.num_channels as i32;
        sample.sample_count = sample_metadata.sample_count as i64;

        if let Some(tempo) = sample_metadata.detected_tempo {
            sample.tempo = Tempo::new(tempo);
        }

        project = project.add_sample_to_song(sample, &request.song_id)?;
        Ok(project)
    }

    fn handle_add_section_with_params(
        &mut self,
        request: AddSectionRequest,
        mut project: Project,
    ) -> anyhow::Result<Project> {
        let section = Section::default()
            .with_name(request.name)
            .with_start(request.start)
            .with_looping(request.looping)
            .with_metronome(request.metronome);

        let song = project
            .song_with_id_mut(&request.song_id)
            .ok_or_else(|| anyhow!("Song not found"))?;

        song.sections.push(section);

        song.sections.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

        Ok(project)
    }

    fn handle_transport_request(&mut self, transport_method: &TransportMethod) {
        match transport_method {
            TransportMethod::Play => self.audio_controller.play(),
            TransportMethod::Stop => self.audio_controller.stop(),
            TransportMethod::Loop => self.audio_controller.enter_loop(),
            TransportMethod::ExitLoop => self.audio_controller.exit_loop(),
            TransportMethod::Queue(queue_request) => self
                .audio_controller
                .queue(&queue_request.song_id, &queue_request.section_id),
        }
    }
}
