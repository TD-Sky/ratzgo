use std::any;

use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
};
use ratatui_crossterm::crossterm::event::KeyEvent;

use crate::core::Component;

pub struct MapMessage<'a, Message, O, W> {
    component: W,
    mapper: Box<dyn Fn(Message) -> O + 'a>,
}

impl<'a, Message, O, W> MapMessage<'a, Message, O, W> {
    pub fn new(component: W, mapper: impl Fn(Message) -> O + 'a) -> Self {
        Self {
            component,
            mapper: Box::new(mapper),
        }
    }
}

impl<'a, Message, O, W> std::fmt::Debug for MapMessage<'a, Message, O, W>
where
    W: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MapMessage")
            .field("component", &self.component)
            .field(
                "mapper",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.mapper)),
            )
            .finish()
    }
}

impl<'a, Message, O, W> Component<O> for MapMessage<'a, Message, O, W>
where
    Message: 'a,
    O: 'a,
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

    fn handle_key(&mut self, key: &KeyEvent) -> Option<O> {
        self.component.handle_key(key).map(&self.mapper)
    }

    fn handle_click(&mut self, pos: Position) -> Option<O> {
        self.component.handle_click(pos).map(&self.mapper)
    }

    fn handle_paste(&mut self, content: &str) -> Option<O> {
        self.component.handle_paste(content).map(&self.mapper)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        self.component.adapt(buf);
    }
}
