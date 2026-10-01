use std::{any, cell::Cell, rc::Rc};

use ratatui_core::{
    buffer::Buffer,
    layout::{Position, Rect},
    style::Style,
    text::Line,
    widgets::Widget,
};
use ratatui_crossterm::crossterm::event::KeyEvent;
pub use ratatui_widgets::borders::{BorderType, Borders};

use crate::core::*;

pub fn block<'a, Message, W>(inner: W) -> Block<'a, Message, W> {
    Block {
        base: ratatui_widgets::block::Block::new(),
        area: Default::default(),
        inner,
        components: vec![],
        on_key: Default::default(),
    }
}

#[derive(Debug)]
pub struct Block<'a, Message, W = Box<dyn Component<Message> + 'a>> {
    base: ratatui_widgets::block::Block<'a>,
    area: Area,
    inner: W,
    components: Vec<Decoration<'a, Message>>,
    on_key: OnKey<'a, Message>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderOrientation {
    Top,
    Bottom,
    Left,
    Right,
}

impl<'a, Message, W> Block<'a, Message, W> {
    pub fn title(mut self, title: impl Into<Line<'a>>) -> Self {
        self.base = self.base.title(title);
        self
    }

    pub fn title_top(mut self, title: impl Into<Line<'a>>) -> Self {
        self.base = self.base.title_top(title);
        self
    }

    pub fn title_bottom(mut self, title: impl Into<Line<'a>>) -> Self {
        self.base = self.base.title_bottom(title);
        self
    }

    pub fn bordered(mut self) -> Self {
        self.base = self.base.borders(Borders::all());
        self
    }

    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.base = self.base.border_type(border_type);
        self
    }

    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.base = self.base.style(style);
        self
    }

    pub fn decorate<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ratatui_widgets::block::Block<'a>) -> ratatui_widgets::block::Block<'a>,
    {
        self.base = f(self.base);
        self
    }

    pub fn top_component(
        self,
        cpt: impl Component<Message> + 'a,
        mut pos: impl FnMut(Rect) -> Rect + 'a,
    ) -> Self {
        self.top_component_opt(cpt, move |v| Some(pos(v)))
    }

    pub fn bottom_component(
        self,
        cpt: impl Component<Message> + 'a,
        mut pos: impl FnMut(Rect) -> Rect + 'a,
    ) -> Self {
        self.bottom_component_opt(cpt, move |v| Some(pos(v)))
    }

    pub fn left_component(
        self,
        cpt: impl Component<Message> + 'a,
        mut pos: impl FnMut(Rect) -> Rect + 'a,
    ) -> Self {
        self.left_component_opt(cpt, move |v| Some(pos(v)))
    }

    pub fn right_component(
        self,
        cpt: impl Component<Message> + 'a,
        mut pos: impl FnMut(Rect) -> Rect + 'a,
    ) -> Self {
        self.right_component_opt(cpt, move |v| Some(pos(v)))
    }

    pub fn top_component_opt(
        self,
        cpt: impl Component<Message> + 'a,
        pos: impl FnMut(Rect) -> Option<Rect> + 'a,
    ) -> Self {
        self.add_component(Decoration {
            base: cpt.boxed(),
            orientation: BorderOrientation::Top,
            pos: Box::new(pos),
        })
    }

    pub fn bottom_component_opt(
        self,
        cpt: impl Component<Message> + 'a,
        pos: impl FnMut(Rect) -> Option<Rect> + 'a,
    ) -> Self {
        self.add_component(Decoration {
            base: cpt.boxed(),
            orientation: BorderOrientation::Bottom,
            pos: Box::new(pos),
        })
    }

    pub fn left_component_opt(
        self,
        cpt: impl Component<Message> + 'a,
        pos: impl FnMut(Rect) -> Option<Rect> + 'a,
    ) -> Self {
        self.add_component(Decoration {
            base: cpt.boxed(),
            orientation: BorderOrientation::Left,
            pos: Box::new(pos),
        })
    }

    pub fn right_component_opt(
        self,
        cpt: impl Component<Message> + 'a,
        pos: impl FnMut(Rect) -> Option<Rect> + 'a,
    ) -> Self {
        self.add_component(Decoration {
            base: cpt.boxed(),
            orientation: BorderOrientation::Right,
            pos: Box::new(pos),
        })
    }
}

impl<'a, Message, W> Block<'a, Message, W> {
    fn add_component(mut self, cpt: Decoration<'a, Message>) -> Self {
        self.components.push(cpt);
        self
    }
}

impl<'a, Message, W> Component<Message> for Block<'a, Message, W>
where
    Message: std::fmt::Debug,
    W: Component<Message>,
{
    fn activity(&self) -> bool {
        self.inner.activity()
    }

    fn area(&self) -> Rect {
        self.area.get()
    }

    fn set_area(&mut self, area: Rect) {
        self.area.set(area);
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        self.inner.handle_key(key).or_else(|| self.on_key.key(key))
    }

    fn handle_click(&mut self, pos: Position) -> Option<Message> {
        self.inner.handle_click(pos)
    }

    fn handle_paste(&mut self, content: &str) -> Option<Message> {
        self.inner.handle_paste(content)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        let area = self.area.get();

        let inner_area = self.base.inner(area);
        self.inner.render(inner_area, buf);
        (&self.base).render(area, buf);

        self.components.retain_mut(|cpt| {
            let border = match cpt.orientation {
                BorderOrientation::Top => area.rows().next(),
                BorderOrientation::Bottom => area.rows().next_back(),
                BorderOrientation::Left => area.columns().next(),
                BorderOrientation::Right => area.columns().next_back(),
            };
            let Some(border_area) = border else {
                return false;
            };

            let Some(pos_area) = (cpt.pos)(border_area) else {
                return false;
            };

            cpt.base.render(pos_area, buf);

            true
        });
    }
}

impl<'a, Message, W> BindArea for Block<'a, Message, W> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> OnKeyBuilder<'a, Message> for Block<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message> {
        &mut self.on_key
    }
}

struct Decoration<'a, Message> {
    base: Box<dyn Component<Message> + 'a>,
    orientation: BorderOrientation,
    pos: Box<dyn FnMut(Rect) -> Option<Rect> + 'a>,
}

impl<'a, Message> std::fmt::Debug for Decoration<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Decoration")
            .field("base", &self.base)
            .field("orientation", &self.orientation)
            .field(
                "pos",
                &format_args!("<closure of `{}`>", any::type_name_of_val(&self.pos)),
            )
            .finish()
    }
}
