use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use ratatui_core::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Style,
    widgets::StatefulWidget,
};
use ratatui_crossterm::crossterm::event::KeyEvent;
pub use ratatui_widgets::table::Row;

use crate::{
    core::*,
    scroll::{ScrollAction, ScrollPosition},
};

pub fn table<'a, Message>(state: &'a mut TableState) -> Table<'a, Message> {
    Table {
        state,
        base: Default::default(),
        activity: false,
        on_key: Default::default(),
    }
}

#[derive(Debug)]
pub struct Table<'a, Message> {
    state: &'a mut TableState,
    base: ratatui_widgets::table::Table<'a>,
    activity: bool,
    on_key: OnKey<'a, Message>,
}

impl<'a, Message> Table<'a, Message> {
    pub fn decorate<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ratatui_widgets::table::Table<'a>) -> ratatui_widgets::table::Table<'a>,
    {
        self.base = f(self.base);
        self
    }

    pub fn header(mut self, header: Row<'a>) -> Self {
        self.base = self.base.header(header);
        self
    }

    pub fn widths(mut self, widths: impl IntoIterator<Item: Into<Constraint>>) -> Self {
        self.base = self.base.widths(widths);
        self
    }

    pub fn rows(mut self, rows: impl IntoIterator<Item = Row<'a>>) -> Self {
        self.base = self.base.rows(rows);
        self
    }

    pub fn footer(mut self, footer: Row<'a>) -> Self {
        self.base = self.base.footer(footer);
        self
    }

    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.base = self.base.style(style);
        self
    }
}

impl<'a, Message> Component<Message> for Table<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn activity(&self) -> bool {
        self.activity
    }

    fn area(&self) -> Rect {
        self.state.area.get()
    }

    fn set_area(&mut self, area: Rect) {
        self.state.area.set(area);
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        self.on_key.key(key)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        (&self.base).render(self.state.area.get(), buf, self.state);

        let offset = self.state.base.offset();
        self.state.pos_vertical.set(offset);
    }
}

impl<'a, Message> BindArea for Table<'a, Message> {
    fn bind_area(self, area: &Rc<Cell<Rect>>) -> Self {
        self.state.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> Activable for Table<'a, Message> {
    fn active_mut(&mut self) -> &mut bool {
        &mut self.activity
    }
}

impl<'a, Message> OnKeyBuilder<'a, Message> for Table<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message> {
        &mut self.on_key
    }
}

#[derive(Debug, Default)]
pub struct TableState {
    base: ratatui_widgets::table::TableState,
    pub area: Area,
    pub pos_vertical: ScrollPosition,
}

impl Deref for TableState {
    type Target = ratatui_widgets::table::TableState;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for TableState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl TableState {
    pub fn reset(&mut self) {
        self.base.select(Some(0));
    }

    /// Assume each [`Row`] height is 1, scroll vertically through the lines.
    pub fn scroll_lines(&mut self, action: ScrollAction, height: usize) {
        let selected_offset = match action {
            ScrollAction::Fixed(n) => n,
            ScrollAction::Viewport(n) => (self.area.get().height as f32 * n as f32 * 0.01) as i16,
        };

        match self.selected_mut() {
            Some(index) => {
                *index = index
                    .saturating_add_signed(selected_offset as isize)
                    .min(height.saturating_sub(1));
            }
            None => {
                self.select(Some(
                    (selected_offset as usize).min(height.saturating_sub(1)),
                ));
            }
        }
    }
}
