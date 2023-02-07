use iced_native::Widget;

pub struct ProgressBar {
    pub progress: f32,
}

impl<Message, Renderer> Widget<Message, Renderer> for ProgressBar
where
    Renderer: iced_native::text::Renderer,
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
        let width = <ProgressBar as iced_native::Widget<Message, Renderer>>::width(self);
        let height = <ProgressBar as iced_native::Widget<Message, Renderer>>::height(self);
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
        renderer.fill_text(iced_native::text::Text {
            content: "test",
            bounds: layout.bounds(),
            size: 15.0,
            color: iced::Color::from_rgb8(20, 150, 20),
            font: Default::default(),
            horizontal_alignment: iced::alignment::Horizontal::Center,
            vertical_alignment: iced::alignment::Vertical::Center,
        });
    }
}
