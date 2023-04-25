use rawdio::Timestamp;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SequencePoint<Data> {
    pub start_time: Timestamp,
    pub duration: Timestamp,
    pub loop_enabled: bool,
    pub data: Data,
}

impl<Data> SequencePoint<Data> {
    pub fn end_time(&self) -> Timestamp {
        self.start_time + self.duration
    }

    pub fn is_playing_at_time(&self, time: Timestamp) -> bool {
        if self.start_time > time {
            return false;
        }

        if !self.loop_enabled && self.end_time() < time {
            return false;
        }

        true
    }
}

impl<Data> Sequence<Data>
where
    Data: Copy,
{
    pub fn point_at_time(&self, time: Timestamp) -> Option<SequencePoint<Data>> {
        self.points.iter().find(|point| point.is_playing_at_time(time)).copied()
    }

    pub fn enable_loop_at_time(&self, time: Timestamp) -> Self {
        let mut sequence = self.clone();

        let mut loop_enabled = false;

        sequence.points.iter_mut().for_each(|point| {
            if !loop_enabled && point.is_playing_at_time(time) {
                point.loop_enabled = true;
                loop_enabled = true;
            } else {
                point.loop_enabled = false;
            }
        });

        sequence
    }

    pub fn cancel_loop_at_time(&self, time: Timestamp) -> Self {
        let mut sequence = self.clone();

        let mut offset = Timestamp::zero();

        sequence.points.iter_mut().for_each(|point| {
            if point.start_time < time && point.loop_enabled && offset == Timestamp::zero() {
                point.loop_enabled = false;

                let elapsed = time.as_seconds() - point.start_time.as_seconds();
                let completed_loop_count = (elapsed / point.duration.as_seconds()).floor();

                let previous_start_time = point.start_time;
                point.start_time =
                    point.start_time + Timestamp::from_seconds(completed_loop_count * point.duration.as_seconds());

                offset = point.start_time - previous_start_time;
            } else {
                point.start_time = point.start_time + offset;
            }
        });

        sequence
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct Sequence<SequenceData> {
    pub points: Vec<SequencePoint<SequenceData>>,
}
