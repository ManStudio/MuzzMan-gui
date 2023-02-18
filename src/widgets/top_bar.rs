use iced::{Background, Command, Element};
use iced_native::renderer::BorderRadius;

use crate::themes::Colors;

pub struct TopBar<'a, Message, Renderer> {
    message: Box<dyn Fn(Command<Message>) -> Message + 'a>,
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> TopBar<'a, Message, Renderer> {
    pub fn new<C, F>(content: C, on_command: F) -> Self
    where
        F: 'a + Fn(Command<Message>) -> Message,
        C: Into<Element<'a, Message, Renderer>>,
    {
        Self {
            content: content.into(),
            message: Box::new(on_command),
        }
    }
}

impl<'l, Message, Renderer> iced_native::Widget<Message, Renderer> for TopBar<'l, Message, Renderer>
where
    Renderer: iced_native::Renderer,
{
    fn width(&self) -> iced::Length {
        iced::Length::Fill
    }

    fn height(&self) -> iced::Length {
        iced::Length::Fixed(40.0)
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let height =
            <TopBar<Message, Renderer> as iced_native::Widget<Message, Renderer>>::height(self);
        let width =
            <TopBar<Message, Renderer> as iced_native::Widget<Message, Renderer>>::width(self);
        let size = limits.height(height).width(width).fill();
        iced_native::layout::Node::with_children(
            size,
            vec![self.content.as_widget().layout(renderer, limits)],
        )
    }

    fn draw(
        &self,
        tree: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        render_style: &iced_native::renderer::Style,
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
        );

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            render_style,
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
        );
    }

    fn tag(&self) -> iced_native::widget::tree::Tag {
        iced_native::widget::tree::Tag::stateless()
    }

    fn state(&self) -> iced_native::widget::tree::State {
        iced_native::widget::tree::State::None
    }

    fn children(&self) -> Vec<iced_native::widget::Tree> {
        vec![iced_native::widget::Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut iced_native::widget::Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn operate(
        &self,
        tree: &mut iced_native::widget::Tree,
        layout: iced_native::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced_native::widget::Operation<Message>,
    ) {
        operation.container(None, &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        tree: &mut iced_native::widget::Tree,
        event: iced::Event,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        renderer: &Renderer,
        clipboard: &mut dyn iced_native::Clipboard,
        shell: &mut iced_native::Shell<'_, Message>,
    ) -> iced::event::Status {
        let res = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor_position,
            renderer,
            clipboard,
            shell,
        );
        if let iced::event::Status::Ignored = res {
            if layout.bounds().contains(cursor_position) {
                if let iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                    iced::mouse::Button::Left,
                )) = event
                {
                    shell.publish((self.message)(iced::window::drag()));
                    return iced::event::Status::Captured;
                }
            }
        }
        res
    }

    fn mouse_interaction(
        &self,
        tree: &iced_native::widget::Tree,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
        viewport: &iced::Rectangle,
        renderer: &Renderer,
    ) -> iced_native::mouse::Interaction {
        let res = self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout.children().next().unwrap(),
            cursor_position,
            viewport,
            renderer,
        );
        let is_mouse_hover = layout.bounds().contains(cursor_position);
        let is_not_used = matches!(res, iced_native::mouse::Interaction::Idle);
        if is_not_used && is_mouse_hover {
            return iced_native::mouse::Interaction::Grabbing;
        }

        res
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut iced_native::widget::Tree,
        layout: iced_native::Layout<'_>,
        renderer: &Renderer,
    ) -> Option<iced_native::overlay::Element<'a, Message, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<TopBar<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: iced_native::Renderer,
    Message: 'a,
    Renderer: 'a,
{
    fn from(value: TopBar<'a, Message, Renderer>) -> Self {
        Element::new(value)
    }
}
