use std::sync::OnceLock;

use ratatui_core::text::Text;

static LOG_SENDER: OnceLock<flume::Sender<Event>> = OnceLock::new();

pub type LogStream = flume::r#async::RecvStream<'static, Event>;

pub fn init() -> LogStream {
    let (tx, rx) = flume::unbounded();

    LOG_SENDER
        .set(tx)
        .expect("`ratzgo::log::init` should only be called once");

    rx.into_stream()
}

#[derive(Debug)]
pub struct Event {
    pub level: Level,
    pub target: &'static str,
    pub text: Text<'static>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Level(u8);

pub fn error(target: &'static str, text: impl Into<Text<'static>>) {
    log_impl(Level::ERROR, target, text.into());
}

pub fn warn(target: &'static str, text: impl Into<Text<'static>>) {
    log_impl(Level::WARN, target, text.into());
}

pub fn info(target: &'static str, text: impl Into<Text<'static>>) {
    log_impl(Level::INFO, target, text.into());
}

pub fn debug(target: &'static str, text: impl Into<Text<'static>>) {
    log_impl(Level::DEBUG, target, text.into());
}

pub fn trace(target: &'static str, text: impl Into<Text<'static>>) {
    log_impl(Level::TRACE, target, text.into());
}

fn log_impl(level: Level, target: &'static str, text: Text<'static>) {
    if let Some(tx) = LOG_SENDER.get() {
        let _ = tx.try_send(Event {
            target,
            level,
            text,
        });
    }
}

impl std::fmt::Debug for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Level").field(&self.as_str()).finish()
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Level {
    pub const ERROR: Self = Self(0);

    pub const WARN: Self = Self(1);

    pub const INFO: Self = Self(2);

    pub const DEBUG: Self = Self(3);

    pub const TRACE: Self = Self(4);

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ERROR => "error",
            Self::WARN => "warn",
            Self::INFO => "info",
            Self::DEBUG => "debug",
            Self::TRACE => "trace",
            _ => "",
        }
    }
}
