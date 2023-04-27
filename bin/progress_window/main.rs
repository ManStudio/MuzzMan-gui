use application::MuzzManProgress;
use flags::Flags;
use iced::Application;

mod application;
mod flags;
mod logic;
mod render;

fn main() {
    MuzzManProgress::run(iced::Settings {
        id: None,
        window: iced::window::Settings {
            size: (70, 20),
            position: iced::window::Position::Centered,
            min_size: Some((1, 1)),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: false,
            transparent: false,
            always_on_top: true,
            icon: None,
            platform_specific: iced::window::PlatformSpecific,
        },
        flags: Flags::default(),
        default_font: None,
        default_text_size: 12.0,
        text_multithreading: false,
        antialiasing: true,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
    .unwrap();
}
