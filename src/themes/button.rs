use iced::{Color, Vector};

pub enum ButtonStyle {
    Flat,
    Download,
}

impl iced::widget::button::StyleSheet for ButtonStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        match self {
            ButtonStyle::Flat => iced::widget::button::Appearance {
                shadow_offset: Vector::new(0.0, 0.0),
                background: None,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::BLACK,
                text_color: Color::WHITE,
            },
            ButtonStyle::Download => iced::widget::button::Appearance {
                shadow_offset: Vector::new(0.0, 0.0),
                background: Some(iced::Background::Color(Color::from_rgb8(0, 125, 0))),
                border_radius: 100.0,
                border_width: 0.0,
                border_color: Color::BLACK,
                text_color: Color::WHITE,
            },
        }
    }
}

impl From<ButtonStyle> for iced::theme::Button {
    fn from(val: ButtonStyle) -> Self {
        iced::theme::Button::Custom(Box::new(val))
    }
}
