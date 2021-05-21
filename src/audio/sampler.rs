use std::cmp::min;

use super::buffer::{AudioBuffer, SampleLocation};

pub fn render<T, U>(output: &mut T, source: &U, from_position: usize, to_position: usize) -> usize
where
    T: AudioBuffer,
    U: AudioBuffer,
{
    let num_channels = min(source.num_channels(), output.num_channels());

    if from_position >= source.num_frames() {
        return 0;
    }

    let mut end_position = from_position + output.num_frames();
    end_position = min(end_position, source.num_frames());
    end_position = min(end_position, to_position);

    if end_position < from_position {
        return 0;
    }

    let num_frames = end_position - from_position;

    let source_location = SampleLocation {
        channel: 0,
        frame: from_position,
    };

    let destination_location = SampleLocation { channel: 0, frame: 0 };

    output.add_from(
        source,
        &source_location,
        &destination_location,
        num_channels,
        num_frames,
    );

    num_frames
}
