use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
};
use ratatui_crossterm::crossterm::event::KeyEvent;

use crate::core::Widget;

#[derive(Debug)]
pub struct Element<'a, Message> {
    widget: Box<dyn Widget<Message> + 'a>,
}

impl<'a, Message> Element<'a, Message> {
    pub fn new(widget: impl Widget<Message> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn as_widget(&self) -> &dyn Widget<Message> {
        &*self.widget
    }

    pub fn as_widget_mut(&mut self) -> &mut dyn Widget<Message> {
        &mut *self.widget
    }

    pub fn map<O>(self, f: impl Fn(Message) -> O + 'a) -> Element<'a, O>
    where
        Message: 'a,
        O: 'a,
    {
        Element::new(Map {
            widget: self.widget,
            mapper: Box::new(f),
        })
    }
}

struct Map<'a, Message, O> {
    widget: Box<dyn Widget<Message> + 'a>,
    mapper: Box<dyn Fn(Message) -> O + 'a>,
}

impl<'a, Message, O> std::fmt::Debug for Map<'a, Message, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map")
            .field("widget", &self.widget)
            .field("mapper", &"(Message) -> M")
            .finish()
    }
}

impl<'a, Message, O> Widget<O> for Map<'a, Message, O> {
    fn activity(&self) -> bool {
        self.widget.activity()
    }

    fn area(&self) -> Rect {
        self.widget.area()
    }

    fn set_area(&mut self, area: Rect) {
        self.widget.set_area(area);
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<O> {
        self.widget.handle_key(key).map(&self.mapper)
    }

    fn handle_click(&mut self, pos: Position) -> Option<O> {
        self.widget.handle_click(pos).map(&self.mapper)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        self.widget.adapt(buf);
    }
}
