use std::time::Duration;

use rawdio::{connect_nodes, Adsr, Context, GraphNode, Level, Oscillator, Timestamp};

use crate::model::Tempo;

use super::sequencer::Sequencer;

pub struct Metronome {
    oscillator: Oscillator,
    adsr: Adsr,
    last_scheduled_time: Timestamp,
}

const BAR_FREQUENCY: f64 = 2_000.0;
const BEAT_FREQUENCY: f64 = 1_000.0;
const OUTPUT_COUNT: usize = 1;
const LOOKAHEAD: f64 = 0.2;

impl Metronome {
    pub fn new(context: &dyn Context) -> Self {
        let mut oscillator = Oscillator::sine(context, BAR_FREQUENCY, OUTPUT_COUNT);

        oscillator
            .gain
            .set_value_at_time(Level::from_db(-6.0).as_linear(), Timestamp::zero());

        let mut adsr = Adsr::new(context, OUTPUT_COUNT, context.get_sample_rate());

        adsr.set_adsr(
            Duration::from_millis(5),
            Duration::from_millis(15),
            Level::zero(),
            Duration::from_millis(15),
        );

        connect_nodes!(oscillator => adsr);

        Self {
            oscillator,
            adsr,
            last_scheduled_time: Timestamp::zero(),
        }
    }

    pub fn output_node(&self) -> &GraphNode {
        &self.adsr.node
    }

    pub fn schedule(&mut self, current_time: &Timestamp, sequencer: &Sequencer) {
        let lookahead_time = current_time.incremented_by_seconds(LOOKAHEAD);

        let sequence_point = sequencer.sequence_point_at_time(lookahead_time);

        let sequence_point = match sequence_point {
            Some(sequence_point) => sequence_point,
            None => return,
        };

        if !sequence_point.data.metronome {
            return;
        }

        let start = *current_time.max(&self.last_scheduled_time);

        self.last_scheduled_time = lookahead_time;

        if lookahead_time <= start {
            return;
        }

        let mut beat_index = last_beat_before(&start, &sequence_point.start_time, &sequence_point.data.tempo);
        let mut beat_position = sequence_point
            .start_time
            .incremented_by_beats(beat_index as f64, sequence_point.data.tempo.get_bpm());

        while beat_position < lookahead_time {
            if start <= beat_position && beat_position < lookahead_time {
                let frequency = if beat_index % 4 == 0 {
                    BAR_FREQUENCY
                } else {
                    BEAT_FREQUENCY
                };

                self.oscillator.frequency.set_value_at_time(frequency, beat_position);
                self.adsr.note_on_at_time(beat_position);
            }

            beat_position = beat_position.incremented_by_beats(1.0, sequence_point.data.tempo.get_bpm());
            beat_index += 1;
        }
    }
}

fn last_beat_before(time: &Timestamp, sequence_start: &Timestamp, tempo: &Tempo) -> i32 {
    let beat_interval = 1.0 / tempo.beat_frequency();
    let elapsed = *time - *sequence_start;
    let elapsed = elapsed.as_seconds();
    let beats_elapsed = elapsed / beat_interval;
    beats_elapsed.floor() as i32
}
