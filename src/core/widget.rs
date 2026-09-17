use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
};
use ratatui_crossterm::crossterm::event::KeyEvent;

use crate::widget::MapMessage;

pub trait Widget<Message>: std::fmt::Debug {
    fn activity(&self) -> bool;

    fn area(&self) -> Rect;

    fn set_area(&mut self, area: Rect);

    #[expect(unused)]
    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        None
    }

    #[expect(unused)]
    fn handle_click(&mut self, pos: Position) -> Option<Message> {
        None
    }

    #[expect(unused)]
    fn handle_paste(&mut self, content: &str) -> Option<Message> {
        None
    }

    fn adapt(&mut self, buf: &mut Buffer);

    fn render(&mut self, area: Rect, buf: &mut Buffer) {
        self.set_area(area);
        self.adapt(buf);
    }
}

pub trait WidgetExt<Message>: Widget<Message> {
    fn boxed<'a>(self) -> Box<dyn Widget<Message> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }

    fn map<'a, O, F>(self, f: F) -> MapMessage<'a, Message, O, Self>
    where
        F: Fn(Message) -> O + 'a,
        Self: Sized + 'a,
    {
        MapMessage::new(self, f)
    }
}

impl<Message, T> WidgetExt<Message> for T where T: Widget<Message> {}

impl<Message, T> Widget<Message> for Box<T>
where
    T: Widget<Message> + ?Sized,
{
    fn activity(&self) -> bool {
        self.as_ref().activity()
    }

    fn area(&self) -> Rect {
        self.as_ref().area()
    }

    fn set_area(&mut self, area: Rect) {
        self.as_mut().set_area(area);
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        self.as_mut().handle_key(key)
    }

    fn handle_click(&mut self, pos: Position) -> Option<Message> {
        self.as_mut().handle_click(pos)
    }

    fn handle_paste(&mut self, content: &str) -> Option<Message> {
        self.as_mut().handle_paste(content)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        self.as_mut().adapt(buf);
    }
}
