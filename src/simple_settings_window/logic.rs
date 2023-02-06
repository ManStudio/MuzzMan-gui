use iced::{mouse::Button, Command};

use crate::application::MuzzManSimpleSettings;

#[derive(Debug)]
pub enum Message {
    Close,
    Mimimize,
    Maximize,
    Event(iced::Event),
    Tick(iced::time::Instant),
    Command(Command<Message>),
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
            Message::Event(event) => match event {
                iced::Event::Mouse(mouse) => {
                    println!("Mouse: {mouse:?}");
                    match mouse {
                        iced::mouse::Event::CursorMoved { position } => {
                            app.mouse_last_position = app.mouse_position;
                            app.mouse_position = position;
                        }
                        iced::mouse::Event::ButtonPressed(button) => match button {
                            Button::Left => {
                                // if app.mouse_position.y <= 40.0 {
                                //     return iced::window::drag();
                                // }
                            }
                            Button::Right => {}
                            Button::Middle => {}
                            Button::Other(_) => {}
                        },
                        _ => {}
                    };
                }
                _ => {}
            },
            Message::Tick(_) => {}
            Message::Command(command) => return command,
        };
        Command::none()
    }
}
