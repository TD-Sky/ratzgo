use std::{any, cell::Cell, rc::Rc};

use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
    text::Line,
    widgets::Widget as _,
};
use ratatui_crossterm::crossterm::event::KeyEvent;
pub use ratatui_widgets::borders::{BorderType, Borders};

use crate::core::*;

pub fn block<'a, Message>(widget: impl Into<Element<'a, Message>>) -> Block<'a, Message> {
    Block {
        base: ratatui_widgets::block::Block::new(),
        area: Default::default(),
        inner: widget.into(),
        widgets: vec![],
    }
}

#[derive(Debug)]
pub struct Block<'a, Message> {
    base: ratatui_widgets::block::Block<'a>,
    area: Area,
    inner: Element<'a, Message>,
    widgets: Vec<WidgetOnBlock<'a, Message>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderOrientation {
    Top,
    Bottom,
    Left,
    Right,
}

impl<'a, Message> Block<'a, Message> {
    pub fn title(mut self, title: impl Into<Line<'a>>) -> Self {
        self.base = self.base.title(title);
        self
    }

    pub fn title_top(mut self, title: impl Into<Line<'a>>) -> Self {
        self.base = self.base.title_top(title);
        self
    }

    pub fn title_bottom(mut self, title: impl Into<Line<'a>>) -> Self {
        self.base = self.base.title_bottom(title);
        self
    }

    pub fn bordered(mut self) -> Self {
        self.base = self.base.borders(Borders::all());
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.base = self.base.border_type(border_type);
        self
    }

    pub fn decorate<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ratatui_widgets::block::Block<'a>) -> ratatui_widgets::block::Block<'a>,
    {
        self.base = f(self.base);
        self
    }

    pub fn widget_top(
        self,
        widget: impl Into<Element<'a, Message>>,
        mut pos: impl FnMut(Rect) -> Rect + 'a,
    ) -> Self {
        self.widget_top_opt(widget, move |v| Some(pos(v)))
    }

    pub fn widget_bottom(
        self,
        widget: impl Into<Element<'a, Message>>,
        mut pos: impl FnMut(Rect) -> Rect + 'a,
    ) -> Self {
        self.widget_bottom_opt(widget, move |v| Some(pos(v)))
    }

    pub fn widget_left(
        self,
        widget: impl Into<Element<'a, Message>>,
        mut pos: impl FnMut(Rect) -> Rect + 'a,
    ) -> Self {
        self.widget_left_opt(widget, move |v| Some(pos(v)))
    }

    pub fn widget_right(
        self,
        widget: impl Into<Element<'a, Message>>,
        mut pos: impl FnMut(Rect) -> Rect + 'a,
    ) -> Self {
        self.widget_right_opt(widget, move |v| Some(pos(v)))
    }

    pub fn widget_top_opt(
        self,
        widget: impl Into<Element<'a, Message>>,
        pos: impl FnMut(Rect) -> Option<Rect> + 'a,
    ) -> Self {
        self.add_widget(WidgetOnBlock {
            base: widget.into(),
            orientation: BorderOrientation::Top,
            pos: Box::new(pos),
        })
    }

    pub fn widget_bottom_opt(
        self,
        widget: impl Into<Element<'a, Message>>,
        pos: impl FnMut(Rect) -> Option<Rect> + 'a,
    ) -> Self {
        self.add_widget(WidgetOnBlock {
            base: widget.into(),
            orientation: BorderOrientation::Bottom,
            pos: Box::new(pos),
        })
    }

    pub fn widget_left_opt(
        self,
        widget: impl Into<Element<'a, Message>>,
        pos: impl FnMut(Rect) -> Option<Rect> + 'a,
    ) -> Self {
        self.add_widget(WidgetOnBlock {
            base: widget.into(),
            orientation: BorderOrientation::Left,
            pos: Box::new(pos),
        })
    }

    pub fn widget_right_opt(
        self,
        widget: impl Into<Element<'a, Message>>,
        pos: impl FnMut(Rect) -> Option<Rect> + 'a,
    ) -> Self {
        self.add_widget(WidgetOnBlock {
            base: widget.into(),
            orientation: BorderOrientation::Right,
            pos: Box::new(pos),
        })
    }
}

impl<'a, Message> Block<'a, Message> {
    fn add_widget(mut self, widget: WidgetOnBlock<'a, Message>) -> Self {
        self.widgets.push(widget);
        self
    }
}

impl<'a, Message> Widget<Message> for Block<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn activity(&self) -> bool {
        self.inner.as_widget().activity()
    }

    fn area(&self) -> Rect {
        self.area.get()
    }

    fn set_area(&mut self, area: Rect) {
        self.area.set(area);
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        self.inner.as_widget_mut().handle_key(key)
    }

    fn handle_click(&mut self, pos: Position) -> Option<Message> {
        self.inner.as_widget_mut().handle_click(pos)
    }

    fn handle_paste(&mut self, content: &str) -> Option<Message> {
        self.inner.as_widget_mut().handle_paste(content)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        let area = self.area.get();

        let inner_area = self.base.inner(area);
        self.inner.as_widget_mut().render(inner_area, buf);
        (&self.base).render(area, buf);

        self.widgets.retain_mut(|widget| {
            let border = match widget.orientation {
                BorderOrientation::Top => area.rows().next(),
                BorderOrientation::Bottom => area.rows().next_back(),
                BorderOrientation::Left => area.columns().next(),
                BorderOrientation::Right => area.columns().next_back(),
            };
            let Some(border_area) = border else {
                return false;
            };

            let Some(pos_area) = (widget.pos)(border_area) else {
                return false;
            };

            widget.base.as_widget_mut().render(pos_area, buf);

            true
        });
    }
}

impl<'a, Message> From<Block<'a, Message>> for Element<'a, Message>
where
    Message: std::fmt::Debug + 'a,
{
    fn from(widget: Block<'a, Message>) -> Self {
        Self::new(widget)
    }
}

impl<'a, Message> BindArea for Block<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

struct WidgetOnBlock<'a, Message> {
    base: Element<'a, Message>,
    orientation: BorderOrientation,
    pos: Box<dyn FnMut(Rect) -> Option<Rect> + 'a>,
}

impl<'a, Message: std::fmt::Debug> std::fmt::Debug for WidgetOnBlock<'a, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WidgetOnBlock")
            .field("base", &self.base)
            .field("orientation", &self.orientation)
            .field(
                "pos",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.pos)),
            )
            .finish()
    }
}
