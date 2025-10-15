use danubec_syntax::{Danube, SyntaxKind, SyntaxNode, SyntaxToken};

pub fn first_token(node: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
    use rowan::NodeOrToken;

    node.children_with_tokens().find_map(|node| match node {
        NodeOrToken::Token(token) if token.kind() == kind => Some(token),
        _ => None,
    })
}

pub fn tokens(node: &SyntaxNode, kind: SyntaxKind) -> impl Iterator<Item = SyntaxToken> {
    use rowan::NodeOrToken;

    node.children_with_tokens()
        .filter_map(move |node| match node {
            NodeOrToken::Token(token) if token.kind() == kind => Some(token),
            _ => None,
        })
}

pub fn first_child<T>(node: &SyntaxNode) -> Option<T>
where
    T: rowan::ast::AstNode<Language = Danube>,
{
    node.children().find_map(T::cast)
}

pub fn children<T>(node: &SyntaxNode) -> impl Iterator<Item = T>
where
    T: rowan::ast::AstNode<Language = Danube>,
{
    node.children().filter_map(T::cast)
}

#[macro_export]
macro_rules! ast_node {
    (
        $(#[$meta:meta])*
        enum $node:ident;

        $(
            variant $variant:ident -> $ty:ident;
        )*
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone)]
        pub enum $node {
            $(
                $variant($ty),
            )*
        }

        impl rowan::ast::AstNode for $node {
            type Language = Danube;

            #[inline]
            fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool {
                $(crate::$ty::can_cast(kind))||+
            }

            #[inline]
            fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self> {
                $(
                    if <$ty>::can_cast(node.kind()) {
                        return Some($node::$variant(<$ty>::cast(node).unwrap()));
                    }
                )*

                None
            }

            #[inline]
            fn syntax(&self) -> &SyntaxNode {
                match self {
                    $(
                        $node::$variant(it) => it.syntax(),
                    )*
                }
            }
        }
    };
    (
        $(#[$meta:meta])*
        struct $node:ident where $kind:ident;

        $($rest:tt)*
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct $node(SyntaxNode);

        impl rowan::ast::AstNode for $node {
            type Language = Danube;

            #[inline]
            fn can_cast(kind: <Self::Language as rowan::Language>::Kind) -> bool {
                matches!(kind, SyntaxKind::$kind)
            }

            #[inline]
            fn cast(node: rowan::SyntaxNode<Self::Language>) -> Option<Self> {
                Self::can_cast(node.kind()).then(|| $node(node))
            }

            #[inline]
            fn syntax(&self) -> &SyntaxNode {
                &self.0
            }
        }

        ast_node!(impl $node { $($rest)* });
    };
    (impl $node:ident { $($body:tt)* }) => {
        impl std::fmt::Display for $node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
            }
        }

        #[allow(non_snake_case)]
        impl crate::$node {
            ast_node!($($body)*);
        }
    };
    (token $token:ident where $kind:ident; $($rest:tt)*) => {
        pub fn $token(&self) -> Option<SyntaxToken> {
            use rowan::ast::AstNode;

            first_token(self.syntax(), SyntaxKind::$kind)
        }

        ast_node!($($rest)*);
    };
    (tokens $token:ident where $kind:ident; $($rest:tt)*) => {
        pub fn $token(&self) -> impl Iterator<Item = SyntaxToken> {
            use rowan::ast::AstNode;

            tokens(self.syntax(), SyntaxKind::$kind)
        }

        ast_node!($($rest)*);
    };
    (node $node:ident -> $ty:ty; $($rest:tt)*) => {
        pub fn $node(&self) -> Option<$ty> {
            use rowan::ast::AstNode;

            first_child(self.syntax())
        }

        ast_node!($($rest)*);
    };
    (nodes $node:ident -> $ty:ty; $($rest:tt)*) => {
        pub fn $node(&self) -> impl Iterator<Item = $ty> {
            use rowan::ast::AstNode;

            children(self.syntax())
        }

        ast_node!($($rest)*);
    };
    () => {
        //
    };
}

ast_node! {
    /// A source file in a krate.
    struct Root where ROOT_NODE;

    nodes attributes -> TopLevelAttribute;
    nodes definitions -> Definition;
}

ast_node! {
    /// ```
    /// #![path(attribute)]
    /// ```
    struct TopLevelAttribute where TOP_LEVEL_ATTRIBUTE_NODE;

    token hash where HASH;
    token exclamation where EXCLAMATION;
    token left_bracket where LEFT_BRACKET;
    node argument -> AttributeArgument;
    token right_bracket where RIGHT_BRACKET;
}

ast_node! {
    /// ```
    /// #[path(attribute)]
    /// ```
    struct Attribute where ATTRIBUTE_NODE;

    token hash where HASH;
    token left_bracket where LEFT_BRACKET;
    node argument -> AttributeArgument;
    token right_bracket where RIGHT_BRACKET;
}

ast_node! {
    /// An argument in an attribute: `value`, `key = value`, `path(arguments)`
    enum AttributeArgument;

    variant Expression -> ExpressionAttributeArgument;
    variant KeyValue -> KeyValueAttributeArgument;
    variant Nested -> NestedAttributeArgument;
}

ast_node! {
    /// ```
    /// expression
    /// ```
    struct ExpressionAttributeArgument where EXPRESSION_ATTRIBUTE_ARGUMENT_NODE;

    node value -> Expression;
}

ast_node! {
    /// ```
    /// key = value
    /// ```
    struct KeyValueAttributeArgument where KEY_VALUE_ATTRIBUTE_ARGUMENT_NODE;

    node key -> Path;
    token equal where EQUAL;
    node value -> Expression;
}

ast_node! {
    /// ```
    /// path(arguments)
    /// ```
    struct NestedAttributeArgument where NESTED_ATTRIBUTE_ARGUMENT_NODE;

    node path -> Path;
    token left_paren where LEFT_PAREN;
    nodes arguments -> AttributeArgument;
    token right_paren where RIGHT_PAREN;
}

ast_node! {
    /// A top-level definition: function, struct, enum, use, module, trait, constant, static, type alias, impl block.
    struct Definition where DEFINITION_NODE;

    nodes attributes -> Attribute;
    node visibility -> Visibility;
    node kind -> DefinitionKind;
}

ast_node! {
    /// A top-level definition: function, struct, enum, use, module, trait, constant, static, type alias, impl block.
    enum DefinitionKind;

    variant Function -> FunctionDefinition;
    variant Struct -> StructDefinition;
    variant Enum -> EnumDefinition;
    variant Use -> UseDefinition;
    variant Module -> ModuleDefinition;
    variant Trait -> TraitDefinition;
    variant Constant -> ConstantDefinition;
    variant Static -> StaticDefinition;
    variant Type -> TypeDefinition;
    variant Implement -> ImplementDefinition;
}

ast_node! {
    /// ```
    /// fn $name<$type_parameters>($params) -> $return_type where $type_bounds { $body }
    /// ```
    struct FunctionDefinition where FUNCTION_DEFINITION_NODE;

    token r#fn where FN;
    node name -> Identifier;

    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    // token right_chevron_for_type_parameters where RIGHT_CHEVRON;

    token left_paren where LEFT_PAREN;
    nodes parameters -> FunctionParameter;
    token right_paren where RIGHT_PAREN;

    token HYPHEN where HYPHEN;
    // token right_chevron_for_return_type where RIGHT_CHEVRON;
    node return_type -> TypeExpression;

    token r#where where WHERE;
    nodes type_bounds -> TypeBound;

    node body -> FunctionBodyKind;
}

impl FunctionDefinition {
    pub fn right_chevron_for_type_parameters(&self) -> Option<SyntaxToken> {
        use rowan::ast::AstNode;

        self.syntax()
            .children_with_tokens()
            .take_while(|node| !matches!(node.kind(), SyntaxKind::HYPHEN))
            .find_map(|node| match node {
                rowan::NodeOrToken::Token(token) if token.kind() == SyntaxKind::RIGHT_CHEVRON => {
                    Some(token)
                }
                _ => None,
            })
    }

    pub fn right_chevron_for_return_type(&self) -> Option<SyntaxToken> {
        use rowan::ast::AstNode;

        self.syntax()
            .children_with_tokens()
            .skip_while(|node| !matches!(node.kind(), SyntaxKind::HYPHEN))
            .find_map(|node| match node {
                rowan::NodeOrToken::Token(token) if token.kind() == SyntaxKind::RIGHT_CHEVRON => {
                    Some(token)
                }
                _ => None,
            })
    }
}

ast_node! {
    /// ```
    /// struct $name<$type_parameters> where $type_bounds { $body }
    /// ```
    struct StructDefinition where STRUCT_DEFINITION_NODE;

    token r#struct where STRUCT;
    node name -> Identifier;

    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;

    token r#where where WHERE;
    nodes type_bounds -> TypeBound;

    node body -> StructBody;
}

ast_node! {
    /// ```
    /// enum $name<$type_parameters> where $type_bounds { $variants }
    /// ```
    struct EnumDefinition where ENUM_DEFINITION_NODE;

    token r#enum where ENUM;
    node name -> Identifier;

    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;

    token r#where where WHERE;
    nodes type_bounds -> TypeBound;

    token left_brace where LEFT_BRACE;
    nodes variants -> EnumVariant;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// ```
    /// use $tree;
    /// ```
    struct UseDefinition where USE_DEFINITION_NODE;

    token r#use where USE;
    node tree -> UseTree;
    token semicolon where SEMICOLON;
}

ast_node! {
    /// ```
    /// mod $name { $definitions }
    /// mod $name;
    /// ```
    struct ModuleDefinition where MODULE_DEFINITION_NODE;

    token r#mod where MOD;
    node name -> Identifier;

    node kind -> ModuleDefinitionKind;
}

ast_node! {
    /// ```
    /// trait $name<$type_parameters> where $type_bounds { $definitions }
    /// ```
    struct TraitDefinition where TRAIT_DEFINITION_NODE;

    token r#trait where TRAIT;
    node name -> Identifier;

    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;

    token r#where where WHERE;
    nodes type_bounds -> TypeBound;

    token left_brace where LEFT_BRACE;
    nodes definitions -> AssociatedDefinition;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// ```
    /// const $name: $type = $value;
    /// ```
    struct ConstantDefinition where CONSTANT_DEFINITION_NODE;

    token r#const where CONST;
    node name -> Identifier;

    token colon where COLON;
    node r#type -> TypeExpression;

    token equal where EQUAL;
    node initializer -> Expression;
    token semicolon where SEMICOLON;
}

ast_node! {
    /// ```
    /// static $name: $type = $value;
    /// ```
    struct StaticDefinition where STATIC_DEFINITION_NODE;

    token r#static where STATIC;
    node name -> Identifier;

    token colon where COLON;
    node r#type -> TypeExpression;

    token equal where EQUAL;
    node initializer -> Expression;
    token semicolon where SEMICOLON;
}

ast_node! {
    /// ```
    /// type $name<$type_parameters> where $type_bounds = $initializer;
    /// ```
    struct TypeDefinition where TYPE_DEFINITION_NODE;

    token r#type where TYPE;
    node name -> Identifier;

    token colon where COLON;
    // node bound -> TypeExpression;

    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;

    token r#where where WHERE;
    nodes type_bounds -> TypeBound;

    token equal where EQUAL;
    // node initializer -> TypeExpression;
    token semicolon where SEMICOLON;
}

impl TypeDefinition {
    pub fn bound(&self) -> Option<TypeExpression> {
        use rowan::ast::AstNode;

        self.syntax()
            .children()
            .take_while(|child| matches!(child.kind(), SyntaxKind::EQUAL))
            .find_map(TypeExpression::cast)
    }

    pub fn initializer(&self) -> Option<TypeExpression> {
        use rowan::ast::AstNode;

        self.syntax()
            .children()
            .skip_while(|child| !matches!(child.kind(), SyntaxKind::EQUAL))
            .find_map(TypeExpression::cast)
    }
}

ast_node! {
    /// ```
    /// impl<$type_parameters> $trait_type for $target_type where $type_bounds { $definitions }
    /// ```
    struct ImplementDefinition where IMPLEMENT_DEFINITION_NODE;

    token r#impl where IMPL;

    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;

    // node trait_type -> TypeExpression;

    token r#for where FOR;
    // node target_type -> TypeExpression;

    token r#where where WHERE;
    nodes type_bounds -> TypeBound;

    token left_brace where LEFT_BRACE;
    nodes definitions -> AssociatedDefinition;
    token right_brace where RIGHT_BRACE;
}

impl ImplementDefinition {
    pub fn trait_type(&self) -> Option<TypeExpression> {
        use rowan::ast::AstNode;

        self.syntax()
            .children()
            .take_while(|child| matches!(child.kind(), SyntaxKind::FOR))
            .find_map(TypeExpression::cast)
    }

    pub fn target_type(&self) -> Option<TypeExpression> {
        use rowan::ast::AstNode;

        self.syntax()
            .children()
            .skip_while(|child| !matches!(child.kind(), SyntaxKind::FOR))
            .find_map(TypeExpression::cast)
    }
}

ast_node! {
    // ```
    /// mod $name { $definitions }
    /// mod $name;
    /// ```
    enum ModuleDefinitionKind;

    variant Inline -> ModuleDefinitionInline;
    variant External -> ModuleDefinitionExternal;
}

ast_node! {
    /// An inline module definition: `mod name { definitions }`
    struct ModuleDefinitionInline where MODULE_DEFINITION_INLINE_NODE;

    token left_brace where LEFT_BRACE;
    nodes definitions -> Definition;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// An outline module definition: `mod name;`
    struct ModuleDefinitionExternal where MODULE_DEFINITION_EXTERNAL_NODE;

    token semicolon where SEMICOLON;
}

ast_node! {
    /// The body of a function: `{ statements }` or `;`
    enum FunctionBodyKind;

    variant Block -> FunctionBodyBlock;
    variant Unit -> FunctionBodyUnit;
}

ast_node! {
    /// A block function body: `{ statements }`
    struct FunctionBodyBlock where FUNCTION_BODY_BLOCK_NODE;

    node body -> BlockExpression;
}

ast_node! {
    /// A unit function body: `;`
    struct FunctionBodyUnit where FUNCTION_BODY_UNIT_NODE;

    token semicolon where SEMICOLON;
}

ast_node! {
    /// A tree in a use definition: `a::b::C`, `a::{b, c}`, `a::*`, etc.
    struct UseTree where USE_TREE_NODE;

    node root -> PathSegmentRoot;
    node kind -> UseTreeKind;
}

ast_node! {
    /// A tree in a use definition: `a::b::C`, `a::{b, c}`, `a::*`, etc.
    enum UseTreeKind;

    variant List -> UseTreeList;
    variant Glob -> UseTreeGlob;
    variant Element -> UseTreeElement;
}

ast_node! {
    /// A renamed import in a use tree: `{ *, a, b as c, ::*, ::a, ::b as c }`, `::{ *, a, b as c, ::*, ::a, ::b as c }`
    struct UseTreeList where USE_TREE_LIST_NODE;

    token left_brace where LEFT_BRACE;
    nodes trees -> UseTree;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// A glob import in a use tree: `*`, `::*`;
    struct UseTreeGlob where USE_TREE_GLOB_NODE;

    token asterisk where ASTERISK;
}

ast_node! {
    /// An element import in a use tree: `a`, `::a`
    struct UseTreeElement where USE_TREE_ELEMENT_NODE;

    node path -> Path;
    node trailing -> UseTreeTrailing;
}

ast_node! {
    enum UseTreeTrailing;

    variant Nested -> UseTreeTrailingNested;
    variant Glob -> UseTreeTrailingGlob;
    variant Rename -> UseTreeTrailingRename;
}

ast_node! {
    /// A renamed import in a use tree: `{ *, a, b as c, ::*, ::a, ::b as c }`, `::{ *, a, b as c, ::*, ::a, ::b as c }`
    struct UseTreeTrailingNested where USE_TREE_LIST_NODE;

    tokens colons where COLON;
    token left_brace where LEFT_BRACE;
    nodes trees -> UseTree;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// A glob import in a use tree: `*`, `::*`;
    struct UseTreeTrailingGlob where USE_TREE_GLOB_NODE;

    tokens colons where COLON;
    node root -> PathSegmentRoot;
    token asterisk where ASTERISK;
}

ast_node! {
    /// A path in a use tree: `a`, `a as b`, `::a`, `::a as b`.
    struct UseTreeTrailingRename where USE_TREE_RENAME_NODE;

    token r#as where AS;
    node identifier -> Identifier;
}

ast_node! {
    /// An associated definition within an impl block.
    struct AssociatedDefinition where ASSOCIATED_DEFINITION_NODE;

    nodes attributes -> Attribute;
    node visibility -> Visibility;
    node kind -> AssociatedDefinitionKind;
}

ast_node! {
    /// An associated definition within an impl block.
    enum AssociatedDefinitionKind;

    variant Function -> FunctionDefinition;
    variant Constant -> ConstantDefinition;
    variant Type -> TypeDefinition;
}

ast_node! {
    /// A parameter in a function definition: `name: Type`
    struct FunctionParameter where FUNCTION_PARAMETER_NODE;

    nodes attributes -> Attribute;
    node pattern -> Pattern;
    token colon where COLON;
    node r#type -> TypeExpression;
    token comma where COMMA;
}

ast_node! {
    /// The body of a struct: `Foo;`, `Foo { a: Type, b: Type }` or `Foo(Type, Type)`
    enum StructBody;

    variant Unit -> StructBodyUnit;
    variant Named -> StructNamed;
    variant Unnamed -> StructUnnamed;
}

ast_node! {
    /// A unit struct: `Foo;`
    struct StructBodyUnit where STRUCT_BODY_UNIT_NODE;

    token semicolon where SEMICOLON;
}

ast_node! {
    /// A struct with named fields: `Foo { a: Type, b: Type }`
    struct StructNamed where STRUCT_BODY_NAMED_NODE;

    token left_brace where LEFT_BRACE;
    nodes fields -> StructNamedField;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// A named field in a named struct: `a: Type`
    struct StructNamedField where STRUCT_BODY_NAMED_FIELD_NODE;

    nodes attributes -> Attribute;
    node visibility -> Visibility;
    node name -> Identifier;
    token colon where COLON;
    node r#type -> TypeExpression;
}

ast_node! {
    /// A struct with unnamed fields: `Foo(Type, Type)`
    struct StructUnnamed where STRUCT_BODY_UNNAMED_NODE;

    token left_paren where LEFT_PAREN;
    nodes fields -> StructUnnamedField;
    token right_paren where RIGHT_PAREN;
}

ast_node! {
    /// An unnamed field in an unnamed struct: `Type`
    struct StructUnnamedField where STRUCT_BODY_UNNAMED_FIELD_NODE;

    nodes attributes -> Attribute;
    node visibility -> Visibility;
    node r#type -> TypeExpression;
    token comma where COMMA;
}

ast_node! {
    /// A variant in an enum: `Variant`, `Variant { a: Type }` or `Variant(Type)`
    enum EnumVariant;

    variant Unit -> EnumVariantUnit;
    variant Scalar -> EnumVariantScalar;
    variant Named -> EnumVariantNamed;
    variant Unnamed -> EnumVariantUnnamed;
}

ast_node! {
    /// A unit variant in an enum: `Variant`
    struct EnumVariantUnit where ENUM_VARIANT_UNIT_NODE;

    nodes attributes -> Attribute;
    node name -> Identifier;
    token comma where COMMA;
}

ast_node! {
    /// A scalar variant in an enum: `Variant = 42`
    struct EnumVariantScalar where ENUM_VARIANT_SCALAR_NODE;

    nodes attributes -> Attribute;
    node name -> Identifier;
    token equal where EQUAL;
    node initializer -> Expression;
    token comma where COMMA;
}

ast_node! {
    /// A named variant in an enum: `Variant { a: Type }`
    struct EnumVariantNamed where ENUM_VARIANT_NAMED_NODE;

    nodes attributes -> Attribute;
    node name -> Identifier;
    token left_brace where LEFT_BRACE;
    nodes fields -> EnumVariantNamedField;
    token right_brace where RIGHT_BRACE;
    token comma where COMMA;
}

ast_node! {
    /// A named field in a named enum variant: `a: Type`
    struct EnumVariantNamedField where ENUM_VARIANT_NAMED_FIELD_NODE;

    nodes attributes -> Attribute;
    node name -> Identifier;
    token colon where COLON;
    node r#type -> TypeExpression;
    token comma where COMMA;
}

ast_node! {
    /// An unnamed variant in an enum: `Variant(Type, Type)`
    struct EnumVariantUnnamed where ENUM_VARIANT_UNNAMED_NODE;

    nodes attributes -> Attribute;
    node name -> Identifier;
    token left_paren where LEFT_PAREN;
    nodes fields -> EnumVariantUnnamedField;
    token right_paren where RIGHT_PAREN;
    token comma where COMMA;
}

ast_node! {
    /// An unnamed field in an unnamed enum variant: `Type`
    struct EnumVariantUnnamedField where ENUM_VARIANT_UNNAMED_FIELD_NODE;

    nodes attributes -> Attribute;
    node r#type -> TypeExpression;
    token comma where COMMA;
}

ast_node! {
    /// A pattern in a function definition: `name: Type`, `Foo { a, b }`, `(a, b, C)`, `42`, `_`, `!`, etc.
    enum Pattern;

    variant Never -> NeverPattern;
    variant Placeholder -> PlaceholderPattern;
    variant Path -> PathPattern;
    variant Mutable -> MutablePattern;
    variant Tuple -> TuplePattern;
    variant Array -> ArrayPattern;
    variant Literal -> LiteralPattern;
    variant Range -> RangePattern;
    variant At -> AtPattern;
    variant Or -> OrPattern;
    variant Named -> NamedPattern;
    variant Unnamed -> UnnamedPattern;
}

ast_node! {
    /// The never pattern: `!`
    struct NeverPattern where NEVER_PATTERN_NODE;

    token exclamation where EXCLAMATION;
}

ast_node! {
    /// The placeholder pattern: `_`
    struct PlaceholderPattern where PLACEHOLDER_PATTERN_NODE;

    token placeholder where PLACEHOLDER;
}

ast_node! {
    /// A path pattern: `a::b::C`, `A::<T>`, etc.
    struct PathPattern where PATH_PATTERN_NODE;

    node path -> Path;
}

ast_node! {
    /// A mutable pattern: `mut pattern`
    struct MutablePattern where MUTABLE_PATTERN_NODE;

    token r#mut where MUT;
    node pattern -> Pattern;
}

ast_node! {
    /// A tuple pattern: `(a, b, C)`
    struct TuplePattern where TUPLE_PATTERN_NODE;

    token left_paren where LEFT_PAREN;
    nodes elements -> Pattern;
    token right_paren where RIGHT_PAREN;
}

ast_node! {
    /// An array pattern: `[a, b, C]`
    struct ArrayPattern where ARRAY_PATTERN_NODE;

    token left_bracket where LEFT_BRACKET;
    nodes elements -> Pattern;
    token right_bracket where RIGHT_BRACKET;
}

ast_node! {
    /// A literal pattern: `42`, `"hello"`, `true`
    struct LiteralPattern where LITERAL_PATTERN_NODE;

    node literal -> LiteralExpression;
}

ast_node! {
    /// A range pattern: `start..end`, `start..=end`, `..end`, `start..`
    enum RangePattern;

    // `start..end`
    variant FromTo -> RangeFromToPattern;
    // `start..=end`
    variant FromToInclusive -> RangeFromToInclusivePattern;
    // `start..`
    variant From -> RangeFromPattern;
    // `..end`
    variant To -> RangeToPattern;
    // `..=end`
    variant ToInclusive -> RangeToInclusivePattern;
}

ast_node! {
    /// A range pattern: `start..end`
    struct RangeFromToPattern where RANGE_FROM_TO_PATTERN_NODE;

    node start -> Pattern;
    token operator where DOT__DOT;
    node end -> Pattern;
}

ast_node! {
    /// A range pattern: `start..=end`
    struct RangeFromToInclusivePattern where RANGE_FROM_TO_INCLUSIVE_PATTERN_NODE;

    node start -> Pattern;
    token operator where DOT__DOT__EQUAL;
    node end -> Pattern;
}

ast_node! {
    /// A range pattern: `start..`
    struct RangeFromPattern where RANGE_FROM_PATTERN_NODE;

    node start -> Pattern;
    token operator where DOT__DOT;
}

ast_node! {
    /// A range pattern: `..end`
    struct RangeToPattern where RANGE_TO_PATTERN_NODE;

    token operator where DOT__DOT;
    node end -> Pattern;
}

ast_node! {
    /// A range pattern: `..=end`
    struct RangeToInclusivePattern where RANGE_TO_INCLUSIVE_PATTERN_NODE;

    token operator where DOT__DOT__EQUAL;
    node end -> Pattern;
}

ast_node! {
    /// An at pattern: `name @ pattern`
    struct AtPattern where AT_PATTERN_NODE;

    // node name -> Pattern;
    token at where AT;
    // node pattern -> Pattern;
}

impl AtPattern {
    pub fn name(&self) -> Option<Pattern> {
        use rowan::ast::AstNode;

        self.syntax()
            .children()
            .take_while(|child| !matches!(child.kind(), SyntaxKind::AT))
            .find_map(Pattern::cast)
    }

    pub fn pattern(&self) -> Option<Pattern> {
        use rowan::ast::AstNode;

        self.syntax()
            .children()
            .skip_while(|child| !matches!(child.kind(), SyntaxKind::AT))
            .find_map(Pattern::cast)
    }
}

ast_node! {
    /// An or pattern: `a | b | C`
    struct OrPattern where OR_PATTERN_NODE;

    nodes patterns -> Pattern;
    tokens pipe where PIPE;
}

ast_node! {
    /// A named pattern: `Foo { a, b, c }`
    struct NamedPattern where NAMED_PATTERN_NODE;

    node path -> PathPattern;
    token left_brace where LEFT_BRACE;
    nodes fields -> NamedPatternField;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// An unnamed pattern: `Foo(a, b, c)`
    struct UnnamedPattern where UNNAMED_PATTERN_NODE;

    node path -> PathPattern;
    token left_paren where LEFT_PAREN;
    nodes elements -> Pattern;
    token right_paren where RIGHT_PAREN;
}

ast_node! {
    /// A field in a named pattern: `a: Pattern`
    struct NamedPatternField where NAMED_PATTERN_FIELD_NODE;

    node name -> Identifier;
    token colon where COLON;
    node pattern -> Pattern;
    token comma where COMMA;
}

ast_node! {
    /// A statement in a function definition: `let x: Type = 42;`, `expression;`, etc.
    enum Statement;

    variant Definition -> DefinitionStatement;
    variant Expression -> ExpressionStatement;
    variant Let -> LetStatement;
    variant Semicolon -> SemicolonStatement;
}

ast_node! {
    /// A definition statement: `fn foo() {}`, `struct Foo {}`, etc.
    struct DefinitionStatement where DEFINITION_STATEMENT_NODE;

    node definition -> Definition;
}

ast_node! {
    /// An expression statement: `expression;`
    struct ExpressionStatement where EXPRESSION_STATEMENT_NODE;

    node expression -> Expression;
    token semicolon where SEMICOLON;
}

ast_node! {
    /// A let statement: `let x: Type = 42;`
    struct LetStatement where LET_STATEMENT_NODE;

    token r#let where LET;
    token r#mut where MUT;
    node pattern -> Pattern;
    token colon where COLON;
    node r#type -> TypeExpression;
    token equal where EQUAL;
    node initializer -> Expression;
    token semicolon where SEMICOLON;
}

ast_node! {
    /// A semicolon statement: `;`
    struct SemicolonStatement where SEMICOLON_STATEMENT_NODE;

    token semicolon where SEMICOLON;
}

ast_node! {
    /// An expression in a function definition: `expression`
    enum Expression;

    variant Break -> BreakExpression;
    variant Continue -> ContinueExpression;
    variant Return -> ReturnExpression;
    variant For -> ForExpression;
    variant While -> WhileExpression;
    variant Loop -> LoopExpression;
    variant If -> IfExpression;
    variant Match -> MatchExpression;
    variant Let -> LetExpression;
    variant Array -> ArrayExpression;
    variant Tuple -> TupleExpression;
    variant Block -> BlockExpression;
    variant Literal -> LiteralExpression;
    variant Path -> PathExpression;
    variant Unary -> UnaryExpression;
    variant Binary -> BinaryExpression;
    variant Assignment -> AssignmentExpression;
    variant FunctionCall -> FunctionCallExpression;
    variant MethodCall -> MethodCallExpression;
    variant Field -> FieldExpression;
    variant Index -> IndexExpression;
    variant Await -> AwaitExpression;
    variant Range -> RangeExpression;
    variant Struct -> StructExpression;
    variant Try -> TryExpression;
    variant Yield -> YieldExpression;
}

ast_node! {
    /// A break expression: `break`
    struct BreakExpression where BREAK_EXPRESSION_NODE;

    token r#break where BREAK;
}

ast_node! {
    /// A continue expression: `continue`
    struct ContinueExpression where CONTINUE_EXPRESSION_NODE;

    token r#continue where CONTINUE;
}

ast_node! {
    /// A return expression: `return`, `return expression`
    struct ReturnExpression where RETURN_EXPRESSION_NODE;

    token r#return where RETURN;
    node expression -> Expression;
}

ast_node! {
    /// A for expression: `for pattern in iterable { body }`
    struct ForExpression where FOR_EXPRESSION_NODE;

    token r#for where FOR;
    node pattern -> Pattern;
    token r#in where IN;
    node iterable -> Expression;
    node body -> BlockExpression;
}

ast_node! {
    /// A while expression: `while condition { body }`
    struct WhileExpression where WHILE_EXPRESSION_NODE;

    token r#while where WHILE;
    node condition -> Expression;
    node body -> BlockExpression;
}

ast_node! {
    /// A loop expression: `loop { body }`
    struct LoopExpression where LOOP_EXPRESSION_NODE;

    token r#loop where LOOP;
    node body -> BlockExpression;
}

ast_node! {
    /// An if expression: `if condition { then_branch } else { else_branch }`
    struct IfExpression where IF_EXPRESSION_NODE;

    token r#if where IF;
    node condition -> Expression;
    node then_branch -> BlockExpression;
    token r#else where ELSE;
    node else_branch -> Expression;
}

ast_node! {
    /// A match expression: `match expression { arms }`
    struct MatchExpression where MATCH_EXPRESSION_NODE;

    token r#match where MATCH;
    node expression -> Expression;
    token left_brace where LEFT_BRACE;
    nodes arms -> MatchArm;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// An arm in a match expression: `pattern -> expression,`
    struct MatchArm where MATCH_ARM_NODE;

    node pattern -> Pattern;
    token hyphen where HYPHEN;
    token right_chevron where RIGHT_CHEVRON;
    node expression -> Expression;
    token comma where COMMA;
}

ast_node! {
    /// A let expression: `let pattern = expression`
    struct LetExpression where LET_EXPRESSION_NODE;

    token r#let where LET;
    token r#mut where MUT;
    node pattern -> Pattern;
    token colon where COLON;
    node r#type -> TypeExpression;
    token equal where EQUAL;
    node initializer -> Expression;
}

ast_node! {
    /// An array expression: `[element, element, ...]`
    struct ArrayExpression where ARRAY_EXPRESSION_NODE;

    token left_bracket where LEFT_BRACKET;
    nodes elements -> Expression;
    token right_bracket where RIGHT_BRACKET;
}

ast_node! {
    /// A tuple expression: `(element, element, ...)`
    struct TupleExpression where TUPLE_EXPRESSION_NODE;

    token left_paren where LEFT_PAREN;
    nodes elements -> Expression;
    token right_paren where RIGHT_PAREN;
}

ast_node! {
    /// A literal expression: `42`, `"hello"`, `true`
    struct LiteralExpression where LITERAL_EXPRESSION_NODE;

    node literal -> Literal;
}

ast_node! {
    /// A path expression: `path::to::item`
    struct PathExpression where PATH_EXPRESSION_NODE;

    node path -> Path;
}

ast_node! {
    /// A unary expression: `-expression`, `!expression`
    struct UnaryExpression where UNARY_EXPRESSION_NODE;

    node operator -> UnaryOperator;
    node operand -> Expression;
}

ast_node! {
    /// A binary expression: `left operator right`
    struct BinaryExpression where BINARY_EXPRESSION_NODE;

    node left -> Expression;
    node operator -> BinaryOperator;
    node right -> Expression;
}

ast_node! {
    /// An assignment expression: `left operator right`
    struct AssignmentExpression where ASSIGNMENT_EXPRESSION_NODE;

    node left -> Expression;
    node operator -> AssignmentOperator;
    node right -> Expression;
}

ast_node! {
    /// A function call expression: `function::<type_arguments>(arguments)`
    struct FunctionCallExpression where FUNCTION_CALL_EXPRESSION_NODE;

    node callee -> Expression;
    tokens colons where COLON;
    token left_chevron where LEFT_CHEVRON;
    nodes type_arguments -> TypeArgument;
    token right_chevron where RIGHT_CHEVRON;
    token left_paren where LEFT_PAREN;
    nodes arguments -> Expression;
    token right_paren where RIGHT_PAREN;
}

ast_node! {
    /// A method call expression: `receiver.method<type_arguments>(arguments)`
    struct MethodCallExpression where METHOD_CALL_EXPRESSION_NODE;

    node receiver -> Expression;
    token dot where DOT;
    node method -> Identifier;
    token left_chevron where LEFT_CHEVRON;
    nodes type_arguments -> TypeArgument;
    token right_chevron where RIGHT_CHEVRON;
    token left_paren where LEFT_PAREN;
    nodes arguments -> Expression;
    token right_paren where RIGHT_PAREN;
}

ast_node! {
    /// A field access expression: `receiver.field`
    struct FieldExpression where FIELD_EXPRESSION_NODE;

    node receiver -> Expression;
    token dot where DOT;
    node field -> Identifier;
}

ast_node! {
    /// An index access expression: `receiver[index]`
    struct IndexExpression where INDEX_EXPRESSION_NODE;

    node receiver -> Expression;
    token left_bracket where LEFT_BRACKET;
    node index -> Expression;
    token right_bracket where RIGHT_BRACKET;
}

ast_node! {
    /// An await expression: `expression.await`
    struct AwaitExpression where AWAIT_EXPRESSION_NODE;

    node expression -> Expression;
    token dot where DOT;
    token r#await where AWAIT;
}

ast_node! {
    /// A range expression: `start..end`, `start..=end`, `..end`, `start..`
    enum RangeExpression;

    // `start..end`
    variant FromTo -> RangeFromToExpression;
    // `start..`
    variant From -> RangeFromExpression;
    // `..end`
    variant To -> RangeToExpression;
    // `..`
    variant Full -> RangeFullExpression;
    // `start..=end`
    variant FromToInclusive -> RangeFromToInclusiveExpression;
    // `..=end`
    variant ToInclusive -> RangeToInclusiveExpression;
}

ast_node! {
    /// A range expression: `start..end`
    struct RangeFromToExpression where RANGE_FROM_TO_EXPRESSION_NODE;

    node start -> Expression;
    token operator where DOT__DOT;
    node end -> Expression;
}

ast_node! {
    /// A range expression: `start..`
    struct RangeFromExpression where RANGE_FROM_EXPRESSION_NODE;

    node start -> Expression;
    token operator where DOT__DOT;
}

ast_node! {
    /// A range expression: `..end`
    struct RangeToExpression where RANGE_TO_EXPRESSION_NODE;

    token operator where DOT__DOT;
    node end -> Expression;
}

ast_node! {
    /// A range expression: `..`
    struct RangeFullExpression where RANGE_FULL_EXPRESSION_NODE;

    token operator where DOT__DOT;
}

ast_node! {
    /// A range expression: `start..=end`
    struct RangeFromToInclusiveExpression where RANGE_FROM_TO_INCLUSIVE_EXPRESSION_NODE;

    node start -> Expression;
    token operator where DOT__DOT__EQUAL;
    node end -> Expression;
}

ast_node! {
    /// A range expression: `..=end`
    struct RangeToInclusiveExpression where RANGE_TO_INCLUSIVE_EXPRESSION_NODE;

    token operator where DOT__DOT__EQUAL;
    node end -> Expression;
}

ast_node! {
    /// A struct expression: `Struct { field: value, ... }` or `Struct(value, ...)`
    struct StructExpression where STRUCT_EXPRESSION_NODE;

    node path -> PathExpression;
    tokens colons where COLON;
    token left_chevron where LEFT_CHEVRON;
    nodes type_arguments -> TypeArgument;
    token right_chevron where RIGHT_CHEVRON;
    token left_brace where LEFT_BRACE;
    nodes fields -> StructExpressionField;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// A try expression: `expression?`
    struct TryExpression where TRY_EXPRESSION_NODE;

    node expression -> Expression;
    token question where QUESTION;
}

ast_node! {
    /// A yield expression: `expression.yield`
    struct YieldExpression where YIELD_EXPRESSION_NODE;

    node expression -> Expression;
    token dot where DOT;
    token r#yield where YIELD;
}

ast_node! {
    /// A block expression: `{ statements }`
    struct BlockExpression where BLOCK_EXPRESSION_NODE;

    nodes attributes -> Attribute;
    token left_brace where LEFT_BRACE;
    nodes statements -> Statement;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// A field in a struct expression: `field: expression`
    struct StructExpressionField where STRUCT_EXPRESSION_FIELD_NODE;

    node name -> Identifier;
    token colon where COLON;
    node value -> Expression;
    token comma where COMMA;
}

ast_node! {
    /// A unary operator: `-`, `!`, etc.
    struct UnaryOperator where UNARY_OPERATOR_NODE;

    token positive where PLUS;
    token negate where HYPHEN;
    token wrapping_negate where HYPHEN__PERCENT;
    token not where EXCLAMATION;
    token bitwise_not where TILDE;
}

ast_node! {
    /// A binary operator: `+`, `-`, `*`, `/`, `==`, etc.
    struct BinaryOperator where BINARY_OPERATOR_NODE;

    token add where PLUS;
    token saturating_add where PLUS__PIPE;
    token wrapping_add where PLUS__PERCENT;
    token subtract where HYPHEN;
    token saturating_subtract where HYPHEN__PIPE;
    token wrapping_subtract where HYPHEN__PERCENT;
    token multiply where ASTERISK;
    token saturating_multiply where ASTERISK__PIPE;
    token wrapping_multiply where ASTERISK__PERCENT;
    token divide where SLASH;
    token remainder where PERCENT;
    token exponent where ASTERISK__ASTERISK;
    token saturating_exponent where ASTERISK__ASTERISK__PIPE;
    token wrapping_exponent where ASTERISK__ASTERISK__PERCENT;
    token bitwise_xor where CARET;
    token bitwise_and where AMPERSAND;
    token bitwise_or where PIPE;
    token logical_and where AMPERSAND__AMPERSAND;
    token logical_or where PIPE__PIPE;
    token equal_equal where EQUAL__EQUAL;
    token not_equal where EXCLAMATION__EQUAL;
    token less_than where LEFT_CHEVRON;
    token less_than_equal where LEFT_CHEVRON__EQUAL;
    token greater_than where RIGHT_CHEVRON;
    token greater_than_equal where RIGHT_CHEVRON__EQUAL;
    token left_shift where LEFT_CHEVRON__LEFT_CHEVRON;
    token right_shift where RIGHT_CHEVRON__RIGHT_CHEVRON;
    token saturating_left_shift where LEFT_CHEVRON__LEFT_CHEVRON__PIPE;
    token right_shift_unsigned where RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON;
}

ast_node! {
    /// An assignment operator: `=`, `+=`, `-=`, etc.
    struct AssignmentOperator where ASSIGNMENT_OPERATOR_NODE;

    token equal where EQUAL;
    token add where PLUS__EQUAL;
    token saturating_add where PLUS__PIPE__EQUAL;
    token wrapping_add where PLUS__PERCENT__EQUAL;
    token subtract where HYPHEN__EQUAL;
    token saturating_subtract where HYPHEN__PIPE__EQUAL;
    token wrapping_subtract where HYPHEN__PERCENT__EQUAL;
    token multiply where ASTERISK__EQUAL;
    token saturating_multiply where ASTERISK__PIPE__EQUAL;
    token wrapping_multiply where ASTERISK__PERCENT__EQUAL;
    token divide where SLASH__EQUAL;
    token remainder where PERCENT__EQUAL;
    token exponent where ASTERISK__ASTERISK__EQUAL;
    token saturating_exponent where ASTERISK__ASTERISK__PIPE__EQUAL;
    token wrapping_exponent where ASTERISK__ASTERISK__PERCENT__EQUAL;
    token bitwise_xor where CARET__EQUAL;
    token bitwise_and where AMPERSAND__EQUAL;
    token bitwise_or where PIPE__EQUAL;
    token logical_and where AMPERSAND__AMPERSAND__EQUAL;
    token logical_or where PIPE__PIPE__EQUAL;
    token left_shift where LEFT_CHEVRON__LEFT_CHEVRON__EQUAL;
    token saturating_left_shift where LEFT_CHEVRON__LEFT_CHEVRON__PIPE__EQUAL;
    token right_shift where RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL;
    token right_shift_unsigned where RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL;
}

ast_node! {
    /// A range operator: `..`, `..=`, `...`
    struct RangeOperator where RANGE_OPERATOR_NODE;

    token dot__dot where DOT__DOT;
    token dot__dot__equal where DOT__DOT__EQUAL;
    token dot__dot__dot where DOT__DOT__EQUAL;
}

ast_node! {
    /// A type parameter in a generic type or function: `T`, `T: Trait`, etc.
    struct TypeParameter where TYPE_PARAMETER_NODE;

    node r#type -> TypeExpression;
    token colon where COLON;
    nodes constraints -> TypeParameterConstraint;
    token comma where COMMA;
}

ast_node! {
    /// A type bound in a where clause: `T: Trait`
    struct TypeBound where TYPE_BOUND_NODE;

    node r#type -> TypeExpression;
    token colon where COLON;
    nodes constraints -> TypeParameterConstraint;
    token comma where COMMA;
}

ast_node! {
    /// A constraint on a type parameter: `T: Trait`
    struct TypeParameterConstraint where TYPE_PARAMETER_CONSTRAINT_NODE;

    node r#type -> TypeExpression;
    token plus where PLUS;
}

ast_node! {
    /// A literal value: `42`, `"hello"`, `true`, etc.
    enum Literal;

    variant Boolean -> BooleanLiteral;
    variant Character -> CharacterLiteral;
    variant Integer -> IntegerLiteral;
    variant Float -> FloatLiteral;
    variant String -> StringLiteral;
    variant Binary -> BinaryLiteral;
    variant Octal -> OctalLiteral;
    variant Hex -> HexLiteral;
}

ast_node! {
    /// A boolean literal: `true`, `false`
    struct BooleanLiteral where BOOLEAN_LITERAL_NODE;

    token r#true where TRUE;
    token r#false where FALSE;
}

ast_node! {
    /// A character literal: `'a'`, `'\n'`, etc.
    struct CharacterLiteral where CHARACTER_LITERAL_NODE;

    token character_start where CHARACTER_START;
    node kind -> CharacterLiteralKind;
    token character_end where CHARACTER_END;
}

ast_node! {
    /// The kind of a character literal.
    enum CharacterLiteralKind;

    variant One -> CharacterLiteralOne;
    variant Escape -> CharacterLiteralEscape;
    variant Unicode -> CharacterLiteralUnicode;
}

ast_node! {
    /// A single character in a character literal: `a`
    struct CharacterLiteralOne where CHARACTER_LITERAL_ONE_NODE;

    token character where CHARACTER_SEGMENT;
}

ast_node! {
    /// An escape sequence in a character literal: `\n`, `\t`, `\\`, etc.
    struct CharacterLiteralEscape where CHARACTER_LITERAL_ESCAPE_NODE;

    token start where ESCAPE_START;
    token segment where ESCAPE_SEGMENT;
}

ast_node! {
    /// A Unicode escape sequence in a character literal: `\u{1F600}`
    struct CharacterLiteralUnicode where CHARACTER_LITERAL_UNICODE_NODE;

    token start where UNICODE_START;
    tokens segments where UNICODE_SEGMENT;
    token end where UNICODE_END;
}

ast_node! {
    /// A integer literal: `42`, `3.14`, etc.
    struct IntegerLiteral where INTEGER_LITERAL_NODE;

    tokens segments where INTEGER_SEGMENT;
}

ast_node! {
    /// A floating-point literal: `3.14`, `2.0`, etc.
    struct FloatLiteral where FLOAT_LITERAL_NODE;

    tokens integer_segments where INTEGER_SEGMENT;
    token fraction_start where FRACTION_START;
    tokens fraction_segments where FRACTION_SEGMENT;
    token exponent_start where EXPONENT_START;
    token exponent_sign where EXPONENT_SIGN;
    tokens exponent_segments where EXPONENT_SEGMENT;
}

ast_node! {
    /// A string literal: `"hello"`, `"world"`, etc.
    struct StringLiteral where STRING_LITERAL_NODE;

    token string_start where STRING_START;
    nodes segments -> StringSegment;
    token string_end where STRING_END;
}

ast_node! {
    /// A segment in a string literal: `hello`, `\n`, etc.
    enum StringSegment;

    variant Text -> StringLiteralText;
    variant Escape -> StringLiteralEscape;
    variant Unicode -> StringLiteralUnicode;
    variant Interpolation -> StringLiteralInterpolation;
}

ast_node! {
    /// A simple segment in a string literal: `hello`
    struct StringLiteralText where STRING_LITERAL_TEXT_NODE;

    token segment where STRING_SEGMENT;
}

ast_node! {
    /// An escape sequence in a string literal: `\n`, `\t`, `\\`, etc.
    struct StringLiteralEscape where STRING_LITERAL_ESCAPE_NODE;

    token start where ESCAPE_START;
    token segment where ESCAPE_SEGMENT;
}

ast_node! {
    /// A Unicode escape sequence in a string literal: `\u{1F600}`
    struct StringLiteralUnicode where STRING_LITERAL_UNICODE_NODE;

    token start where UNICODE_START;
    tokens segments where UNICODE_SEGMENT;
    token end where UNICODE_END;
}

ast_node! {
    /// An interpolation in a string literal: `${ expression }`
    struct StringLiteralInterpolation where STRING_LITERAL_INTERPOLATION_NODE;

    token start where INTERPOLATION_START;
    node expression -> Expression;
    token end where INTERPOLATION_END;
}

ast_node! {
    /// A binary literal: `0b101010`
    struct BinaryLiteral where BINARY_NUMERIC_LITERAL_NODE;

    token start where BINARY_START;
    tokens segments where BINARY_SEGMENT;
}

ast_node! {
    /// An octal literal: `0o52`
    struct OctalLiteral where OCTAL_NUMERIC_LITERAL_NODE;

    token start where OCTAL_START;
    tokens segments where OCTAL_SEGMENT;
}

ast_node! {
    /// A hexadecimal literal: `0x2A`
    struct HexLiteral where HEX_NUMERIC_LITERAL_NODE;

    token start where HEX_START;
    tokens segments where HEX_SEGMENT;
}

ast_node! {
    /// A type: `i32`, `Option<T>`, `[T]`, `(A, B)`, `!`, etc.
    enum TypeExpression;

    variant Never -> NeverType;
    variant Mutable -> MutableType;
    variant Path -> PathType;
    variant Slice -> SliceType;
    variant Tuple -> TupleType;
}

ast_node! {
    /// The never type: `!`
    struct NeverType where NEVER_TYPE_NODE;

    token exclamation where EXCLAMATION;
}

ast_node! {
    /// A mutable type: `mut T`
    struct MutableType where MUTABLE_TYPE_NODE;

    token r#mut where MUT;
    node r#type -> TypeExpression;
}

ast_node! {
    /// A path type: `a::b::C<T>`
    struct PathType where PATH_TYPE_NODE;

    node path -> Path;

    token left_chevron where LEFT_CHEVRON;
    nodes arguments -> TypeArgument;
    token right_chevron where RIGHT_CHEVRON;
}

ast_node! {
    /// A slice type: `[T]`
    struct SliceType where SLICE_TYPE_NODE;

    token left_bracket where LEFT_BRACKET;
    node r#type -> TypeExpression;
    token right_bracket where RIGHT_BRACKET;
}

ast_node! {
    /// A tuple type: `(A, B, C)`
    struct TupleType where TUPLE_TYPE_NODE;

    token left_bracket where LEFT_BRACKET;
    nodes arguments -> TypeArgument;
    token right_bracket where RIGHT_BRACKET;
}

ast_node! {
    /// A type argument in a generic type or function: `T`
    struct TypeArgument where TYPE_ARGUMENT_NODE;

    node r#type -> TypeExpression;
    token comma where COMMA;
}

ast_node! {
    struct Visibility where VISIBILITY_NODE;

    token r#pub where PUB;
    token left_paren where LEFT_PAREN;
    token krate where CRATE;
    token self_ where SELF;
    token super_ where SUPER;
    token right_paren where RIGHT_PAREN;
}

ast_node! {
    /// A path: `::foo`, `a::b::C`, `::std::option::Option`, etc.
    struct Path where PATH_NODE;

    nodes segments -> PathSegment;
}

ast_node! {
    /// A segment in a path: `::`, `foo`, `Bar`, etc.
    enum PathSegment;

    variant Root -> PathSegmentRoot;
    variant Self_ -> PathSegmentSelf;
    variant Super_ -> PathSegmentSuper;
    variant Krate -> PathSegmentKrate;
    variant Identifier -> PathSegmentIdentifier;
}

ast_node! {
    /// The root of a path: `::`.
    struct PathSegmentRoot where PATH_SEGMENT_ROOT_NODE;

    tokens colon where COLON;
}

ast_node! {
    /// The `self` segment in a path: `self`.
    struct PathSegmentSelf where PATH_SEGMENT_SELF_NODE;

    token self_ where SELF;
}

ast_node! {
    /// The `super` segment in a path: `super`.
    struct PathSegmentSuper where PATH_SEGMENT_SUPER_NODE;

    token super_ where SUPER;
}

ast_node! {
    /// The `crate` segment in a path: `crate`.
    struct PathSegmentKrate where PATH_SEGMENT_KRATE_NODE;

    token krate where CRATE;
}

ast_node! {
    /// An identifier segment in a path: `foo`, `Bar`, `_baz`, etc.
    struct PathSegmentIdentifier where PATH_SEGMENT_IDENTIFIER_NODE;

    node identifier -> Identifier;
}

ast_node! {
    /// An identifier: `foo`, `Bar`, `_baz`, etc.
    struct Identifier where IDENTIFIER_NODE;

    token raw_start where RAW_IDENTIFIER_START;
    node segment -> IdentifierSegment;
}

ast_node! {
    /// A segment in an identifier: `foo`, `Bar`, `_baz`, etc.
    struct IdentifierSegment where IDENTIFIER_SEGMENT;

    token identifier where IDENTIFIER;
}
