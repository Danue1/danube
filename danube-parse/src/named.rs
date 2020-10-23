use crate::*;

pub trait Named {
    fn name(&self) -> &str;
}

impl Named for IdentNode {
    fn name(&self) -> &str {
        self.raw.as_ref()
    }
}

impl Named for PathNode {
    fn name(&self) -> &str {
        self.ident_list.last().unwrap().name()
    }
}

impl Named for ModuleNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for StructNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for EnumNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for FunctionNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for FunctionArgumentNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for GenericNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for TypeAliasNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for ClosureArgumentNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for TraitNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for TraitItemKind {
    fn name(&self) -> &str {
        match self {
            TraitItemKind::OutputType(node) => node.name(),
            TraitItemKind::Constant(node) => node.name(),
            TraitItemKind::Function(node) => node.name(),
        }
    }
}

impl Named for TraitItemOutputTypeNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for TraitItemConstantNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for TraitItemFunctionNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for ConstantNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for StaticNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for ImplementOutputTypeNode {
    fn name(&self) -> &str {
        self.ident.name()
    }
}

impl Named for AttributeNode {
    fn name(&self) -> &str {
        self.path.name()
    }
}
