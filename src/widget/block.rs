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
    }
}

#[derive(Debug)]
pub struct Block<'a, Message> {
    base: ratatui_widgets::block::Block<'a>,
    area: Rect,
    inner: Element<'a, Message>,
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
}

impl<'a, Message> Widget<Message> for Block<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn activity(&self) -> bool {
        self.inner.as_widget().activity()
    }

    fn area(&self) -> Rect {
        self.area
    }

    fn set_area(&mut self, area: Rect) {
        self.area = area;
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
        let inner_area = self.base.inner(self.area);
        self.inner.as_widget_mut().render(inner_area, buf);
        (&self.base).render(self.area, buf);
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
