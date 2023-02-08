use iced::Command;

use crate::application::MuzzManInstaller;

#[derive(Debug)]
pub enum Message {
    Command(Command<Message>),
    Close,
    Mimimize,
}

unsafe impl Send for Message {}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::Close => Self::Close,
            Message::Mimimize => Self::Mimimize,
            Message::Command(_) => todo!(),
        }
    }
}

impl Message {
    pub fn process(self, app: &mut MuzzManInstaller) -> Command<Message> {
        match self {
            Message::Command(command) => command,
            Message::Close => iced::window::close(),
            Message::Mimimize => iced::window::minimize(true),
        }
    }
}
