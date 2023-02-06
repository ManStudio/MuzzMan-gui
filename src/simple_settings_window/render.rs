use iced::widget::{button, column, container, horizontal_space, row, svg, text, vertical_space};
use muzzman_iced::{
    themes::{ButtonStyle, ContainerStyle},
    widgets::top_bar::TopBar,
};

use crate::{application::MuzzManSimpleSettings, logic::Message};

impl MuzzManSimpleSettings {
    pub fn render(&self) -> iced::Element<Message, iced::Renderer> {
        let top_bar = {
            let minimize_svg = iced_native::svg::Handle::from_memory(
                &include_bytes!("../../Minimize Button.svg")[..],
            );
            let maximize_svg = iced_native::svg::Handle::from_memory(
                &include_bytes!("../../Maximize Button.svg")[..],
            );
            let close_svg = iced_native::svg::Handle::from_memory(
                &include_bytes!("../../Close Button.svg")[..],
            );

            let minimize_svg = svg(minimize_svg);
            let maximize_svg = svg(maximize_svg);
            let close_svg = svg(close_svg);

            let close_button = button(close_svg)
                .style(ButtonStyle::Flat.into())
                .width(iced::Length::Units(36))
                .height(iced::Length::Units(36))
                .on_press(Message::Close);
            let minimize_button = button(minimize_svg)
                .style(ButtonStyle::Flat.into())
                .width(iced::Length::Units(36))
                .height(iced::Length::Units(36))
                .on_press(Message::Mimimize);
            let maximize_button = button(maximize_svg)
                .style(ButtonStyle::Flat.into())
                .width(iced::Length::Units(36))
                .height(iced::Length::Units(36))
                .on_press(Message::Maximize);

            let content = container(row(vec![
                horizontal_space(iced::Length::Fill).into(),
                minimize_button.into(),
                maximize_button.into(),
                close_button.into(),
            ]))
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Units(40));
            TopBar::new(content, Message::Command)
        };

        let body = {
            container(column(vec![row(vec![
                text("Testing: ").into(),
                text("Value").into(),
            ])
            .into()]))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
        };

        let footer = {
            container(row(vec![
                horizontal_space(iced::Length::Fill).into(),
                button("Load").on_press(Message::Load).into(),
                button("Save").on_press(Message::Save).into(),
            ]))
            .width(iced::Length::Fill)
            .height(iced::Length::Units(30))
            .center_y()
            .style(ContainerStyle::Bar)
        };

        container(column(vec![top_bar.into(), body.into(), footer.into()]))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
