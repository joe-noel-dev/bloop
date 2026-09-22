use iced::Size;

pub fn display_units(scale: f32) -> f32 {
    scale * 8.0
}

const REFERENCE_WIDTH: f32 = 1024.0;
const REFERENCE_HEIGHT: f32 = 600.0;
const MIN_CONTENT_SCALE: f32 = 0.75;
const MAX_CONTENT_SCALE: f32 = 1.25;
const MIN_TOUCH_TARGET: f32 = 48.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeClass {
    Compact,
    Regular,
    Spacious,
}

#[derive(Debug, Clone, Copy)]
pub struct UiMetrics {
    pub size_class: SizeClass,
    pub scale: f32,
}

impl UiMetrics {
    pub fn from_viewport(viewport: Size) -> Self {
        let size_class = if viewport.height < 560.0 {
            SizeClass::Compact
        } else if viewport.height < 760.0 {
            SizeClass::Regular
        } else {
            SizeClass::Spacious
        };

        let width_scale = viewport.width / REFERENCE_WIDTH;
        let height_scale = viewport.height / REFERENCE_HEIGHT;
        let scale = width_scale
            .min(height_scale)
            .clamp(MIN_CONTENT_SCALE, MAX_CONTENT_SCALE);

        Self { size_class, scale }
    }

    pub fn spacing(self, units: f32) -> f32 {
        let density = match self.size_class {
            SizeClass::Compact => 0.75,
            SizeClass::Regular => 1.0,
            SizeClass::Spacious => 1.1,
        };

        display_units(units) * self.scale * density
    }

    pub fn touch_target(self) -> f32 {
        let base = match self.size_class {
            SizeClass::Compact => 56.0,
            SizeClass::Regular => 64.0,
            SizeClass::Spacious => 72.0,
        };

        (base * self.scale).max(MIN_TOUCH_TARGET)
    }

    pub fn control_icon(self) -> f32 {
        let base = match self.size_class {
            SizeClass::Compact => 32.0,
            SizeClass::Regular => 40.0,
            SizeClass::Spacious => 44.0,
        };

        (base * self.scale).clamp(24.0, 52.0)
    }

    pub fn utility_icon(self) -> f32 {
        (24.0 * self.scale).clamp(20.0, 30.0)
    }

    pub fn song_text(self) -> f32 {
        let base = match self.size_class {
            SizeClass::Compact => 56.0,
            SizeClass::Regular => 64.0,
            SizeClass::Spacious => 72.0,
        };

        (base * self.scale).clamp(40.0, 80.0)
    }

    pub fn section_text(self) -> f32 {
        let base = match self.size_class {
            SizeClass::Compact => 48.0,
            SizeClass::Regular => 56.0,
            SizeClass::Spacious => 64.0,
        };

        (base * self.scale).clamp(34.0, 72.0)
    }

    pub fn beat_size(self) -> f32 {
        let base = match self.size_class {
            SizeClass::Compact => 52.0,
            SizeClass::Regular => 64.0,
            SizeClass::Spacious => 72.0,
        };

        (base * self.scale).clamp(40.0, 80.0)
    }
}

#[cfg(test)]
mod tests {
    use super::{SizeClass, UiMetrics, MIN_TOUCH_TARGET};
    use iced::Size;

    #[test]
    fn eight_hundred_by_four_eighty_uses_compact_metrics() {
        let metrics = UiMetrics::from_viewport(Size::new(800.0, 480.0));

        assert_eq!(metrics.size_class, SizeClass::Compact);
        assert!(metrics.song_text() < 64.0);
        assert!(metrics.section_text() < 64.0);
        assert!(metrics.touch_target() >= MIN_TOUCH_TARGET);
    }

    #[test]
    fn reference_viewport_uses_regular_unscaled_metrics() {
        let metrics = UiMetrics::from_viewport(Size::new(1024.0, 600.0));

        assert_eq!(metrics.size_class, SizeClass::Regular);
        assert_eq!(metrics.scale, 1.0);
        assert_eq!(metrics.touch_target(), 64.0);
    }

    #[test]
    fn content_scale_is_bounded_for_extreme_window_sizes() {
        let small = UiMetrics::from_viewport(Size::new(320.0, 200.0));
        let large = UiMetrics::from_viewport(Size::new(3840.0, 2160.0));

        assert_eq!(small.scale, 0.75);
        assert_eq!(large.scale, 1.25);
        assert_eq!(small.touch_target(), MIN_TOUCH_TARGET);
    }
}
