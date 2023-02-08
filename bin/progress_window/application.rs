use std::time::Duration;

use iced::{Application, Command};
use muzzman_daemon::{
    common::get_muzzman_dir,
    prelude::{ERef, ElementId, TSession},
    DaemonSession,
};
use muzzman_iced::config::{Config, WrapConfig};

use crate::{
    flags::{self, Flags},
    logic::{self, Message},
};

pub struct MuzzManProgress {
    pub config: WrapConfig<Config>,
    pub session: Box<dyn TSession>,
    pub element: Option<ERef>,
    pub progress: f32,
    pub show_close: bool,
}

impl Application for MuzzManProgress {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let config_path = if let Some(config_path) = flags.config_path {
            config_path
        } else {
            get_muzzman_dir().join("iced").join("config.toml")
        };

        let session = DaemonSession::new().unwrap();
        let session = session.create_session();
        let mut element = None;

        let config = WrapConfig::load(&config_path).expect("Invalid config!");

        match flags.command {
            flags::Command::Info { element_id } => {
                let id: ElementId =
                    serde_json::from_str(&element_id).expect("Invalid element id format!");
                element = Some(session.get_element_ref(&id).expect("Invalid element"));
            }
        };

        let res = Self {
            config,
            session,
            element,
            progress: 0.0,
            show_close: false,
        };
        (res, Command::none())
    }

    fn title(&self) -> String {
        String::from("MuzzMan Progress")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        message.process(self)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        self.render()
    }

    fn theme(&self) -> Self::Theme {
        muzzman_iced::themes::get_theme()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let subscriptions = [
            iced::subscription::events().map(Message::Event),
            iced::time::every(iced::time::Duration::from_millis(self.config.tick))
                .map(Message::Tick),
        ];
        iced::Subscription::batch(subscriptions)
    }
}
