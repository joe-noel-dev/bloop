use iced::{
    widget::{svg, Svg},
    ContentFit,
};

#[derive(Debug, Clone, Copy)]
pub enum Icon {
    ArrowLeft,
    ArrowRight,
    Play,
    Stop,
    Loop,
    #[allow(unused)]
    Metronome,
}

impl Icon {
    pub fn to_svg(self) -> Svg<'static> {
        let bytes = match self {
            Icon::ArrowLeft => include_bytes!("./resources/arrow-left.svg").as_slice(),
            Icon::ArrowRight => include_bytes!("./resources/arrow-right.svg").as_slice(),
            Icon::Play => include_bytes!("./resources/play.svg").as_slice(),
            Icon::Stop => include_bytes!("./resources/stop.svg").as_slice(),
            Icon::Loop => include_bytes!("./resources/loop.svg").as_slice(),
            Icon::Metronome => include_bytes!("./resources/metronome.svg").as_slice(),
        };

        svg(svg::Handle::from_memory(bytes))
    }

    pub fn to_svg_with_size(self, dimension: f32) -> Svg<'static> {
        self.to_svg()
            .width(dimension)
            .height(dimension)
            .content_fit(ContentFit::Contain)
            .style(|theme, _status| svg::Style {
                color: Some(theme.extended_palette().primary.base.text),
            })
    }
}
