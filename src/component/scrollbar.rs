use std::{any, cell::Cell, mem, rc::Rc};

use ratatui_core::{buffer::Buffer, layout::Rect, style::Style, widgets::StatefulWidget};
use ratatui_crossterm::crossterm::event::{KeyEvent, MouseEvent};
use ratatui_widgets::scrollbar::ScrollbarState;
pub use ratatui_widgets::scrollbar::{ScrollDirection, ScrollbarOrientation};

use crate::{core::*, scroll::ScrollPosition};

type OnMouse<'a, Message> = Box<dyn FnMut(&MouseEvent) -> Option<Message> + 'a>;

#[derive(Debug, Clone)]
pub struct ScrollbarParams {
    pub content_length: usize,
    pub viewport: Area,
    pub position: ScrollPosition,
}

pub fn scrollbar<'a, Message>(params: ScrollbarParams) -> Scrollbar<'a, Message> {
    Scrollbar {
        base: Default::default(),
        area: Default::default(),
        params,
        orientation: Default::default(),
        on_key: Default::default(),
        on_mouse: None,
    }
}

pub struct Scrollbar<'a, Message> {
    base: ratatui_widgets::scrollbar::Scrollbar<'a>,
    area: Area,
    params: ScrollbarParams,
    orientation: ScrollbarOrientation,
    on_key: OnKey<'a, Message>,
    on_mouse: Option<OnMouse<'a, Message>>,
}

impl<Message> std::fmt::Debug for Scrollbar<'_, Message>
where
    Message: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scrollbar")
            .field("base", &self.base)
            .field("area", &self.area)
            .field("params", &self.params)
            .field("orientation", &self.orientation)
            .field("on_key", &self.on_key)
            .field(
                "on_mouse",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.on_mouse)),
            )
            .finish()
    }
}

impl<'a, Message> Scrollbar<'a, Message> {
    pub fn orientation(mut self, orientation: ScrollbarOrientation) -> Self {
        self.orientation = orientation.clone();
        self.base = self.base.orientation(orientation);
        self
    }

    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.base = self.base.style(style);
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

    pub fn on_mouse<F>(mut self, f: F) -> Self
    where
        F: FnMut(&MouseEvent) -> Option<Message> + 'a,
    {
        self.on_mouse = Some(Box::new(f));
        self
    }
}

impl<'a, Message> Component<Message> for Scrollbar<'a, Message>
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

    fn handle_mouse(&mut self, mouse: &MouseEvent) -> Option<Message> {
        self.on_mouse.as_mut()?(mouse)
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
            .position(self.params.position.get().min(max_offset))
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
