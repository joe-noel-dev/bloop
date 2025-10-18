use super::{directories::Directories, project_store::ProjectStore, waveform_store::WaveformStore};

use crate::{
    audio::AudioController,
    backend::{create_filesystem_backend, create_pocketbase_auth, create_pocketbase_backend, sync_project, Backend},
    bloop::*,
    config::AppConfig,
    control::user_store::UserStore,
    midi::MidiController,
    model::{Action, Project, Sample, Section, Tempo, INVALID_ID},
    preferences::{default_audio_preferences, default_midi_preferences, default_preferences, read_preferences},
    samples::SamplesCache,
    switch,
};

use anyhow::anyhow;
use log::{error, info, warn};
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{broadcast, mpsc},
    time,
};

pub async fn run_main_controller(
    request_rx: mpsc::Receiver<Request>,
    response_tx: broadcast::Sender<Response>,
    app_config: AppConfig,
) {
    let mut main_controller = MainController::new(request_rx, response_tx, app_config);
    main_controller.run().await;
}

struct MainController {
    samples_cache: SamplesCache,
    project_store: ProjectStore,
    user_store: UserStore,
    request_rx: mpsc::Receiver<Request>,
    response_tx: broadcast::Sender<Response>,
    project: Project,
    audio_controller: AudioController,
    waveform_store: WaveformStore,
    _midi_controller: Option<MidiController>,
    action_rx: mpsc::Receiver<Action>,
    action_tx: mpsc::Sender<Action>,
    should_save: bool,
    preferences: Preferences,
    project_info: ProjectInfo,
    user: Option<User>,
    local_backend: Arc<dyn Backend>,
    remote_backend: Arc<dyn Backend>,
}

impl MainController {
    pub fn new(
        request_rx: mpsc::Receiver<Request>,
        response_tx: broadcast::Sender<Response>,
        app_config: AppConfig,
    ) -> Self {
        let directories = Directories::new(app_config.root_directory);

        let (action_tx, action_rx) = mpsc::channel(128);

        let preferences = match read_preferences(&directories.root) {
            Ok(preferences) => {
                info!("Preferences loaded: {preferences:#?}");
                preferences
            }
            Err(error) => {
                warn!("Unable to read preferences, using default: {error}");
                default_preferences()
            }
        };

        let audio_preferences = preferences.clone().audio.unwrap_or(default_audio_preferences());
        let midi_preferences = preferences.clone().midi.unwrap_or(default_midi_preferences());

        let auth = create_pocketbase_auth(app_config.api_url.clone(), &directories.backend);
        let remote_backend = create_pocketbase_backend(app_config.api_url, auth.clone());

        let local_backend = create_filesystem_backend(&directories.projects);

        let midi_controller = if app_config.use_midi {
            Some(MidiController::new(action_tx.clone(), midi_preferences))
        } else {
            None
        };

        Self {
            samples_cache: SamplesCache::new(&directories.samples),
            project_store: ProjectStore::new(&directories.projects, local_backend.clone(), remote_backend.clone()),
            user_store: UserStore::new(auth.clone()),
            request_rx,
            response_tx: response_tx.clone(),
            project: Project::empty().with_songs(1, 1),
            audio_controller: AudioController::new(response_tx.clone(), audio_preferences, app_config.use_dummy_audio),
            waveform_store: WaveformStore::new(response_tx),
            _midi_controller: midi_controller,
            action_rx,
            action_tx,
            should_save: false,
            preferences,
            project_info: ProjectInfo::empty(),
            user: None,
            local_backend,
            remote_backend,
        }
    }

    async fn load_last_project(&mut self) {
        match self.project_store.load_last_project(&mut self.samples_cache).await {
            Ok((project_info, project)) => {
                self.set_project(project);
                self.set_project_info(project_info);
            }
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
            project = self.handle_load(load_request).await?;
        }

        if let Some(remove_request) = request.remove.as_ref() {
            project = self.handle_remove(project, remove_request).await?;
        }

        if let Some(remove_sample_request) = request.remove_sample.as_ref() {
            project = project.remove_sample_from_song(remove_sample_request.song_id)?;
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

        if let Some(remove_project_request) = request.remove_project.as_ref() {
            self.project_store
                .remove_project(&remove_project_request.project_id)
                .await?;
            let projects = self.project_store.projects().await?;
            self.send_response(Response::default().with_projects(&projects));
        }

        if let Some(duplicate_project_request) = request.duplicate_project.as_ref() {
            let (source_project, source_project_info) = self
                .project_store
                .load(&duplicate_project_request.project_id, &mut self.samples_cache)
                .await?;

            let dest_project_id = self
                .project_store
                .save(None, source_project, &self.samples_cache, &self.get_user_id())
                .await?;

            let mut project_info = source_project_info.clone();
            project_info.id = dest_project_id.clone();
            self.set_project_info(project_info);
        }

        if let Some(rename_project_request) = request.rename_project.as_ref() {
            self.project_store
                .rename_project(&rename_project_request.project_id, &rename_project_request.new_name)
                .await?;
        }

        if let Some(login_request) = request.login.as_ref() {
            let username = login_request.username.clone();
            let password = login_request.password.clone();
            let user = self.user_store.log_in(username, password).await;
            match user {
                Ok(user) => {
                    info!("Logged in successfully: {}", user.name);
                    self.set_user(Some(user));
                }
                Err(error) => {
                    error!("Unable to log in: {error}");
                    self.send_error_response(&format!("Login failed: {error}"));
                    self.set_user(None);
                }
            }
        }

        if request.logout.as_ref().is_some() {
            if let Err(error) = self.user_store.log_out().await {
                error!("Error logging out: {error}");
            }
            self.set_user(None);
        }

        if let Some(project_sync) = request.project_sync.as_ref() {
            self.handle_project_sync(project_sync).await?;
        }

        self.set_project(project);
        Ok(())
    }

    fn get_user_id(&self) -> String {
        match &self.user {
            Some(user) => user.id.clone(),
            None => String::new(),
        }
    }

    async fn save_project(&mut self, project: Project) -> anyhow::Result<Project> {
        let project_id = if self.project_info.id.is_empty() {
            None
        } else {
            Some(self.project_info.id.clone())
        };

        let project_id = self
            .project_store
            .save(project_id, project.clone(), &self.samples_cache, &self.get_user_id())
            .await?;

        let mut project_info = self.project_info.clone();
        project_info.id = project_id.clone();
        self.set_project_info(project_info);

        Ok(project)
    }

    fn set_project(&mut self, project: Project) {
        if self.project.songs != project.songs {
            self.should_save = true;
        }

        if self.project != project {
            self.project = project;
            self.send_project_response(&self.project);
            self.audio_controller
                .on_project_updated(&self.project, &self.samples_cache);
        }
    }

    fn set_project_info(&mut self, project_info: ProjectInfo) {
        if self.project_info != project_info {
            self.project_info = project_info.clone();
            self.send_response(Response::default().with_project_info(&project_info));
        }
    }

    fn set_user(&mut self, user: Option<User>) {
        if self.user != user {
            self.user = user.clone();
            self.send_response(Response::default().with_user(self.user.clone()));
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
            Entity::ALL => {
                let user = self.user_store.refresh_auth().await;
                match user {
                    Ok(user) => {
                        info!("Successfully refreshed auth: {}", user.name);
                        self.set_user(Some(user));
                    }
                    Err(error) => {
                        info!("Unable to refresh auth: {error}");
                        self.set_user(None);
                    }
                }

                self.send_response(
                    Response::default()
                        .with_project(&self.project)
                        .with_playback_state(self.audio_controller.get_playback_state())
                        .with_project_info(&self.project_info)
                        .with_user(self.user.clone()),
                )
            }
            Entity::PROJECTS => {
                let projects = self.project_store.projects().await?;

                let cloud_projects = match self.project_store.cloud_projects().await {
                    Ok(cloud_projects) => cloud_projects,
                    Err(error) => {
                        warn!("Error getting cloud projects: {error}");
                        vec![]
                    }
                };

                self.send_response(
                    Response::default()
                        .with_projects(&projects)
                        .with_cloud_projects(&cloud_projects),
                );
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
        error!("{message}");
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
        if let Err(error) = self.samples_cache.scan().await {
            warn!("Error scanning samples cache: {error}");
        }

        self.load_last_project().await;

        let switch_preferences = self.preferences.clone().switch.unwrap_or_default();
        let switch_task = switch::run(self.action_tx.clone(), switch_preferences);

        let mut save_interval = time::interval(Duration::from_secs(2));

        loop {
            tokio::select! {
                Some(request) = self.request_rx.recv() => {
                    if let Err(error) = self.handle_request(request).await {
                        warn!("Error handling request: {error}");
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
            Action::ACTION_UNKNOWN => (),
            Action::ACTION_PREVIOUS_SONG => self.previous_song(),
            Action::ACTION_NEXT_SONG => self.next_song(),
            Action::ACTION_PREVIOUS_SECTION => self.previous_section(),
            Action::ACTION_NEXT_SECTION => self.next_section(),
            Action::ACTION_QUEUE_SELECTED => self.queue_selected(),
            Action::ACTION_TOGGLE_LOOP => self.audio_controller.toggle_loop(),
            Action::ACTION_TOGGLE_PLAY => self.audio_controller.toggle_play(),
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

    fn handle_add(&mut self, project: Project, request: &AddRequest) -> anyhow::Result<Project> {
        match request.entity.enum_value_or_default() {
            Entity::SECTION => self.handle_add_section(project, request),
            Entity::SONG => Ok(project.add_song(1)),
            Entity::PROJECT => {
                let project_info = ProjectInfo::empty();
                self.set_project_info(project_info);

                Ok(Project::empty().with_songs(1, 1))
            }
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
            _ => Ok(project),
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

    async fn handle_load(&mut self, request: &LoadProjectRequest) -> anyhow::Result<Project> {
        let (project, project_info) = self
            .project_store
            .load(&request.project_id, &mut self.samples_cache)
            .await?;

        self.set_project_info(project_info);

        Ok(project)
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

    async fn handle_project_sync(&mut self, project_sync: &ProjectSyncRequest) -> anyhow::Result<()> {
        let user_id = match &self.user {
            Some(user) => user.id.clone(),
            None => {
                return Err(anyhow!("User not logged in, cannot sync project"));
            }
        };

        self.send_response(Response::default().with_project_sync(&ProjectSyncResponse {
            project_id: project_sync.project_id.clone(),
            status: SyncStatus::SYNC_STATUS_IN_PROGRESS.into(),
            ..Default::default()
        }));

        let result = match project_sync.method.enum_value_or_default() {
            SyncMethod::SYNC_METHOD_UNDEFINED => {
                return Err(anyhow!("Undefined sync method"));
            }
            SyncMethod::SYNC_METHOD_PUSH => {
                sync_project(
                    &user_id,
                    &project_sync.project_id,
                    self.local_backend.as_ref(),
                    self.remote_backend.as_ref(),
                )
                .await
            }
            SyncMethod::SYNC_METHOD_PULL => {
                sync_project(
                    &user_id,
                    &project_sync.project_id,
                    self.remote_backend.as_ref(),
                    self.local_backend.as_ref(),
                )
                .await
            }
        };

        let projects = self.project_store.projects().await?;
        let cloud_projects = self.project_store.cloud_projects().await.unwrap_or_default();

        let response = Response::default()
            .with_project_sync(&ProjectSyncResponse {
                project_id: project_sync.project_id.clone(),
                status: if result.is_err() {
                    SyncStatus::SYNC_STATUS_ERROR
                } else {
                    SyncStatus::SYNC_STATUS_COMPLETE
                }
                .into(),
                ..Default::default()
            })
            .with_projects(&projects)
            .with_cloud_projects(&cloud_projects);

        self.send_response(response);

        Ok(())
    }
}
