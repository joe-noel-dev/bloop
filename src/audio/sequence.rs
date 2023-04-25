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
        if self.start_time >= time {
            return false;
        }

        if !self.loop_enabled && self.end_time() < time {
            return false;
        }

        true
    }

    pub fn completed_loop_count(&self, at_time: Timestamp) -> f64 {
        let elapsed = at_time.as_seconds() - self.start_time.as_seconds();
        (elapsed / self.duration.as_seconds()).floor()
    }
}

impl<Data> Sequence<Data>
where
    Data: Copy,
{
    pub fn point_at_time(&self, time: Timestamp) -> Option<SequencePoint<Data>> {
        self.points.iter().find(|point| point.is_playing_at_time(time)).copied()
    }

    pub fn next_transition(&self, time: Timestamp) -> Timestamp {
        match self.point_at_time(time) {
            Some(current_point) => {
                if current_point.loop_enabled {
                    let loop_count = current_point.completed_loop_count(time);
                    let next_loop_count = loop_count + 1.0;
                    current_point.start_time
                        + Timestamp::from_seconds(next_loop_count * current_point.duration.as_seconds())
                } else {
                    current_point.end_time()
                }
            }
            None => time,
        }
    }

    pub fn truncate_to_time(&self, time: Timestamp) -> Self {
        let mut sequence = self.clone();

        sequence.points.iter_mut().for_each(|point| {
            if !point.is_playing_at_time(time) {
                return;
            }

            if point.loop_enabled {
                let loop_count = point.completed_loop_count(time);
                point.start_time = point.start_time + Timestamp::from_seconds(loop_count * point.duration.as_seconds());
                point.loop_enabled = false;
            }

            point.duration = time - point.start_time;
        });

        sequence.points.retain(|point| point.start_time <= time);

        sequence
    }

    pub fn append(&self, mut sequence_to_append: Sequence<Data>) -> Self {
        let mut sequence = self.clone();
        sequence.points.append(&mut sequence_to_append.points);
        sequence
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
            if point.is_playing_at_time(time) && offset == Timestamp::zero() {
                point.loop_enabled = false;

                let completed_loop_count = point.completed_loop_count(time);

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
