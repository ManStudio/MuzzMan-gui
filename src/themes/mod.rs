mod button;
mod container;
mod progress_bar;
mod text_input;

pub use button::ButtonStyle;
pub use container::ContainerStyle;
use iced::Color;
pub use progress_bar::ProgressBarStyle;
pub use text_input::TextInputStyle;

fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: a as f32 / 255.0,
    }
}

// Colors
pub struct Colors {
    pub background: Color,
    pub deep_background: Color,
    pub text_background: Color,

    pub text: Color,
    pub primary: Color,
    pub seccundary: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            background: new(0x32, 0x32, 0x32, 0xff),
            deep_background: new(0x28, 0x28, 0x28, 0xff),
            text_background: new(0x1E, 0x1E, 0x1E, 0xff),
            text: new(0xff, 0xff, 0xff, 0xff),
            primary: new(0x14, 0xA0, 0x14, 0xff),
            seccundary: new(0xC8, 0x14, 0x14, 0xff),
        }
    }
}

// End Colors

pub fn get_theme() -> iced::Theme {
    let colors = Colors::default();
    iced::Theme::Custom(Box::new(iced::theme::Custom::new(iced::theme::Palette {
        background: colors.background,
        text: colors.text,
        primary: colors.primary,
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
