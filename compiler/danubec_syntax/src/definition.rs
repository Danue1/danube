ast_node! {
    /// The root of a krate.
    struct Krate where KRATE_NODE;

    node top_level_attribute -> TopLevelAttribute;
}

ast_node! {
    /// ```
    /// #![attribute]
    /// ```
    struct TopLevelAttribute where TOP_LEVEL_ATTRIBUTE_NODE;

    token hash where HASH;
    token exclamation where EXCLAMATION;
    token left_bracket where LEFT_BRACKET;
    token right_bracket where RIGHT_BRACKET;
}

ast_node! {
    /// ```
    /// #[attribute]
    /// ```
    struct Attribute where ATTRIBUTE_NODE;

    token hash where HASH;
    token left_bracket where LEFT_BRACKET;
    token right_bracket where RIGHT_BRACKET;
}

ast_node! {
    enum Definition;

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
    /// fn $name<$type_parameters>($params) -> $return_type where $constraints { $body }
    /// ```
    struct FunctionDefinition where FUNCTION_DEFINITION_NODE;

    token r#fn where FN;
    node name -> Identifier;
    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;
    token left_paren where LEFT_PAREN;
    nodes parameters -> FunctionParameter;
    token right_paren where RIGHT_PAREN;
    token arrow where HYPHEN__RIGHT_CHEVRON;
    node r#type -> Type;
    token r#where where WHERE;
    nodes constraints -> TypeParameterConstraint;
    node body -> BlockExpression;
}

ast_node! {
    /// ```
    /// struct $name<$type_parameters> where $constraints { $body }
    /// ```
    struct StructDefinition where STRUCT_DEFINITION_NODE;

    token r#struct where STRUCT;
    node name -> Identifier;
    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;
    token r#where where WHERE;
    nodes constraints -> TypeParameterConstraint;
    node body -> StructBody;
}

ast_node! {
    /// ```
    /// enum $name<$type_parameters> where $constraints { $variants }
    /// ```
    struct EnumDefinition where ENUM_DEFINITION_NODE;

    token r#enum where ENUM;
    node name -> Identifier;
    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;
    token r#where where WHERE;
    nodes constraints -> TypeParameterConstraint;
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
    token left_brace where LEFT_BRACE;
    nodes definitions -> Definition;
    token right_brace where RIGHT_BRACE;
    token semicolon where SEMICOLON;
}

ast_node! {
    /// ```
    /// trait $name<$type_parameters> where $constraints { $definitions }
    /// ```
    struct TraitDefinition where TRAIT_DEFINITION_NODE;

    token r#trait where TRAIT;
    node name -> Identifier;
    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;
    token r#where where WHERE;
    nodes constraints -> TypeParameterConstraint;
    token left_brace where LEFT_BRACE;
    nodes definitions -> Definition;
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
    node r#type -> Type;
    token equal where EQUAL;
    node expression -> Expression;
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
    node r#type -> Type;
    token equal where EQUAL;
    node expression -> Expression;
    token semicolon where SEMICOLON;
}

ast_node! {
    /// ```
    /// type $name<$type_parameters> where $constraints = $initializer;
    /// ```
    struct TypeDefinition where TYPE_DEFINITION_NODE;

    token r#type where TYPE;
    node name -> Identifier;
    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;
    token r#where where WHERE;
    nodes constraints -> TypeParameterConstraint;
    token equal where EQUAL;
    node initializer -> Type;
    token semicolon where SEMICOLON;
}

ast_node! {
    /// ```
    /// impl<$type_parameters> $trait_type for $target_type where $constraints { $definitions }
    /// ```
    struct ImplementDefinition where IMPLEMENT_DEFINITION_NODE;

    token r#impl where IMPL;
    token left_chevron where LEFT_CHEVRON;
    nodes type_parameters -> TypeParameter;
    token right_chevron where RIGHT_CHEVRON;
    node trait_type -> Type;
    token r#for where FOR;
    node target_type -> Type;
    token r#where where WHERE;
    nodes constraints -> TypeParameterConstraint;
    token left_brace where LEFT_BRACE;
    nodes definitions -> AssociatedDefinition;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// A tree in a use definition: `a::b::C`, `a::{b, c}`, `a::*`, etc.
    struct UseTree where USE_TREE_NODE;

    token colon__colon where COLON__COLON;
    node kind -> UseTreeKind;
    token comma where COMMA;
}

ast_node! {
    /// The kind of a use tree.
    enum UseTreeKind;

    variant Path -> UseTreePath;
    variant Glob -> UseTreeGlob;
    variant Nested -> UseTreeNested;
}

ast_node! {
    /// A path in a use tree: `a`, `a::b::C`, etc.
    struct UseTreePath where USE_TREE_PATH_NODE;

    node path -> Path;
    token r#as where AS;
    node rename -> Identifier;
}

ast_node! {
    /// A glob import in a use tree: `*`
    struct UseTreeGlob where USE_TREE_GLOB_NODE;

    token asterisk where ASTERISK;
}

ast_node! {
    /// A renamed import in a use tree: `{ a as b, c }`
    struct UseTreeNested where USE_TREE_NESTED_NODE;

    token left_brace where LEFT_BRACE;
    nodes trees -> UseTree;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// An associated definition within an impl block.
    enum AssociatedDefinition;

    variant Function -> FunctionDefinition;
    variant Constant -> ConstantDefinition;
    variant Type -> TypeDefinition;
}

ast_node! {
    /// A parameter in a function definition: `name: Type`
    struct FunctionParameter where FUNCTION_PARAMETER_NODE;

    node pattern -> Pattern;
    token colon where COLON;
    node r#type -> Type;
    token comma where COMMA;
}

ast_node! {
    /// The body of a struct: `Foo { a: Type, b: Type }` or `Foo(Type, Type)`
    enum StructBody;

    variant Named -> StructNamed;
    variant Unnamed -> StructUnnamed;
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

    node name -> Identifier;
    token colon where COLON;
    node r#type -> Type;
    token comma where COMMA;
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

    node r#type -> Type;
    token comma where COMMA;
}

ast_node! {
    /// A variant in an enum: `Variant { a: Type }` or `Variant(Type)`
    enum EnumVariant;

    variant Named -> EnumVariantNamed;
    variant Unnamed -> EnumVariantUnnamed;
}

ast_node! {
    /// A named variant in an enum: `Variant { a: Type }`
    struct EnumVariantNamed where ENUM_VARIANT_NAMED_NODE;

    node name -> Identifier;
    token left_brace where LEFT_BRACE;
    nodes fields -> EnumVariantNamedField;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// A named field in a named enum variant: `a: Type`
    struct EnumVariantNamedField where ENUM_VARIANT_NAMED_FIELD_NODE;

    node name -> Identifier;
    token colon where COLON;
    node r#type -> Type;
    token comma where COMMA;
}

ast_node! {
    /// An unnamed variant in an enum: `Variant(Type, Type)`
    struct EnumVariantUnnamed where ENUM_VARIANT_UNNAMED_NODE;

    node name -> Identifier;
    token left_paren where LEFT_PAREN;
    nodes fields -> EnumVariantUnnamedField;
    token right_paren where RIGHT_PAREN;
}

ast_node! {
    /// An unnamed field in an unnamed enum variant: `Type`
    struct EnumVariantUnnamedField where ENUM_VARIANT_UNNAMED_FIELD_NODE;

    node r#type -> Type;
    token comma where COMMA;
}

ast_node! {
    /// A pattern in a function definition: `name: Type`, `Foo { a, b }`, `(a, b, C)`, `42`, `_`, `!`, etc.
    enum Pattern;

    variant Never -> NeverPattern;
    variant Placeholder -> PlaceholderPattern;
    variant Path -> PathPattern;
    variant Tuple -> TuplePattern;
    variant Array -> ArrayPattern;
    variant Literal -> LiteralPattern;
    variant Rest -> RestPattern;
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

    token underscore where UNDERSCORE;
}

ast_node! {
    /// A path pattern: `a::b::C`, `A::<T>`, etc.
    struct PathPattern where PATH_PATTERN_NODE;

    node path -> Path;
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

    node literal -> Literal;
}

ast_node! {
    /// A rest pattern: `..`
    struct RestPattern where REST_PATTERN_NODE;

    token dot1 where DOT__DOT;
}

ast_node! {
    /// An or pattern: `a | b | C`
    struct OrPattern where OR_PATTERN_NODE;

    node left -> Pattern;
    token pipe where PIPE;
}

impl OrPattern {
    pub fn right(&self) -> Option<Pattern> {
        use rowan::ast::AstNode;

        self.syntax().children().filter_map(Pattern::cast).nth(1)
    }
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
    node r#type -> Type;
    token equal where EQUAL;
    node expression -> Expression;
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
    variant Block -> BlockExpression;
    variant Literal -> LiteralExpression;
    variant Path -> PathExpression;
    variant Unary -> UnaryExpression;
    variant Binary -> BinaryExpression;
    variant Assignment -> AssignmentExpression;
    variant FunctionCall -> CallExpression;
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
    token arrow where EQUAL__RIGHT_CHEVRON;
    node expression -> Expression;
    token comma where COMMA;
}

ast_node! {
    /// A let expression: `let pattern = expression`
    struct LetExpression where LET_EXPRESSION_NODE;

    token r#let where LET;
    token r#mut where MUT;
    node pattern -> Pattern;
    token equal where EQUAL;
    node expression -> Expression;
}

ast_node! {
    /// An array expression: `[element, element, ...]`
    struct ArrayExpression where ARRAY_EXPRESSION_NODE;

    token left_bracket where LEFT_BRACKET;
    nodes elements -> Expression;
    token right_bracket where RIGHT_BRACKET;
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
    /// A function call expression: `function(arguments)`
    struct CallExpression where FUNCTION_CALL_EXPRESSION_NODE;

    node callee -> Expression;
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
    struct RangeExpression where RANGE_EXPRESSION_NODE;

    node left -> Expression;
    node operator -> RangeOperator;
    node right -> Expression;
}

ast_node! {
    /// A struct expression: `Struct { field: value, ... }` or `Struct(value, ...)`
    struct StructExpression where STRUCT_EXPRESSION_NODE;

    node path -> PathExpression;
    token colon__colon where COLON__COLON;
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

    token left_brace where LEFT_BRACE;
    nodes statements -> Statement;
    token right_brace where RIGHT_BRACE;
}

ast_node! {
    /// A field in a struct expression: `field: expression`
    struct StructExpressionField where STRUCT_EXPRESSION_FIELD_NODE;

    node name -> Identifier;
    token colon where COLON;
    node expression -> Expression;
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
    /// A type parameter in a generic type or function: `T`
    struct TypeParameter where TYPE_PARAMETER_NODE;

    node r#type -> Type;
    token colon where COLON;
    nodes constraints -> TypeParameterConstraint;
    token comma where COMMA;
}

ast_node! {
    /// A constraint on a type parameter: `T: Trait`
    struct TypeParameterConstraint where TYPE_PARAMETER_CONSTRAINT_NODE;

    node r#type -> Type;
    token plus where PLUS;
}

ast_node! {
    /// A literal value: `42`, `"hello"`, `true`, etc.
    enum Literal;

    variant Array -> ArrayLiteral;
    variant Boolean -> BooleanLiteral;
    variant Character -> CharacterLiteral;
    variant Numeric -> NumericLiteral;
    variant String -> StringLiteral;
    variant Binary -> BinaryLiteral;
    variant Octal -> OctalLiteral;
    variant Decimal -> DecimalLiteral;
    variant Hexadecimal -> HexadecimalLiteral;
}

ast_node! {
    /// An array literal: `[1, 2, 3]`
    struct ArrayLiteral where ARRAY_LITERAL_NODE;

    token left_bracket where LEFT_BRACKET;
    nodes elements -> Literal;
    token right_bracket where RIGHT_BRACKET;
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

    node raw -> Raw;
}

ast_node! {
    /// A numeric literal: `42`, `3.14`, etc.
    struct NumericLiteral where NUMERIC_LITERAL_NODE;

    node raw -> Raw;
}

ast_node! {
    /// A string literal: `"hello"`, `"world"`, etc.
    struct StringLiteral where STRING_LITERAL_NODE;

    token left_double_quote where DOUBLE_QUOTE;
    node raw -> Raw;
}

impl StringLiteral {
    pub fn right_double_quote(&self) -> Option<crate::SyntaxToken> {
        use rowan::ast::AstNode;

        self.syntax()
            .children_with_tokens()
            .find_map(|child| match child {
                rowan::NodeOrToken::Token(token)
                    if token.kind() == crate::SyntaxKind::DOUBLE_QUOTE =>
                {
                    Some(token)
                }
                _ => None,
            })
    }
}

ast_node! {
    /// A binary literal: `0b101010`
    struct BinaryLiteral where BINARY_NUMERIC_LITERAL_NODE;

    node raw -> Raw;
}

ast_node! {
    /// An octal literal: `0o52`
    struct OctalLiteral where OCTAL_NUMERIC_LITERAL_NODE;

    node raw -> Raw;
}

ast_node! {
    /// A decimal literal: `0d42`
    struct DecimalLiteral where DECIMAL_NUMERIC_LITERAL_NODE;

    node raw -> Raw;
}

ast_node! {
    /// A hexadecimal literal: `0x2A`
    struct HexadecimalLiteral where HEX_NUMERIC_LITERAL_NODE;

    node raw -> Raw;
}

ast_node! {
    /// A type: `i32`, `Option<T>`, `[T]`, `(A, B)`, `!`, etc.
    enum Type;

    variant Never -> NeverType;
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
    /// A path type: `a::b::C<T>`
    struct PathType where PATH_TYPE_NODE;

    node path -> Path;
}

ast_node! {
    /// A slice type: `[T]`
    struct SliceType where SLICE_TYPE_NODE;

    token left_bracket where LEFT_BRACKET;
    node r#type -> Type;
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

    node r#type -> Type;
    token comma where COMMA;
}

ast_node! {
    struct Path where PATH_NODE;

    nodes fragments -> PathFragment;
}

ast_node! {
    // A fragment in a path: `a`, `b`, `C::<T>`, etc.
    struct PathFragment where PATH_FRAGMENT_NODE;

    token colon__colon where COLON__COLON;
    node segment -> PathSegment;
}

ast_node! {
    /// A segment in a path: `A::<T>`
    struct PathSegment where PATH_SEGMENT_NODE;

    node name -> Identifier;
    token colon__colon where COLON__COLON;
    token left_chevron where LEFT_CHEVRON;
    node type_arguments -> TypeArgument;
    token right_chevron where RIGHT_CHEVRON;
}

ast_node! {
    /// An identifier: `foo`, `Bar`, `_baz`, etc.
    struct Identifier where IDENTIFIER_NODE;

    node raw -> Raw;
}

ast_node! {
    /// Raw text content of a token.
    struct Raw where RAW_NODE;
}

impl Raw {
    pub fn text(&self) -> String {
        use rowan::ast::AstNode;

        self.syntax().text().to_string()
    }
}
