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

    fn remove_all_points_after_loop(&self) -> Self {
        let mut sequence = self.clone();

        let mut found_loop = false;

        sequence.points.retain(|point| {
            if found_loop {
                return false;
            }

            if point.loop_enabled {
                found_loop = true;
            }

            true
        });

        sequence
    }

    pub fn truncate_to_time(&self, time: Timestamp) -> Self {
        let sequence = self.clone();

        let mut sequence = sequence.remove_all_points_after_loop();

        sequence.points.iter_mut().for_each(|point| {
            if !point.is_playing_at_time(time) {
                return;
            }

            if point.loop_enabled {
                let loop_count = point.completed_loop_count(time);
                point.start_time = point.start_time + Timestamp::from_seconds(loop_count * point.duration.as_seconds());

                if point.start_time == time {
                    point.start_time = point.start_time - point.duration;
                }

                point.loop_enabled = false;
            }

            point.duration = time - point.start_time;
        });

        sequence.points.retain(|point| point.start_time < time);

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
            }
        });

        sequence
    }

    pub fn cancel_loop_at_time(&self, time: Timestamp) -> Self {
        let mut sequence = self.clone();

        let playing_point = match sequence.points.iter_mut().find(|point| point.is_playing_at_time(time)) {
            Some(point) => point,
            None => return sequence,
        };

        playing_point.loop_enabled = false;

        let completed_loop_count = playing_point.completed_loop_count(time);
        let offset = Timestamp::from_seconds(completed_loop_count * playing_point.duration.as_seconds());

        for point in sequence.points.iter_mut() {
            point.start_time = point.start_time + offset;
        }

        sequence
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sequence<SequenceData> {
    pub points: Vec<SequencePoint<SequenceData>>,
}

#[cfg(test)]
mod test {
    use super::*;

    fn example_sequence() -> Sequence<()> {
        Sequence {
            points: vec![
                SequencePoint {
                    start_time: Timestamp::from_seconds(1.0),
                    duration: Timestamp::from_seconds(2.0),
                    loop_enabled: false,
                    data: {},
                },
                SequencePoint {
                    start_time: Timestamp::from_seconds(3.0),
                    duration: Timestamp::from_seconds(4.0),
                    loop_enabled: true,
                    data: {},
                },
                SequencePoint {
                    start_time: Timestamp::from_seconds(7.0),
                    duration: Timestamp::from_seconds(6.0),
                    loop_enabled: false,
                    data: {},
                },
            ],
        }
    }

    #[test]
    fn truncate_sequence() {
        let sequence = example_sequence();

        let truncated = sequence.truncate_to_time(Timestamp::from_seconds(9.0));

        let expected = vec![
            SequencePoint {
                start_time: Timestamp::from_seconds(1.0),
                duration: Timestamp::from_seconds(2.0),
                loop_enabled: false,
                data: {},
            },
            SequencePoint {
                start_time: Timestamp::from_seconds(7.0),
                duration: Timestamp::from_seconds(2.0),
                loop_enabled: false,
                data: {},
            },
        ];

        assert_eq!(expected, truncated.points);
    }

    #[test]
    fn truncate_on_boundary() {
        let sequence = example_sequence();

        let truncated = sequence.truncate_to_time(Timestamp::from_seconds(3.0));

        let expected = vec![SequencePoint {
            start_time: Timestamp::from_seconds(1.0),
            duration: Timestamp::from_seconds(2.0),
            loop_enabled: false,
            data: {},
        }];

        assert_eq!(expected, truncated.points);
    }

    #[test]
    fn truncate_on_loop_boundary() {
        let sequence = example_sequence();

        let truncated = sequence.truncate_to_time(Timestamp::from_seconds(11.0));

        let expected = vec![
            SequencePoint {
                start_time: Timestamp::from_seconds(1.0),
                duration: Timestamp::from_seconds(2.0),
                loop_enabled: false,
                data: {},
            },
            SequencePoint {
                start_time: Timestamp::from_seconds(7.0),
                duration: Timestamp::from_seconds(4.0),
                loop_enabled: false,
                data: {},
            },
        ];

        assert_eq!(expected, truncated.points);
    }

    #[test]
    fn cancel_loop() {
        let sequence = example_sequence();

        let cancelled = sequence.cancel_loop_at_time(Timestamp::from_seconds(10.0));

        let expected = vec![
            SequencePoint {
                start_time: Timestamp::from_seconds(5.0),
                duration: Timestamp::from_seconds(2.0),
                loop_enabled: false,
                data: {},
            },
            SequencePoint {
                start_time: Timestamp::from_seconds(7.0),
                duration: Timestamp::from_seconds(4.0),
                loop_enabled: false,
                data: {},
            },
            SequencePoint {
                start_time: Timestamp::from_seconds(11.0),
                duration: Timestamp::from_seconds(6.0),
                loop_enabled: false,
                data: {},
            },
        ];

        assert_eq!(expected, cancelled.points);
    }

    #[test]
    fn enable_loop() {
        let sequence = example_sequence();

        let cancelled = sequence.enable_loop_at_time(Timestamp::from_seconds(2.0));

        let expected = vec![
            SequencePoint {
                start_time: Timestamp::from_seconds(1.0),
                duration: Timestamp::from_seconds(2.0),
                loop_enabled: true,
                data: {},
            },
            SequencePoint {
                start_time: Timestamp::from_seconds(3.0),
                duration: Timestamp::from_seconds(4.0),
                loop_enabled: true,
                data: {},
            },
            SequencePoint {
                start_time: Timestamp::from_seconds(7.0),
                duration: Timestamp::from_seconds(6.0),
                loop_enabled: false,
                data: {},
            },
        ];

        assert_eq!(expected, cancelled.points);
    }
}
