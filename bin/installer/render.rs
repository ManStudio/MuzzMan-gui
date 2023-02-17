use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, svg, text, vertical_space,
};
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

        let body = {
            container(
                column(vec![
                    scrollable(text(&self.output_log).width(iced::Length::Fill))
                        .height(iced::Length::Fill)
                        .into(),
                    column(vec![row(vec![
                        button("Install").on_press(Message::Install).into(),
                        button("UnInstall").on_press(Message::UnInstall).into(),
                    ])
                    .into()])
                    .width(iced::Length::Fill)
                    .align_items(iced::Alignment::Center)
                    .into(),
                ])
                .height(iced::Length::Fill)
                .width(iced::Length::Fill),
            )
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
        };

        let status_bar = {
            container(text("Status Bar"))
                .style(ContainerStyle::Bar)
                .center_x()
                .center_y()
                .width(iced::Length::Fill)
                .height(iced::Length::Units(30))
        };

        let content: iced::Element<Message, iced::Renderer<iced::Theme>> =
            container(column(vec![top_bar.into(), body.into(), status_bar.into()]))
                .style(ContainerStyle::Background)
                .into();

        if false {
            content.explain(iced::Color::from_rgb(0.9, 0.9, 0.9))
        } else {
            content
        }
    }
}
