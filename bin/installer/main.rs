use application::MuzzManInstaller;
use flags::Flags;
use iced::Application;

mod application;
mod flags;
mod logger;
mod logic;
mod render;
mod task_manager;

fn main() {
    let flags = Flags::default();

    MuzzManInstaller::run(iced::Settings {
        id: None,
        window: iced::window::Settings {
            size: (700, 400),
            position: iced::window::Position::Centered,
            min_size: None,
            max_size: None,
            visible: true,
            resizable: true,
            decorations: false,
            transparent: false,
            always_on_top: false,
            icon: None,
        },
        flags,
        default_font: None,
        default_text_size: 21.0,
        text_multithreading: false,
        antialiasing: false,
        exit_on_close_request: false,
        try_opengles_first: false,
    })
    .unwrap();
}
