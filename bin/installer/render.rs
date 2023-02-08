use iced::widget::{button, column, container, horizontal_space, row, svg, text, vertical_space};
use muzzman_iced::{
    themes::{ButtonStyle, ContainerStyle},
    widgets::top_bar::TopBar,
};

use crate::{application::MuzzManInstaller, logic::Message};

impl MuzzManInstaller {
    pub fn render(&self) -> iced::Element<Message, iced::Renderer> {
        let top_bar = {
            let mimimize_icon_bytes = include_bytes!("../../Minimize Button.svg");
            let mimimize_icon = iced_native::svg::Handle::from_memory(&mimimize_icon_bytes[..]);
            let mimimize_svg = svg(mimimize_icon);

            let mimimize_button = button(mimimize_svg)
                .on_press(Message::Mimimize)
                .width(iced::Length::Units(38))
                .height(iced::Length::Units(38))
                .style(ButtonStyle::Flat.into());

            let close_icon_bytes = include_bytes!("../../Close Button.svg");
            let close_icon = iced_native::svg::Handle::from_memory(&close_icon_bytes[..]);
            let close_svg = svg(close_icon);

            let close_button = button(close_svg)
                .on_press(Message::Close)
                .width(iced::Length::Units(38))
                .height(iced::Length::Units(38))
                .style(ButtonStyle::Flat.into());

            let top_bar = container(row(vec![
                horizontal_space(iced::Length::Fill).into(),
                mimimize_button.into(),
                close_button.into(),
            ]));
            TopBar::new(top_bar, Message::Command)
        };

        let body = { container(text("Body")).height(iced::Length::Fill) };

        let status_bar = {
            container(text("Status Bar"))
                .style(ContainerStyle::Bar)
                .center_x()
                .center_y()
                .width(iced::Length::Fill)
                .height(iced::Length::Units(30))
        };

        container(column(vec![top_bar.into(), body.into(), status_bar.into()]))
            .style(ContainerStyle::Background)
            .into()
    }
}
