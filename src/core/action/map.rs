use std::any::type_name_of_val;

use ratatui_crossterm::crossterm::event::KeyEvent;

pub struct Map<'a, Message> {
    inner: Inner<'a, Message>,
}

type Inner<'a, Message> = Box<dyn FnOnce(&KeyEvent) -> Option<Message> + 'a>;

impl<'a, Message> std::fmt::Debug for Map<'a, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Map")
            .field(
                "inner",
                &format_args!("<closure of `{}`>", type_name_of_val(&self.inner)),
            )
            .finish()
    }
}

impl<'a, Message> Map<'a, Message> {
    pub fn new(f: impl FnOnce(&KeyEvent) -> Option<Message> + 'a) -> Self {
        Self { inner: Box::new(f) }
    }

    pub fn key(self, key: &KeyEvent) -> Option<Message> {
        (self.inner)(key)
    }
}
