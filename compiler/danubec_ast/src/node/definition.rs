use super::{Expression, Path, Statement, Type, Visibility};
use danubec_symbol::Symbol;

pub struct Definition {
    pub kind: DefinitionKind,
}

pub enum DefinitionKind {
    Const {
        visibility: Visibility,
        ident: Symbol,
        ty: Type,
        expression: Expression,
    },
    Enum {
        visibility: Visibility,
        ident: Symbol,
        type_parameters: Vec<TypeParameter>,
        predicates: Vec<Predicate>,
        variants: Vec<EnumVariant>,
    },
    Function(FunctionDef),
    Impl {
        type_parameters: Vec<TypeParameter>,
        trait_type: Option<Type>,
        target_type: Type,
        predicates: Vec<Predicate>,
        definitions: Vec<ImplItem>,
    },
    Module {
        visibility: Visibility,
        ident: Symbol,
        definitions: Vec<Definition>,
    },
    Static {
        visibility: Visibility,
        ident: Symbol,
        ty: Type,
        expression: Expression,
    },
    Struct {
        visibility: Visibility,
        ident: Symbol,
        type_parameters: Vec<TypeParameter>,
        predicates: Vec<Predicate>,
        kind: Option<StructKind>,
    },
    Trait {
        visibility: Visibility,
        ident: Symbol,
        type_parameters: Vec<TypeParameter>,
        predicates: Vec<Predicate>,
        definitions: Vec<TraitItem>,
    },
    Type {
        visibility: Visibility,
        ident: Symbol,
        type_parameters: Vec<TypeParameter>,
        ty: Type,
    },
    Use {
        visibility: Visibility,
        tree: UseTree,
    },
}

pub struct FunctionDef {
    pub visibility: Visibility,
    pub ident: Symbol,
    pub type_parameters: Vec<TypeParameter>,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub predicates: Vec<Predicate>,
    pub body: Vec<Statement>,
}

pub struct Parameter {
    pub ident: Symbol,
    pub ty: Type,
}

pub struct TypeParameter {
    pub ident: Symbol,
    pub bounds: Vec<Type>,
}

pub struct Predicate {
    pub ty: Type,
    pub bounds: Vec<Type>,
}

pub struct EnumVariant {
    pub ident: Symbol,
    pub kind: EnumVariantKind,
}

pub enum EnumVariantKind {
    Unit,
    Named(Vec<(Symbol, Type)>),
    Unnamed(Vec<Type>),
    Sequence(Expression),
}

pub enum ImplItem {
    Const {
        visibility: Visibility,
        ident: Symbol,
        ty: Type,
        expression: Expression,
    },
    Function(FunctionDef),
    Type {
        visibility: Visibility,
        ident: Symbol,
        type_parameters: Vec<TypeParameter>,
        bounds: Vec<Predicate>,
        ty: Type,
    },
}

pub enum TraitItem {
    Const {
        visibility: Visibility,
        ident: Symbol,
        ty: Type,
        expression: Option<Expression>,
    },
    Function(FunctionDef),
    Type {
        visibility: Visibility,
        ident: Symbol,
        type_parameters: Vec<TypeParameter>,
        bounds: Vec<Predicate>,
        ty: Option<Type>,
    },
}

pub enum StructKind {
    Named(Vec<(Visibility, Symbol, Type)>),
    Unnamed(Vec<(Visibility, Type)>),
}

pub struct UseTree {
    pub path: Path,
    pub kind: Option<UseTreeKind>,
}

pub enum UseTreeKind {
    Barrel,
    Alias(Symbol),
    Nested(Vec<UseTree>),
}
