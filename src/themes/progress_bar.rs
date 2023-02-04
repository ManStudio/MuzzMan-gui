use iced::{Background, Color};

use super::Colors;

pub enum ProgressBarStyle {
    Normal,
}

impl iced::widget::progress_bar::StyleSheet for ProgressBarStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::progress_bar::Appearance {
        match self {
            Self::Normal => iced::widget::progress_bar::Appearance {
                background: Background::Color(Colors::default().deep_background),
                bar: Background::Color(Colors::default().seccundary),
                border_radius: 100.0,
            },
        }
    }
}

impl From<ProgressBarStyle> for iced::theme::ProgressBar {
    fn from(val: ProgressBarStyle) -> Self {
        iced::theme::ProgressBar::Custom(Box::new(val))
    }
}
