use ratatui_core::{buffer::Buffer, layout::Rect};
use ratatui_crossterm::crossterm::event::KeyEvent;

use crate::core::*;

#[derive(Debug)]
pub struct Stack<'a, Message> {
    area: Rect,
    activity: bool,
    on_key: OnKey<'a, Message>,
    elts: Vec<Element<'a, Message>>,
}

impl<'a, Message> From<Stack<'a, Message>> for Element<'a, Message>
where
    Message: std::fmt::Debug + 'a,
{
    fn from(widget: Stack<'a, Message>) -> Self {
        Element::new(widget)
    }
}

impl<'a, Message> Stack<'a, Message> {
    pub fn new(elts: impl IntoIterator<Item = Element<'a, Message>>) -> Self {
        Self {
            area: Default::default(),
            activity: false,
            on_key: Default::default(),
            elts: elts.into_iter().collect(),
        }
    }
}

impl<'a, Message> Widget<Message> for Stack<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn activity(&self) -> bool {
        self.activity || self.elts.iter().any(|v| v.as_widget().activity())
    }

    fn area(&self) -> Rect {
        self.area
    }

    fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        self.elts
            .iter_mut()
            .rev()
            .find_map(|v| {
                let v = v.as_widget_mut();
                v.activity().then_some(v)
            })
            .and_then(|v| v.handle_key(key))
            .or_else(|| self.on_key.key(key))
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        for elt in &mut self.elts {
            elt.as_widget_mut().render(self.area, buf);
        }
    }
}

impl<'a, Message> Activable for Stack<'a, Message> {
    fn active_mut(&mut self) -> &mut bool {
        &mut self.activity
    }
}

impl<'a, Message> OnKeyBuilder<'a, Message> for Stack<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message> {
        &mut self.on_key
    }
}

#[macro_export]
macro_rules! stack {
    ($($widget:expr),+ $(,)?) => {
        $crate::widget::Stack::new(
            [$(::std::convert::Into::<$crate::core::Element<_>>::into($widget)),+],
        )
    };
}
pub use stack;
