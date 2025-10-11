use crate::{
    collect::DefinitionCollector,
    file_system::{FileId, FileSystem},
    symbol::{Symbol, SymbolInterner},
    table::{Definition, DefinitionKind, Namespace, Table},
};
use danubec_diagnostic::Diagnostic;
use danubec_parse::parse;
use danubec_syntax::SyntaxNode;
use std::collections::{HashSet, VecDeque};

pub fn build(
    fs: &mut FileSystem,
    table: &mut Table,
    symbols: &mut SymbolInterner,
    diagnostic: &mut Diagnostic,
    root: FileId,
) {
    let root = table.module(root, None);

    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(module) = queue.pop_front() {
        let file = table[module].file;
        let Some(source) = fs.source(file) else {
            diagnostic.report(miette!("File not found: {:?}", fs.path(file)));
            continue;
        };
        let node = parse(&source, diagnostic);

        {
            let mut collector = DefinitionCollector::new(file, module, symbols, table);
            collector.root(node.clone())
        }

        for name in external_modules(&node, symbols) {
            let Some(child_file) = fs.module(file, &symbols[name]) else {
                diagnostic.report(miette!("Module '{}' not found", &symbols[name]));
                continue;
            };
            let child_module = table.module(child_file, Some(module));
            table[module].children.insert(name, child_module);
            queue.push_back(child_module);

            let definition_id = table.definition(Definition {
                parent_scope: table[module].scope,
                name,
                namespace: Namespace::Type,
                kind: DefinitionKind::Module,
                file,
            });
            let scope = table[module].scope;
            table.scopes[scope].types.insert(name, definition_id);
        }
    }
}

fn external_modules(node: &SyntaxNode, symbols: &mut SymbolInterner) -> HashSet<Symbol> {
    use danubec_syntax::{AstNode, Definition, DefinitionKind, ModuleDefinitionKind};

    node.children()
        .filter_map(Definition::cast)
        .filter_map(|d| match d.kind() {
            Some(DefinitionKind::Module(m))
                if matches!(m.kind(), Some(ModuleDefinitionKind::External(_))) =>
            {
                m.name()
                    .and_then(|n| n.segment())
                    .and_then(|s| s.identifier())
                    .map(|name| symbols.intern(name.text()))
            }
            _ => None,
        })
        .collect()
}
