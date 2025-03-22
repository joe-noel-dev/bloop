use crate::bloop::{WaveformData, WaveformGroup, WaveformProperties};

impl WaveformData {
    pub fn empty(sample_rate: i32) -> Self {
        Self {
            sample_rate,
            peaks: Vec::new(),
            ..Default::default()
        }
    }

    fn get_group_mut(&mut self, properties: &WaveformProperties) -> Option<&mut WaveformGroup> {
        self.peaks.iter_mut().find(|group| match group.properties.as_ref() {
            None => false,
            Some(group_properties) => group_properties == properties,
        })
    }

    pub fn sort(&mut self) {
        self.peaks.sort_by(|a, b| {
            let a = a.properties.as_ref().unwrap();
            let b = b.properties.as_ref().unwrap();
            let a_sort = (a.length, a.channel);
            let b_sort = (b.length, b.channel);
            a_sort.cmp(&b_sort)
        });
    }

    pub fn push(&mut self, properties: &WaveformProperties, value: f32) {
        match self.get_group_mut(properties) {
            None => {
                self.peaks.push(WaveformGroup {
                    properties: Some(properties.clone()).into(),
                    values: vec![value],
                    ..Default::default()
                });
            }
            Some(group) => group.values.push(value),
        }
    }

    pub fn add(&mut self, mut other: Self) {
        if other == *self {
            return;
        }

        if other.sample_rate != self.sample_rate {
            return;
        }

        self.peaks.append(&mut other.peaks);
    }
}
