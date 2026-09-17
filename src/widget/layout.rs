use std::{cell::Cell, rc::Rc};

use ratatui_core::{
    buffer::Buffer,
    layout::{Constraint, Direction, Position, Rect},
};
use ratatui_crossterm::crossterm::event::KeyEvent;

use crate::core::*;

#[derive(Debug)]
pub struct Layout<'a, Message> {
    base: ratatui_core::layout::Layout,
    area: Area,
    activity: bool,
    on_key: OnKey<'a, Message>,
    elts: Vec<Box<dyn Widget<Message> + 'a>>,
}

impl<'a, Message> Layout<'a, Message> {
    pub fn vertical<C, W>(constraints: C, widgets: W) -> Self
    where
        C: IntoIterator<Item: Into<Constraint>>,
        W: IntoIterator<Item: Widget<Message> + 'a>,
    {
        Self::new(Direction::Vertical, constraints, widgets)
    }

    pub fn horizontal<C, W>(constraints: C, widgets: W) -> Self
    where
        C: IntoIterator<Item: Into<Constraint>>,
        W: IntoIterator<Item: Widget<Message> + 'a>,
    {
        Self::new(Direction::Horizontal, constraints, widgets)
    }

    pub fn decorate<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ratatui_core::layout::Layout) -> ratatui_core::layout::Layout,
    {
        self.base = f(self.base);
        self
    }
}

impl<'a, Message> Layout<'a, Message> {
    fn new<C, W>(direction: Direction, constraints: C, widgets: W) -> Self
    where
        C: IntoIterator<Item: Into<Constraint>>,
        W: IntoIterator<Item: Widget<Message> + 'a>,
    {
        let constraints: Vec<_> = constraints.into_iter().collect();
        let elts: Vec<_> = widgets.into_iter().map(|elt| elt.boxed()).collect();

        debug_assert_eq!(constraints.len(), elts.len());

        Self {
            area: Default::default(),
            activity: false,
            on_key: OnKey::default(),
            base: ratatui_core::layout::Layout::new(direction, constraints),
            elts,
        }
    }
}

impl<'a, Message> Widget<Message> for Layout<'a, Message>
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
            .find_map(|v| v.activity().then_some(v))
            .and_then(|v| v.handle_key(key))
            .or_else(|| self.on_key.key(key))
    }

    fn handle_click(&mut self, pos: Position) -> Option<Message> {
        // TODO: click which part

        let widget = self
            .elts
            .iter_mut()
            .find_map(|v| v.area().contains(pos).then_some(v))?;
        widget.handle_click(pos)
    }

    fn handle_paste(&mut self, content: &str) -> Option<Message> {
        self.elts
            .iter_mut()
            .find_map(|v| v.activity().then_some(v))
            .and_then(|v| v.handle_paste(content))
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        for (elt, &area) in self.elts.iter_mut().zip(&*self.base.split(self.area.get())) {
            elt.render(area, buf);
        }
    }
}

impl<'a, Message> BindArea for Layout<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> Activable for Layout<'a, Message> {
    fn active_mut(&mut self) -> &mut bool {
        &mut self.activity
    }
}

impl<'a, Message> OnKeyBuilder<'a, Message> for Layout<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message> {
        &mut self.on_key
    }
}

#[macro_export]
macro_rules! column {
    ($constraints:expr; [$($widget:expr),+ $(,)?]) => {
        $crate::widget::Layout::vertical(
            $constraints,
            [$($crate::core::WidgetExt::boxed($widget)),+],
        )
    };
}
pub use column;

#[macro_export]
macro_rules! row {
    ($constraints:expr; [$($widget:expr),+ $(,)?]) => {
        $crate::widget::Layout::horizontal(
            $constraints,
            [$($crate::core::WidgetExt::boxed($widget)),+],
        )
    };
}
pub use row;
