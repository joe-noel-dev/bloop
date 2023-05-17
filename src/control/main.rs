use super::{directories::Directories, project_store::ProjectStore, waveform_store::WaveformStore};
use crate::{
    api::*,
    audio::{manager::Audio, manager::AudioManager},
    midi::{Action, MidiManager},
    model::{Project, Sample, Tempo},
    samples::SamplesCache,
};
use anyhow::anyhow;
use tokio::sync::{broadcast, mpsc};

pub struct MainController {
    samples_cache: SamplesCache,
    project_store: ProjectStore,
    request_rx: mpsc::Receiver<Request>,
    response_tx: broadcast::Sender<Response>,
    project: Project,
    audio_manager: AudioManager,
    waveform_store: WaveformStore,
    _midi_manager: MidiManager,
    midi_action_rx: mpsc::Receiver<Action>,
}

impl ResponseBroadcaster for MainController {
    fn broadcast(&self, response: Response) {
        self.send_response(response);
    }
}

impl MainController {
    pub fn new(request_rx: mpsc::Receiver<Request>, response_tx: broadcast::Sender<Response>) -> Self {
        let directories = Directories::new();
        let samples_cache = SamplesCache::new(&directories.samples);
        let project_store = ProjectStore::new(&directories.projects);
        let waveform_store = WaveformStore::new(response_tx.clone());
        let audio_manager = AudioManager::new(response_tx.clone(), &directories.preferences);
        let (midi_action_tx, midi_action_rx) = mpsc::channel(128);
        let midi_manager = MidiManager::new(midi_action_tx);

        Self {
            samples_cache,
            project_store,
            request_rx,
            response_tx,
            project: Project::new(),
            audio_manager,
            waveform_store,
            _midi_manager: midi_manager,
            midi_action_rx,
        }
    }

    async fn handle_request(&mut self, request: Request) {
        let project = self.project.clone();
        let result = match request {
            Request::Add(add_request) => self.handle_add(project, &add_request),
            Request::Get(get_request) => self.handle_get(&get_request).await.map(|_| project),
            Request::Load(load_request) => self.handle_load(&load_request).await,
            Request::Remove(remove_request) => self.handle_remove(project, &remove_request).await,
            Request::RemoveSample(remove_request) => {
                project.remove_sample(&remove_request.sample_id, &remove_request.song_id)
            }
            Request::Rename(rename_request) => self.handle_rename(project, &rename_request),
            Request::Save => self
                .project_store
                .save(project.clone(), &self.samples_cache)
                .await
                .map(|_| project),
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
        };

        match result {
            Ok(project) => self.set_project(project),
            Err(error) => self.send_error_response(&error.to_string()),
        }
    }

    fn set_project(&mut self, project: Project) {
        if self.project != project {
            self.project = project;
            self.send_project_response(&self.project);
            self.audio_manager
                .on_project_updated(&self.project, &self.samples_cache);
        }
    }

    async fn handle_get(&mut self, get_request: &GetRequest) -> anyhow::Result<()> {
        match get_request.entity {
            Entity::All => self.send_response(
                Response::default()
                    .with_project(&self.project)
                    .with_playback_state(self.audio_manager.playback_state()),
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

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(request) = self.request_rx.recv() => self.handle_request(request).await,
                _ = self.audio_manager.run() => (),
                Some(midi_action) = self.midi_action_rx.recv() => self.handle_midi_action(midi_action),
                else => break,
            }
        }
    }

    fn handle_midi_action(&mut self, action: Action) {
        match action {
            Action::PreviousSong => self.previous_song(),
            Action::NextSong => self.next_song(),
            Action::PreviousSection => self.previous_section(),
            Action::NextSection => self.next_section(),
            Action::QueueSelected => self.queue_selected(),
            Action::ToggleLoop => self.audio_manager.toggle_loop(),
            Action::TogglePlay => self.audio_manager.toggle_play(),
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
                self.audio_manager.queue(&song_id, &section_id);
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

    fn handle_update(&self, project: Project, update_request: &UpdateRequest) -> anyhow::Result<Project> {
        match update_request {
            UpdateRequest::Song(song) => project.replace_song(song),
            UpdateRequest::Section(section) => project.replace_section(section),
            UpdateRequest::Sample(sample) => project.replace_sample(sample),
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
        println!("Upload started {}", request.upload_id);
        self.samples_cache
            .begin_upload(&request.upload_id, &request.format, &request.filename);
        self.send_response(Response::default().with_upload_ack(UploadAck {
            upload_id: request.upload_id,
        }));
        Ok(())
    }

    fn handle_complete_upload(&mut self, request: CompleteUploadRequest) -> anyhow::Result<()> {
        println!("Upload complete {}", request.upload_id);
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

    fn handle_transport_request(&mut self, transport_method: &TransportMethod) {
        match transport_method {
            TransportMethod::Play => self.audio_manager.play(),
            TransportMethod::Stop => self.audio_manager.stop(),
            TransportMethod::Loop => self.audio_manager.enter_loop(),
            TransportMethod::ExitLoop => self.audio_manager.exit_loop(),
            TransportMethod::Queue(queue_request) => self
                .audio_manager
                .queue(&queue_request.song_id, &queue_request.section_id),
        }
    }
}
