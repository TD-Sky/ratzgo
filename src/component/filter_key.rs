use std::{any, marker::PhantomData};

use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
};
use ratatui_crossterm::crossterm::event::KeyEvent;

use crate::core::Component;

pub struct FilterKey<'a, Message, W> {
    component: W,
    filter: Box<dyn Fn(&KeyEvent) -> bool + 'a>,
    _marker: PhantomData<Message>,
}

impl<'a, Message, W> FilterKey<'a, Message, W> {
    pub fn new(component: W, filter: impl Fn(&KeyEvent) -> bool + 'a) -> Self {
        Self {
            component,
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
            .field("component", &self.component)
            .field(
                "filter",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.filter)),
            )
            .field("_marker", &self._marker)
            .finish()
    }
}

impl<'a, Message, W> Component<Message> for FilterKey<'a, Message, W>
where
    W: Component<Message>,
{
    fn activity(&self) -> bool {
        self.component.activity()
    }

    fn area(&self) -> Rect {
        self.component.area()
    }

    fn set_area(&mut self, area: Rect) {
        self.component.set_area(area);
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        if (self.filter)(key) {
            self.component.handle_key(key)
        } else {
            None
        }
    }

    fn handle_click(&mut self, pos: Position) -> Option<Message> {
        self.component.handle_click(pos)
    }

    fn handle_paste(&mut self, content: &str) -> Option<Message> {
        self.component.handle_paste(content)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        self.component.adapt(buf);
    }
}
