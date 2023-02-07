use iced::Command;
use iced_native::{command::Action, window};
use muzzman_daemon::prelude::{TElement, TLocation};

use crate::MuzzManSimple;

#[derive(Debug)]
pub enum Message {
    Close,
    Minimize,
    Settings,
    Morph,
    ChangeUrl(String),
    DownloadOrStop,
    Event(iced::Event),
    Tick(iced::time::Instant),
    Command(Command<Message>),
}

unsafe impl Sync for Message {}
unsafe impl Send for Message {}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::Close => Message::Close,
            Message::Minimize => Message::Minimize,
            Message::Settings => Message::Settings,
            Message::Morph => Message::Morph,
            Message::ChangeUrl(url) => Message::ChangeUrl(url.clone()),
            Message::DownloadOrStop => Message::DownloadOrStop,
            Message::Event(event) => Message::Event(event.clone()),
            Message::Tick(tick) => Message::Tick(*tick),
            Message::Command(_) => todo!(),
        }
    }
}

impl Message {
    pub fn progess(self, app: &mut MuzzManSimple) -> Command<Message> {
        match self {
            Message::Close => return Command::single(Action::Window(window::Action::Close)),
            Message::Minimize => {
                return Command::single(Action::Window(window::Action::Minimize(true)))
            }
            Message::ChangeUrl(new) => app.url = new,
            Message::DownloadOrStop => {
                if app.downloading {
                    if let Some(element) = &app.element {
                        element.set_enabled(false, None).unwrap()
                    }
                } else {
                    let last = app.element.take();
                    if let Some(last) = last {
                        last.set_enabled(false, None).unwrap();
                        if app.config.destroy_element {
                            let _ = last.destroy();
                        }
                    }

                    let location = app
                        .session
                        .get_location_ref(&app.config.location_id)
                        .unwrap();
                    let Ok(filename) = app.get_filename()else{
                        app.status = String::from("Cannot resolv filename!");
                        return Command::none()};
                    let new_element = location.create_element(&filename).unwrap();
                    new_element.set_url(Some(app.url.clone())).unwrap();
                    if new_element.resolv_module().unwrap() {
                        new_element.init().unwrap();
                        new_element.set_enabled(true, None).unwrap();
                        app.element = Some(new_element);
                    } else {
                        app.status = String::from(
                            "Cannot resolv element that means you typed an invalid url!",
                        );
                        let _ = new_element.destroy();
                    }
                }
            }
            Message::Event(_event) => {}
            Message::Tick(_) => {
                if let Some(element) = &app.element {
                    match element.get_status_msg() {
                        Ok(status) => app.status = status,
                        Err(_) => {
                            app.status = String::from("No element!");
                            let _ = app.element.take();
                            return Command::none();
                        }
                    };
                    app.progress = element.get_progress().unwrap();
                    if element.is_enabled().unwrap() {
                        app.downloading = true
                    } else {
                        app.downloading = false;
                    }
                } else {
                    app.progress = 1.0;
                    app.downloading = false;
                }
            }
            Message::Settings => todo!(),
            Message::Morph => todo!(),
            Message::Command(command) => return command,
        }
        Command::none()
    }
}
