use iced::{mouse::Button, Command};
use muzzman_daemon::prelude::TElement;

use crate::application::MuzzManProgress;

#[derive(Debug, Clone)]
pub enum Message {
    Event(iced::Event),
    Tick(iced::time::Instant),
    Close,
}

impl Message {
    pub fn process(self, app: &mut MuzzManProgress) -> Command<Message> {
        match self {
            Message::Event(event) => {
                if let iced::Event::Mouse(iced::mouse::Event::ButtonPressed(button)) = event {
                    match button {
                        Button::Left => {
                            return Command::single(iced_native::command::Action::Window(
                                iced_native::window::Action::Drag,
                            ))
                        }
                        Button::Right => {
                            app.show_close = !app.show_close;
                        }
                        _ => {}
                    }
                }
            }
            Message::Tick(_) => {
                if let Some(element) = &app.element {
                    if let Ok(progress) = element.get_progress() {
                        app.progress = progress
                    } else {
                        return Command::single(iced_native::command::Action::Window(
                            iced_native::window::Action::Close,
                        ));
                    }
                }
            }
            Message::Close => {
                return Command::single(iced_native::command::Action::Window(
                    iced_native::window::Action::Close,
                ))
            }
        }
        Command::none()
    }
}
