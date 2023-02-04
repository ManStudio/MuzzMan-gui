use iced::{mouse::Button, Command};

use crate::application::MuzzManSimpleSettings;

#[derive(Clone, Debug)]
pub enum Message {
    Close,
    Mimimize,
    Maximize,
    Event(iced::Event),
    Tick(iced::time::Instant),
}

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
            Message::Event(event) => {
                println!("Event: {event:?}");
                match event {
                    iced::Event::Mouse(iced::mouse::Event::ButtonPressed(Button::Left)) => {
                        return Command::single(iced_native::command::Action::Window(
                            iced_native::window::Action::Drag,
                        ));
                    }
                    iced::Event::Mouse(mouse) => {
                        println!("Mouse: {mouse:?}")
                    }
                    _ => {}
                }
            }
            Message::Tick(_) => {}
        };
        Command::none()
    }
}
