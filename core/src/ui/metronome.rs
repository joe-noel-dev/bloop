use iced::{widget::row, Color, Element};

use crate::model::{PlaybackState, PlayingState, Progress};

use super::{constants::display_units, message::Message, theme};

pub fn metronome(playback_state: &PlaybackState, progress: &Progress) -> Element<'static, Message> {
    let beat = (progress.section_beat % 4.0).floor() as i64;
    let is_playing = playback_state.playing.enum_value_or_default() == PlayingState::PLAYING;

    row((0..4).map(|beat_index| {
        let is_active = is_playing && beat_index == beat;
        let size = display_units(8.0);
        let color = match (is_playing, is_active) {
            (true, true) => theme::PRIMARY,
            (true, false) => theme::palette::COLOR_4,
            (false, _) => theme::neutral::N6,
        };
        let border_radius = display_units(1.0);

        square::square(size)
            .with_color(color)
            .with_border_radius(border_radius)
            .into()
    }))
    .spacing(display_units(2.0))
    .into()
}

mod square {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::renderer;
    use iced::advanced::widget::{self, Widget};
    use iced::border;
    use iced::mouse;
    use iced::{Color, Element, Length, Rectangle, Size};

    pub struct Square {
        side_length: f32,
        border_radius: f32,
        color: iced::Color,
    }

    impl Square {
        pub fn new(side_length: f32) -> Self {
            Self {
                side_length,
                border_radius: 0.0,
                color: Color::WHITE,
            }
        }

        pub fn with_color(mut self, color: Color) -> Self {
            self.color = color;
            self
        }

        pub fn with_border_radius(mut self, radius: f32) -> Self {
            self.border_radius = radius;
            self
        }
    }

    pub fn square(side_length: f32) -> Square {
        Square::new(side_length)
    }

    impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Square
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
            layout::Node::new(Size::new(self.side_length, self.side_length))
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
                    border: border::rounded(self.border_radius),
                    ..renderer::Quad::default()
                },
                self.color,
            );
        }
    }

    impl<Message, Theme, Renderer> From<Square> for Element<'_, Message, Theme, Renderer>
    where
        Renderer: renderer::Renderer,
    {
        fn from(square: Square) -> Self {
            Self::new(square)
        }
    }
}
