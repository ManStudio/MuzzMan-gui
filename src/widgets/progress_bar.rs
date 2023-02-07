use iced::{Background, Color, Command, Element};
use iced_native::Widget;

use crate::themes::Colors;

pub struct ProgressBar<'a, Message> {
    pub progress: f32,
    pub on_command: Box<dyn Fn(Command<Message>) -> Message + 'a>,
    pub on_right: Option<Message>,
}

impl<'a, Message> ProgressBar<'a, Message> {
    pub fn new<F>(progress: f32, on_command: F) -> Self
    where
        F: 'a + Fn(Command<Message>) -> Message,
    {
        Self {
            progress,
            on_command: Box::new(on_command),
            on_right: None,
        }
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for ProgressBar<'a, Message>
where
    Renderer: iced_native::text::Renderer,
    Message: Clone,
{
    fn width(&self) -> iced::Length {
        iced::Length::Fill
    }

    fn height(&self) -> iced::Length {
        iced::Length::Units(38)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let width = <ProgressBar<Message> as iced_native::Widget<Message, Renderer>>::width(self);
        let height = <ProgressBar<Message> as iced_native::Widget<Message, Renderer>>::height(self);
        iced_native::layout::Node::new(limits.width(width).height(height).fill())
    }

    fn draw(
        &self,
        state: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &iced_native::renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            iced_native::renderer::Quad {
                bounds: layout.bounds(),
                border_radius: 100.0.into(),
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            Background::Color(Colors::default().deep_background),
        );

        let bounds = layout.bounds();

        renderer.fill_quad(
            iced_native::renderer::Quad {
                bounds: iced::Rectangle {
                    x: bounds.x,
                    y: bounds.y,
                    width: bounds.width * self.progress,
                    height: bounds.height,
                },
                border_radius: 100.0.into(),
                border_width: 0.0,
                border_color: Color::BLACK,
            },
            Background::Color(Colors::default().seccundary),
        );

        let x = layout.bounds().center_x();
        let y = layout.bounds().center_y();

        renderer.fill_text(iced_native::text::Text {
            content: &format!("{:.2}%", self.progress * 100.0),
            bounds: iced::Rectangle {
                x,
                y,
                ..layout.bounds()
            },
            size: 21.0,
            color: iced::Color::from_rgb8(20, 150, 20),
            font: Default::default(),
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
        });
    }
}

impl<'a, Message, Renderer> From<ProgressBar<'a, Message>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    Message: 'a + Clone,
{
    fn from(value: ProgressBar<'a, Message>) -> Self {
        Element::new(value)
    }
}
