use iced::{Background, Command, Element};
use iced_native::renderer::BorderRadius;

use crate::themes::Colors;

pub struct TopBar<'a, Message> {
    message: Box<dyn Fn(Command<Message>) -> Message + 'a>,
}

impl<'a, Message> TopBar<'a, Message> {
    pub fn new<F>(on_command: F) -> Self
    where
        F: 'a + Fn(Command<Message>) -> Message,
    {
        Self {
            message: Box::new(on_command),
        }
    }
}

impl<'l, Message, Renderer> iced_native::Widget<Message, Renderer> for TopBar<'l, Message>
where
    Renderer: iced_native::Renderer,
{
    fn width(&self) -> iced::Length {
        iced::Length::Fill
    }

    fn height(&self) -> iced::Length {
        iced::Length::Units(40)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let height = <TopBar<Message> as iced_native::Widget<Message, Renderer>>::height(self);
        let width = <TopBar<Message> as iced_native::Widget<Message, Renderer>>::width(self);
        let size = limits.height(height).width(width).fill();
        iced_native::layout::Node::new(size)
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
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: iced::Color::BLACK,
            },
            Background::Color(Colors::default().text_background),
        )
    }

    fn tag(&self) -> iced_native::widget::tree::Tag {
        iced_native::widget::tree::Tag::stateless()
    }

    fn state(&self) -> iced_native::widget::tree::State {
        iced_native::widget::tree::State::None
    }

    fn children(&self) -> Vec<iced_native::widget::Tree> {
        Vec::new()
    }

    fn diff(&self, _tree: &mut iced_native::widget::Tree) {}

    fn operate(
        &self,
        _state: &mut iced_native::widget::Tree,
        _layout: iced_native::Layout<'_>,
        _renderer: &Renderer,
        _operation: &mut dyn iced_native::widget::Operation<Message>,
    ) {
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
            if let iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                iced::mouse::Button::Left,
            )) = event
            {
                shell.publish((self.message)(iced::window::drag()));
                return iced::event::Status::Captured;
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
        let is_mouse_hover = layout.bounds().contains(cursor_position);
        if is_mouse_hover {
            iced_native::mouse::Interaction::Grabbing
        } else {
            iced_native::mouse::Interaction::Idle
        }
    }

    fn overlay<'a>(
        &'a mut self,
        _state: &'a mut iced_native::widget::Tree,
        _layout: iced_native::Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'a, Message, Renderer>> {
        None
    }
}

impl<'a, Message, Renderer> From<TopBar<'a, Message>> for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Message: 'a,
    Renderer: 'a,
{
    fn from(value: TopBar<'a, Message>) -> Self {
        Element::new(value)
    }
}
