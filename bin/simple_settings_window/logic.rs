use iced::{mouse::Button, Command};

use crate::application::MuzzManSimpleSettings;

#[derive(Debug)]
pub enum Message {
    Close,
    Mimimize,
    Maximize,
    Event(iced::Event),
    Tick(iced::time::Instant),
    Save,
    Load,
    Command(Command<Message>),
}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::Close => Message::Close,
            Message::Mimimize => Message::Mimimize,
            Message::Maximize => Message::Maximize,
            Message::Event(event) => Message::Event(event.clone()),
            Message::Tick(tick) => Message::Tick(*tick),
            Message::Save => Message::Save,
            Message::Load => Message::Load,
            Message::Command(_) => todo!(),
        }
    }
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

impl Message {
    pub fn process(self, app: &mut MuzzManSimpleSettings) -> Command<Message> {
        match self {
            Message::Close => {
                return Command::single(iced_native::command::Action::Window(
                    iced_native::window::Action::Close,
                ))
            }
            Message::Mimimize => {
                return Command::single(iced_native::command::Action::Window(
                    iced_native::window::Action::Minimize(true),
                ))
            }
            Message::Maximize => {
                return Command::single(iced_native::command::Action::Window(
                    iced_native::window::Action::Maximize(true),
                ))
            }
            Message::Event(_event) => {}
            Message::Tick(_) => {}
            Message::Command(command) => return command,
            Message::Save => app.config.update(),
            Message::Load => app.config.reload(),
        };
        Command::none()
    }
}
