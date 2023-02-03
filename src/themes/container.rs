use iced::{widget::container::Appearance, Color};

pub enum ContainerStyle {
    Bar,
}

impl iced::widget::container::StyleSheet for ContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match self {
            ContainerStyle::Bar => Appearance {
                text_color: Some(Color::WHITE),
                background: Some(iced::Background::Color(Color::from_rgb(0.08, 0.08, 0.08))),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::BLACK,
            },
        }
    }
}

impl Into<iced::theme::Container> for ContainerStyle {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(self))
    }
}
