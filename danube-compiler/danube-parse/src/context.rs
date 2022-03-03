use crate::Cursor;
use danube_diagnostics::{Diagnostics, Message};
use danube_token::Token;
use std::cell::RefCell;

pub(crate) struct Context<'context> {
    pub(crate) cursor: Cursor<'context>,
    diagnostics: &'context RefCell<Diagnostics>,
}

impl<'context> Context<'context> {
    pub(crate) fn new(
        token: &'context [Token],
        diagnostics: &'context RefCell<Diagnostics>,
    ) -> Self {
        Context {
            cursor: Cursor::new(token),
            diagnostics,
        }
    }

    pub(crate) fn report<T>(&self, message: Message) -> Result<T, ()> {
        self.diagnostics.borrow_mut().report(message);

        Err(())
    }
}

#[macro_export]
macro_rules! assert_node {
    (
        $(
            $(
                #[$meta:meta]
            )+
            fn $name:ident() -> $node:ident {
                let source = $source:expr;

                assert_eq!(source, $expect:expr,);
            }
        )+
    ) => {
        $(
            $(
                #[$meta]
            )+
            fn $name() {
                use crate::{Context, Parse};
                use danube_diagnostics::Diagnostics;
                use danube_lex::Lex;
                use danube_token::Token;
                use std::cell::RefCell;

                let tokens: Vec<Token> = Lex::new($source).filter_map(|token| token.ok()).collect();
                let diagnostics = RefCell::new(Diagnostics::new());
                let mut context = Context::new(&tokens, &diagnostics);

                assert_eq!($node::parse(&mut context), $expect);
            }
        )+
    };
}
