use super::{directories::Directories, project_store::ProjectStore, waveform_store::WaveformStore};

use crate::{
    audio::AudioController,
    bloop::*,
    midi::MidiController,
    model::{Action, Project, Sample, Section, Tempo, INVALID_ID},
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
            project: Project::empty().with_songs(1, 1),
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

    async fn handle_request(&mut self, request: Request) -> anyhow::Result<()> {
        let mut project = self.project.clone();

        if let Some(add_request) = request.add.as_ref() {
            project = self.handle_add(project, add_request)?;
        }

        if let Some(get_request) = request.get.as_ref() {
            self.handle_get(get_request).await?;
        }

        if let Some(load_request) = request.load.as_ref() {
            self.handle_load(load_request).await?;
        }

        if let Some(remove_request) = request.remove.as_ref() {
            project = self.handle_remove(project, remove_request).await?;
        }

        if let Some(duplicate_request) = request.duplicate.as_ref() {
            project = self.handle_duplicate(project, duplicate_request).await?;
        }

        if let Some(remove_sample_request) = request.remove_sample.as_ref() {
            project = project.remove_sample_from_song(remove_sample_request.song_id)?;
        }

        if let Some(rename_request) = request.rename.as_ref() {
            project = self.handle_rename(project, rename_request)?;
        }

        if request.save.as_ref().is_some() {
            project = self.save_project(project).await?;
        }

        if let Some(select_request) = request.select.as_ref() {
            project = self.handle_select(project, select_request)?;
        }

        if let Some(transport_request) = request.transport.as_ref() {
            self.handle_transport_request(transport_request)?;
        }

        if let Some(update_request) = request.update.as_ref() {
            project = self.handle_update(project, update_request)?;
        }

        if let Some(begin_upload_request) = request.begin_upload.as_ref() {
            self.handle_begin_upload(begin_upload_request)?;
        }

        if let Some(upload_request) = request.upload.as_ref() {
            self.handle_upload(upload_request).await?;
        }

        if let Some(complete_upload_request) = request.complete_upload.as_ref() {
            self.handle_complete_upload(complete_upload_request)?;
        }

        if let Some(add_sample_request) = request.add_sample.as_ref() {
            project = self.handle_add_sample(add_sample_request, project)?;
        }

        if let Some(add_section_request) = request.add_section.as_ref() {
            project = self.handle_add_section_with_params(add_section_request, project)?;
        }

        if let Some(project_export_request) = request.project_export.as_ref() {
            project = self.handle_project_export(project, project_export_request).await?;
        }

        if let Some(project_import_request) = request.project_import.as_ref() {
            project = self.handle_project_import(project, project_import_request).await?;
        }

        self.set_project(project);
        Ok(())
    }

    async fn save_project(&mut self, project: Project) -> anyhow::Result<Project> {
        self.project_store
            .save(project.clone(), &self.samples_cache)
            .await
            .map(|_| project)
    }

    async fn handle_project_export(
        &mut self,
        project: Project,
        request: &ProjectExportRequest,
    ) -> anyhow::Result<Project> {
        let project_id = request.project_id;

        let (data, more_coming) = self.project_store.export(project_id).await?;

        self.send_response(Response::default().with_export_response(&ExportResponse {
            project_id,
            data,
            more_coming,
            ..Default::default()
        }));

        Ok(project)
    }

    async fn handle_project_import(
        &mut self,
        project: Project,
        request: &ProjectImportRequest,
    ) -> anyhow::Result<Project> {
        self.project_store
            .import(request.import_id, &request.data, request.more_coming)
            .await?;

        self.send_response(Response::default().with_import_response(&ImportResponse {
            import_id: request.import_id,
            ..Default::default()
        }));

        Ok(project)
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
        let entity = match get_request.entity.enum_value() {
            Ok(entity) => entity,
            Err(error) => {
                self.send_error_response(&format!("Invalid entity type: {error}"));
                return Ok(());
            }
        };

        match entity {
            Entity::ALL => self.send_response(
                Response::default()
                    .with_project(&self.project)
                    .with_playback_state(self.audio_controller.get_playback_state()),
            ),
            Entity::PROJECTS => {
                let projects = self.project_store.projects().await?;
                self.send_response(Response::default().with_projects(&projects));
            }
            Entity::WAVEFORM => {
                self.waveform_store.get_waveform(get_request.id, &self.samples_cache)?;
            }
            _ => (),
        };

        Ok(())
    }

    fn send_project_response(&self, project: &Project) {
        self.send_response(Response::default().with_project(project));
    }

    fn send_error_response(&self, message: &str) {
        error!("{}", message);
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
                Some(request) = self.request_rx.recv() => {
                    if let Err(error) = self.handle_request(request).await {
                        self.send_error_response(&error.to_string());
                    }
                }
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
        let song_id = self.project.selections.song;
        let section_id = self.project.selections.section;
        if song_id != INVALID_ID && section_id != INVALID_ID {
            self.audio_controller.queue(song_id, section_id);
        }
    }

    fn handle_add(&self, project: Project, request: &AddRequest) -> anyhow::Result<Project> {
        match request.entity.enum_value_or_default() {
            Entity::SECTION => self.handle_add_section(project, request),
            Entity::SONG => Ok(project.add_song(1)),
            Entity::PROJECT => Ok(Project::empty()),
            _ => Ok(project),
        }
    }

    fn handle_add_section(&self, project: Project, request: &AddRequest) -> anyhow::Result<Project> {
        if request.id == INVALID_ID {
            return Err(anyhow!("Missing parent ID"));
        }

        project.add_section_to_song(request.id)
    }

    fn handle_select(&self, project: Project, select_request: &SelectRequest) -> anyhow::Result<Project> {
        match select_request.entity.enum_value_or_default() {
            Entity::SONG => {
                info!("Select song request id={}", select_request.id);
                Ok(project.select_song_with_id(select_request.id))
            }
            Entity::SECTION => project.select_section(select_request.id),
            _ => Ok(project),
        }
    }

    async fn handle_remove(&self, project: Project, remove_request: &RemoveRequest) -> anyhow::Result<Project> {
        match remove_request.entity.enum_value_or_default() {
            Entity::SONG => project.remove_song(remove_request.id),
            Entity::SECTION => project.remove_section(remove_request.id),
            Entity::PROJECT => {
                self.project_store.remove_project(remove_request.id).await?;
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
        match duplicate_request.entity.enum_value() {
            Ok(Entity::PROJECT) => {
                let project = self
                    .project_store
                    .load(duplicate_request.id, &mut self.samples_cache)
                    .await?;
                let project = project.replace_ids();
                Ok(project)
            }
            Ok(_) => Ok(project),
            Err(error) => Err(anyhow!("Invalid entity type: {error}")),
        }
    }

    fn handle_update(&self, mut project: Project, update_request: &UpdateRequest) -> anyhow::Result<Project> {
        if let Some(song) = update_request.song.as_ref() {
            project = project.replace_song(song)?;
        }

        if let Some(section) = update_request.section.as_ref() {
            project = project.replace_section(section)?;
        }

        if let Some(sample) = update_request.sample.as_ref() {
            project = project.replace_sample(sample)?;
        }

        if let Some(new_project) = update_request.project.as_ref() {
            if !new_project.is_valid() {
                return Err(anyhow!("Invalid project"));
            }

            project = new_project.clone();
        }

        Ok(project)
    }

    fn handle_rename(&self, project: Project, rename_request: &RenameRequest) -> anyhow::Result<Project> {
        match rename_request.entity.enum_value() {
            Ok(Entity::PROJECT) => Ok(project.with_name(rename_request.name.clone())),
            Ok(_) => Ok(project),
            Err(error) => Err(anyhow!("Invalid entity type: {error}")),
        }
    }

    async fn handle_load(&mut self, request: &LoadRequest) -> anyhow::Result<Project> {
        self.project_store.load(request.id, &mut self.samples_cache).await
    }

    fn handle_begin_upload(&mut self, request: &BeginUploadRequest) -> anyhow::Result<()> {
        info!("Upload started {}", request.upload_id);
        self.samples_cache.begin_upload(
            request.upload_id,
            request.format.enum_value_or_default(),
            &request.filename,
        );
        self.send_response(Response::default().with_upload_ack(&UploadAck {
            upload_id: request.upload_id,
            ..Default::default()
        }));
        Ok(())
    }

    fn handle_complete_upload(&mut self, request: &CompleteUploadRequest) -> anyhow::Result<()> {
        info!("Upload complete {}", request.upload_id);
        self.samples_cache.complete_upload(request.upload_id)?;
        self.send_response(Response::default().with_upload_ack(&UploadAck {
            upload_id: request.upload_id,
            ..Default::default()
        }));
        Ok(())
    }

    async fn handle_upload(&mut self, request: &UploadRequest) -> anyhow::Result<()> {
        self.samples_cache.upload(request.upload_id, &request.data).await?;
        self.send_response(Response::default().with_upload_ack(&UploadAck {
            upload_id: request.upload_id,
            ..Default::default()
        }));
        Ok(())
    }

    fn handle_add_sample(&mut self, request: &AddSampleRequest, mut project: Project) -> anyhow::Result<Project> {
        let sample_metadata = self.samples_cache.get_sample_metadata(request.upload_id)?;

        let mut sample = Sample::new_with_id(&request.upload_id);
        sample.name = sample_metadata.name;
        sample.sample_rate = sample_metadata.sample_rate as i32;
        sample.channel_count = sample_metadata.num_channels as i32;
        sample.sample_count = sample_metadata.sample_count as i64;

        if let Some(tempo) = sample_metadata.detected_tempo {
            sample.tempo = Some(Tempo::new_with_bpm(tempo)).into();
        }

        project = project.add_sample_to_song(sample, request.song_id)?;
        Ok(project)
    }

    fn handle_add_section_with_params(
        &mut self,
        request: &AddSectionRequest,
        mut project: Project,
    ) -> anyhow::Result<Project> {
        let section = Section::empty()
            .with_name(request.name.clone())
            .with_start(request.start)
            .with_loop(request.loop_)
            .with_metronome(request.metronome);

        let song = project
            .song_with_id_mut(request.song_id)
            .ok_or_else(|| anyhow!("Song not found"))?;

        song.sections.push(section);

        song.sections.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

        Ok(project)
    }

    fn handle_transport_request(&mut self, transport_request: &TransportRequest) -> anyhow::Result<()> {
        match transport_request.method.enum_value() {
            Ok(TransportMethod::PLAY) => self.audio_controller.play(),
            Ok(TransportMethod::STOP) => self.audio_controller.stop(),
            Ok(TransportMethod::LOOP) => self.audio_controller.enter_loop(),
            Ok(TransportMethod::EXIT_LOOP) => self.audio_controller.exit_loop(),
            Ok(TransportMethod::QUEUE) => self
                .audio_controller
                .queue(transport_request.queue.song_id, transport_request.queue.section_id),
            Err(error) => {
                return Err(anyhow!("Invalid transport method: {error}"));
            }
        }

        Ok(())
    }
}
