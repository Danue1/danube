#![warn(clippy::all)]

use std::borrow::Cow;

pub struct Diagnostic {
    messages: Vec<Message>,
}

pub struct Message {
    level: Level,
    message: Cow<'static, str>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    #[default]
    ERROR,
    WARN,
    DEBUG,
    INFO,
}

impl Diagnostic {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn report(&mut self, level: Level, message: impl Into<Cow<'static, str>>) {
        self.messages.push(Message {
            level,
            message: message.into(),
        });
    }
}

impl std::fmt::Debug for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for message in &self.messages {
            writeln!(f, "{:?}", message)?;
        }

        Ok(())
    }
}

impl std::fmt::Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.level, self.message)
    }
}

#[macro_export]
macro_rules! error {
    ($diagnostic:expr, $($arg:tt)*) => {
        $diagnostic.report($crate::Level::ERROR, format!($($arg)*))
    };
}
