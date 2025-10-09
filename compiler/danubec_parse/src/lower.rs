use danubec_diagnostic::Diagnostic;
use std::collections::HashMap;

macro_rules! list {
    ($node:ident . $field:ident() . $lower:ident ( $diagnostic:ident )) => {{
        let mut items = vec![];
        for item in $node.$field() {
            match $lower(item, $diagnostic) {
                Ok(item) => items.push(item),
                Err(_) => continue,
            }
        }
        items
    }};
}

pub fn lower_krate(
    krate: danubec_syntax::Krate,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Krate, ()> {
    let attributes = list!(krate.attributes().lower_top_level_attribute(diagnostic));
    let definitions = list!(krate.definitions().lower_definition(diagnostic));

    Ok(danubec_ast::Krate {
        attributes,
        definitions,
        children: HashMap::new(),
    })
}

pub fn lower_root(
    root: danubec_syntax::Root,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Root, ()> {
    let definitions = list!(root.definitions().lower_definition(diagnostic));

    Ok(danubec_ast::Root {
        definitions,
        children: HashMap::new(),
    })
}

pub fn lower_top_level_attribute(
    attribute: danubec_syntax::TopLevelAttribute,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::TopLevelAttribute, ()> {
    let Some(argument) = attribute.argument() else {
        diagnostic.report(miette!("Expected attribute in top-level attribute"));
        return Err(());
    };
    let Ok(argument) = lower_attribute_argument(argument, diagnostic) else {
        return Err(());
    };

    Ok(danubec_ast::TopLevelAttribute { argument })
}

pub fn lower_attribute_argument(
    argument: danubec_syntax::AttributeArgument,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::AttributeArgument, ()> {
    std::todo!();
}

pub fn lower_definition(
    definition: danubec_syntax::Definition,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Definition, ()> {
    std::todo!();
}

pub fn lower_path(
    path: danubec_syntax::Path,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Path, ()> {
    std::todo!();
}
