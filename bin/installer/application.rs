use std::pin::Pin;

use iced::{Application, Command};

use crate::{flags::Flags, logger::Logger, logic::Message, task_manager::TaskManager};

pub struct MuzzManInstaller {
    // this means that is the full repo with all the application and src
    // if is false will be downloaded from the internet
    pub local: bool,
    pub output_log: String,
    pub log_reciver: std::sync::mpsc::Receiver<String>,
    pub installer: TaskManager,
}

impl Application for MuzzManInstaller {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let command = if let Some(command) = flags.command {
            match command {
                crate::flags::Command::Install => Command::perform(async {}, |_| Message::Install),
                crate::flags::Command::Uninstall => {
                    Command::perform(async {}, |_| Message::UnInstall)
                }
            }
        } else {
            Command::none()
        };

        let (log_sender, log_reciver) = std::sync::mpsc::channel::<String>();

        let installer = TaskManager::new(log_sender);

        (
            Self {
                local: flags.local,
                output_log: "First Log".into(),
                installer,
                log_reciver,
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
