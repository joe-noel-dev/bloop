use super::{directories::Directories, project_store::ProjectStore};
use crate::{
    api::{
        request::{
            AddRequest, Entity, GetRequest, LoadRequest, RemoveRequest, RenameRequest, Request, SelectRequest,
            TransportMethod, UpdateRequest, UploadSampleRequest,
        },
        response::{Response, ResponseBroadcaster},
    },
    audio::{manager::Audio, manager::AudioManager},
    generators::projects,
    model::{project::Project, sample::Sample},
    samples::cache::SamplesCache,
};
use tokio::sync::{broadcast, mpsc};

pub struct MainController {
    samples_cache: SamplesCache,
    project_store: ProjectStore,
    request_rx: mpsc::Receiver<Request>,
    response_tx: broadcast::Sender<Response>,
    project: Project,
    audio_manager: AudioManager,
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

        let audio_manager = AudioManager::new(response_tx.clone());

        Self {
            samples_cache,
            project_store,
            request_rx,
            response_tx,
            project: projects::generate_project(4, 3, 3),
            audio_manager,
        }
    }

    fn handle_request(&mut self, request: Request) {
        let project = self.project.clone();
        let result = match request {
            Request::Add(add_request) => self.handle_add(project, &add_request),
            Request::Get(get_request) => match self.handle_get(&get_request) {
                Ok(_) => Ok(project),
                Err(error) => Err(error),
            },
            Request::Load(load_request) => self.handle_load(&load_request),
            Request::Remove(remove_request) => self.handle_remove(project, &remove_request),
            Request::RemoveSample(remove_request) => {
                project.remove_sample(&remove_request.sample_id, &remove_request.song_id)
            }
            Request::Rename(rename_request) => self.handle_rename(project, &rename_request),
            Request::Save => match self.project_store.save(project.clone(), &self.samples_cache) {
                Ok(_) => Ok(project),
                Err(error) => Err(error),
            },
            Request::Select(select_request) => self.handle_select(project, &select_request),
            Request::Transport(transport_method) => {
                self.handle_transport_request(&transport_method);
                Ok(project)
            }
            Request::Update(update_request) => self.handle_update(project, &update_request),
            Request::Upload(upload_request) => self.handle_upload(&upload_request, project),
        };

        match result {
            Ok(project) => {
                if self.project != project {
                    self.project = project;
                    self.send_project_response(&self.project);
                    self.audio_manager.on_project_updated(&self.project);
                }
            }
            Err(error) => self.send_error_response(&error),
        }
    }

    fn handle_get(&self, get_request: &GetRequest) -> Result<(), String> {
        match get_request.entity {
            Entity::All => self.send_project_response(&self.project),
            Entity::Projects => {
                let projects = self.project_store.projects()?;
                self.send_response(Response::new().with_projects(&projects));
            }
            _ => (),
        };

        Ok(())
    }

    fn send_project_response(&self, project: &Project) {
        self.send_response(Response::new().with_project(project));
    }

    fn send_error_response(&self, message: &str) {
        self.send_response(Response::new().with_error(message));
    }

    fn send_response(&self, response: Response) {
        self.response_tx.send(response).unwrap();
    }

    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                Some(request) = self.request_rx.recv() => self.handle_request(request),
                _ = self.audio_manager.run() => (),
                else => break,
            }
        }
    }

    fn handle_add(&self, project: Project, request: &AddRequest) -> Result<Project, String> {
        match request.entity {
            Entity::Channel => project.add_channel(),
            Entity::Section => self.handle_add_section(project, request),
            Entity::Song => Ok(project.add_song(1)),
            Entity::Project => Ok(Project::new()),
            _ => Ok(project),
        }
    }

    fn handle_add_section(&self, project: Project, request: &AddRequest) -> Result<Project, String> {
        let song_id = match request.id {
            Some(id) => id,
            None => return Err("Missing parent ID".to_string()),
        };

        project.add_section_to_song(&song_id)
    }

    fn handle_select(&self, project: Project, select_request: &SelectRequest) -> Result<Project, String> {
        match select_request.entity {
            Entity::Song => Ok(project.select_song_with_id(&select_request.id)),
            Entity::Section => project.select_section(&select_request.id),
            _ => Ok(project),
        }
    }

    fn handle_remove(&self, project: Project, remove_request: &RemoveRequest) -> Result<Project, String> {
        match remove_request.entity {
            Entity::Song => project.remove_song(&remove_request.id),
            Entity::Section => project.remove_section(&remove_request.id),
            Entity::Channel => project.remove_channel(&remove_request.id),
            Entity::Project => {
                self.project_store.remove_project(&remove_request.id)?;
                let projects = self.project_store.projects()?;
                self.send_response(Response::new().with_projects(&projects));
                Ok(project)
            }
            _ => Ok(project),
        }
    }

    fn handle_update(&self, project: Project, update_request: &UpdateRequest) -> Result<Project, String> {
        match update_request {
            UpdateRequest::Song(song) => project.replace_song(song),
            UpdateRequest::Section(section) => project.replace_section(section),
            UpdateRequest::Sample(sample) => project.replace_sample(sample),
        }
    }

    fn handle_rename(&self, project: Project, rename_request: &RenameRequest) -> Result<Project, String> {
        match rename_request.entity {
            Entity::Project => Ok(project.with_name(&rename_request.name)),
            _ => Ok(project),
        }
    }

    fn handle_load(&mut self, request: &LoadRequest) -> Result<Project, String> {
        self.project_store.load(&request.id, &mut self.samples_cache)
    }

    fn handle_upload(&mut self, request: &UploadSampleRequest, project: Project) -> Result<Project, String> {
        let mut sample = Sample::new();
        let sample_metadata =
            self.samples_cache
                .add_sample_from_data(&sample.id, &request.format, &request.file_data)?;

        sample.name = request.name.clone();
        sample.sample_rate = sample_metadata.sample_rate as i32;
        sample.channel_count = sample_metadata.num_channels as i32;
        sample.sample_count = sample_metadata.sample_count as i64;

        project.add_sample_to_song(sample, &request.song_id)
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
