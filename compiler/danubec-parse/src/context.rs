use danubec_syntax_kind::SyntaxKind;
use rowan::{Checkpoint, GreenNodeBuilder};

pub struct Context {
    pub tokens: Vec<(SyntaxKind, String)>,
    pub builder: GreenNodeBuilder<'static>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Continue,
    Stop,
}

impl Context {
    #[inline]
    pub fn new(tokens: Vec<(SyntaxKind, String)>) -> Self {
        Self {
            tokens,
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    pub fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind.into());
    }

    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    pub fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn bump(&mut self) {
        let (kind, text) = self.tokens.pop().unwrap();
        self.builder.token(kind.into(), text.as_str());
    }

    pub fn peek(&self) -> SyntaxKind {
        match self.tokens.last() {
            Some((kind, _)) => *kind,
            None => SyntaxKind::EOF,
        }
    }

    #[inline]
    pub fn is_eof(&self) -> bool {
        self.peek() == SyntaxKind::EOF
    }

    pub fn skip_whitespace(&mut self) {
        while self.peek().is_whitespace() {
            self.bump();
        }
    }
}

#[macro_export]
macro_rules! expect {
    ($context:ident, $pat:pat) => {
        match $context.peek() {
            $pat => {
                $context.bump();
                true
            }
            _ => false,
        }
    };
}

#[macro_export]
macro_rules! guard {
    ($context:ident, $match:ident, $kind:ident) => {
        guard!($context, $match, $kind, bump);
    };
    ($context:ident, $match:ident, $kind:ident, $bump:ident) => {
        match $context.peek() {
            SyntaxKind::$match => {
                $context.start_node(SyntaxKind::$kind.into());
                $context.$bump();
            }
            _ => return crate::State::Continue,
        }
    };
}

#[macro_export]
macro_rules! one_of {
    ($head:expr) => {
        $head
    };
    ($head:expr, $($tail:expr),+) => {
        if $head == crate::State::Continue {
            one_of!($($tail),+)
        } else {
            crate::State::Stop
        }
    };
}
