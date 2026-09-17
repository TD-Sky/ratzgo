use std::{any, cell::Cell, mem, rc::Rc};

use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
    widgets::Widget as _,
};
use ratatui_crossterm::crossterm::event::KeyEvent;
use ratatui_widgets::clear::Clear;
use thin_cell::unsync::ThinCell;

use crate::core::*;

#[derive(Debug, Clone)]
pub struct MountPoint<Message> {
    inner: ThinCell<Option<Inner<'static, Message>>>,
}

impl<Message> Default for MountPoint<Message> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<Message> MountPoint<Message> {
    pub fn new() -> Self {
        Self {
            inner: ThinCell::new(None),
        }
    }

    pub fn mount<'a>(
        &self,
        widget: impl Widget<Message> + 'a,
        constraint: impl FnOnce(Rect) -> Rect + 'a,
    ) {
        let inner = Inner {
            elt: widget.boxed(),
            constraint: Some(Box::new(constraint)),
        };
        let inner: Inner<'static, Message> = unsafe { mem::transmute(inner) };

        *self.inner.borrow() = Some(inner);
    }

    pub fn view<'a>(&self) -> MountView<'a, Message> {
        let inner: ThinCell<Option<Inner<'a, Message>>> =
            unsafe { mem::transmute(self.inner.clone()) };

        MountView {
            inner,
            area: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MountView<'a, Message> {
    inner: ThinCell<Option<Inner<'a, Message>>>,
    area: Area,
}

impl<'a, Message> Drop for MountView<'a, Message> {
    fn drop(&mut self) {
        self.inner.borrow().take();
    }
}

impl<'a, Message> Widget<Message> for MountView<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn activity(&self) -> bool {
        self.inner.borrow().is_some()
    }

    fn area(&self) -> Rect {
        self.area.get()
    }

    fn set_area(&mut self, area: Rect) {
        self.area.set(area);
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        self.inner.borrow().as_mut()?.elt.handle_key(key)
    }

    fn handle_click(&mut self, pos: Position) -> Option<Message> {
        self.inner.borrow().as_mut()?.elt.handle_click(pos)
    }

    fn handle_paste(&mut self, content: &str) -> Option<Message> {
        self.inner.borrow().as_mut()?.elt.handle_paste(content)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        if let Some(inner) = &mut *self.inner.borrow() {
            let area = (inner
                .constraint
                .take()
                .expect("`constraint` must be `Some`"))(self.area.get());

            Clear.render(area, buf);
            inner.elt.render(area, buf);
        }
    }
}

impl<'a, Message> BindArea for MountView<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

struct Inner<'a, Message> {
    elt: Box<dyn Widget<Message> + 'a>,
    constraint: Option<Box<dyn FnOnce(Rect) -> Rect + 'a>>,
}

impl<'a, Message> std::fmt::Debug for Inner<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inner")
            .field("elt", &self.elt)
            .field(
                "constraint",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.constraint)),
            )
            .finish()
    }
}
