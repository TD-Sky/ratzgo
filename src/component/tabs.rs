use std::{any, cell::Cell, rc::Rc};

use ratatui_core::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::Widget,
};
use ratatui_crossterm::crossterm::event::{KeyEvent, MouseEvent};
use unicode_width::UnicodeWidthStr;

use crate::core::*;

type OnMouse<'a, Message> = Box<dyn FnMut(&MouseEvent, TabElements<'_>) -> Option<Message> + 'a>;

pub struct Tabs<'a, Message> {
    base: ratatui_widgets::tabs::Tabs<'a>,
    area: Area,
    activity: bool,
    on_key: OnKey<'a, Message>,
    on_mouse: Option<OnMouse<'a, Message>>,
    attrs: Attrs,
}

impl<'a, Message: std::fmt::Debug> std::fmt::Debug for Tabs<'a, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tabs")
            .field("base", &self.base)
            .field("area", &self.area)
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

#[derive(Debug, Clone)]
struct Attrs {
    padding_left_width: usize,
    title_widths: Vec<usize>,
    padding_right_width: usize,
    divider_width: usize,
}

impl<'a, Message> Tabs<'a, Message> {
    pub fn new(titles: impl IntoIterator<Item = Line<'a>>) -> Self {
        let titles: Vec<_> = titles.into_iter().collect();
        let attrs = Attrs {
            title_widths: titles.iter().map(|v| v.width_cjk()).collect(),
            ..Default::default()
        };

        Self {
            base: ratatui_widgets::tabs::Tabs::new(titles),
            area: Default::default(),
            activity: false,
            on_key: OnKey::default(),
            on_mouse: None,
            attrs,
        }
    }

    pub fn padding_left(mut self, padding: impl Into<Line<'a>>) -> Self {
        let v = padding.into();
        self.attrs.padding_left_width = v.width_cjk();
        self.base = self.base.padding_left(v);
        self
    }

    pub fn padding_right(mut self, padding: impl Into<Line<'a>>) -> Self {
        let v = padding.into();
        self.attrs.padding_right_width = v.width_cjk();
        self.base = self.base.padding_right(v);
        self
    }

    pub fn divider(mut self, divider: impl Into<Span<'a>>) -> Self {
        let v = divider.into();
        self.attrs.divider_width = v.width_cjk();
        self.base = self.base.divider(v);
        self
    }

    pub fn select(mut self, index: impl Into<Option<usize>>) -> Self {
        self.base = self.base.select(index);
        self
    }

    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.base = self.base.style(style);
        self
    }

    pub fn decorate<F>(mut self, f: F) -> Self
    where
        F: FnOnce(ratatui_widgets::tabs::Tabs<'a>) -> ratatui_widgets::tabs::Tabs<'a>,
    {
        self.base = f(self.base);
        self
    }

    pub fn on_mouse<F>(mut self, f: F) -> Self
    where
        F: FnMut(&MouseEvent, TabElements) -> Option<Message> + 'a,
    {
        self.on_mouse = Some(Box::new(f));
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TabElement {
    pub kind: TabElementKind,
    pub area: Rect,
}

#[derive(Debug, Clone, Copy)]
pub enum TabElementKind {
    Start,
    Title { index: usize },
    PaddingLeft,
    Divider,
    PaddingRight,
    End,
}

#[derive(Debug)]
pub struct TabElements<'t> {
    cur: TabElement,
    cur_title: usize,
    area: Rect,
    attrs: &'t Attrs,
}

impl Iterator for TabElements<'_> {
    type Item = TabElement;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur = match self.cur.kind {
            TabElementKind::Start => TabElement {
                kind: TabElementKind::PaddingLeft,
                area: Rect {
                    x: self.area.x,
                    width: self.attrs.padding_left_width as u16,
                    ..self.area
                },
            },
            TabElementKind::PaddingLeft => match self.attrs.title_widths.get(self.cur_title) {
                Some(&width) => TabElement {
                    kind: TabElementKind::Title {
                        index: self.cur_title,
                    },
                    area: Rect {
                        x: self.cur.area.x + self.cur.area.width,
                        width: width as u16,
                        ..self.area
                    },
                },
                None => TabElement::END,
            },
            TabElementKind::Title { .. } => {
                self.cur_title += 1;
                TabElement {
                    kind: TabElementKind::PaddingRight,
                    area: Rect {
                        x: self.cur.area.x + self.cur.area.width,
                        width: self.attrs.padding_right_width as u16,
                        ..self.area
                    },
                }
            }
            TabElementKind::PaddingRight => {
                if self.cur_title < self.attrs.title_widths.len() {
                    TabElement {
                        kind: TabElementKind::Divider,
                        area: Rect {
                            x: self.cur.area.x + self.cur.area.width,
                            width: self.attrs.divider_width as u16,
                            ..self.area
                        },
                    }
                } else {
                    TabElement::END
                }
            }
            TabElementKind::Divider => TabElement {
                kind: TabElementKind::PaddingLeft,
                area: Rect {
                    x: self.cur.area.x + self.cur.area.width,
                    width: self.attrs.padding_left_width as u16,
                    ..self.area
                },
            },
            TabElementKind::End => return None,
        };

        (!matches!(self.cur.kind, TabElementKind::End)).then_some(self.cur)
    }
}

impl TabElement {
    pub const START: Self = Self {
        kind: TabElementKind::Start,
        area: Rect::ZERO,
    };

    pub const END: Self = Self {
        kind: TabElementKind::End,
        area: Rect::ZERO,
    };
}

impl<'a, Message> Component<Message> for Tabs<'a, Message>
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

    fn handle_mouse(&mut self, mouse: &MouseEvent) -> Option<Message> {
        let elmts = TabElements {
            cur: TabElement::START,
            cur_title: 0,
            area: self.area(),
            attrs: &self.attrs,
        };

        let f = self.on_mouse.as_mut()?;
        f(mouse, elmts)
    }

    fn handle_key(&mut self, key: &KeyEvent) -> Option<Message> {
        self.on_key.key(key)
    }

    fn adapt(&mut self, buf: &mut Buffer) {
        (&self.base).render(self.area.get(), buf);
    }
}

impl<'a, Message> BindArea for Tabs<'a, Message> {
    fn bind_area(mut self, area: &Rc<Cell<Rect>>) -> Self {
        self.area = Area::Ref(area.clone());
        self
    }
}

impl<'a, Message> Activable for Tabs<'a, Message> {
    fn active_mut(&mut self) -> &mut bool {
        &mut self.activity
    }
}

impl<'a, Message> OnKeyBuilder<'a, Message> for Tabs<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message> {
        &mut self.on_key
    }
}

impl Default for Attrs {
    fn default() -> Self {
        Self {
            padding_left_width: " ".width_cjk(),
            title_widths: vec![],
            padding_right_width: " ".width_cjk(),
            divider_width: "|".width_cjk(),
        }
    }
}

#[macro_export]
macro_rules! tabs {
    ($($title:expr),+ $(,)?) => {
        $crate::component::Tabs::new(
            [$($crate::text::Line::from($title)),+],
        )
    };
}
pub use tabs;
