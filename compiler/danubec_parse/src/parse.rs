use crate::{
    event::Event,
    grammar::{Parser, krate},
    lower::lower_krate,
};
use danubec_ast::Krate;
use danubec_diagnostic::Diagnostic;
use danubec_lex::lex;
use danubec_syntax::{AstNode, GreenNodeBuilder, SyntaxKind, SyntaxNode};
use std::path::PathBuf;

pub fn parse_crate_main(path: &PathBuf) -> Result<Krate, Diagnostic> {
    parse_krate(path.join("src/main.dnb"))
}

pub fn parse_crate_lib(path: &PathBuf) -> Result<Krate, Diagnostic> {
    parse_krate(path.join("src/lib.dnb"))
}

pub fn parse_krate(parent: PathBuf) -> Result<Krate, Diagnostic> {
    let mut diagnostic = Diagnostic::new();
    let source = match read(&parent, &mut diagnostic) {
        Ok(source) => source,
        Err(()) => return Err(diagnostic),
    };
    let (node, mut diagnostic) = parse(&source, diagnostic, krate);
    let node = danubec_syntax::Krate::cast(node)
        .and_then(|node| lower_krate(node, &mut diagnostic).ok())
        .ok_or_else(|| diagnostic)?;

    Ok(node)
}

fn read(path: &PathBuf, diagnostic: &mut Diagnostic) -> Result<String, ()> {
    match std::fs::read_to_string(path) {
        Ok(source) => Ok(source),
        Err(error) => {
            diagnostic.report(miette!("Failed to read file {}: {}", path.display(), error));

            Err(())
        }
    }
}

pub(crate) fn parse<P>(source: &str, diagnostic: Diagnostic, parse: P) -> (SyntaxNode, Diagnostic)
where
    P: FnOnce(&mut Parser),
{
    let tokens = lex(source);
    let (events, diagnostic) = {
        let tokens: Vec<_> = tokens.iter().map(|&(kind, _)| kind).collect();
        let mut parser = Parser::new(&tokens, diagnostic);
        parse(&mut parser);
        parser.finish()
    };
    let node = build(&tokens, events);

    (node, diagnostic)
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
                    Some((kind, _)) if kind.at_trivia() => {
                        advance!();
                    }
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
