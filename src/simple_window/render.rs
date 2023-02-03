use iced::{
    widget::{button, horizontal_space, progress_bar, svg, text, text_input, vertical_space},
    Element, Padding,
};
use iced_native::widget::container;
use muzzman_iced::themes::{ButtonStyle, ContainerStyle, TextInputStyle};

use crate::{logic::Message, MuzzManSimple};

impl MuzzManSimple {
    pub fn render(&self) -> Element<Message, iced::Renderer<iced::Theme>> {
        //
        // TopBar
        //

        let top_bar = {
            let close_icon_bytes = include_bytes!("../../close_button.svg");
            let close_icon = iced_native::svg::Handle::from_memory(&close_icon_bytes[..]);
            let close_svg = svg(close_icon);

            let close_button = button(close_svg)
                .on_press(Message::Close)
                .padding(Padding::from(3))
                .width(iced::Length::Units(38))
                .height(iced::Length::Units(38))
                .style(ButtonStyle::Flat.into());

            let mimimize_icon_bytes = include_bytes!("../../mimimize_button.svg");
            let mimimize_icon = iced_native::svg::Handle::from_memory(&mimimize_icon_bytes[..]);
            let mimimize_svg = svg(mimimize_icon);

            let mimimize_button = button(mimimize_svg)
                .on_press(Message::Minimize)
                .padding(Padding::from(3))
                .width(iced::Length::Units(38))
                .height(iced::Length::Units(38))
                .style(ButtonStyle::Flat.into());

            let bigger_icon_bytes = include_bytes!("../../bigger_button.svg");
            let bigger_icon = iced_native::svg::Handle::from_memory(&bigger_icon_bytes[..]);
            let bigger_svg = svg(bigger_icon);

            let bigger_button = button(bigger_svg)
                .on_press(Message::SimpleSettingsOrManager)
                .padding(Padding::from(3))
                .width(iced::Length::Units(38))
                .height(iced::Length::Units(38))
                .style(ButtonStyle::Flat.into());

            let progress_top_bar = progress_bar(0.0..=1.0, self.progress).width(iced::Length::Fill);
            let progress_top_bar = iced::widget::column(vec![progress_top_bar.into()])
                .padding(Padding::from(3))
                .width(iced::Length::Fill);

            let top_bar = iced::widget::row(vec![
                bigger_button.into(),
                progress_top_bar.into(),
                mimimize_button.into(),
                horizontal_space(iced::Length::Units(5)).into(),
                close_button.into(),
            ]);

            let top_bar = iced::widget::column(vec![top_bar.into()]).width(iced::Length::Fill);

            container(top_bar)
                .width(iced::Length::Fill)
                .style(ContainerStyle::Bar)
                .height(iced::Length::Units(40))
                .center_y()
        };

        //
        // Body
        //

        let body = {
            let url_text = text_input("Url: ", &self.url, Message::ChangeUrl)
                .size(21)
                .on_submit(Message::DownloadOrStop)
                .style(TextInputStyle::Text);

            let text_for_download_btn = if self.downloading { "Stop" } else { "Download" };
            let download_button = button(text(text_for_download_btn).size(21))
                .style(ButtonStyle::Download.into())
                .on_press(Message::DownloadOrStop)
                .height(iced::Length::Units(30));

            iced::widget::row(vec![url_text.into(), download_button.into()]).spacing(5)
        };

        //
        // Footer
        //

        let footer = {
            let status = text(&self.status).size(12);
            container(status)
                .style(ContainerStyle::Bar)
                .width(iced::Length::Fill)
                .center_x()
        };

        let raws = iced::widget::column(vec![
            top_bar.into(),
            vertical_space(iced::Length::Fill).into(),
            body.into(),
            vertical_space(iced::Length::Fill).into(),
            footer.into(),
        ]);
        container(raws).into()
    }
}
