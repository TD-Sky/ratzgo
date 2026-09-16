use std::{cell::Cell, marker::PhantomData, rc::Rc};

use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span, Text},
    widgets::Widget as _,
};

use crate::core::*;

pub fn span<'a, Message>(span: impl Into<Span<'a>>) -> SpanWidget<'a, Message> {
    SpanWidget {
        base: span.into(),
        area: Default::default(),
        _marker: PhantomData,
    }
}

pub fn line<'a, Message>(line: impl Into<Line<'a>>) -> LineWidget<'a, Message> {
    LineWidget {
        base: line.into(),
        area: Default::default(),
        _marker: PhantomData,
    }
}

pub fn text<'a, Message>(text: impl Into<Text<'a>>) -> TextWidget<'a, Message> {
    TextWidget {
        base: text.into(),
        area: Default::default(),
        _marker: PhantomData,
    }
}

#[derive(Debug)]
pub struct SpanWidget<'a, Message> {
    base: Span<'a>,
    area: Area,
    _marker: PhantomData<Message>,
}

impl<'a, Message> Widget<Message> for SpanWidget<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn activity(&self) -> bool {
        false
    }

    fn area(&self) -> Rect {
        self.area.get()
    }

    fn set_area(&mut self, area: Rect) {
        self.area.set(area);
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        (&self.base).render(self.area.get(), buf);
    }
}

impl<'a, Message> BindArea for SpanWidget<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> From<SpanWidget<'a, Message>> for Element<'a, Message>
where
    Message: std::fmt::Debug + 'a,
{
    fn from(widget: SpanWidget<'a, Message>) -> Self {
        Self::new(widget)
    }
}

#[derive(Debug)]
pub struct LineWidget<'a, Message> {
    base: Line<'a>,
    area: Area,
    _marker: PhantomData<Message>,
}

impl<'a, Message> Widget<Message> for LineWidget<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn activity(&self) -> bool {
        false
    }

    fn area(&self) -> Rect {
        self.area.get()
    }

    fn set_area(&mut self, area: Rect) {
        self.area.set(area);
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        (&self.base).render(self.area.get(), buf);
    }
}

impl<'a, Message> BindArea for LineWidget<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> From<LineWidget<'a, Message>> for Element<'a, Message>
where
    Message: std::fmt::Debug + 'a,
{
    fn from(widget: LineWidget<'a, Message>) -> Self {
        Self::new(widget)
    }
}

#[derive(Debug)]
pub struct TextWidget<'a, Message> {
    base: Text<'a>,
    area: Area,
    _marker: PhantomData<Message>,
}

impl<'a, Message> Widget<Message> for TextWidget<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn activity(&self) -> bool {
        false
    }

    fn area(&self) -> Rect {
        self.area.get()
    }

    fn set_area(&mut self, area: Rect) {
        self.area.set(area);
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        (&self.base).render(self.area.get(), buf);
    }
}

impl<'a, Message> BindArea for TextWidget<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> From<TextWidget<'a, Message>> for Element<'a, Message>
where
    Message: std::fmt::Debug + 'a,
{
    fn from(widget: TextWidget<'a, Message>) -> Self {
        Self::new(widget)
    }
}
