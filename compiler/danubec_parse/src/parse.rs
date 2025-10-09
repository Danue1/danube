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
    let (node, mut diagnostic) = parse(&source, diagnostic);
    let krate = match danubec_syntax::Krate::cast(node) {
        Some(node) => lower_krate(node, &mut diagnostic).map_err(|_| diagnostic)?,
        None => {
            diagnostic.report(miette!("ICE: Expected krate node"));
            return Err(diagnostic);
        }
    };

    Ok(krate)
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

pub(crate) fn parse(source: &str, diagnostic: Diagnostic) -> (SyntaxNode, Diagnostic) {
    let tokens = lex(source);
    let (events, diagnostic) = {
        let tokens: Vec<_> = tokens.iter().map(|&(kind, _)| kind).collect();
        let mut parser = Parser::new(&tokens, diagnostic);
        krate(&mut parser);
        parser.finish()
    };
    let node = build(&tokens, events);

    (node, diagnostic)
}

fn build(mut tokens: &[(SyntaxKind, &str)], events: Vec<Event>) -> SyntaxNode {
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

    for event in events {
        match event {
            Event::Start(kind) => {
                builder.start_node(kind.into());
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
