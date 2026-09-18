use std::{
    cell::Cell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use ratatui_core::{buffer::Buffer, layout::Rect, style::Style, widgets::StatefulWidget};
use ratatui_crossterm::crossterm::event::KeyEvent;
pub use ratatui_widgets::list::ListItem;

use crate::{core::*, scroll::ScrollAction};

pub fn list<'a, Message>(state: &'a mut ListState) -> List<'a, Message> {
    List {
        state,
        base: Default::default(),
        activity: false,
        on_key: Default::default(),
    }
}

#[derive(Debug)]
pub struct List<'a, Message> {
    state: &'a mut ListState,
    base: ratatui_widgets::list::List<'a>,
    activity: bool,
    on_key: OnKey<'a, Message>,
}

impl<'a, Message> List<'a, Message> {
    pub fn items(mut self, items: impl IntoIterator<Item: Into<ListItem<'a>>>) -> Self {
        self.base = self.base.items(items);
        self
    }

    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.base = self.base.style(style);
        self
    }

    pub fn decorate<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ratatui_widgets::list::List<'a>) -> ratatui_widgets::list::List<'a>,
    {
        self.base = f(self.base);
        self
    }
}

impl<'a, Message> Component<Message> for List<'a, Message>
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
    }
}

impl<'a, Message> BindArea for List<'a, Message> {
    fn bind_area(self, area: &Rc<Cell<Rect>>) -> Self {
        self.state.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> Activable for List<'a, Message> {
    fn active_mut(&mut self) -> &mut bool {
        &mut self.activity
    }
}

impl<'a, Message> OnKeyBuilder<'a, Message> for List<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message> {
        &mut self.on_key
    }
}

#[derive(Debug, Default)]
pub struct ListState {
    base: ratatui_widgets::list::ListState,
    pub area: Area,
}

impl Deref for ListState {
    type Target = ratatui_widgets::list::ListState;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for ListState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl ListState {
    pub fn reset(&mut self) {
        self.base.select(Some(0));
    }

    /// Assume each [`ListItem`] height is 1, scroll vertically through the lines.
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
