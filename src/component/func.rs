use std::{any, cell::Cell, marker::PhantomData, rc::Rc};

use ratatui_core::{buffer::Buffer, layout::Rect};
use ratatui_crossterm::crossterm::event::{KeyEvent, MouseEvent};

use crate::core::*;

type OnMouse<'a, Message> = Box<dyn FnMut(&MouseEvent, Rect) -> Option<Message> + 'a>;

pub fn from_render<'a, Message>(f: impl FnMut(Rect, &mut Buffer) + 'a) -> RenderFn<'a, Message> {
    RenderFn {
        render: Box::new(f),
        activity: false,
        on_key: Default::default(),
        on_mouse: None,
        area: Default::default(),
        _marker: PhantomData,
    }
}

pub struct RenderFn<'a, Message> {
    #[expect(clippy::type_complexity)]
    render: Box<dyn FnMut(Rect, &mut Buffer) + 'a>,
    activity: bool,
    on_key: OnKey<'a, Message>,
    on_mouse: Option<OnMouse<'a, Message>>,
    area: Area,
    _marker: PhantomData<Message>,
}

impl<'a, Message> std::fmt::Debug for RenderFn<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderFn")
            .field(
                "render",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.render)),
            )
            .field("activity", &self.activity)
            .field("on_key", &self.on_key)
            .field(
                "on_mouse",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.on_mouse)),
            )
            .field("area", &self.area)
            .field("_marker", &self._marker)
            .finish()
    }
}

impl<'a, Message> RenderFn<'a, Message> {
    /// Callback also gets the area this component was rendered in,
    /// so it can hit-test the same way
    /// a stateful component does with its own area.
    pub fn on_mouse<F>(mut self, f: F) -> Self
    where
        F: FnMut(&MouseEvent, Rect) -> Option<Message> + 'a,
    {
        self.on_mouse = Some(Box::new(f));
        self
    }
}

impl<'a, Message> BindArea for RenderFn<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> Activable for RenderFn<'a, Message> {
    fn active_mut(&mut self) -> &mut bool {
        &mut self.activity
    }
}

impl<'a, Message> OnKeyBuilder<'a, Message> for RenderFn<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message> {
        &mut self.on_key
    }
}

impl<'a, Message> Component<Message> for RenderFn<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn activity(&self) -> bool {
        self.activity
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
        let area = self.area.get();
        self.on_mouse.as_mut()?(mouse, area)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        let area = self.area();
        (self.render)(area, buf);
    }
}
