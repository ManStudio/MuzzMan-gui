use iced::{widget::text_input::Appearance, Color};

pub enum TextInputStyle {
    Text,
}

impl iced::widget::text_input::StyleSheet for TextInputStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> Appearance {
        match self {
            TextInputStyle::Text => Appearance {
                background: iced::Background::Color(Color::from_rgb(0.1, 0.1, 0.1)),
                border_radius: 100.0,
                border_width: 1.0,
                border_color: Color::BLACK,
            },
        }
    }

    fn focused(&self, style: &Self::Style) -> Appearance {
        self.active(style)
    }

    fn placeholder_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.8, 0.8, 0.8)
    }

    fn value_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(1.0, 1.0, 1.0)
    }

    fn selection_color(&self, _style: &Self::Style) -> iced::Color {
        Color::from_rgb(0.7, 0.0, 0.0)
    }
}

impl From<TextInputStyle> for iced::theme::TextInput {
    fn from(val: TextInputStyle) -> Self {
        iced::theme::TextInput::Custom(Box::new(val))
    }
}
