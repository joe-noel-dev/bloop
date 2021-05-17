use crate::{
    api::{
        request::{RemoveSampleRequest, Request, UploadSampleRequest},
        response::{Response, ResponseBroadcaster},
    },
    model::{project::Project, proxy::Proxy, sample::Sample},
    samples::cache::SamplesCache,
};

pub fn handle_request(
    request: &Request,
    project_proxy: &mut dyn Proxy<Project>,
    samples_cache: &mut SamplesCache,
    response_broadcaster: &dyn ResponseBroadcaster,
) {
    let result = match request {
        Request::Upload(upload_request) => handle_upload(upload_request, project_proxy, samples_cache),
        Request::RemoveSample(remove_request) => handle_remove_sample(remove_request, project_proxy),
        _ => Ok(()),
    };

    if let Err(error) = result {
        response_broadcaster.broadcast(Response::new().with_error(&error));
    }
}

fn handle_upload(
    request: &UploadSampleRequest,
    project_proxy: &mut dyn Proxy<Project>,
    samples_cache: &mut SamplesCache,
) -> Result<(), String> {
    let mut project = project_proxy.get();

    let mut sample = Sample::new();
    let sample_metadata = samples_cache.add_sample_from_data(&sample.id, &request.format, &request.file_data)?;

    sample.name = request.name.clone();
    sample.sample_rate = sample_metadata.sample_rate as i32;
    sample.channel_count = sample_metadata.num_channels as i32;
    sample.sample_count = sample_metadata.sample_count as i64;

    project = project.add_sample_to_song(sample, &request.song_id)?;

    project_proxy.set(project);
    Ok(())
}

fn handle_remove_sample(request: &RemoveSampleRequest, project_proxy: &mut dyn Proxy<Project>) -> Result<(), String> {
    let mut project = project_proxy.get();
    project = project.remove_sample(&request.sample_id, &request.song_id)?;
    // TODO: Remove from samples cache?
    project_proxy.set(project);
    Ok(())
}
