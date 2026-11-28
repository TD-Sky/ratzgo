use crossterm::event::MouseEvent;
use ratatui_core::layout::Position;

pub trait RatPosition {
    fn position(&self) -> Position;
}

impl RatPosition for MouseEvent {
    fn position(&self) -> Position {
        Position {
            x: self.column,
            y: self.row,
        }
    }
}
