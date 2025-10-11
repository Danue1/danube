use crate::{
    event::Event,
    grammar::{Context, krate},
};
use danubec_diagnostic::Diagnostic;
use danubec_lex::lex;
use danubec_syntax::{GreenNodeBuilder, SyntaxKind, SyntaxNode};

pub fn parse(source: &str, diagnostic: &mut Diagnostic) -> SyntaxNode {
    let mut parse = |tokens: &[_]| {
        let tokens: Vec<_> = tokens.iter().map(|&(kind, _)| kind).collect();
        let mut context = Context::new(&tokens, diagnostic);
        krate(&mut context);

        context.finish()
    };

    let tokens = lex(source);
    let events = parse(&tokens);

    build(&tokens, events)
}

fn build(mut tokens: &[(SyntaxKind, &str)], mut events: Vec<Event>) -> SyntaxNode {
    let mut builder = GreenNodeBuilder::new();

    macro_rules! advance {
        () => {
            if let Some(&(kind, text)) = tokens.get(0) {
                builder.token(kind.into(), text);
                tokens = &tokens[1..];
            }
        };
    }

    macro_rules! trivia {
        () => {
            loop {
                match tokens.get(0) {
                    Some((kind, _)) if kind.at_trivia() => advance!(),
                    _ => break,
                }
            }
        };
    }

    for index in 0..events.len() {
        match std::mem::replace(&mut events[index], Event::Placeholder) {
            event @ Event::Start { forward_parent, .. }
            | event @ Event::Expire { forward_parent } => {
                let mut forward_parents = match event {
                    Event::Start { kind, .. } => vec![kind],
                    _ => vec![],
                };
                let mut index = index;
                let mut forward_parent = forward_parent;

                while let Some(fp) = forward_parent {
                    index += fp;
                    match std::mem::replace(&mut events[index], Event::Placeholder) {
                        Event::Start {
                            kind,
                            forward_parent: fp,
                        } => {
                            forward_parents.push(kind);
                            forward_parent = fp;
                        }
                        Event::Expire { forward_parent: fp } => {
                            forward_parent = fp;
                        }
                        _ => {
                            //
                        }
                    }
                }

                for kind in forward_parents.into_iter().rev() {
                    builder.start_node(kind.into());
                }

                trivia!();
            }
            Event::Token => {
                advance!();
                trivia!();
            }
            Event::End => builder.finish_node(),
            Event::Placeholder => {
                //
            }
        }
    }

    SyntaxNode::new_root(builder.finish())
}
