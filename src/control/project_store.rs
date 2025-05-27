use crate::{
    backend::Backend,
    bloop::{AudioFileFormat, User},
    model::{Project, ProjectInfo, ID},
    samples::SamplesCache,
};
use anyhow::{anyhow, Context, Ok};
use log::{debug, error, info};
use protobuf::Message;
use std::str::FromStr;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct ProjectStore {
    root_directory: PathBuf,
    temporary_directory: tempfile::TempDir,
    backend: Box<dyn Backend>,
}

impl ProjectStore {
    pub fn new(root_directory: &Path, backend: Box<dyn Backend>) -> Self {
        if !root_directory.exists() {
            fs::create_dir_all(root_directory)
                .unwrap_or_else(|_| panic!("Couldn't create directory: {}", root_directory.to_str().unwrap()));
        }

        Self {
            root_directory: PathBuf::from(root_directory),
            temporary_directory: tempfile::TempDir::new().expect("Unable to create temporary directory"),
            backend,
        }
    }

    pub async fn log_in(&mut self, username: String, password: String) -> anyhow::Result<User> {
        let db_user = self
            .backend
            .log_in(username, password)
            .await
            .context("Error logging in")?;

        info!("Logged in successfully");

        Ok(User {
            id: db_user.id,
            name: db_user.name,
            email: db_user.email,
            ..Default::default()
        })
    }

    pub async fn save(
        &mut self,
        project_id: Option<String>,
        project: Project,
        samples_cache: &SamplesCache,
        user_id: &str,
    ) -> anyhow::Result<String> {
        let project_id = match project_id {
            Some(id) => id,
            None => {
                let new_project = self.backend.create_project(user_id).await?;
                new_project.id
            }
        };

        self.copy_samples_from_cache(&project_id, &project, samples_cache)
            .await?;
        self.write_project_json(&project_id, project).await?;
        Ok(project_id)
    }

    async fn save_last_project(&self, project_id: &str) -> anyhow::Result<()> {
        let last_project_file = self.last_project_file();

        tokio::fs::write(last_project_file, project_id.to_string().as_bytes())
            .await
            .context("Error writing last project ID")
    }

    pub async fn load(
        &mut self,
        project_id: &str,
        samples_cache: &mut SamplesCache,
    ) -> anyhow::Result<(Project, ProjectInfo)> {
        let (project, project_info) = self.read_project_file(project_id).await?;
        self.load_samples_into_cache(project_id, samples_cache).await?;
        self.save_last_project(project_id).await?;
        Ok((project, project_info))
    }

    pub async fn load_last_project(
        &mut self,
        samples_cache: &mut SamplesCache,
    ) -> anyhow::Result<(ProjectInfo, Project)> {
        let last_project_file = self.last_project_file();
        let last_project_id = tokio::fs::read_to_string(last_project_file)
            .await
            .context("Opening last project file")?;
        let (project, project_info) = self.load(&last_project_id, samples_cache).await?;
        Ok((project_info, project))
    }

    fn last_project_file(&self) -> PathBuf {
        let mut last_project_file = self.root_directory.clone();
        last_project_file.push("last_project");
        last_project_file
    }

    pub async fn projects(&self) -> anyhow::Result<Vec<ProjectInfo>> {
        let projects = self.backend.get_projects().await.context("Error getting projects")?;

        let projects_info = projects
            .iter()
            .map(|db_project| ProjectInfo {
                id: db_project.id.clone(),
                name: db_project.name.clone(),
                last_saved: db_project.updated.to_rfc3339(),
                ..Default::default()
            })
            .collect();

        Ok(projects_info)
    }

    pub async fn remove_project(&self, project_id: &str) -> anyhow::Result<()> {
        self.backend
            .remove_project(project_id)
            .await
            .context(format!("Removing project: {project_id}"))?;

        info!("Project removed: id = {}", project_id);
        Ok(())
    }

    pub async fn rename_project(&self, project_id: &str, name: &str) -> anyhow::Result<()> {
        self.backend
            .update_project_name(project_id, name)
            .await
            .context(format!("Renaming project: {project_id}"))?;

        info!("Project renamed: id = {}", project_id);
        Ok(())
    }

    // fn directory_for_project(&self, project_id: ID) -> PathBuf {
    //     let mut project_directory = self.root_directory.clone();
    //     project_directory.push(project_id.to_string());
    //     project_directory
    // }

    async fn write_project_json(&mut self, project_id: &str, project: Project) -> anyhow::Result<()> {
        let data = project.write_to_bytes()?;
        self.backend
            .update_project_file(project_id, &data)
            .await
            .context("Updating project file")?;

        Ok(())
    }

    async fn read_project_file(&self, project_id: &str) -> anyhow::Result<(Project, ProjectInfo)> {
        let project = self.backend.get_project(project_id).await.context("Get project")?;

        let project_info = ProjectInfo {
            id: project.id.clone(),
            name: project.name.clone(),
            last_saved: project.updated.to_rfc3339(),
            ..Default::default()
        };

        let project_data = self
            .backend
            .get_project_file(project_id, &project.project)
            .await
            .context("Get project file")?;
        let project = Project::parse_from_bytes(&project_data).context("Parse project data")?;

        Ok((project, project_info))
    }

    async fn copy_samples_from_cache(
        &self,
        project_id: &str,
        project: &Project,
        samples_cache: &SamplesCache,
    ) -> anyhow::Result<()> {
        let db_project = self.backend.get_project(project_id).await.context("Get project")?;

        for song in project.songs.iter() {
            let sample = match song.sample.as_ref() {
                Some(sample) => sample,
                None => continue,
            };

            let db_filename = db_project
                .samples
                .iter()
                .find(|sample_name| sample_name.contains(&sample.id.to_string()));

            if db_filename.is_some() {
                continue;
            }

            let cached_sample = match samples_cache.get_sample(sample.id) {
                Some(sample) => sample,
                None => {
                    return Err(anyhow!("Missing sample in cache: {}", sample.id));
                }
            };

            if !cached_sample.is_cached() {
                return Err(anyhow!("Sample isn't cached: {}", sample.id));
            }

            let cached_sample_path = cached_sample.get_path();

            debug!("Reading sample from cache: {}", cached_sample_path.display());
            let cached_sample_bytes = tokio::fs::read(&cached_sample_path)
                .await
                .context(format!("Error reading cached sample: {}", cached_sample_path.display()))?;

            debug!("Adding sample to project: {}", cached_sample_path.display());
            self.backend
                .add_project_sample(project_id, &cached_sample_bytes, &sample.id.to_string())
                .await?;
        }

        Ok(())
    }

    async fn load_samples_into_cache(
        &mut self,
        project_id: &str,
        samples_cache: &mut SamplesCache,
    ) -> anyhow::Result<()> {
        samples_cache.clear();

        let db_project = self.backend.get_project(project_id).await.context("Get project")?;

        for sample in db_project.samples.iter() {
            let sample_id = match ID::from_str(sample) {
                std::result::Result::Ok(id) => id,
                Err(error) => {
                    error!("Invalid sample ID ({}): {}", sample, error);
                    continue;
                }
            };

            let sample_bytes = self
                .backend
                .get_project_file(project_id, sample)
                .await
                .context(format!("Error getting project file: {sample}"))?;

            let sample_path = self.temporary_directory.path().join(sample.to_string() + ".wav");

            tokio::fs::write(&sample_path, &sample_bytes)
                .await
                .context(format!("Error writing sample file: {}", sample_path.display()))?;

            samples_cache
                .add_sample_from_file(sample_id, AudioFileFormat::WAV, &sample_path)
                .await?;
        }

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use crate::{backend::DbUser, generators::generate_project, model::random_id};
//     use anyhow::Result;

//     struct MockBackend {}

//     impl Backend for MockBackend {
//         async fn log_in(&mut self, username: String, password: String) -> Result<DbUser> {
//             Err(anyhow::anyhow!("Not implemented"))
//         }

//         async fn log_out(&mut self) -> Result<()> {
//             Err(anyhow::anyhow!("Not implemented"))
//         }

//         async fn get_user(&self, user_id: &str) -> Result<DbUser> {
//             Err(anyhow::anyhow!("Not implemented"))
//         }

//         async fn get_projects(&self) -> Result<Vec<DbProject>> {}

//         async fn get_project(&self, project_id: &str) -> Result<DbProject> {}

//         async fn create_project(&self, user_id: &str) -> Result<DbProject> {}

//         async fn update_project_name(&self, project_id: &str, name: &str) -> Result<DbProject> {}

//         async fn update_project_file(&self, project_id: &str, project_bytes: &[u8]) -> Result<DbProject> {}

//         async fn add_project_sample(
//             &self,
//             project_id: &str,
//             sample_bytes: &[u8],
//             sample_name: &str,
//         ) -> Result<DbProject> {
//         }

//         async fn remove_project_sample(&self, project_id: &str, sample_name: &str) -> Result<DbProject> {}

//         async fn remove_project(&self, project_id: &str) -> Result<()> {}

//         async fn get_project_file(&self, project_id: &str, project_filename: &str) -> Result<Vec<u8>> {}
//     }

//     struct Fixture {
//         temp_dir: tempfile::TempDir,
//         project_store: ProjectStore,
//         samples_cache: SamplesCache,
//     }

//     impl Fixture {
//         pub fn new() -> Self {
//             let temp_dir = tempfile::TempDir::new().expect("Unable to create temporary directory");
//             let root_dir = temp_dir.path();
//             let project_directory = root_dir.join("projects");
//             let samples_directory = root_dir.join("samples");
//             let backend = Box::new(MockBackend {});

//             Self {
//                 temp_dir,
//                 project_store: ProjectStore::new(&project_directory, backend),
//                 samples_cache: SamplesCache::new(&samples_directory),
//             }
//         }

//         pub fn root_dir(&self) -> PathBuf {
//             self.temp_dir.path().to_path_buf()
//         }

//         pub async fn save(&mut self, project: Project) {
//             self.project_store
//                 .save(project, &self.samples_cache)
//                 .await
//                 .expect("Unable to save project");
//         }

//         pub async fn load(&mut self, project_id: ID) -> Project {
//             self.project_store
//                 .load(project_id, &mut self.samples_cache)
//                 .await
//                 .expect("Unable to load project")
//         }

//         pub async fn list(&self) -> Vec<ProjectInfo> {
//             self.project_store.projects().await.expect("Unable to list projects")
//         }

//         pub async fn remove(&self, project_id: ID) {
//             self.project_store
//                 .remove_project(project_id)
//                 .await
//                 .expect("Unable to remove project")
//         }

//         pub async fn export(&mut self, project_id: ID) -> Vec<u8> {
//             let mut data = Vec::new();

//             loop {
//                 let (chunk, more_coming) = self
//                     .project_store
//                     .export(project_id)
//                     .await
//                     .expect("Unable to export project");

//                 data.extend(chunk);

//                 if !more_coming {
//                     break;
//                 }
//             }

//             data
//         }

//         pub async fn import(&mut self, project_id: ID, data: &[u8]) {
//             let mut offset = 0;
//             let mut more_coming = true;

//             while more_coming {
//                 let chunk_size = 1024.min(data.len() - offset);
//                 let chunk = &data[offset..offset + chunk_size];
//                 more_coming = offset + chunk_size < data.len();

//                 self.project_store
//                     .import(project_id, chunk, more_coming)
//                     .await
//                     .expect("Unable to import project");

//                 offset += chunk_size;
//             }
//         }
//     }

//     #[test]
//     fn creates_directory() {
//         let fixture = Fixture::new();
//         let project_dir = fixture.root_dir().join("projects");
//         assert!(project_dir.exists());
//     }

//     #[tokio::test]
//     async fn save_and_load_project() {
//         let mut fixture = Fixture::new();

//         let song_count = 4;
//         let section_count = 5;

//         let project = generate_project(song_count, section_count);
//         let project_id = project.info.id;
//         fixture.save(project).await;

//         let project2 = fixture.load(project_id).await;
//         assert_eq!(project2.songs.len(), song_count);
//         assert!(project2.songs.iter().all(|song| song.sections.len() == section_count));
//     }

//     #[tokio::test]
//     async fn export_and_import_project() {
//         let mut fixture = Fixture::new();
//         let project = generate_project(4, 5);
//         let project_id = project.info.id;
//         fixture.save(project).await;
//         let exported = fixture.export(project_id).await;
//         fixture.remove(project_id).await;

//         let new_project_id = random_id();
//         fixture.import(new_project_id, &exported).await;
//         let project2 = fixture.load(project_id).await;
//         assert_eq!(project2.songs.len(), 4);
//         assert!(project2.songs.iter().all(|song| song.sections.len() == 5));
//     }

//     #[tokio::test]
//     async fn list_projects() {
//         let mut fixture = Fixture::new();

//         let project1 = generate_project(4, 5);
//         let project2 = generate_project(4, 5);
//         let project3 = generate_project(4, 5);

//         let project1_id = project1.info.id;
//         let project2_id = project1.info.id;
//         let project3_id = project1.info.id;

//         fixture.save(project1).await;
//         fixture.save(project2).await;
//         fixture.save(project3).await;

//         let projects = fixture.list().await;

//         assert_eq!(projects.len(), 3, "Should be 3 projects on disk");
//         assert!(
//             projects.iter().any(|info| info.id == project1_id),
//             "Project 1 not found"
//         );
//         assert!(
//             projects.iter().any(|info| info.id == project2_id),
//             "Project 2 not found"
//         );
//         assert!(
//             projects.iter().any(|info| info.id == project3_id),
//             "Project 3 not found"
//         );
//     }
// }
