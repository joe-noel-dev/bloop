use crate::backend::Backend;
use anyhow::Result;

pub async fn sync_project(
    user_id: &str,
    source_project_id: &str,
    source: &dyn Backend,
    destination: &dyn Backend,
) -> Result<()> {
    let source_project = source.read_project(source_project_id).await?;

    let destination_project = (destination.read_project(source_project_id).await).ok();

    if let Some(destination_project) = &destination_project {
        if destination_project.updated == source_project.updated {
            return Ok(());
        }
    }

    // Possibly create
    let destination_project = match destination_project {
        Some(project) => project,
        None => {
            destination
                .create_project(user_id, Some(source_project.id.clone()))
                .await?
        }
    };

    let destination_project_id = &destination_project.id;

    // Update metadata (name, updated timestamp)
    if destination_project.name != source_project.name {
        destination
            .update_project_name(destination_project_id, &source_project.name)
            .await?;
    }

    // Sync samples
    push_samples(source_project_id, source, destination_project_id, destination).await?;

    // Update the project file
    if let Ok(project_bytes) = source.read_project_file(source_project_id).await {
        destination
            .update_project_file(destination_project_id, &project_bytes)
            .await?;
    }

    Ok(())
}

async fn push_samples(
    source_project_id: &str,
    source: &dyn Backend,
    destination_project_id: &str,
    destination: &dyn Backend,
) -> Result<()> {
    let source_samples = source.get_samples(source_project_id).await?;
    let destination_samples = destination.get_samples(destination_project_id).await?;

    for sample in source_samples.iter() {
        if !destination_samples.contains(sample) {
            let sample_bytes = source.read_sample(source_project_id, sample).await?;
            destination
                .add_project_sample(destination_project_id, &sample_bytes, sample)
                .await?;
        }
    }

    for sample in destination_samples.iter() {
        if !source_samples.contains(sample) {
            destination
                .remove_project_sample(destination_project_id, sample)
                .await?;
        }
    }

    Ok(())
}
