use crate::{
    file_system::FileId,
    symbol::SymbolInterner,
    table::{ModuleId, ScopeId, Table},
};
use danubec_syntax::{AstNode, Definition, DefinitionKind, SyntaxNode};

pub struct DefinitionCollector<'lowering> {
    module: ModuleId,
    symbols: &'lowering mut SymbolInterner,
    table: &'lowering mut Table,
    scopes: Vec<ScopeId>,
    file: FileId,
}

impl<'lowering> DefinitionCollector<'lowering> {
    pub fn new(
        file: FileId,
        module: ModuleId,
        symbols: &'lowering mut SymbolInterner,
        table: &'lowering mut Table,
    ) -> Self {
        let scope = table[module].scope;

        Self {
            file,
            module,
            symbols,
            table,
            scopes: vec![scope],
        }
    }

    pub fn root(&mut self, node: SyntaxNode) {
        for definition in node.children().filter_map(Definition::cast) {
            self.definition(definition);
        }
    }

    fn definition(&mut self, node: Definition) {
        match node.kind() {
            Some(DefinitionKind::Function(_)) => {
                //
            }
            Some(DefinitionKind::Struct(_)) => {
                //
            }
            Some(DefinitionKind::Enum(_)) => {
                //
            }
            Some(DefinitionKind::Use(_)) => {
                //
            }
            Some(DefinitionKind::Module(_)) => {
                //
            }
            Some(DefinitionKind::Trait(_)) => {
                //
            }
            Some(DefinitionKind::Constant(_)) => {
                //
            }
            Some(DefinitionKind::Static(_)) => {
                //
            }
            Some(DefinitionKind::Type(_)) => {
                //
            }
            Some(DefinitionKind::Implement(_)) => {
                //
            }
            None => {
                //
            }
        }
    }
}
