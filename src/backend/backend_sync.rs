use crate::backend::Backend;
use anyhow::Result;

pub async fn sync_project(
    user_id: &str,
    project_id: &str,
    source: &dyn Backend,
    destination: &dyn Backend,
) -> Result<()> {
    let source_project = source.read_project(project_id).await?;

    let destination_project = match destination.read_project(project_id).await {
        Ok(project) => Some(project),
        Err(_) => None,
    };

    if let Some(destination_project) = &destination_project {
        if destination_project.updated == source_project.updated {
            return Ok(());
        }
    }

    // Possibly create
    let destination_project = if let Some(destination_project) = destination_project {
        destination_project
    } else {
        destination.create_project(user_id).await?
    };

    // Update metadata (name, updated timestamp)
    if destination_project.name != source_project.name {
        destination
            .update_project_name(project_id, &source_project.name)
            .await?;
    }

    // Sync samples
    push_samples(project_id, source, destination).await?;

    // Update the project file
    if let Ok(project_bytes) = source.read_project_file(project_id).await {
        destination.update_project_file(project_id, &project_bytes).await?;
    }

    Ok(())
}

async fn push_samples(project_id: &str, source: &dyn Backend, destination: &dyn Backend) -> Result<()> {
    let source_samples = source.get_samples(project_id).await?;
    let destination_samples = destination.get_samples(project_id).await?;

    for sample in source_samples.iter() {
        if !destination_samples.contains(sample) {
            let sample_bytes = source.read_sample(project_id, sample).await?;
            destination
                .add_project_sample(project_id, &sample_bytes, sample)
                .await?;
        }
    }

    for sample in destination_samples.iter() {
        if !source_samples.contains(sample) {
            destination.remove_project_sample(project_id, sample).await?;
        }
    }

    Ok(())
}
