use iced::{Application, Command, Point};
use muzzman_daemon::{common::get_muzzman_dir, prelude::TSession, DaemonSession};
use muzzman_iced::config::{Config, WrapConfig};

use crate::{flags::Flags, logic::Message};

pub struct MuzzManSimpleSettings {
    pub config: WrapConfig<Config>,
    pub session: Box<dyn TSession>,
}

impl Application for MuzzManSimpleSettings {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let session = DaemonSession::new().unwrap().create_session();

        let config_path = if let Some(config_path) = flags.config {
            config_path
        } else {
            get_muzzman_dir().join("iced").join("config.toml")
        };

        let config = WrapConfig::<Config>::load(&config_path).unwrap();
        (Self { config, session }, Command::none())
    }

    fn title(&self) -> String {
        String::from("MuzzMan Simple Settings")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        message.process(self)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        self.render()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let sub = [iced::subscription::events().map(Message::Event)];
        iced::Subscription::batch(sub)
    }
}
