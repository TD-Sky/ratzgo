use std::{any, cell::Cell, rc::Rc};

use ratatui_core::{buffer::Buffer, layout::Rect, style::Style, text::Text, widgets::Widget};
use ratatui_crossterm::crossterm::event::{KeyEvent, MouseEvent};
pub use ratatui_widgets::paragraph::Wrap;

use crate::{
    core::*,
    scroll::{ScrollAction, scroll_horizontal, scroll_vertical},
};

type OnMouse<'a, Message> = Box<dyn FnMut(&MouseEvent, &ParagraphState) -> Option<Message> + 'a>;

pub fn paragraph<'a, Message>(
    text: impl Into<Text<'a>>,
    state: &'a mut ParagraphState,
) -> Paragraph<'a, Message> {
    let text = text.into();

    Paragraph {
        base: ratatui_widgets::paragraph::Paragraph::new(text),
        activity: false,
        on_key: Default::default(),
        on_mouse: None,
        state,
    }
}

pub struct Paragraph<'a, Message> {
    base: ratatui_widgets::paragraph::Paragraph<'a>,
    activity: bool,
    on_key: OnKey<'a, Message>,
    on_mouse: Option<OnMouse<'a, Message>>,
    state: &'a mut ParagraphState,
}

impl<'a, Message: std::fmt::Debug> std::fmt::Debug for Paragraph<'a, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Paragraph")
            .field("base", &self.base)
            .field("activity", &self.activity)
            .field("on_key", &self.on_key)
            .field(
                "on_mouse",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.on_mouse)),
            )
            .field("state", &self.state)
            .finish()
    }
}

impl<'a, Message> Paragraph<'a, Message> {
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.base = self.base.wrap(wrap);
        self
    }

    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.base = self.base.style(style);
        self
    }

    pub fn decorate<F>(mut self, f: F) -> Self
    where
        F: FnOnce(
            ratatui_widgets::paragraph::Paragraph<'a>,
        ) -> ratatui_widgets::paragraph::Paragraph<'a>,
    {
        self.base = f(self.base);
        self
    }

    pub fn on_mouse<F>(mut self, f: F) -> Self
    where
        F: FnMut(&MouseEvent, &ParagraphState) -> Option<Message> + 'a,
    {
        self.on_mouse = Some(Box::new(f));
        self
    }
}

impl<'a, Message> Component<Message> for Paragraph<'a, Message>
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

    fn handle_mouse(&mut self, mouse: &MouseEvent) -> Option<Message> {
        self.on_mouse.as_mut()?(mouse, self.state)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        let base = if self.state.scroll != (0, 0) {
            &std::mem::take(&mut self.base).scroll(self.state.scroll)
        } else {
            &self.base
        };

        base.render(self.state.area.get(), buf);
    }
}

impl<'a, Message> BindArea for Paragraph<'a, Message> {
    fn bind_area(self, area: &Rc<Cell<Rect>>) -> Self {
        self.state.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> Activable for Paragraph<'a, Message> {
    fn active_mut(&mut self) -> &mut bool {
        &mut self.activity
    }
}

impl<'a, Message> OnKeyBuilder<'a, Message> for Paragraph<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message> {
        &mut self.on_key
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParagraphState {
    pub scroll: (u16, u16),
    pub area: Area,
}

impl ParagraphState {
    pub fn reset(&mut self) {
        self.scroll = (0, 0);
    }

    pub fn scroll_vertical(&mut self, action: ScrollAction, height: usize) {
        self.scroll.0 = scroll_vertical(action, self.scroll.0, height, self.area.get().height);
    }

    pub fn scroll_horizontal(&mut self, action: ScrollAction, width: usize) {
        self.scroll.1 = scroll_horizontal(action, self.scroll.1, width, self.area.get().width);
    }
}
