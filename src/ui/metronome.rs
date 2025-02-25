use iced::{widget::row, Color, Element};

use crate::model::{PlaybackState, PlayingState, Progress};

use super::{constants::display_units, message::Message};

pub fn metronome(playback_state: &PlaybackState, progress: &Progress) -> Element<'static, Message> {
    let beat = (progress.section_beat % 4.0).floor() as i64;
    let is_playing = playback_state.playing == PlayingState::Playing;

    row((0..4).map(|beat_index| {
        let is_active = is_playing && beat_index == beat;
        let size = display_units(4.0);
        let color = match (is_playing, is_active) {
            (true, true) => Color::from_rgb8(0x32, 0xD9, 0x87),
            (true, false) => Color::from_rgb8(0xDC, 0x30, 0x1A),
            (false, _) => Color::from_rgb(0.1, 0.1, 0.1),
        };

        circle::circle(size).with_color(color).into()
    }))
    .spacing(display_units(2.0))
    .into()
}

mod circle {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::border;
    use iced::mouse;
    use iced::{Color, Element, Length, Rectangle, Size};

    pub struct Circle {
        radius: f32,
        color: iced::Color,
    }

    impl Circle {
        pub fn new(radius: f32) -> Self {
            Self {
                radius,
                color: Color::WHITE,
            }
        }

        pub fn with_color(mut self, color: Color) -> Self {
            self.color = color;
            self
        }
    }

    pub fn circle(radius: f32) -> Circle {
        Circle::new(radius)
    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
    where
        Renderer: renderer::Renderer,
    {
        fn size(&self) -> Size<Length> {
            Size {
                width: Length::Shrink,
                height: Length::Shrink,
            }
        }

        fn layout(&self, _tree: &mut widget::Tree, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
            layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
        }

        fn draw(
            &self,
            _state: &widget::Tree,
            renderer: &mut Renderer,
            _theme: &Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor: mouse::Cursor,
            _viewport: &Rectangle,
        ) {
            renderer.fill_quad(
                renderer::Quad {
                    bounds: layout.bounds(),
                    border: border::rounded(self.radius),
                    ..renderer::Quad::default()
                },
                self.color,
            );
        }
    }

    impl<Message, Theme, Renderer> From<Circle> for Element<'_, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(circle: Circle) -> Self {
            Self::new(circle)
        }
    }
}
