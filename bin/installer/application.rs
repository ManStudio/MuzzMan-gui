use iced::{Application, Command};

use crate::{flags::Flags, logic::Message};

pub struct MuzzManInstaller {
    // this means that is the full repo with all the application and src
    // if is false will be downloaded from the internet
    pub local: bool,
    pub output_log: String,
}

impl Application for MuzzManInstaller {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let command = if let Some(command) = flags.command {
            match command {
                crate::flags::Command::Install => {
                    Command::perform(do_nothing(), |_| Message::Install)
                }
                crate::flags::Command::Uninstall => {
                    Command::perform(do_nothing(), |_| Message::UnInstall)
                }
            }
        } else {
            Command::none()
        };

        (
            Self {
                local: flags.local,
                output_log: "First Log".into(),
            },
            command,
        )
    }

    fn title(&self) -> String {
        String::from("MuzzMan Installer")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        message.process(self)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        self.render()
    }
}

// this is needed becaus in current version of rust cannot create async clasure
pub async fn do_nothing() {}
