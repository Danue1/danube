#[allow(unused_variables)]
pub trait Visitor: Sized {
    fn visit_root(&mut self, root: crate::Root) {
        walk_root(self, root);
    }

    fn visit_definition(&mut self, definition: crate::Definition) {
        walk_definition(self, definition);
    }

    fn visit_definition_kind(&mut self, kind: crate::DefinitionKind) {
        walk_definition_kind(self, kind);
    }

    fn visit_function_definition(&mut self, function: crate::FunctionDefinition) {
        walk_function_definition(self, function);
    }

    fn visit_type_definition(&mut self, r#type: crate::TypeDefinition) {
        walk_type_definition(self, r#type);
    }

    fn visit_struct_definition(&mut self, struct_: crate::StructDefinition) {
        walk_struct_definition(self, struct_);
    }

    fn visit_enum_definition(&mut self, enum_: crate::EnumDefinition) {
        walk_enum_definition(self, enum_);
    }

    fn visit_trait_definition(&mut self, trait_: crate::TraitDefinition) {
        walk_trait_definition(self, trait_);
    }

    fn visit_impl_definition(&mut self, impl_: crate::ImplDefinition) {
        walk_impl_definition(self, impl_);
    }

    fn visit_const_definition(&mut self, const_: crate::ConstDefinition) {
        walk_const_definition(self, const_);
    }

    fn visit_static_definition(&mut self, static_: crate::StaticDefinition) {
        walk_static_definition(self, static_);
    }

    fn visit_use_definition(&mut self, use_: crate::UseDefinition) {
        walk_use_definition(self, use_);
    }

    fn visit_module_definition(&mut self, module: crate::ModuleDefinition) {
        walk_module_definition(self, module);
    }

    #[inline]
    fn visit_identifier(&mut self, identifier: crate::Identifier) {
        //
    }

    fn visit_type_parameter(&mut self, type_parameter: crate::TypeParameter) {
        walk_type_parameter(self, type_parameter);
    }

    fn visit_parameter(&mut self, parameter: crate::FunctionParameter) {
        walk_parameter(self, parameter);
    }

    fn visit_type(&mut self, r#type: crate::Type) {
        walk_type(self, r#type);
    }

    fn visit_block_expression(&mut self, block: crate::BlockExpression) {
        walk_block_expression(self, block);
    }

    fn visit_where_clause(&mut self, where_clause: crate::WhereClause) {
        walk_where_clause(self, where_clause);
    }

    fn visit_struct_body_kind(&mut self, body: crate::StructBodyKind) {
        walk_struct_body_kind(self, body);
    }

    fn visit_enum_variant(&mut self, variant: crate::EnumVariant) {
        walk_enum_variant(self, variant);
    }

    fn visit_impl_item_kind(&mut self, kind: crate::ImplItemKind) {
        walk_impl_item_kind(self, kind);
    }

    fn visit_expression(&mut self, expression: crate::Expression) {
        walk_expression(self, expression);
    }

    fn visit_use_tree_kind(&mut self, kind: crate::UseTreeKind) {
        walk_use_tree_kind(self, kind);
    }

    fn visit_type_kind(&mut self, kind: crate::TypeKind) {
        walk_type_kind(self, kind);
    }

    fn visit_statement(&mut self, statement: crate::Statement) {
        walk_statement(self, statement);
    }

    fn visit_type_constraint(&mut self, type_constraint: crate::TypeConstraint) {
        walk_type_constraint(self, type_constraint);
    }

    fn visit_struct_body_named(&mut self, named: crate::StructBodyNamed) {
        walk_struct_body_named(self, named);
    }

    fn visit_struct_body_unnamed(&mut self, unnamed: crate::StructBodyUnnamed) {
        walk_struct_body_unnamed(self, unnamed);
    }

    fn visit_enum_variant_kind(&mut self, kind: crate::EnumVariantKind) {
        walk_enum_variant_kind(self, kind);
    }

    fn visit_expression_kind(&mut self, kind: crate::ExpressionKind) {
        walk_expression_kind(self, kind);
    }

    #[inline]
    fn visit_use_tree_barrel(&mut self, barrel: crate::UseTreeBarrel) {
        //
    }

    fn visit_use_tree_ident(&mut self, ident: crate::UseTreeIdent) {
        walk_use_tree_ident(self, ident);
    }

    fn visit_use_tree_nested(&mut self, nested: crate::UseTreeNested) {
        walk_use_tree_nested(self, nested);
    }

    fn visit_path_type(&mut self, path: crate::PathType) {
        // TODO
    }

    fn visit_statement_kind(&mut self, kind: crate::StatementKind) {
        walk_statement_kind(self, kind);
    }

    fn visit_type_constraint_parameter(
        &mut self,
        type_constraint_parameter: crate::TypeConstraintParameter,
    ) {
        walk_type_constraint_parameter(self, type_constraint_parameter);
    }

    fn visit_struct_body_named_field(&mut self, field: crate::StructBodyNamedField) {
        walk_struct_body_named_field(self, field);
    }

    fn visit_struct_body_unnamed_field(&mut self, field: crate::StructBodyUnnamedField) {
        walk_struct_body_unnamed_field(self, field);
    }

    fn visit_enum_variant_named(&mut self, named: crate::EnumVariantNamed) {
        walk_enum_variant_named(self, named);
    }

    fn visit_enum_variant_unnamed(&mut self, unnamed: crate::EnumVariantUnnamed) {
        walk_enum_variant_unnamed(self, unnamed);
    }

    fn visit_enum_variant_sequence(&mut self, sequence: crate::EnumVariantSequence) {
        walk_enum_variant_sequence(self, sequence);
    }

    fn visit_assignment_expression(&mut self, assignment: crate::AssignmentExpression) {
        walk_assignment_expression(self, assignment);
    }

    fn visit_binary_expression(&mut self, binary: crate::BinaryExpression) {
        walk_binary_expression(self, binary);
    }

    fn visit_let_expression(&mut self, let_: crate::LetExpression) {
        walk_let_expression(self, let_);
    }

    fn visit_literal_expression(&mut self, literal: crate::LiteralExpression) {
        walk_literal_expression(self, literal);
    }

    fn visit_unary_expression(&mut self, unary: crate::UnaryExpression) {
        walk_unary_expression(self, unary);
    }

    fn visit_definition_statement(&mut self, definition: crate::DefinitionStatement) {
        walk_definition_statement(self, definition);
    }

    fn visit_expression_statement(&mut self, expression: crate::ExpressionStatement) {
        walk_expression_statement(self, expression);
    }

    fn visit_let_statement(&mut self, let_: crate::LetStatement) {
        walk_let_statement(self, let_);
    }

    #[inline]
    fn visit_semicolon_statement(&mut self, semicolon: crate::SemicolonStatement) {
        //
    }

    fn visit_visibility(&mut self, visibility: crate::Visibility) {
        walk_visibility(self, visibility);
    }

    fn visit_enum_variant_named_field(&mut self, field: crate::EnumVariantNamedField) {
        walk_enum_variant_named_field(self, field);
    }

    fn visit_enum_variant_unnamed_field(&mut self, field: crate::EnumVariantUnnamedField) {
        walk_enum_variant_unnamed_field(self, field);
    }

    #[inline]
    fn visit_assignment_operator(&mut self, operator: crate::AssignmentOperator) {
        //
    }

    #[inline]
    fn visit_binary_operator(&mut self, operator: crate::BinaryOperator) {
        //
    }

    fn visit_literal(&mut self, literal: crate::Literal) {
        walk_literal(self, literal);
    }

    #[inline]
    fn visit_unary_operator(&mut self, operator: crate::UnaryOperator) {
        //
    }

    fn visit_visibility_kind(&mut self, kind: crate::VisibilityKind) {
        walk_visibility_kind(self, kind);
    }

    fn visit_array_literal(&mut self, array: crate::ArrayLiteral) {
        walk_array_literal(self, array);
    }

    #[inline]
    fn visit_boolean_literal(&mut self, boolean: crate::BooleanLiteral) {
        //
    }

    fn visit_char_literal(&mut self, char: crate::CharLiteral) {
        walk_char_literal(self, char);
    }

    fn visit_numeric_literal(&mut self, numeric: crate::NumericLiteral) {
        walk_numeric_literal(self, numeric);
    }

    fn visit_string_literal(&mut self, string: crate::StringLiteral) {
        walk_string_literal(self, string);
    }

    fn visit_string_literal_fragment(&mut self, fragment: crate::StringLiteralFragment) {
        walk_string_literal_fragment(self, fragment);
    }

    #[inline]
    fn visit_visibility_crate(&mut self, crate_: crate::VisibilityCrate) {
        //
    }

    #[inline]
    fn visit_visibility_super(&mut self, super_: crate::VisibilitySuper) {
        //
    }

    fn visit_visibility_in(&mut self, in_: crate::VisibilityIn) {
        walk_visibility_in(self, in_);
    }

    #[inline]
    fn visit_raw(&mut self, raw: crate::Raw) {
        //
    }

    fn visit_numeric_literal_kind(&mut self, kind: crate::NumericLiteralKind) {
        walk_numeric_literal_kind(self, kind);
    }

    fn visit_escape(&mut self, escape: crate::Escape) {
        walk_escape(self, escape);
    }

    fn visit_interpolation(&mut self, interpolation: crate::Interpolation) {
        walk_interpolation(self, interpolation);
    }

    fn visit_decimal_numeric_literal(&mut self, decimal: crate::DecimalNumericLiteral) {
        walk_decimal_numeric_literal(self, decimal);
    }

    fn visit_binary_numeric_literal(&mut self, binary: crate::BinaryNumericLiteral) {
        walk_binary_numeric_literal(self, binary);
    }

    fn visit_octal_numeric_literal(&mut self, octal: crate::OctalNumericLiteral) {
        walk_octal_numeric_literal(self, octal);
    }

    fn visit_hex_numeric_literal(&mut self, hex: crate::HexNumericLiteral) {
        walk_hex_numeric_literal(self, hex);
    }

    fn visit_integer_part(&mut self, integer: crate::IntegerPart) {
        walk_integer_part(self, integer);
    }

    fn visit_fraction_part(&mut self, fraction: crate::FractionPart) {
        walk_fraction_part(self, fraction);
    }

    fn visit_exponent_part(&mut self, exponent: crate::ExponentPart) {
        walk_exponent_part(self, exponent);
    }

    #[inline]
    fn visit_numeric_fragment(&mut self, fragment: crate::NumericFragment) {
        //
    }
}

pub fn walk_root<V: Visitor>(visitor: &mut V, root: crate::Root) {
    for definition in root.definitions() {
        visitor.visit_definition(definition);
    }
}

pub fn walk_definition<V: Visitor>(visitor: &mut V, definition: crate::Definition) {
    if let Some(kind) = definition.kind() {
        visitor.visit_definition_kind(kind);
    }
}

pub fn walk_definition_kind<V: Visitor>(visitor: &mut V, kind: crate::DefinitionKind) {
    match kind {
        crate::DefinitionKind::Function(function) => visitor.visit_function_definition(function),
        crate::DefinitionKind::Type(r#type) => visitor.visit_type_definition(r#type),
        crate::DefinitionKind::Struct(struct_) => visitor.visit_struct_definition(struct_),
        crate::DefinitionKind::Enum(enum_) => visitor.visit_enum_definition(enum_),
        crate::DefinitionKind::Trait(trait_) => visitor.visit_trait_definition(trait_),
        crate::DefinitionKind::Impl(impl_) => visitor.visit_impl_definition(impl_),
        crate::DefinitionKind::Const(const_) => visitor.visit_const_definition(const_),
        crate::DefinitionKind::Static(static_) => visitor.visit_static_definition(static_),
        crate::DefinitionKind::Use(use_) => visitor.visit_use_definition(use_),
        crate::DefinitionKind::Module(module) => visitor.visit_module_definition(module),
    }
}

pub fn walk_function_definition<V: Visitor>(visitor: &mut V, function: crate::FunctionDefinition) {
    if let Some(name) = function.name() {
        visitor.visit_identifier(name);
    }
    for type_parameter in function.type_parameters() {
        visitor.visit_type_parameter(type_parameter);
    }
    for parameter in function.parameters() {
        visitor.visit_parameter(parameter);
    }
    if let Some(r#type) = function.return_type() {
        visitor.visit_type(r#type);
    }
    if let Some(body) = function.body() {
        visitor.visit_block_expression(body);
    }
}

pub fn walk_type_definition<V: Visitor>(visitor: &mut V, r#type: crate::TypeDefinition) {
    if let Some(name) = r#type.identifier() {
        visitor.visit_identifier(name);
    }
    for type_parameter in r#type.type_parameters() {
        visitor.visit_type_parameter(type_parameter);
    }
    if let Some(r#type) = r#type.ty() {
        visitor.visit_type(r#type);
    }
}

pub fn walk_struct_definition<V: Visitor>(visitor: &mut V, struct_: crate::StructDefinition) {
    if let Some(name) = struct_.identifier() {
        visitor.visit_identifier(name);
    }
    for type_parameter in struct_.type_parameters() {
        visitor.visit_type_parameter(type_parameter);
    }
    if let Some(where_clause) = struct_.where_clause() {
        visitor.visit_where_clause(where_clause);
    }
    if let Some(body) = struct_.body() {
        visitor.visit_struct_body_kind(body);
    }
}

pub fn walk_enum_definition<V: Visitor>(visitor: &mut V, enum_: crate::EnumDefinition) {
    if let Some(name) = enum_.identifier() {
        visitor.visit_identifier(name);
    }
    for type_parameter in enum_.type_parameters() {
        visitor.visit_type_parameter(type_parameter);
    }
    if let Some(where_clause) = enum_.where_clause() {
        visitor.visit_where_clause(where_clause);
    }
    for variant in enum_.variants() {
        visitor.visit_enum_variant(variant);
    }
}

pub fn walk_trait_definition<V: Visitor>(visitor: &mut V, trait_: crate::TraitDefinition) {
    if let Some(name) = trait_.identifier() {
        visitor.visit_identifier(name);
    }
    for type_parameter in trait_.type_parameters() {
        visitor.visit_type_parameter(type_parameter);
    }
    if let Some(where_clause) = trait_.where_clause() {
        visitor.visit_where_clause(where_clause);
    }
    for kind in trait_.items() {
        visitor.visit_impl_item_kind(kind);
    }
}

pub fn walk_impl_definition<V: Visitor>(visitor: &mut V, impl_: crate::ImplDefinition) {
    for type_parameter in impl_.type_parameters() {
        visitor.visit_type_parameter(type_parameter);
    }
    for r#type in impl_.types() {
        visitor.visit_type(r#type);
    }
    if let Some(where_clause) = impl_.where_clause() {
        visitor.visit_where_clause(where_clause);
    }
    for kind in impl_.items() {
        visitor.visit_impl_item_kind(kind);
    }
}

pub fn walk_const_definition<V: Visitor>(visitor: &mut V, const_: crate::ConstDefinition) {
    if let Some(name) = const_.identifier() {
        visitor.visit_identifier(name);
    }
    if let Some(r#type) = const_.ty() {
        visitor.visit_type(r#type);
    }
    if let Some(expression) = const_.expression() {
        visitor.visit_expression(expression);
    }
}

pub fn walk_static_definition<V: Visitor>(visitor: &mut V, static_: crate::StaticDefinition) {
    if let Some(name) = static_.identifier() {
        visitor.visit_identifier(name);
    }
    if let Some(r#type) = static_.ty() {
        visitor.visit_type(r#type);
    }
    if let Some(expression) = static_.expression() {
        visitor.visit_expression(expression);
    }
}

pub fn walk_use_definition<V: Visitor>(visitor: &mut V, use_: crate::UseDefinition) {
    if let Some(kind) = use_.kind() {
        visitor.visit_use_tree_kind(kind);
    }
}

pub fn walk_module_definition<V: Visitor>(visitor: &mut V, module: crate::ModuleDefinition) {
    if let Some(name) = module.identifier() {
        visitor.visit_identifier(name);
    }
    for definition in module.definitions() {
        visitor.visit_definition(definition);
    }
}

pub fn walk_type_parameter<V: Visitor>(visitor: &mut V, type_parameter: crate::TypeParameter) {
    if let Some(name) = type_parameter.identifier() {
        visitor.visit_identifier(name);
    }
    for r#type in type_parameter.types() {
        visitor.visit_type(r#type);
    }
}

pub fn walk_parameter<V: Visitor>(visitor: &mut V, parameter: crate::FunctionParameter) {
    if let Some(name) = parameter.identifier() {
        visitor.visit_identifier(name);
    }
    if let Some(r#type) = parameter.ty() {
        visitor.visit_type(r#type);
    }
}

pub fn walk_type<V: Visitor>(visitor: &mut V, r#type: crate::Type) {
    if let Some(kind) = r#type.kind() {
        visitor.visit_type_kind(kind);
    }
}

pub fn walk_block_expression<V: Visitor>(visitor: &mut V, block: crate::BlockExpression) {
    for statement in block.statements() {
        visitor.visit_statement(statement);
    }
}

pub fn walk_where_clause<V: Visitor>(visitor: &mut V, where_clause: crate::WhereClause) {
    for type_constraint in where_clause.type_constraints() {
        visitor.visit_type_constraint(type_constraint);
    }
}

pub fn walk_struct_body_kind<V: Visitor>(visitor: &mut V, body: crate::StructBodyKind) {
    match body {
        crate::StructBodyKind::Named(named) => visitor.visit_struct_body_named(named),
        crate::StructBodyKind::Unnamed(unnamed) => visitor.visit_struct_body_unnamed(unnamed),
    }
}

pub fn walk_enum_variant<V: Visitor>(visitor: &mut V, variant: crate::EnumVariant) {
    if let Some(name) = variant.identifier() {
        visitor.visit_identifier(name);
    }
    if let Some(kind) = variant.kind() {
        visitor.visit_enum_variant_kind(kind);
    }
}

pub fn walk_impl_item_kind<V: Visitor>(visitor: &mut V, kind: crate::ImplItemKind) {
    match kind {
        crate::ImplItemKind::Function(function) => visitor.visit_function_definition(function),
        crate::ImplItemKind::Type(r#type) => visitor.visit_type_definition(r#type),
        crate::ImplItemKind::Const(const_) => visitor.visit_const_definition(const_),
        crate::ImplItemKind::Static(static_) => visitor.visit_static_definition(static_),
    }
}

pub fn walk_expression<V: Visitor>(visitor: &mut V, expression: crate::Expression) {
    if let Some(kind) = expression.kind() {
        visitor.visit_expression_kind(kind);
    }
}

pub fn walk_use_tree_kind<V: Visitor>(visitor: &mut V, kind: crate::UseTreeKind) {
    match kind {
        crate::UseTreeKind::Barrel(barrel) => visitor.visit_use_tree_barrel(barrel),
        crate::UseTreeKind::Ident(ident) => visitor.visit_use_tree_ident(ident),
        crate::UseTreeKind::Nested(nested) => visitor.visit_use_tree_nested(nested),
    }
}

pub fn walk_type_kind<V: Visitor>(visitor: &mut V, kind: crate::TypeKind) {
    match kind {
        crate::TypeKind::Path(path) => visitor.visit_path_type(path),
    }
}

pub fn walk_statement<V: Visitor>(visitor: &mut V, statement: crate::Statement) {
    if let Some(kind) = statement.kind() {
        visitor.visit_statement_kind(kind);
    }
}

pub fn walk_type_constraint<V: Visitor>(visitor: &mut V, type_constraint: crate::TypeConstraint) {
    if let Some(r#type) = type_constraint.lhs() {
        visitor.visit_type(r#type);
    }
    if let Some(type_constraint_parameter) = type_constraint.rhs() {
        visitor.visit_type_constraint_parameter(type_constraint_parameter);
    }
}

pub fn walk_struct_body_named<V: Visitor>(visitor: &mut V, named: crate::StructBodyNamed) {
    for field in named.fields() {
        visitor.visit_struct_body_named_field(field);
    }
}

pub fn walk_struct_body_unnamed<V: Visitor>(visitor: &mut V, unnamed: crate::StructBodyUnnamed) {
    for field in unnamed.fields() {
        visitor.visit_struct_body_unnamed_field(field);
    }
}

pub fn walk_enum_variant_kind<V: Visitor>(visitor: &mut V, kind: crate::EnumVariantKind) {
    match kind {
        crate::EnumVariantKind::Named(named) => visitor.visit_enum_variant_named(named),
        crate::EnumVariantKind::Unnamed(unnamed) => visitor.visit_enum_variant_unnamed(unnamed),
        crate::EnumVariantKind::Sequence(sequence) => visitor.visit_enum_variant_sequence(sequence),
    }
}

pub fn walk_expression_kind<V: Visitor>(visitor: &mut V, kind: crate::ExpressionKind) {
    match kind {
        crate::ExpressionKind::Assignment(assignment) => {
            visitor.visit_assignment_expression(assignment)
        }
        crate::ExpressionKind::Binary(binary) => visitor.visit_binary_expression(binary),
        crate::ExpressionKind::Block(block) => visitor.visit_block_expression(block),
        crate::ExpressionKind::Let(let_) => visitor.visit_let_expression(let_),
        crate::ExpressionKind::Literal(literal) => visitor.visit_literal_expression(literal),
        crate::ExpressionKind::Unary(unary) => visitor.visit_unary_expression(unary),
    }
}

pub fn walk_use_tree_ident<V: Visitor>(visitor: &mut V, ident: crate::UseTreeIdent) {
    if let Some(lhs) = ident.lhs() {
        visitor.visit_identifier(lhs);
    }
    if let Some(rhs) = ident.rhs() {
        visitor.visit_identifier(rhs);
    }
}

pub fn walk_use_tree_nested<V: Visitor>(visitor: &mut V, nested: crate::UseTreeNested) {
    for kind in nested.kinds() {
        visitor.visit_use_tree_kind(kind);
    }
}

pub fn walk_statement_kind<V: Visitor>(visitor: &mut V, kind: crate::StatementKind) {
    match kind {
        crate::StatementKind::Definition(definition) => {
            visitor.visit_definition_statement(definition)
        }
        crate::StatementKind::Expression(expression) => {
            visitor.visit_expression_statement(expression)
        }
        crate::StatementKind::Let(let_) => visitor.visit_let_statement(let_),
        crate::StatementKind::Semicolon(semicolon) => visitor.visit_semicolon_statement(semicolon),
    }
}

pub fn walk_type_constraint_parameter<V: Visitor>(
    visitor: &mut V,
    type_constraint_parameter: crate::TypeConstraintParameter,
) {
    for r#type in type_constraint_parameter.types() {
        visitor.visit_type(r#type);
    }
}

pub fn walk_struct_body_named_field<V: Visitor>(
    visitor: &mut V,
    field: crate::StructBodyNamedField,
) {
    if let Some(visibility) = field.visibility() {
        visitor.visit_visibility(visibility);
    }
    if let Some(name) = field.identifier() {
        visitor.visit_identifier(name);
    }
    if let Some(r#type) = field.ty() {
        visitor.visit_type(r#type);
    }
}

pub fn walk_struct_body_unnamed_field<V: Visitor>(
    visitor: &mut V,
    field: crate::StructBodyUnnamedField,
) {
    if let Some(visibility) = field.visibility() {
        visitor.visit_visibility(visibility);
    }
    if let Some(r#type) = field.ty() {
        visitor.visit_type(r#type);
    }
}

pub fn walk_enum_variant_named<V: Visitor>(visitor: &mut V, named: crate::EnumVariantNamed) {
    for field in named.fields() {
        visitor.visit_enum_variant_named_field(field);
    }
}

pub fn walk_enum_variant_unnamed<V: Visitor>(visitor: &mut V, unnamed: crate::EnumVariantUnnamed) {
    for field in unnamed.fields() {
        visitor.visit_enum_variant_unnamed_field(field);
    }
}

pub fn walk_enum_variant_sequence<V: Visitor>(
    visitor: &mut V,
    sequence: crate::EnumVariantSequence,
) {
    if let Some(expression) = sequence.expression() {
        visitor.visit_expression(expression);
    }
}

pub fn walk_assignment_expression<V: Visitor>(
    visitor: &mut V,
    assignment: crate::AssignmentExpression,
) {
    if let Some(lhs) = assignment.lhs() {
        visitor.visit_expression(lhs);
    }
    if let Some(operator) = assignment.operator() {
        visitor.visit_assignment_operator(operator);
    }
    if let Some(rhs) = assignment.rhs() {
        visitor.visit_expression(rhs);
    }
}

pub fn walk_binary_expression<V: Visitor>(visitor: &mut V, binary: crate::BinaryExpression) {
    if let Some(lhs) = binary.lhs() {
        visitor.visit_expression(lhs);
    }
    if let Some(operator) = binary.operator() {
        visitor.visit_binary_operator(operator);
    }
    if let Some(rhs) = binary.rhs() {
        visitor.visit_expression(rhs);
    }
}

pub fn walk_let_expression<V: Visitor>(visitor: &mut V, let_: crate::LetExpression) {
    if let Some(name) = let_.lhs() {
        visitor.visit_identifier(name);
    }
    if let Some(expression) = let_.rhs() {
        visitor.visit_expression(expression);
    }
}

pub fn walk_literal_expression<V: Visitor>(visitor: &mut V, literal: crate::LiteralExpression) {
    if let Some(literal) = literal.literal() {
        visitor.visit_literal(literal);
    }
}

pub fn walk_unary_expression<V: Visitor>(visitor: &mut V, unary: crate::UnaryExpression) {
    if let Some(operator) = unary.operator() {
        visitor.visit_unary_operator(operator);
    }
    if let Some(expression) = unary.expression() {
        visitor.visit_expression(expression);
    }
}

pub fn walk_definition_statement<V: Visitor>(
    visitor: &mut V,
    definition: crate::DefinitionStatement,
) {
    if let Some(definition) = definition.definition() {
        visitor.visit_definition(definition);
    }
}

pub fn walk_expression_statement<V: Visitor>(
    visitor: &mut V,
    expression: crate::ExpressionStatement,
) {
    if let Some(expression) = expression.expression() {
        visitor.visit_expression(expression);
    }
}

pub fn walk_let_statement<V: Visitor>(visitor: &mut V, let_: crate::LetStatement) {
    if let Some(lhs) = let_.lhs() {
        visitor.visit_identifier(lhs);
    }
    if let Some(r#type) = let_.ty() {
        visitor.visit_type(r#type);
    }
    if let Some(rhs) = let_.rhs() {
        visitor.visit_expression(rhs);
    }
}

pub fn walk_visibility<V: Visitor>(visitor: &mut V, visibility: crate::Visibility) {
    if let Some(kind) = visibility.kind() {
        visitor.visit_visibility_kind(kind);
    }
}

pub fn walk_enum_variant_named_field<V: Visitor>(
    visitor: &mut V,
    field: crate::EnumVariantNamedField,
) {
    if let Some(name) = field.identifier() {
        visitor.visit_identifier(name);
    }
    if let Some(r#type) = field.ty() {
        visitor.visit_type(r#type);
    }
}

pub fn walk_enum_variant_unnamed_field<V: Visitor>(
    visitor: &mut V,
    field: crate::EnumVariantUnnamedField,
) {
    if let Some(r#type) = field.ty() {
        visitor.visit_type(r#type);
    }
}

pub fn walk_literal<V: Visitor>(visitor: &mut V, literal: crate::Literal) {
    match literal {
        crate::Literal::Array(array) => visitor.visit_array_literal(array),
        crate::Literal::Boolean(boolean) => visitor.visit_boolean_literal(boolean),
        crate::Literal::Char(char) => visitor.visit_char_literal(char),
        crate::Literal::Numeric(numeric) => visitor.visit_numeric_literal(numeric),
        crate::Literal::String(string) => visitor.visit_string_literal(string),
    }
}

pub fn walk_visibility_kind<V: Visitor>(visitor: &mut V, kind: crate::VisibilityKind) {
    match kind {
        crate::VisibilityKind::Crate(crate_) => visitor.visit_visibility_crate(crate_),
        crate::VisibilityKind::Super(super_) => visitor.visit_visibility_super(super_),
        crate::VisibilityKind::In(in_) => visitor.visit_visibility_in(in_),
    }
}

pub fn walk_array_literal<V: Visitor>(visitor: &mut V, array: crate::ArrayLiteral) {
    for expression in array.elements() {
        visitor.visit_expression(expression);
    }
}

pub fn walk_char_literal<V: Visitor>(visitor: &mut V, char: crate::CharLiteral) {
    if let Some(raw) = char.raw() {
        visitor.visit_raw(raw);
    }
}

pub fn walk_numeric_literal<V: Visitor>(visitor: &mut V, numeric: crate::NumericLiteral) {
    if let Some(kind) = numeric.kind() {
        visitor.visit_numeric_literal_kind(kind);
    }
}

pub fn walk_string_literal<V: Visitor>(visitor: &mut V, string: crate::StringLiteral) {
    for fragment in string.fragments() {
        visitor.visit_string_literal_fragment(fragment);
    }
}

pub fn walk_string_literal_fragment<V: Visitor>(
    visitor: &mut V,
    fragment: crate::StringLiteralFragment,
) {
    match fragment {
        crate::StringLiteralFragment::Raw(raw) => visitor.visit_raw(raw),
        crate::StringLiteralFragment::Escape(escape) => visitor.visit_escape(escape),
        crate::StringLiteralFragment::Interpolation(interpolation) => {
            visitor.visit_interpolation(interpolation)
        }
    }
}

pub fn walk_visibility_in<V: Visitor>(visitor: &mut V, in_: crate::VisibilityIn) {
    if let Some(identifier) = in_.identifier() {
        visitor.visit_identifier(identifier);
    }
}

pub fn walk_numeric_literal_kind<V: Visitor>(visitor: &mut V, kind: crate::NumericLiteralKind) {
    match kind {
        crate::NumericLiteralKind::Decimal(decimal) => {
            visitor.visit_decimal_numeric_literal(decimal)
        }
        crate::NumericLiteralKind::Binary(binary) => visitor.visit_binary_numeric_literal(binary),
        crate::NumericLiteralKind::Octal(octal) => visitor.visit_octal_numeric_literal(octal),
        crate::NumericLiteralKind::Hex(hex) => visitor.visit_hex_numeric_literal(hex),
    }
}

pub fn walk_escape<V: Visitor>(visitor: &mut V, escape: crate::Escape) {
    if let Some(raw) = escape.raw() {
        visitor.visit_raw(raw);
    }
}

pub fn walk_interpolation<V: Visitor>(visitor: &mut V, interpolation: crate::Interpolation) {
    if let Some(expression) = interpolation.expression() {
        visitor.visit_expression(expression);
    }
}

pub fn walk_decimal_numeric_literal<V: Visitor>(
    visitor: &mut V,
    decimal: crate::DecimalNumericLiteral,
) {
    if let Some(integer) = decimal.integer() {
        visitor.visit_integer_part(integer);
    }
    if let Some(fraction) = decimal.fraction() {
        visitor.visit_fraction_part(fraction);
    }
    if let Some(exponent) = decimal.exponent() {
        visitor.visit_exponent_part(exponent);
    }
}

pub fn walk_binary_numeric_literal<V: Visitor>(
    visitor: &mut V,
    binary: crate::BinaryNumericLiteral,
) {
    if let Some(fragment) = binary.fragment() {
        visitor.visit_numeric_fragment(fragment);
    }
}

pub fn walk_octal_numeric_literal<V: Visitor>(visitor: &mut V, octal: crate::OctalNumericLiteral) {
    if let Some(fragment) = octal.fragment() {
        visitor.visit_numeric_fragment(fragment);
    }
}

pub fn walk_hex_numeric_literal<V: Visitor>(visitor: &mut V, hex: crate::HexNumericLiteral) {
    if let Some(fragment) = hex.fragment() {
        visitor.visit_numeric_fragment(fragment);
    }
}

pub fn walk_integer_part<V: Visitor>(visitor: &mut V, integer: crate::IntegerPart) {
    if let Some(fragment) = integer.fragment() {
        visitor.visit_numeric_fragment(fragment);
    }
}

pub fn walk_fraction_part<V: Visitor>(visitor: &mut V, fraction: crate::FractionPart) {
    if let Some(fragment) = fraction.fragment() {
        visitor.visit_numeric_fragment(fragment);
    }
}

pub fn walk_exponent_part<V: Visitor>(visitor: &mut V, exponent: crate::ExponentPart) {
    if let Some(fragment) = exponent.fragment() {
        visitor.visit_numeric_fragment(fragment);
    }
}
