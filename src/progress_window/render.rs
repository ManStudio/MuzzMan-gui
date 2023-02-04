use iced::widget::{button, progress_bar};
use iced_native::widget::row;
use muzzman_iced::themes::ButtonStyle;

use crate::{application::MuzzManProgress, logic::Message};

impl MuzzManProgress {
    pub fn render(&self) -> iced::Element<Message, iced::Renderer> {
        let mut r = row(vec![progress_bar(0.0..=1.0, self.progress)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()]);
        if self.show_close {
            r = r.push(
                button("close")
                    .on_press(Message::Close)
                    .style(ButtonStyle::Flat.into())
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill),
            );
        }
        r.into()
    }
}
