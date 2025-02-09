use danubec_lex::Lex;
use danubec_syntax::{Checkpoint, GreenNodeBuilder, SyntaxKind, SyntaxNode};

pub struct Context {
    builder: GreenNodeBuilder,
}

impl Context {
    pub fn new() -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
        }
    }

    #[inline]
    pub(crate) fn token(&mut self, kind: SyntaxKind, source: &str) {
        self.builder.token(kind.into(), source);
    }

    #[inline]
    pub(crate) fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(kind.into());
    }

    #[inline]
    pub(crate) fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    #[inline]
    pub(crate) fn checkpoint(&mut self) -> Checkpoint {
        self.builder.checkpoint()
    }

    #[inline]
    pub(crate) fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder.start_node_at(checkpoint, kind.into());
    }

    #[inline]
    pub fn finish(self) -> SyntaxNode {
        SyntaxNode::new_root(self.builder.finish())
    }

    pub fn error(&mut self, lex: &mut Lex) {
        if let Some((kind, text)) = lex.next() {
            self.start_node(SyntaxKind::Error);
            self.token(kind, text);
            self.finish_node();
        }
    }
}

#[macro_export]
macro_rules! expect {
    ($context:expr, $lex:expr, $kind:pat) => {
        match $lex.clone().next() {
            Some((kind @ $kind, source)) => {
                $context.token(kind, source);
                $lex.next();

                true
            }
            _ => false,
        }
    };
    ($context:expr, $lex:expr, $kind:ident, $($pat:pat,)+) => {{
        let mut lex = $lex.clone();
        let mut source = String::new();
        let mut matched = true;
        let mut count = 0;

        $(
            if matched {
                if let Some(($pat, text)) = lex.next() {
                    source.push_str(text);
                    count += 1;
                } else {
                    matched = false;
                }
            }
        )+

        if matched {
            $context.token(SyntaxKind::$kind, &source);

            for _ in 0..count {
                $lex.next();
            }
        }

        matched
    }};
    ($context:expr, $lex:expr, $kind:ident -> $($source:pat,)+) => {{
        let mut lex = $lex.clone();
        let mut source = String::new();
        let mut matched = true;
        let mut count = 0;

        $(
            if matched {
                if let Some((_, text @ $source)) = lex.next() {
                    source.push_str(text);
                    count += 1;
                } else {
                    matched = false;
                }
            }
        )+

        if matched {
            $context.token(SyntaxKind::$kind, &source);

            for _ in 0..count {
                $lex.next();
            }
        }

        matched
    }};
}

#[macro_export]
macro_rules! consume_while {
    ($context:expr, $lex:expr, $kind:pat) => {{
        while let Some((kind @ $kind, source)) = $lex.clone().next() {
            $context.token(kind, source);
            $lex.next();
        }
    }};
    ($context:expr, $lex:expr, $kind:ident, $pat:pat) => {{
        let mut lex = $lex.clone();
        let mut source = String::new();
        let mut count = 0;
        let mut matched = false;

        while let Some(($pat, text)) = lex.next() {
            source.push_str(text);
            count += 1;
            matched = true;
        }

        if matched {
            $context.token(SyntaxKind::$kind, &source);

            for _ in 0..count {
                $lex.next();
            }
        }

        matched
    }};
}
