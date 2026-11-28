use std::{
    any,
    cell::Cell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use ratatui_core::{buffer::Buffer, layout::Rect, style::Style, widgets::StatefulWidget};
use ratatui_crossterm::crossterm::event::{KeyEvent, MouseEvent};
pub use ratatui_widgets::list::ListItem;

use crate::{
    core::*,
    scroll::{ScrollAction, ScrollPosition},
};

type OnMouse<'a, Message> =
    Box<dyn FnMut(&MouseEvent, &ListState, ListItemAttrs<'_>) -> Option<Message> + 'a>;

pub fn list<'a, Message>(state: &'a mut ListState) -> List<'a, Message> {
    List {
        state,
        base: Default::default(),
        activity: false,
        on_key: Default::default(),
        on_mouse: None,
        attrs: Default::default(),
    }
}

pub struct List<'a, Message> {
    state: &'a mut ListState,
    base: ratatui_widgets::list::List<'a>,
    activity: bool,
    on_key: OnKey<'a, Message>,
    on_mouse: Option<OnMouse<'a, Message>>,
    attrs: Attrs,
}

impl<'a, Message: std::fmt::Debug> std::fmt::Debug for List<'a, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("List")
            .field("state", &self.state)
            .field("base", &self.base)
            .field("activity", &self.activity)
            .field("on_key", &self.on_key)
            .field(
                "on_mouse",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.on_mouse)),
            )
            .field("attrs", &self.attrs)
            .finish()
    }
}

#[derive(Debug, Default)]
struct Attrs {
    item_heights: Vec<usize>,
}

#[derive(Debug)]
pub struct ListItemAttrs<'a> {
    attrs: &'a Attrs,
    cur: ListItemAttr,
    cur_item: usize,
}

impl Iterator for ListItemAttrs<'_> {
    type Item = (usize, ListItemAttr);

    fn next(&mut self) -> Option<Self::Item> {
        self.attrs
            .item_heights
            .get(self.cur_item)
            .map(|&v| {
                (
                    self.cur_item,
                    ListItemAttr {
                        area: Rect {
                            y: self.cur.area.y + self.cur.area.height,
                            height: v as u16,
                            ..self.cur.area
                        },
                    },
                )
            })
            .inspect(|v| {
                self.cur = v.1;
                self.cur_item += 1;
            })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ListItemAttr {
    pub area: Rect,
}

impl<'a, Message> List<'a, Message> {
    pub fn items(mut self, items: impl IntoIterator<Item: Into<ListItem<'a>>>) -> Self {
        let items: Vec<_> = items.into_iter().map(Into::into).collect();
        self.attrs.item_heights = items.iter().map(|v| v.height()).collect();
        self.base = self.base.items(items);
        self
    }

    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.base = self.base.style(style);
        self
    }

    pub fn on_mouse<F>(mut self, f: F) -> Self
    where
        F: FnMut(&MouseEvent, &ListState, ListItemAttrs<'_>) -> Option<Message> + 'a,
    {
        self.on_mouse = Some(Box::new(f));
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

    fn handle_mouse(&mut self, mouse: &MouseEvent) -> Option<Message> {
        let area = self.area();
        let on_mouse = self.on_mouse.as_mut()?;

        let item_attrs = ListItemAttrs {
            attrs: &self.attrs,
            cur: ListItemAttr {
                area: Rect { height: 0, ..area },
            },
            cur_item: self.state.offset(),
        };

        on_mouse(mouse, self.state, item_attrs)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        (&self.base).render(self.state.area.get(), buf, self.state);

        let offset = self.state.base.offset();
        self.state.position.set(offset);
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
    pub position: ScrollPosition,
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
