use std::{cell::Cell, rc::Rc};

use ratatui_core::{buffer::Buffer, layout::Rect};
use ratatui_crossterm::crossterm::event::KeyEvent;

use crate::core::*;

#[derive(Debug)]
pub struct Stack<'a, Message> {
    area: Area,
    activity: bool,
    on_key: OnKey<'a, Message>,
    elts: Vec<Box<dyn Component<Message> + 'a>>,
}

impl<'a, Message> Stack<'a, Message> {
    pub fn new(elts: impl IntoIterator<Item: Component<Message> + 'a>) -> Self {
        Self {
            area: Default::default(),
            activity: false,
            on_key: Default::default(),
            elts: elts.into_iter().map(|elt| elt.boxed()).collect(),
        }
    }
}

impl<'a, Message> Component<Message> for Stack<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn activity(&self) -> bool {
        self.activity || self.elts.iter().any(|v| v.activity())
    }

    fn area(&self) -> Rect {
        self.area.get()
    }

    fn set_area(&mut self, area: Rect) {
        self.area.set(area);
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        self.elts
            .iter_mut()
            .rev()
            .find_map(|v| v.activity().then_some(v))
            .and_then(|v| v.handle_key(key))
            .or_else(|| self.on_key.key(key))
    }

    fn handle_paste(&mut self, content: &str) -> Option<Message> {
        self.elts
            .iter_mut()
            .rev()
            .find_map(|v| v.activity().then_some(v))
            .and_then(|v| v.handle_paste(content))
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        let area = self.area.get();

        for elt in &mut self.elts {
            elt.render(area, buf);
        }
    }
}

impl<'a, Message> BindArea for Stack<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
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
        $crate::component::Stack::new([$($crate::core::ComponentExt::boxed($widget)),+])
    };
}
pub use stack;
