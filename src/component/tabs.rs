use std::{cell::Cell, rc::Rc};

use ratatui_core::{buffer::Buffer, layout::Rect, text::Line, widgets::Widget};
use ratatui_crossterm::crossterm::event::KeyEvent;

use crate::core::*;

#[derive(Debug)]
pub struct Tabs<'a, Message> {
    base: ratatui_widgets::tabs::Tabs<'a>,
    area: Area,
    activity: bool,
    on_key: OnKey<'a, Message>,
}

impl<'a, Message> Tabs<'a, Message> {
    pub fn new(titles: impl IntoIterator<Item = Line<'a>>) -> Self {
        Self {
            base: ratatui_widgets::tabs::Tabs::new(titles),
            area: Default::default(),
            activity: false,
            on_key: OnKey::default(),
        }
    }

    pub fn select(mut self, index: impl Into<Option<usize>>) -> Self {
        self.base = self.base.select(index);
        self
    }

    pub fn decorate<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ratatui_widgets::tabs::Tabs<'a>) -> ratatui_widgets::tabs::Tabs<'a>,
    {
        self.base = f(self.base);
        self
    }
}

impl<'a, Message> Component<Message> for Tabs<'a, Message>
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

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        self.on_key.key(key)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        (&self.base).render(self.area.get(), buf);
    }
}

impl<'a, Message> BindArea for Tabs<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> Activable for Tabs<'a, Message> {
    fn active_mut(&mut self) -> &mut bool {
        &mut self.activity
    }
}

impl<'a, Message> OnKeyBuilder<'a, Message> for Tabs<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message> {
        &mut self.on_key
    }
}

#[macro_export]
macro_rules! tabs {
    ($($title:expr),+ $(,)?) => {
        $crate::component::Tabs::new(
            [$($crate::text::Line::from($title)),+],
        )
    };
}
pub use tabs;
