use std::{any, marker::PhantomData};

use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
};
use ratatui_crossterm::crossterm::event::KeyEvent;

use crate::core::Widget;

pub struct FilterKey<'a, Message, W> {
    widget: W,
    filter: Box<dyn Fn(&KeyEvent) -> bool + 'a>,
    _marker: PhantomData<Message>,
}

impl<'a, Message, W> FilterKey<'a, Message, W> {
    pub fn new(widget: W, filter: impl Fn(&KeyEvent) -> bool + 'a) -> Self {
        Self {
            widget,
            filter: Box::new(filter),
            _marker: PhantomData,
        }
    }
}

impl<'a, Message, W> std::fmt::Debug for FilterKey<'a, Message, W>
where
    W: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FilterKey")
            .field("widget", &self.widget)
            .field(
                "filter",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.filter)),
            )
            .finish()
    }
}

impl<'a, Message, W> Widget<Message> for FilterKey<'a, Message, W>
where
    W: Widget<Message>,
{
    fn activity(&self) -> bool {
        self.widget.activity()
    }

    fn area(&self) -> Rect {
        self.widget.area()
    }

    fn set_area(&mut self, area: Rect) {
        self.widget.set_area(area);
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        if (self.filter)(key) {
            self.widget.handle_key(key)
        } else {
            None
        }
    }

    fn handle_click(&mut self, pos: Position) -> Option<Message> {
        self.widget.handle_click(pos)
    }

    fn handle_paste(&mut self, content: &str) -> Option<Message> {
        self.widget.handle_paste(content)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        self.widget.adapt(buf);
    }
}
