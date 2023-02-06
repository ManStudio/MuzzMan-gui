use iced::{Application, Command, Point};

use crate::{flags::Flags, logic::Message};

pub struct MuzzManSimpleSettings {
    pub mouse_position: Point,
    pub mouse_last_position: Point,
}

impl Application for MuzzManSimpleSettings {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = Flags;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                mouse_position: Point::default(),
                mouse_last_position: Point::default(),
            },
            Command::none(),
        )
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
