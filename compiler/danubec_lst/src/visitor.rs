#[allow(unused_variables)]
pub trait Visitor: Sized {
    fn visit_root(&mut self, node: crate::Root) {
        walk_root(self, node);
    }

    fn visit_definition(&mut self, node: crate::Definition) {
        walk_definition(self, node);
    }

    fn visit_definition_kind(&mut self, node: crate::DefinitionKind) {
        walk_definition_kind(self, node);
    }

    fn visit_function_definition(&mut self, node: crate::FunctionDefinition) {
        walk_function_definition(self, node);
    }

    fn visit_type_definition(&mut self, r#node: crate::TypeDefinition) {
        walk_type_definition(self, node);
    }

    fn visit_struct_definition(&mut self, node: crate::StructDefinition) {
        walk_struct_definition(self, node);
    }

    fn visit_enum_definition(&mut self, node: crate::EnumDefinition) {
        walk_enum_definition(self, node);
    }

    fn visit_trait_definition(&mut self, node: crate::TraitDefinition) {
        walk_trait_definition(self, node);
    }

    fn visit_impl_definition(&mut self, node: crate::ImplDefinition) {
        walk_impl_definition(self, node);
    }

    fn visit_const_definition(&mut self, node: crate::ConstDefinition) {
        walk_const_definition(self, node);
    }

    fn visit_static_definition(&mut self, node: crate::StaticDefinition) {
        walk_static_definition(self, node);
    }

    fn visit_use_definition(&mut self, node: crate::UseDefinition) {
        walk_use_definition(self, node);
    }

    fn visit_module_definition(&mut self, node: crate::ModuleDefinition) {
        walk_module_definition(self, node);
    }

    #[inline]
    fn visit_identifier(&mut self, node: crate::Identifier) {
        //
    }

    fn visit_type_parameter(&mut self, node: crate::TypeParameter) {
        walk_type_parameter(self, node);
    }

    fn visit_parameter(&mut self, node: crate::FunctionParameter) {
        walk_parameter(self, node);
    }

    fn visit_type(&mut self, r#node: crate::Type) {
        walk_type(self, node);
    }

    fn visit_block_expression(&mut self, node: crate::BlockExpression) {
        walk_block_expression(self, node);
    }

    fn visit_where_clause(&mut self, node: crate::WhereClause) {
        walk_where_clause(self, node);
    }

    fn visit_struct_body_kind(&mut self, node: crate::StructBodyKind) {
        walk_struct_body_kind(self, node);
    }

    fn visit_enum_variant(&mut self, node: crate::EnumVariant) {
        walk_enum_variant(self, node);
    }

    fn visit_associated_definition(&mut self, node: crate::AssociatedItem) {
        walk_associated_definition(self, node);
    }

    fn visit_associated_item_kind(&mut self, node: crate::AssociatedItemKind) {
        walk_associated_item_kind(self, node);
    }

    fn visit_expression(&mut self, node: crate::Expression) {
        walk_expression(self, node);
    }

    fn visit_use_tree(&mut self, node: crate::UseTree) {
        walk_use_tree(self, node);
    }

    fn visit_use_tree_kind(&mut self, node: crate::UseTreeKind) {
        walk_use_tree_kind(self, node);
    }

    fn visit_type_kind(&mut self, node: crate::TypeKind) {
        walk_type_kind(self, node);
    }

    fn visit_slice_type(&mut self, node: crate::SliceType) {
        walk_slice_type(self, node);
    }

    fn visit_tuple_type(&mut self, node: crate::TupleType) {
        walk_tuple_type(self, node);
    }

    fn visit_path_type(&mut self, node: crate::PathType) {
        walk_path_type(self, node);
    }

    #[inline]
    fn visit_never_type(&mut self, node: crate::NeverType) {
        //
    }

    fn visit_tuple_type_element(&mut self, node: crate::TupleTypeElement) {
        walk_tuple_type_element(self, node);
    }

    fn visit_statement(&mut self, node: crate::Statement) {
        walk_statement(self, node);
    }

    fn visit_type_constraint(&mut self, node: crate::TypeConstraint) {
        walk_type_constraint(self, node);
    }

    fn visit_struct_body_named(&mut self, node: crate::StructBodyNamed) {
        walk_struct_body_named(self, node);
    }

    fn visit_struct_body_unnamed(&mut self, node: crate::StructBodyUnnamed) {
        walk_struct_body_unnamed(self, node);
    }

    fn visit_enum_variant_kind(&mut self, node: crate::EnumVariantKind) {
        walk_enum_variant_kind(self, node);
    }

    fn visit_expression_kind(&mut self, node: crate::ExpressionKind) {
        walk_expression_kind(self, node);
    }

    #[inline]
    fn visit_use_tree_barrel(&mut self, node: crate::UseTreeBarrel) {
        //
    }

    fn visit_use_tree_ident(&mut self, node: crate::UseTreeIdent) {
        walk_use_tree_ident(self, node);
    }

    fn visit_use_tree_nested(&mut self, node: crate::UseTreeNested) {
        walk_use_tree_nested(self, node);
    }

    fn visit_statement_kind(&mut self, node: crate::StatementKind) {
        walk_statement_kind(self, node);
    }

    fn visit_type_constraint_parameter(&mut self, node: crate::TypeConstraintParameter) {
        walk_type_constraint_parameter(self, node);
    }

    fn visit_struct_body_named_field(&mut self, node: crate::StructBodyNamedField) {
        walk_struct_body_named_field(self, node);
    }

    fn visit_struct_body_unnamed_field(&mut self, node: crate::StructBodyUnnamedField) {
        walk_struct_body_unnamed_field(self, node);
    }

    fn visit_enum_variant_named(&mut self, node: crate::EnumVariantNamed) {
        walk_enum_variant_named(self, node);
    }

    fn visit_enum_variant_unnamed(&mut self, node: crate::EnumVariantUnnamed) {
        walk_enum_variant_unnamed(self, node);
    }

    fn visit_enum_variant_sequence(&mut self, node: crate::EnumVariantSequence) {
        walk_enum_variant_sequence(self, node);
    }

    fn visit_assignment_expression(&mut self, node: crate::AssignmentExpression) {
        walk_assignment_expression(self, node);
    }

    fn visit_binary_expression(&mut self, node: crate::BinaryExpression) {
        walk_binary_expression(self, node);
    }

    fn visit_let_expression(&mut self, node: crate::LetExpression) {
        walk_let_expression(self, node);
    }

    fn visit_literal_expression(&mut self, node: crate::LiteralExpression) {
        walk_literal_expression(self, node);
    }

    fn visit_unary_expression(&mut self, node: crate::UnaryExpression) {
        walk_unary_expression(self, node);
    }

    fn visit_definition_statement(&mut self, node: crate::DefinitionStatement) {
        walk_definition_statement(self, node);
    }

    fn visit_expression_statement(&mut self, node: crate::ExpressionStatement) {
        walk_expression_statement(self, node);
    }

    fn visit_let_statement(&mut self, node: crate::LetStatement) {
        walk_let_statement(self, node);
    }

    #[inline]
    fn visit_semicolon_statement(&mut self, node: crate::SemicolonStatement) {
        //
    }

    fn visit_visibility(&mut self, node: crate::Visibility) {
        walk_visibility(self, node);
    }

    fn visit_enum_variant_named_field(&mut self, node: crate::EnumVariantNamedField) {
        walk_enum_variant_named_field(self, node);
    }

    fn visit_enum_variant_unnamed_field(&mut self, node: crate::EnumVariantUnnamedField) {
        walk_enum_variant_unnamed_field(self, node);
    }

    #[inline]
    fn visit_assignment_operator(&mut self, node: crate::AssignmentOperator) {
        //
    }

    #[inline]
    fn visit_binary_operator(&mut self, node: crate::BinaryOperator) {
        //
    }

    fn visit_literal(&mut self, node: crate::Literal) {
        walk_literal(self, node);
    }

    #[inline]
    fn visit_unary_operator(&mut self, node: crate::UnaryOperator) {
        //
    }

    fn visit_visibility_kind(&mut self, node: crate::VisibilityKind) {
        walk_visibility_kind(self, node);
    }

    fn visit_array_expression(&mut self, node: crate::ArrayExpression) {
        walk_array_expression(self, node);
    }

    fn visit_array_expression_element(&mut self, node: crate::ArrayExpressionElement) {
        walk_array_expression_element(self, node);
    }

    #[inline]
    fn visit_boolean_literal(&mut self, node: crate::BooleanLiteral) {
        //
    }

    fn visit_char_literal(&mut self, node: crate::CharLiteral) {
        walk_char_literal(self, node);
    }

    fn visit_numeric_literal(&mut self, node: crate::NumericLiteral) {
        walk_numeric_literal(self, node);
    }

    fn visit_string_literal(&mut self, node: crate::StringLiteral) {
        walk_string_literal(self, node);
    }

    fn visit_string_literal_fragment(&mut self, node: crate::StringLiteralFragment) {
        walk_string_literal_fragment(self, node);
    }

    #[inline]
    fn visit_visibility_crate(&mut self, node: crate::VisibilityCrate) {
        //
    }

    #[inline]
    fn visit_visibility_super(&mut self, node: crate::VisibilitySuper) {
        //
    }

    fn visit_visibility_in(&mut self, node: crate::VisibilityIn) {
        walk_visibility_in(self, node);
    }

    #[inline]
    fn visit_raw(&mut self, node: crate::Raw) {
        //
    }

    fn visit_numeric_literal_kind(&mut self, node: crate::NumericLiteralKind) {
        walk_numeric_literal_kind(self, node);
    }

    fn visit_escape(&mut self, node: crate::Escape) {
        walk_escape(self, node);
    }

    fn visit_interpolation(&mut self, node: crate::Interpolation) {
        walk_interpolation(self, node);
    }

    fn visit_decimal_numeric_literal(&mut self, node: crate::DecimalNumericLiteral) {
        walk_decimal_numeric_literal(self, node);
    }

    fn visit_binary_numeric_literal(&mut self, node: crate::BinaryNumericLiteral) {
        walk_binary_numeric_literal(self, node);
    }

    fn visit_octal_numeric_literal(&mut self, node: crate::OctalNumericLiteral) {
        walk_octal_numeric_literal(self, node);
    }

    fn visit_hex_numeric_literal(&mut self, node: crate::HexNumericLiteral) {
        walk_hex_numeric_literal(self, node);
    }

    fn visit_integer_part(&mut self, node: crate::IntegerPart) {
        walk_integer_part(self, node);
    }

    fn visit_fraction_part(&mut self, node: crate::FractionPart) {
        walk_fraction_part(self, node);
    }

    fn visit_exponent_part(&mut self, node: crate::ExponentPart) {
        walk_exponent_part(self, node);
    }

    #[inline]
    fn visit_numeric_fragment(&mut self, node: crate::NumericFragment) {
        //
    }

    fn visit_path(&mut self, node: crate::Path) {
        walk_path(self, node);
    }

    fn visit_path_segment(&mut self, node: crate::PathSegment) {
        walk_path_segment(self, node);
    }

    fn visit_type_argument(&mut self, node: crate::TypeArgument) {
        walk_type_argument(self, node);
    }

    fn visit_target_type(&mut self, node: crate::TargetType) {
        walk_target_type(self, node);
    }

    fn visit_pattern(&mut self, node: crate::Pattern) {
        walk_pattern(self, node);
    }

    fn visit_pattern_kind(&mut self, node: crate::PatternKind) {
        walk_pattern_kind(self, node);
    }

    #[inline]
    fn visit_never_pattern(&mut self, node: crate::NeverPattern) {
        //
    }

    #[inline]
    fn visit_placeholder_pattern(&mut self, node: crate::PlaceholderPattern) {
        //
    }

    fn visit_path_pattern(&mut self, node: crate::PathPattern) {
        walk_path_pattern(self, node);
    }

    fn visit_tuple_pattern(&mut self, node: crate::TuplePattern) {
        walk_tuple_pattern(self, node);
    }

    fn visit_tuple_pattern_element(&mut self, node: crate::TuplePatternElement) {
        walk_tuple_pattern_element(self, node);
    }

    fn visit_array_pattern(&mut self, node: crate::ArrayPattern) {
        walk_array_pattern(self, node);
    }

    fn visit_array_pattern_element(&mut self, node: crate::ArrayPatternElement) {
        walk_array_pattern_element(self, node);
    }

    fn visit_literal_pattern(&mut self, node: crate::LiteralPattern) {
        walk_literal_pattern(self, node);
    }

    #[inline]
    fn visit_rest_pattern(&mut self, node: crate::RestPattern) {
        //
    }

    fn visit_or_pattern(&mut self, node: crate::OrPattern) {
        walk_or_pattern(self, node);
    }

    fn visit_named_pattern(&mut self, node: crate::NamedPattern) {
        walk_named_pattern(self, node);
    }

    fn visit_named_pattern_element(&mut self, node: crate::NamedPatternElement) {
        walk_named_pattern_element(self, node);
    }

    fn visit_unnamed_pattern(&mut self, node: crate::UnnamedPattern) {
        walk_unnamed_pattern(self, node);
    }

    fn visit_unnamed_pattern_element(&mut self, node: crate::UnnamedPatternElement) {
        walk_unnamed_pattern_element(self, node);
    }
}

macro_rules! visit_optional {
    ($visitor:ident.$method:ident($node:expr)) => {
        if let Some(node) = $node {
            $visitor.$method(node);
        }
    };
}

macro_rules! visit_each {
    ($visitor:ident.$method:ident($nodes:expr)) => {
        for node in $nodes {
            $visitor.$method(node);
        }
    };
}

pub fn walk_root<V: Visitor>(visitor: &mut V, node: crate::Root) {
    visit_each!(visitor.visit_definition(node.definitions()));
}

pub fn walk_definition<V: Visitor>(visitor: &mut V, node: crate::Definition) {
    visit_optional!(visitor.visit_visibility(node.visibility()));
    visit_optional!(visitor.visit_definition_kind(node.kind()));
}

pub fn walk_definition_kind<V: Visitor>(visitor: &mut V, node: crate::DefinitionKind) {
    match node {
        crate::DefinitionKind::Function(node) => visitor.visit_function_definition(node),
        crate::DefinitionKind::Type(node) => visitor.visit_type_definition(node),
        crate::DefinitionKind::Struct(node) => visitor.visit_struct_definition(node),
        crate::DefinitionKind::Enum(node) => visitor.visit_enum_definition(node),
        crate::DefinitionKind::Trait(node) => visitor.visit_trait_definition(node),
        crate::DefinitionKind::Impl(node) => visitor.visit_impl_definition(node),
        crate::DefinitionKind::Const(node) => visitor.visit_const_definition(node),
        crate::DefinitionKind::Static(node) => visitor.visit_static_definition(node),
        crate::DefinitionKind::Use(node) => visitor.visit_use_definition(node),
        crate::DefinitionKind::Module(node) => visitor.visit_module_definition(node),
    }
}

pub fn walk_function_definition<V: Visitor>(visitor: &mut V, node: crate::FunctionDefinition) {
    visit_optional!(visitor.visit_identifier(node.name()));
    visit_each!(visitor.visit_type_parameter(node.type_parameters()));
    visit_each!(visitor.visit_parameter(node.parameters()));
    visit_optional!(visitor.visit_type(node.return_type()));
    visit_optional!(visitor.visit_where_clause(node.where_clause()));
    visit_optional!(visitor.visit_block_expression(node.body()));
}

pub fn walk_type_definition<V: Visitor>(visitor: &mut V, node: crate::TypeDefinition) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_each!(visitor.visit_type_parameter(node.type_parameters()));
    visit_optional!(visitor.visit_type(node.ty()));
}

pub fn walk_struct_definition<V: Visitor>(visitor: &mut V, node: crate::StructDefinition) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_each!(visitor.visit_type_parameter(node.type_parameters()));
    visit_optional!(visitor.visit_where_clause(node.where_clause()));
    visit_optional!(visitor.visit_struct_body_kind(node.kind()));
}

pub fn walk_enum_definition<V: Visitor>(visitor: &mut V, node: crate::EnumDefinition) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_each!(visitor.visit_type_parameter(node.type_parameters()));
    visit_optional!(visitor.visit_where_clause(node.where_clause()));
    visit_each!(visitor.visit_enum_variant(node.variants()));
}

pub fn walk_trait_definition<V: Visitor>(visitor: &mut V, node: crate::TraitDefinition) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_each!(visitor.visit_type_parameter(node.type_parameters()));
    visit_optional!(visitor.visit_where_clause(node.where_clause()));
    visit_each!(visitor.visit_associated_definition(node.items()));
}

pub fn walk_impl_definition<V: Visitor>(visitor: &mut V, node: crate::ImplDefinition) {
    visit_each!(visitor.visit_type_parameter(node.type_parameters()));
    visit_optional!(visitor.visit_type(node.ty()));
    visit_optional!(visitor.visit_target_type(node.target_type()));
    visit_optional!(visitor.visit_where_clause(node.where_clause()));
    visit_each!(visitor.visit_associated_definition(node.items()));
}

pub fn walk_const_definition<V: Visitor>(visitor: &mut V, node: crate::ConstDefinition) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_optional!(visitor.visit_type(node.ty()));
    visit_optional!(visitor.visit_expression(node.expression()));
}

pub fn walk_static_definition<V: Visitor>(visitor: &mut V, node: crate::StaticDefinition) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_optional!(visitor.visit_type(node.ty()));
    visit_optional!(visitor.visit_expression(node.expression()));
}

pub fn walk_use_definition<V: Visitor>(visitor: &mut V, node: crate::UseDefinition) {
    visit_optional!(visitor.visit_use_tree(node.tree()));
}

pub fn walk_module_definition<V: Visitor>(visitor: &mut V, node: crate::ModuleDefinition) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_each!(visitor.visit_definition(node.definitions()));
}

pub fn walk_type_parameter<V: Visitor>(visitor: &mut V, node: crate::TypeParameter) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_each!(visitor.visit_type(node.types()));
}

pub fn walk_parameter<V: Visitor>(visitor: &mut V, node: crate::FunctionParameter) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_optional!(visitor.visit_type(node.ty()));
}

pub fn walk_type<V: Visitor>(visitor: &mut V, node: crate::Type) {
    visit_optional!(visitor.visit_type_kind(node.kind()));
}

pub fn walk_block_expression<V: Visitor>(visitor: &mut V, node: crate::BlockExpression) {
    visit_each!(visitor.visit_statement(node.statements()));
}

pub fn walk_where_clause<V: Visitor>(visitor: &mut V, node: crate::WhereClause) {
    visit_each!(visitor.visit_type_constraint(node.type_constraints()));
}

pub fn walk_struct_body_kind<V: Visitor>(visitor: &mut V, node: crate::StructBodyKind) {
    match node {
        crate::StructBodyKind::Named(node) => visitor.visit_struct_body_named(node),
        crate::StructBodyKind::Unnamed(node) => visitor.visit_struct_body_unnamed(node),
    }
}

pub fn walk_enum_variant<V: Visitor>(visitor: &mut V, node: crate::EnumVariant) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_optional!(visitor.visit_enum_variant_kind(node.kind()));
}

pub fn walk_associated_definition<V: Visitor>(visitor: &mut V, node: crate::AssociatedItem) {
    visit_optional!(visitor.visit_visibility(node.visibility()));
    visit_optional!(visitor.visit_associated_item_kind(node.kind()));
}

pub fn walk_associated_item_kind<V: Visitor>(visitor: &mut V, node: crate::AssociatedItemKind) {
    match node {
        crate::AssociatedItemKind::Function(node) => visitor.visit_function_definition(node),
        crate::AssociatedItemKind::Type(node) => visitor.visit_type_definition(node),
        crate::AssociatedItemKind::Const(node) => visitor.visit_const_definition(node),
    }
}

pub fn walk_expression<V: Visitor>(visitor: &mut V, node: crate::Expression) {
    visit_optional!(visitor.visit_expression_kind(node.kind()));
}

pub fn walk_use_tree_kind<V: Visitor>(visitor: &mut V, node: crate::UseTreeKind) {
    match node {
        crate::UseTreeKind::Barrel(node) => visitor.visit_use_tree_barrel(node),
        crate::UseTreeKind::Ident(node) => visitor.visit_use_tree_ident(node),
        crate::UseTreeKind::Nested(node) => visitor.visit_use_tree_nested(node),
    }
}

pub fn walk_type_kind<V: Visitor>(visitor: &mut V, node: crate::TypeKind) {
    match node {
        crate::TypeKind::Never(node) => visitor.visit_never_type(node),
        crate::TypeKind::Path(node) => visitor.visit_path_type(node),
        crate::TypeKind::Slice(node) => visitor.visit_slice_type(node),
        crate::TypeKind::Tuple(node) => visitor.visit_tuple_type(node),
    }
}

pub fn walk_slice_type<V: Visitor>(visitor: &mut V, node: crate::SliceType) {
    visit_optional!(visitor.visit_type(node.ty()));
}

pub fn walk_tuple_type<V: Visitor>(visitor: &mut V, node: crate::TupleType) {
    visit_each!(visitor.visit_tuple_type_element(node.elements()));
}

pub fn walk_path_type<V: Visitor>(visitor: &mut V, node: crate::PathType) {
    visit_optional!(visitor.visit_path(node.path()));
}

pub fn walk_tuple_type_element<V: Visitor>(visitor: &mut V, node: crate::TupleTypeElement) {
    visit_optional!(visitor.visit_type(node.ty()));
}

pub fn walk_statement<V: Visitor>(visitor: &mut V, node: crate::Statement) {
    visit_optional!(visitor.visit_statement_kind(node.kind()));
}

pub fn walk_type_constraint<V: Visitor>(visitor: &mut V, node: crate::TypeConstraint) {
    visit_optional!(visitor.visit_type(node.lhs()));
    visit_optional!(visitor.visit_type_constraint_parameter(node.rhs()));
}

pub fn walk_struct_body_named<V: Visitor>(visitor: &mut V, node: crate::StructBodyNamed) {
    visit_each!(visitor.visit_struct_body_named_field(node.fields()));
}

pub fn walk_struct_body_unnamed<V: Visitor>(visitor: &mut V, node: crate::StructBodyUnnamed) {
    visit_each!(visitor.visit_struct_body_unnamed_field(node.fields()));
}

pub fn walk_enum_variant_kind<V: Visitor>(visitor: &mut V, node: crate::EnumVariantKind) {
    match node {
        crate::EnumVariantKind::Named(node) => visitor.visit_enum_variant_named(node),
        crate::EnumVariantKind::Unnamed(node) => visitor.visit_enum_variant_unnamed(node),
        crate::EnumVariantKind::Sequence(node) => visitor.visit_enum_variant_sequence(node),
    }
}

pub fn walk_expression_kind<V: Visitor>(visitor: &mut V, node: crate::ExpressionKind) {
    match node {
        crate::ExpressionKind::Array(node) => visitor.visit_array_expression(node),
        crate::ExpressionKind::Assignment(assignment) => {
            visitor.visit_assignment_expression(assignment)
        }
        crate::ExpressionKind::Binary(node) => visitor.visit_binary_expression(node),
        crate::ExpressionKind::Block(node) => visitor.visit_block_expression(node),
        crate::ExpressionKind::Let(node) => visitor.visit_let_expression(node),
        crate::ExpressionKind::Literal(node) => visitor.visit_literal_expression(node),
        crate::ExpressionKind::Unary(node) => visitor.visit_unary_expression(node),
        _ => std::todo!(),
    }
}

pub fn walk_use_tree<V: Visitor>(visitor: &mut V, node: crate::UseTree) {
    visit_optional!(visitor.visit_path(node.path()));
    visit_optional!(visitor.visit_use_tree_kind(node.kind()));
}

pub fn walk_use_tree_ident<V: Visitor>(visitor: &mut V, node: crate::UseTreeIdent) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
}

pub fn walk_use_tree_nested<V: Visitor>(visitor: &mut V, node: crate::UseTreeNested) {
    visit_each!(visitor.visit_use_tree(node.trees()));
}

pub fn walk_statement_kind<V: Visitor>(visitor: &mut V, node: crate::StatementKind) {
    match node {
        crate::StatementKind::Definition(node) => visitor.visit_definition_statement(node),
        crate::StatementKind::Expression(node) => visitor.visit_expression_statement(node),
        crate::StatementKind::Let(node) => visitor.visit_let_statement(node),
        crate::StatementKind::Semicolon(node) => visitor.visit_semicolon_statement(node),
    }
}

pub fn walk_type_constraint_parameter<V: Visitor>(
    visitor: &mut V,
    node: crate::TypeConstraintParameter,
) {
    visit_each!(visitor.visit_type(node.types()));
}

pub fn walk_struct_body_named_field<V: Visitor>(
    visitor: &mut V,
    node: crate::StructBodyNamedField,
) {
    visit_optional!(visitor.visit_visibility(node.visibility()));
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_optional!(visitor.visit_type(node.ty()));
}

pub fn walk_struct_body_unnamed_field<V: Visitor>(
    visitor: &mut V,
    node: crate::StructBodyUnnamedField,
) {
    visit_optional!(visitor.visit_visibility(node.visibility()));
    visit_optional!(visitor.visit_type(node.ty()));
}

pub fn walk_enum_variant_named<V: Visitor>(visitor: &mut V, node: crate::EnumVariantNamed) {
    visit_each!(visitor.visit_enum_variant_named_field(node.fields()));
}

pub fn walk_enum_variant_unnamed<V: Visitor>(visitor: &mut V, node: crate::EnumVariantUnnamed) {
    visit_each!(visitor.visit_enum_variant_unnamed_field(node.fields()));
}

pub fn walk_enum_variant_sequence<V: Visitor>(visitor: &mut V, node: crate::EnumVariantSequence) {
    visit_optional!(visitor.visit_expression(node.expression()));
}

pub fn walk_assignment_expression<V: Visitor>(visitor: &mut V, node: crate::AssignmentExpression) {
    visit_optional!(visitor.visit_expression(node.lhs()));
    visit_optional!(visitor.visit_assignment_operator(node.operator()));
    visit_optional!(visitor.visit_expression(node.rhs()));
}

pub fn walk_binary_expression<V: Visitor>(visitor: &mut V, node: crate::BinaryExpression) {
    visit_optional!(visitor.visit_expression(node.lhs()));
    visit_optional!(visitor.visit_binary_operator(node.operator()));
    visit_optional!(visitor.visit_expression(node.rhs()));
}

pub fn walk_let_expression<V: Visitor>(visitor: &mut V, node: crate::LetExpression) {
    visit_optional!(visitor.visit_pattern(node.pattern()));
    visit_optional!(visitor.visit_expression(node.expression()));
}

pub fn walk_literal_expression<V: Visitor>(visitor: &mut V, node: crate::LiteralExpression) {
    visit_optional!(visitor.visit_literal(node.literal()));
}

pub fn walk_unary_expression<V: Visitor>(visitor: &mut V, node: crate::UnaryExpression) {
    visit_optional!(visitor.visit_unary_operator(node.operator()));
    visit_optional!(visitor.visit_expression(node.expression()));
}

pub fn walk_definition_statement<V: Visitor>(visitor: &mut V, node: crate::DefinitionStatement) {
    visit_optional!(visitor.visit_definition(node.definition()));
}

pub fn walk_expression_statement<V: Visitor>(visitor: &mut V, node: crate::ExpressionStatement) {
    visit_optional!(visitor.visit_expression(node.expression()));
}

pub fn walk_let_statement<V: Visitor>(visitor: &mut V, node: crate::LetStatement) {
    visit_optional!(visitor.visit_pattern(node.pattern()));
    visit_optional!(visitor.visit_type(node.ty()));
    visit_optional!(visitor.visit_expression(node.expression()));
}

pub fn walk_visibility<V: Visitor>(visitor: &mut V, node: crate::Visibility) {
    visit_optional!(visitor.visit_visibility_kind(node.kind()));
}

pub fn walk_enum_variant_named_field<V: Visitor>(
    visitor: &mut V,
    node: crate::EnumVariantNamedField,
) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_optional!(visitor.visit_type(node.ty()));
}

pub fn walk_enum_variant_unnamed_field<V: Visitor>(
    visitor: &mut V,
    node: crate::EnumVariantUnnamedField,
) {
    visit_optional!(visitor.visit_type(node.ty()));
}

pub fn walk_literal<V: Visitor>(visitor: &mut V, node: crate::Literal) {
    match node {
        crate::Literal::Boolean(node) => visitor.visit_boolean_literal(node),
        crate::Literal::Char(node) => visitor.visit_char_literal(node),
        crate::Literal::Numeric(node) => visitor.visit_numeric_literal(node),
        crate::Literal::String(node) => visitor.visit_string_literal(node),
    }
}

pub fn walk_visibility_kind<V: Visitor>(visitor: &mut V, node: crate::VisibilityKind) {
    match node {
        crate::VisibilityKind::Crate(node) => visitor.visit_visibility_crate(node),
        crate::VisibilityKind::Super(node) => visitor.visit_visibility_super(node),
        crate::VisibilityKind::In(node) => visitor.visit_visibility_in(node),
    }
}

pub fn walk_array_expression<V: Visitor>(visitor: &mut V, node: crate::ArrayExpression) {
    visit_each!(visitor.visit_array_expression_element(node.elements()));
}

pub fn walk_array_expression_element<V: Visitor>(
    visitor: &mut V,
    node: crate::ArrayExpressionElement,
) {
    visit_optional!(visitor.visit_expression(node.expression()));
}

pub fn walk_char_literal<V: Visitor>(visitor: &mut V, node: crate::CharLiteral) {
    visit_optional!(visitor.visit_raw(node.raw()));
}

pub fn walk_numeric_literal<V: Visitor>(visitor: &mut V, node: crate::NumericLiteral) {
    visit_optional!(visitor.visit_numeric_literal_kind(node.kind()));
}

pub fn walk_string_literal<V: Visitor>(visitor: &mut V, node: crate::StringLiteral) {
    visit_each!(visitor.visit_string_literal_fragment(node.fragments()));
}

pub fn walk_string_literal_fragment<V: Visitor>(
    visitor: &mut V,
    node: crate::StringLiteralFragment,
) {
    match node {
        crate::StringLiteralFragment::Raw(node) => visitor.visit_raw(node),
        crate::StringLiteralFragment::Escape(node) => visitor.visit_escape(node),
        crate::StringLiteralFragment::Interpolation(node) => visitor.visit_interpolation(node),
    }
}

pub fn walk_visibility_in<V: Visitor>(visitor: &mut V, node: crate::VisibilityIn) {
    visit_optional!(visitor.visit_path(node.path()));
}

pub fn walk_numeric_literal_kind<V: Visitor>(visitor: &mut V, node: crate::NumericLiteralKind) {
    match node {
        crate::NumericLiteralKind::Decimal(node) => visitor.visit_decimal_numeric_literal(node),
        crate::NumericLiteralKind::Binary(node) => visitor.visit_binary_numeric_literal(node),
        crate::NumericLiteralKind::Octal(node) => visitor.visit_octal_numeric_literal(node),
        crate::NumericLiteralKind::Hex(node) => visitor.visit_hex_numeric_literal(node),
    }
}

pub fn walk_escape<V: Visitor>(visitor: &mut V, node: crate::Escape) {
    visit_optional!(visitor.visit_raw(node.raw()));
}

pub fn walk_interpolation<V: Visitor>(visitor: &mut V, node: crate::Interpolation) {
    visit_optional!(visitor.visit_expression(node.expression()));
}

pub fn walk_decimal_numeric_literal<V: Visitor>(
    visitor: &mut V,
    node: crate::DecimalNumericLiteral,
) {
    visit_optional!(visitor.visit_integer_part(node.integer()));
    visit_optional!(visitor.visit_fraction_part(node.fraction()));
    visit_optional!(visitor.visit_exponent_part(node.exponent()));
}

pub fn walk_binary_numeric_literal<V: Visitor>(visitor: &mut V, node: crate::BinaryNumericLiteral) {
    visit_optional!(visitor.visit_numeric_fragment(node.fragment()));
}

pub fn walk_octal_numeric_literal<V: Visitor>(visitor: &mut V, node: crate::OctalNumericLiteral) {
    visit_optional!(visitor.visit_numeric_fragment(node.fragment()));
}

pub fn walk_hex_numeric_literal<V: Visitor>(visitor: &mut V, node: crate::HexNumericLiteral) {
    visit_optional!(visitor.visit_numeric_fragment(node.fragment()));
}

pub fn walk_integer_part<V: Visitor>(visitor: &mut V, node: crate::IntegerPart) {
    visit_optional!(visitor.visit_numeric_fragment(node.fragment()));
}

pub fn walk_fraction_part<V: Visitor>(visitor: &mut V, node: crate::FractionPart) {
    visit_optional!(visitor.visit_numeric_fragment(node.fragment()));
}

pub fn walk_exponent_part<V: Visitor>(visitor: &mut V, node: crate::ExponentPart) {
    visit_optional!(visitor.visit_numeric_fragment(node.fragment()));
}

pub fn walk_path<V: Visitor>(visitor: &mut V, node: crate::Path) {
    visit_each!(visitor.visit_path_segment(node.segments()));
}

pub fn walk_path_segment<V: Visitor>(visitor: &mut V, node: crate::PathSegment) {
    visit_optional!(visitor.visit_identifier(node.identifier()));
    visit_optional!(visitor.visit_type_argument(node.type_argument()));
}

pub fn walk_type_argument<V: Visitor>(visitor: &mut V, node: crate::TypeArgument) {
    visit_each!(visitor.visit_type(node.types()));
}

pub fn walk_target_type<V: Visitor>(visitor: &mut V, node: crate::TargetType) {
    visit_optional!(visitor.visit_type(node.ty()));
}

pub fn walk_pattern<V: Visitor>(visitor: &mut V, node: crate::Pattern) {
    visit_optional!(visitor.visit_pattern_kind(node.kind()));
}

pub fn walk_pattern_kind<V: Visitor>(visitor: &mut V, node: crate::PatternKind) {
    match node {
        crate::PatternKind::Never(node) => visitor.visit_never_pattern(node),
        crate::PatternKind::Placeholder(node) => visitor.visit_placeholder_pattern(node),
        crate::PatternKind::Path(node) => visitor.visit_path_pattern(node),
        crate::PatternKind::Tuple(node) => visitor.visit_tuple_pattern(node),
        crate::PatternKind::Array(node) => visitor.visit_array_pattern(node),
        crate::PatternKind::Literal(node) => visitor.visit_literal_pattern(node),
        crate::PatternKind::Rest(node) => visitor.visit_rest_pattern(node),
        crate::PatternKind::Or(node) => visitor.visit_or_pattern(node),
        crate::PatternKind::Named(node) => visitor.visit_named_pattern(node),
        crate::PatternKind::Unnamed(node) => visitor.visit_unnamed_pattern(node),
    }
}

pub fn walk_path_pattern<V: Visitor>(visitor: &mut V, node: crate::PathPattern) {
    visit_optional!(visitor.visit_path(node.path()));
}

pub fn walk_tuple_pattern<V: Visitor>(visitor: &mut V, node: crate::TuplePattern) {
    visit_each!(visitor.visit_tuple_pattern_element(node.elements()));
}

pub fn walk_tuple_pattern_element<V: Visitor>(visitor: &mut V, node: crate::TuplePatternElement) {
    visit_optional!(visitor.visit_pattern(node.pattern()));
}

pub fn walk_array_pattern<V: Visitor>(visitor: &mut V, node: crate::ArrayPattern) {
    visit_each!(visitor.visit_array_pattern_element(node.elements()));
}

pub fn walk_array_pattern_element<V: Visitor>(visitor: &mut V, node: crate::ArrayPatternElement) {
    visit_optional!(visitor.visit_pattern(node.pattern()));
}

pub fn walk_literal_pattern<V: Visitor>(visitor: &mut V, node: crate::LiteralPattern) {
    visit_optional!(visitor.visit_literal(node.literal()));
}

pub fn walk_or_pattern<V: Visitor>(visitor: &mut V, node: crate::OrPattern) {
    visit_each!(visitor.visit_pattern(node.patterns()));
}

pub fn walk_named_pattern<V: Visitor>(visitor: &mut V, node: crate::NamedPattern) {
    visit_optional!(visitor.visit_path(node.path()));
    visit_each!(visitor.visit_named_pattern_element(node.elements()));
}

pub fn walk_named_pattern_element<V: Visitor>(visitor: &mut V, node: crate::NamedPatternElement) {
    visit_optional!(visitor.visit_pattern(node.pattern()));
}

pub fn walk_unnamed_pattern<V: Visitor>(visitor: &mut V, node: crate::UnnamedPattern) {
    visit_each!(visitor.visit_unnamed_pattern_element(node.elements()));
}

pub fn walk_unnamed_pattern_element<V: Visitor>(
    visitor: &mut V,
    node: crate::UnnamedPatternElement,
) {
    visit_optional!(visitor.visit_pattern(node.pattern()));
}
