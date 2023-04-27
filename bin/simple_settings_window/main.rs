use application::MuzzManSimpleSettings;
use flags::Flags;
use iced::{Application, Settings};

mod application;
mod flags;
mod logic;
mod render;

fn main() {
    MuzzManSimpleSettings::run(Settings {
        id: None,
        window: iced::window::Settings {
            size: (500, 300),
            position: iced::window::Position::Centered,
            min_size: Some((100, 100)),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: false,
            transparent: false,
            always_on_top: false,
            icon: None,
            platform_specific: iced::window::PlatformSpecific,
        },
        flags: Flags::default(),
        default_font: None,
        default_text_size: 12.0,
        text_multithreading: false,
        antialiasing: false,
        exit_on_close_request: false,
        try_opengles_first: false,
    })
    .unwrap();
}
