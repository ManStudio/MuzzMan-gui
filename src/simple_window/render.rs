use iced::{
    widget::{button, horizontal_space, progress_bar, svg, text, text_input, vertical_space},
    Element, Padding,
};
use iced_native::widget::container;
use muzzman_iced::{
    themes::{ButtonStyle, ContainerStyle, ProgressBarStyle, TextInputStyle},
    widgets::top_bar::TopBar,
};

use crate::{logic::Message, MuzzManSimple};

impl MuzzManSimple {
    pub fn render(&self) -> Element<Message, iced::Renderer<iced::Theme>> {
        //
        // TopBar
        //

        let top_bar = {
            let settings_icon_bytes = include_bytes!("../../Settings.svg");
            let settings_icon = iced_native::svg::Handle::from_memory(&settings_icon_bytes[..]);
            let settings_svg = svg(settings_icon);

            let settings_button = button(settings_svg)
                .on_press(Message::Settings)
                .width(iced::Length::Units(38))
                .height(iced::Length::Units(38))
                .style(ButtonStyle::Flat.into());

            let morph_icon_bytes = include_bytes!("../../Morph.svg");
            let morph_icon = iced_native::svg::Handle::from_memory(&morph_icon_bytes[..]);
            let morph_svg = svg(morph_icon);

            let morph_button = button(morph_svg)
                .on_press(Message::Morph)
                .width(iced::Length::Units(38))
                .height(iced::Length::Units(38))
                .style(ButtonStyle::Flat.into());

            let progress_top_bar = progress_bar(0.0..=1.0, self.progress)
                .width(iced::Length::Fill)
                .style(ProgressBarStyle::Normal);
            let progress_top_bar = iced::widget::column(vec![progress_top_bar.into()])
                .padding(Padding {
                    top: 5,
                    right: 0,
                    bottom: 0,
                    left: 5,
                })
                .width(iced::Length::Fill)
                .height(iced::Length::Fill);

            let mimimize_icon_bytes = include_bytes!("../../Minimize Button.svg");
            let mimimize_icon = iced_native::svg::Handle::from_memory(&mimimize_icon_bytes[..]);
            let mimimize_svg = svg(mimimize_icon);

            let mimimize_button = button(mimimize_svg)
                .on_press(Message::Minimize)
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

            let top_bar = iced::widget::row(vec![
                settings_button.into(),
                morph_button.into(),
                progress_top_bar.into(),
                mimimize_button.into(),
                close_button.into(),
            ]);

            let top_bar = iced::widget::column(vec![top_bar.into()]).width(iced::Length::Fill);

            let content = container(top_bar)
                .width(iced::Length::Fill)
                .style(ContainerStyle::Bar)
                .height(iced::Length::Units(40))
                .center_y();
            TopBar::new(content, Message::Command)
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
            let status = text(&self.status).size(14);
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
