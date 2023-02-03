mod button;
mod container;
mod text_input;

pub use button::ButtonStyle;
pub use container::ContainerStyle;
pub use text_input::TextInputStyle;

pub fn get_theme() -> iced::Theme {
    iced::Theme::Custom(Box::new(iced::theme::Custom::new(iced::theme::Palette {
        background: iced::Color {
            r: 0.126,
            g: 0.126,
            b: 0.126,
            a: 1.0,
        },
        text: iced::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        primary: iced::Color {
            r: 0.0,
            g: 0.4,
            b: 0.0,
            a: 1.0,
        },
        success: iced::Color {
            r: 0.0,
            g: 0.5,
            b: 0.0,
            a: 1.0,
        },
        danger: iced::Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    })))
}
