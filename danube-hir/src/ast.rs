use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub feature_list: Vec<Feature>,
    pub items: BTreeMap<Id, Item>,
}

#[derive(Debug, PartialEq)]
pub struct Feature {
    pub path: Path,
    pub args: BTreeMap<String, Option<LiteralKind>>,
}

#[derive(Debug, PartialEq)]
pub struct Item {
    pub id: Id,
    pub visibility: VisibilityKind,
    pub ident: Ident,
    pub attribute_list: Vec<Attribute>,
    pub kind: ItemKind,
}

#[derive(Debug, PartialEq)]
pub struct Attribute {
    pub path: Path,
    pub args: BTreeMap<String, Option<LiteralKind>>,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum ItemKind {
    Use(Path),
    Module,
    Struct(VariantKind),
    Enum,
    Function,
    TypeAlias(Ty, Generics),
    Trait,
    Constant,
    Static,
    Implement,
}

#[derive(Debug, PartialEq)]
pub enum VariantKind {
    Named(Vec<StructField>),
    Unnamed(Vec<StructField>, Id),
}

#[derive(Debug, PartialEq)]
pub struct StructField {
    pub id: Id,
    pub ident: Ident,
    pub visibility: VisibilityKind,
    pub ty: Ty,
}

#[derive(Debug, PartialEq)]
pub enum VisibilityKind {
    Public,
    Restricted { id: Id, path: Path },
    Current,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionKind {
    // FIXME
    Todo,
}

#[derive(Debug, PartialEq)]
pub struct Generics {
    pub parameter_list: Vec<GenericParameter>,
}

#[derive(Debug, PartialEq)]
pub enum TypeKind {
    Array(Ty),
    Function(FunctionDeclaration, BodyId),
    Inference,
    Never,
    Error,
}

#[derive(Debug, PartialEq)]
pub struct GenericParameter {
    // FIXME
    todo: String,
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    pub self_type: Option<ImmutablityKind>,
    pub input_list: Vec<Ty>,
    pub output_list: FunctionReturnTypeKind,
}

#[derive(Debug, PartialEq)]
pub enum FunctionReturnTypeKind {
    Default,
    Return(Ty),
}

#[derive(Debug, PartialEq)]
pub struct FunctionBody {
    pub parameter_list: Vec<FunctionArgument>,
    pub value: ExpressionKind,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArgument {
    pub immutablity: ImmutablityKind,
    pub pattern: PatternKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ImmutablityKind {
    Yes,
    Nope,
}

#[derive(Debug, PartialEq)]
pub struct Ty {
    pub id: Id,
    pub kind: Box<TypeKind>,
}

#[derive(Debug, PartialEq)]
pub enum StatementKind {
    Todo,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PatternKind {
    Placeholder,
    Path(Path),
    // TODO: implement other
    // Literal(),
    // NamedStruct(),
    // UnnamedStruct(ResolveKind, ),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub resolve: ResolveKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ResolveKind {
    Local(Id),
    Primitive(PrimitiveKind),
    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveKind {
    Int,
    Float,
    String,
}

#[derive(Debug, PartialEq)]
pub struct Ident {
    pub name: String,
}

impl Ident {
    pub fn from_usize(name: usize) -> Ident {
        Ident {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Copy, Default)]
pub struct Id(u32);

impl Clone for Id {
    fn clone(&self) -> Self {
        *self
    }
}

impl Id {
    pub fn from_usize(id: usize) -> Self {
        Id(id as u32)
    }

    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Default)]
pub struct BodyId(u32);
