use super::{Expression, Path, Statement, Type, Visibility};
use danubec_symbol::Symbol;
use danubec_syntax::SyntaxNode;

pub struct Definition {
    pub syntax: SyntaxNode,
    pub kind: DefinitionKind,
}

pub enum DefinitionKind {
    Const(ConstDef),
    Enum {
        syntax: SyntaxNode,
        visibility: Option<Visibility>,
        ident: Symbol,
        types: Vec<Predicate>,
        predicates: Vec<Predicate>,
        variants: Vec<EnumVariant>,
    },
    Function(FunctionDef),
    Impl {
        type_parameters: Vec<Predicate>,
        trait_type: Option<Type>,
        target_type: Type,
        predicates: Vec<Predicate>,
        definitions: Vec<AssociatedDefinition>,
    },
    Module {
        visibility: Option<Visibility>,
        ident: Symbol,
        definitions: Vec<Definition>,
    },
    Static {
        visibility: Option<Visibility>,
        ident: Symbol,
        ty: Type,
        expression: Expression,
    },
    Struct {
        syntax: SyntaxNode,
        visibility: Option<Visibility>,
        ident: Symbol,
        type_parameters: Vec<Predicate>,
        predicates: Vec<Predicate>,
        kind: StructKind,
    },
    Trait {
        syntax: SyntaxNode,
        visibility: Option<Visibility>,
        ident: Symbol,
        type_parameters: Vec<Predicate>,
        predicates: Vec<Predicate>,
        definitions: Vec<AssociatedDefinition>,
    },
    Type {
        visibility: Option<Visibility>,
        ident: Symbol,
        type_parameters: Vec<Predicate>,
        ty: Type,
    },
    Use {
        visibility: Option<Visibility>,
        tree: UseTree,
    },
}

pub struct ConstDef {
    pub syntax: SyntaxNode,
    pub visibility: Option<Visibility>,
    pub ident: Symbol,
    pub ty: Type,
    pub expression: Expression,
}

pub struct FunctionDef {
    pub syntax: SyntaxNode,
    pub visibility: Option<Visibility>,
    pub ident: Symbol,
    pub type_parameters: Vec<Predicate>,
    pub parameters: Vec<(Symbol, Type)>,
    pub return_type: Option<Type>,
    pub predicates: Vec<Predicate>,
    pub body: Vec<Statement>,
}

pub struct Predicate {
    pub syntax: SyntaxNode,
    pub ty: Type,
    pub bounds: Type,
}

pub struct EnumVariant {
    pub syntax: SyntaxNode,
    pub ident: Symbol,
    pub kind: EnumVariantKind,
}

pub enum EnumVariantKind {
    Named(Vec<(Symbol, Type)>),
    Unnamed(Vec<Type>),
    Sequence(Vec<Expression>),
}

pub enum AssociatedDefinition {
    Function(FunctionDef),
    Type {
        visibility: Option<Visibility>,
        ident: Symbol,
        type_parameters: Vec<Predicate>,
        bounds: Vec<Predicate>,
        ty: Option<Type>,
    },
    Const(ConstDef),
}

pub enum StructKind {
    Named(Vec<(Symbol, Type)>),
    Unnamed(Vec<Type>),
}

pub struct UseTree {
    pub syntax: SyntaxNode,
    pub path: Path,
    pub kind: UseTreeKind,
}

pub enum UseTreeKind {
    Barrel,
    Alias(Symbol),
    Nested(Vec<UseTree>),
}
