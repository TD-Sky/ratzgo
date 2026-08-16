use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
};
use ratatui_crossterm::crossterm::event::KeyEvent;

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
