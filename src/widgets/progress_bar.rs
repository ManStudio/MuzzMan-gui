use iced::{Background, Color, Command, Element};
use iced_native::Widget;

use crate::themes::Colors;

pub struct ProgressBar<Message> {
    pub progress: f32,
    pub on_right: Option<Message>,
}

impl<Message> ProgressBar<Message> {
    pub fn new(progress: f32) -> Self {
        Self {
            progress,
            on_right: None,
        }
    }

    pub fn on_right(self, on_right: Message) -> Self {
        Self {
            progress: self.progress,
            on_right: Some(on_right),
        }
    }
}

impl<Message, Renderer> Widget<Message, Renderer> for ProgressBar<Message>
where
    Renderer: iced_native::text::Renderer,
    Message: Clone,
{
    fn width(&self) -> iced::Length {
        iced::Length::Fill
    }

    fn height(&self) -> iced::Length {
        iced::Length::Fill
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

    fn on_event(
        &mut self,
        _state: &mut iced_native::widget::Tree,
        event: iced::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced::event::Status {
        if layout.bounds().contains(cursor_position) {
            if let iced::Event::Mouse(iced::mouse::Event::ButtonReleased(
                iced::mouse::Button::Right,
            )) = event
            {
                if let Some(message) = &self.on_right {
                    shell.publish(message.clone());
                    return iced::event::Status::Captured;
                }
            }
        }
        iced::event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _state: &iced_native::widget::Tree,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced_native::mouse::Interaction {
        iced_native::mouse::Interaction::Idle
    }
}

impl<'a, Message, Renderer> From<ProgressBar<Message>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + iced_native::text::Renderer,
    Message: 'a + Clone,
{
    fn from(value: ProgressBar<Message>) -> Self {
        Element::new(value)
    }
}
