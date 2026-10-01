use std::{cell::Cell, rc::Rc};

use ratatui_core::layout::Rect;

#[derive(Debug, Clone)]
pub enum Area {
    Literal(Rect),
    Ref(Rc<Cell<Rect>>),
}

impl Default for Area {
    fn default() -> Self {
        Self::Literal(Default::default())
    }
}

impl From<Rect> for Area {
    fn from(value: Rect) -> Self {
        Self::Literal(value)
    }
}

impl From<Rc<Cell<Rect>>> for Area {
    fn from(value: Rc<Cell<Rect>>) -> Self {
        Self::Ref(value)
    }
}

impl From<&Rc<Cell<Rect>>> for Area {
    fn from(value: &Rc<Cell<Rect>>) -> Self {
        Self::Ref(value.clone())
    }
}

impl Area {
    pub fn get(&self) -> Rect {
        match self {
            Self::Literal(v) => *v,
            Self::Ref(v) => v.get(),
        }
    }

    pub fn set(&mut self, area: Rect) {
        match self {
            Self::Literal(v) => *v = area,
            Self::Ref(v) => v.set(area),
        }
    }
}

pub trait BindArea {
    fn bind_area(self, area: &Rc<Cell<Rect>>) -> Self;
}
