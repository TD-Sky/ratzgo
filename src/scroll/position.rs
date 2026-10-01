use std::{cell::Cell, rc::Rc};

#[derive(Debug, Clone)]
pub enum ScrollPosition {
    Literal(usize),
    Ref(Rc<Cell<usize>>),
}

impl Default for ScrollPosition {
    fn default() -> Self {
        Self::Literal(0)
    }
}

impl From<usize> for ScrollPosition {
    fn from(value: usize) -> Self {
        Self::Literal(value)
    }
}

impl From<Rc<Cell<usize>>> for ScrollPosition {
    fn from(value: Rc<Cell<usize>>) -> Self {
        Self::Ref(value)
    }
}

impl From<&Rc<Cell<usize>>> for ScrollPosition {
    fn from(value: &Rc<Cell<usize>>) -> Self {
        Self::Ref(value.clone())
    }
}

impl ScrollPosition {
    pub fn get(&self) -> usize {
        match self {
            Self::Literal(v) => *v,
            Self::Ref(v) => v.get(),
        }
    }

    pub fn set(&mut self, position: usize) {
        match self {
            Self::Literal(v) => *v = position,
            Self::Ref(v) => v.set(position),
        }
    }
}
