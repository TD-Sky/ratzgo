use std::{cell::Cell, mem, rc::Rc};

use ratatui_core::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use ratatui_crossterm::crossterm::event::KeyEvent;
use ratatui_widgets::scrollbar::ScrollbarState;
pub use ratatui_widgets::scrollbar::{ScrollDirection, ScrollbarOrientation};

use crate::core::*;

#[derive(Debug, Clone)]
pub struct ScrollbarParams {
    pub content_length: usize,
    pub viewport: Area,
    pub position: usize,
}

pub fn scrollbar<'a, Message>(params: ScrollbarParams) -> Scrollbar<'a, Message> {
    Scrollbar {
        base: Default::default(),
        area: Default::default(),
        params,
        orientation: Default::default(),
        on_key: Default::default(),
    }
}

#[derive(Debug)]
pub struct Scrollbar<'a, Message> {
    base: ratatui_widgets::scrollbar::Scrollbar<'a>,
    area: Area,
    params: ScrollbarParams,
    orientation: ScrollbarOrientation,
    on_key: OnKey<'a, Message>,
}

impl<'a, Message> Scrollbar<'a, Message> {
    pub fn orientation(mut self, orientation: ScrollbarOrientation) -> Self {
        self.orientation = orientation.clone();
        self.base = self.base.orientation(orientation);
        self
    }

    pub fn decorate<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            ratatui_widgets::scrollbar::Scrollbar<'a>,
        ) -> ratatui_widgets::scrollbar::Scrollbar<'a>,
    {
        self.base = f(self.base);
        self
    }
}

impl<'a, Message> Widget<Message> for Scrollbar<'a, Message>
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
        let widget = mem::take(&mut self.base);

        let viewport_length = if self.orientation.is_vertical() {
            self.params.viewport.get().height
        } else {
            self.params.viewport.get().width
        } as usize;
        let max_offset = self.params.content_length.saturating_sub(viewport_length);
        let mut state = ScrollbarState::default()
            .content_length(self.params.content_length)
            .position(self.params.position.min(max_offset))
            .content_length(max_offset + 1)
            .viewport_content_length(viewport_length);

        widget.render(self.area.get(), buf, &mut state);
    }
}

impl<'a, Message> BindArea for Scrollbar<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> OnKeyBuilder<'a, Message> for Scrollbar<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message> {
        &mut self.on_key
    }
}

impl<'a, Message> From<Scrollbar<'a, Message>> for Element<'a, Message>
where
    Message: std::fmt::Debug + 'a,
{
    fn from(widget: Scrollbar<'a, Message>) -> Self {
        Self::new(widget)
    }
}
