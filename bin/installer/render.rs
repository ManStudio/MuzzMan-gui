use iced::widget::{
    button, checkbox, column, container, horizontal_space, row, scrollable, svg, text,
    vertical_space,
};
use muzzman_iced::{
    themes::{ButtonStyle, ContainerStyle},
    widgets::{progress_bar::ProgressBar, top_bar::TopBar},
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
                .padding(0)
                .style(ButtonStyle::Flat.into());

            let close_icon_bytes = include_bytes!("../../Close Button.svg");
            let close_icon = iced_native::svg::Handle::from_memory(&close_icon_bytes[..]);
            let close_svg = svg(close_icon);

            let close_button = button(close_svg)
                .padding(0)
                .style(ButtonStyle::Flat.into())
                .on_press(Message::Close);

            let progress_bar = ProgressBar::new(
                (self.installer.steps.len() as f32 - self.installer.to_do.len() as f32)
                    / self.installer.steps.len() as f32,
            );

            let mut buttons = vec![progress_bar.into(), mimimize_button.into()];

            if !self.should_close {
                buttons.push(close_button.into())
            }

            let top_bar =
                container(row(buttons).padding(5).spacing(3)).height(iced::Length::Units(40));
            TopBar::new(top_bar, Message::Command)
        };

        let body = {
            container(
                column(vec![scrollable(
                    row(vec![
                        text(&self.section_log).into(),
                        text(&self.output_log).into(),
                    ])
                    .spacing(20)
                    .padding(10),
                )
                .horizontal_scroll(iced::widget::scrollable::Properties::default())
                .id(self.output_scroll_id.clone())
                .height(iced::Length::Fill)
                .into()])
                .height(iced::Length::Fill)
                .width(iced::Length::Fill),
            )
            .height(iced::Length::Fill)
            .width(iced::Length::Fill)
        };

        let mut buttons = Vec::new();
        if self.installer.to_do.is_empty() {
            buttons.push(button("Install").on_press(Message::Install).into());
            buttons.push(button("UnInstall").on_press(Message::UnInstall).into())
        } else {
            // buttons.push(button("Stop").into())
        }
        buttons.push(checkbox("AutoScroll", self.auto_scroll, Message::ChangeAutoScroll).into());
        let status_bar = {
            container(
                column(vec![row(buttons).spacing(5).into()])
                    .width(iced::Length::Fill)
                    .align_items(iced::Alignment::Center),
            )
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
