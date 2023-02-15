use iced::Command;

use crate::application::MuzzManInstaller;

#[derive(Debug)]
pub enum Message {
    Command(Command<Message>),
    Close,
    Mimimize,
    Install,
    UnInstall,
}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::Close => Self::Close,
            Message::Mimimize => Self::Mimimize,
            Message::Install => Self::Install,
            Message::UnInstall => Self::UnInstall,
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
            Message::Install => {
                // TODO: the first thing that should be done is to install rustup and rust
                // on the system
                if app.local {
                } else {
                    todo!("Downloading from the internet is not complited")
                }
                Command::none()
            }
            Message::UnInstall => {
                todo!("The UnInstallProcess is not implemented");
                Command::none()
            }
        }
    }
}
