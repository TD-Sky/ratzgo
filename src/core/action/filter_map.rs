use std::any::type_name_of_val;

use ratatui_crossterm::crossterm::event::KeyEvent;

#[derive(Debug)]
pub struct FilterMap<'a, Message> {
    key2msg: Vec<FilterMapFn<'a, Message>>,
}

impl<'a, Message> Default for FilterMap<'a, Message> {
    fn default() -> Self {
        Self {
            key2msg: Default::default(),
        }
    }
}

struct FilterMapFn<'a, Message> {
    cond: Box<dyn FnOnce(&KeyEvent) -> bool + 'a>,
    msg: Message,
}

impl<'a, Message> std::fmt::Debug for FilterMapFn<'a, Message>
where
    Message: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KeyMsgFn")
            .field(
                "cond",
                &format_args!("<closure of `{}`>", type_name_of_val(&self.cond)),
            )
            .field("msg", &self.msg)
            .finish()
    }
}

impl<'a, Message> FilterMap<'a, Message> {
    pub fn key(self, key: &KeyEvent) -> Option<Message> {
        self.key2msg
            .into_iter()
            .find_map(|v| (v.cond)(key).then_some(v.msg))
    }

    pub fn on_key(&mut self, cond: impl FnOnce(&KeyEvent) -> bool + 'a, msg: Message) -> &mut Self {
        self.key2msg.push(FilterMapFn {
            cond: Box::new(cond),
            msg,
        });
        self
    }
}
