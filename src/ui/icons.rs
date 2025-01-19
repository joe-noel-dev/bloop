use iced::widget::{svg, Svg};

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    ArrowLeft,
    ArrowRight,
    Play,
    Stop,
}

impl Icon {
    pub fn to_svg(self) -> Svg<'static> {
        let bytes = match self {
            Icon::ArrowLeft => include_bytes!("./resources/arrow-left.svg").as_slice(),
            Icon::ArrowRight => include_bytes!("./resources/arrow-right.svg").as_slice(),
            Icon::Play => include_bytes!("./resources/play.svg").as_slice(),
            Icon::Stop => include_bytes!("./resources/stop.svg").as_slice(),
        };

        svg(svg::Handle::from_memory(bytes))
    }

    pub fn to_svg_with_size(self, dimension: f32) -> Svg<'static> {
        self.to_svg()
            .width(dimension)
            .height(dimension)
            .style(|theme, _status| svg::Style {
                color: Some(theme.palette().text),
            })
    }
}
