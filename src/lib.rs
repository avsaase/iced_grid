use iced::Element;
use iced_widget::{
    core::{self, Widget},
    renderer, style,
};

type Renderer<Theme = style::Theme> = renderer::Renderer<Theme>;

pub struct Grid<'a, Message, Renderer = crate::Renderer> {
    columns: u32,
    rows: Vec<Row<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Grid<'a, Message, Renderer>
where
    Renderer: core::Renderer,
{
    pub fn new() -> Self {
        Self {
            columns: 2,
            rows: Vec::new(),
        }
    }

    pub fn push(mut self, row: Row<'a, Message, Renderer>) -> Self {
        self.rows.push(row);
        self
    }
}

pub struct Row<'a, Message, Renderer = crate::Renderer> {
    elements: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> Row<'a, Message, Renderer>
where
    Renderer: core::Renderer,
{
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn push<E>(mut self, element: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.elements.push(element.into());
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Grid<'a, Message, Renderer>
where
    Renderer: core::Renderer,
{
    fn width(&self) -> core::Length {
        todo!()
    }

    fn height(&self) -> core::Length {
        todo!()
    }

    fn layout(&self, renderer: &Renderer, limits: &core::layout::Limits) -> core::layout::Node {
        todo!()
    }

    fn draw(
        &self,
        state: &core::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as core::Renderer>::Theme,
        style: &core::renderer::Style,
        layout: core::Layout<'_>,
        cursor: core::mouse::Cursor,
        viewport: &core::Rectangle,
    ) {
        todo!()
    }

    fn tag(&self) -> core::widget::tree::Tag {
        core::widget::tree::Tag::stateless()
    }

    fn state(&self) -> core::widget::tree::State {
        core::widget::tree::State::None
    }

    fn children(&self) -> Vec<core::widget::Tree> {
        Vec::new()
    }

    fn diff(&self, _tree: &mut core::widget::Tree) {}

    fn operate(
        &self,
        _state: &mut core::widget::Tree,
        _layout: core::Layout<'_>,
        _renderer: &Renderer,
        _operation: &mut dyn core::widget::Operation<Message>,
    ) {
    }

    fn on_event(
        &mut self,
        _state: &mut core::widget::Tree,
        _event: core::Event,
        _layout: core::Layout<'_>,
        _cursor: core::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn core::Clipboard,
        _shell: &mut core::Shell<'_, Message>,
        _viewport: &core::Rectangle,
    ) -> core::event::Status {
        core::event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _state: &core::widget::Tree,
        _layout: core::Layout<'_>,
        _cursor: core::mouse::Cursor,
        _viewport: &core::Rectangle,
        _renderer: &Renderer,
    ) -> core::mouse::Interaction {
        core::mouse::Interaction::Idle
    }

    fn overlay<'b>(
        &'b mut self,
        _state: &'b mut core::widget::Tree,
        _layout: core::Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<core::overlay::Element<'b, Message, Renderer>> {
        None
    }
}

impl<'a, Message, Renderer> From<Grid<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: core::Renderer + 'a,
    Message: 'static,
{
    fn from(grid: Grid<'a, Message, Renderer>) -> Self {
        Element::new(grid)
    }
}
