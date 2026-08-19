use std::mem;

use ratatui_crossterm::crossterm::event::KeyEvent;

use super::{filter_map::FilterMap, map::Map};

#[derive(Debug, Default)]
pub enum OnKey<'a, Message> {
    Map(Map<'a, Message>),
    FilterMap(FilterMap<'a, Message>),
    #[default]
    None,
}

impl<'a, Message> OnKey<'a, Message> {
    pub fn key(&mut self, key: &KeyEvent) -> Option<Message> {
        let on_key = mem::take(self);

        match on_key {
            Self::Map(v) => v.key(key),
            Self::FilterMap(v) => v.key(key),
            Self::None => None,
        }
    }

    #[track_caller]
    pub fn on_key<F>(&mut self, cond: F, msg: Message) -> &mut Self
    where
        F: FnOnce(&KeyEvent) -> bool + 'a,
    {
        match self {
            Self::FilterMap(v) => {
                v.on_key(cond, msg);
            }
            Self::None => {
                let mut v = FilterMap::default();
                v.on_key(cond, msg);
                *self = Self::FilterMap(v);
            }
            _ => panic!("`on_key` and `on_key_with` are mutually exclusive; choose only one"),
        }

        self
    }

    #[track_caller]
    pub fn on_key_with<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&KeyEvent) -> Option<Message> + 'a,
    {
        if let Self::FilterMap(_) = self {
            panic!("`on_key` and `on_key_with` are mutually exclusive; choose only one");
        }

        *self = Self::Map(Map::new(f));
        self
    }
}

pub trait OnKeyBuilder<'a, Message> {
    fn on_key_mut(&mut self) -> &mut OnKey<'a, Message>;

    #[track_caller]
    fn on_key<F>(mut self, cond: F, msg: Message) -> Self
    where
        F: FnOnce(&KeyEvent) -> bool + 'a,
        Self: Sized,
    {
        self.on_key_mut().on_key(cond, msg);
        self
    }

    #[track_caller]
    fn on_key_with<F>(mut self, f: F) -> Self
    where
        F: FnOnce(&KeyEvent) -> Option<Message> + 'a,
        Self: Sized,
    {
        self.on_key_mut().on_key_with(f);
        self
    }
}
