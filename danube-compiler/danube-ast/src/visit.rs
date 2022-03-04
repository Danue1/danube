use crate::ast::*;
#[allow(unused_variables)]
pub trait Visit<'ast>: Sized {
    type Context;
    fn visit_package_node(&mut self, context: &Self::Context, package_node: &'ast PackageNode) {
        walk_package_node(self, context, package_node);
    }
    fn visit_attribute_node(
        &mut self,
        context: &Self::Context,
        attribute_node: &'ast AttributeNode,
    ) {
        walk_attribute_node(self, context, attribute_node);
    }
    fn visit_attribute_argument_node(
        &mut self,
        context: &Self::Context,
        attribute_argument_node: &'ast AttributeArgumentNode,
    ) {
        walk_attribute_argument_node(self, context, attribute_argument_node);
    }
    fn visit_path_node(&mut self, context: &Self::Context, path_node: &'ast PathNode) {
        walk_path_node(self, context, path_node);
    }
    fn visit_ident_node(&mut self, context: &Self::Context, ident_node: &'ast IdentNode) {
        walk_ident_node(self, context, ident_node);
    }
    fn visit_item_node(&mut self, context: &Self::Context, item_node: &'ast ItemNode) {
        walk_item_node(self, context, item_node);
    }
    fn visit_item_kind(&mut self, context: &Self::Context, item_kind: &'ast ItemKind) {
        walk_item_kind(self, context, item_kind);
    }
    fn visit_mod_node(&mut self, context: &Self::Context, mod_node: &'ast ModNode) {
        walk_mod_node(self, context, mod_node);
    }
    fn visit_use_node(&mut self, context: &Self::Context, use_node: &'ast UseNode) {
        walk_use_node(self, context, use_node);
    }
    fn visit_visibility_kind(
        &mut self,
        context: &Self::Context,
        visibility_kind: &'ast VisibilityKind,
    ) {
        walk_visibility_kind(self, context, visibility_kind);
    }
    fn visit_struct_node(&mut self, context: &Self::Context, struct_node: &'ast StructNode) {
        walk_struct_node(self, context, struct_node);
    }
    fn visit_generic_node(&mut self, context: &Self::Context, generic_node: &'ast GenericNode) {
        walk_generic_node(self, context, generic_node);
    }
    fn visit_struct_field_kind(
        &mut self,
        context: &Self::Context,
        struct_field_kind: &'ast StructFieldKind,
    ) {
        walk_struct_field_kind(self, context, struct_field_kind);
    }
    fn visit_unnamed_struct_field(
        &mut self,
        context: &Self::Context,
        unnamed_struct_field: &'ast UnnamedStructField,
    ) {
        walk_unnamed_struct_field(self, context, unnamed_struct_field);
    }
    fn visit_named_struct_field(
        &mut self,
        context: &Self::Context,
        named_struct_field: &'ast NamedStructField,
    ) {
        walk_named_struct_field(self, context, named_struct_field);
    }
    fn visit_immutability_kind(
        &mut self,
        context: &Self::Context,
        immutability_kind: &'ast ImmutabilityKind,
    ) {
        walk_immutability_kind(self, context, immutability_kind);
    }
    fn visit_type_node(&mut self, context: &Self::Context, type_node: &'ast TypeNode) {
        walk_type_node(self, context, type_node);
    }
    fn visit_type_kind(&mut self, context: &Self::Context, type_kind: &'ast TypeKind) {
        walk_type_kind(self, context, type_kind);
    }
    fn visit_generic_type_node(
        &mut self,
        context: &Self::Context,
        generic_type_node: &'ast GenericTypeNode,
    ) {
        walk_generic_type_node(self, context, generic_type_node);
    }
    fn visit_enum_node(&mut self, context: &Self::Context, enum_node: &'ast EnumNode) {
        walk_enum_node(self, context, enum_node);
    }
    fn visit_enum_variant_node(
        &mut self,
        context: &Self::Context,
        enum_variant_node: &'ast EnumVariantNode,
    ) {
        walk_enum_variant_node(self, context, enum_variant_node);
    }
    fn visit_enum_variant_kind(
        &mut self,
        context: &Self::Context,
        enum_variant_kind: &'ast EnumVariantKind,
    ) {
        walk_enum_variant_kind(self, context, enum_variant_kind);
    }
    fn visit_enum_named_variant_node(
        &mut self,
        context: &Self::Context,
        enum_named_variant_node: &'ast EnumNamedVariantNode,
    ) {
        walk_enum_named_variant_node(self, context, enum_named_variant_node);
    }
    fn visit_function_node(&mut self, context: &Self::Context, function_node: &'ast FunctionNode) {
        walk_function_node(self, context, function_node);
    }
    fn visit_function_parameter_node(
        &mut self,
        context: &Self::Context,
        function_parameter_node: &'ast FunctionParameterNode,
    ) {
        walk_function_parameter_node(self, context, function_parameter_node);
    }
    fn visit_block_node(&mut self, context: &Self::Context, block_node: &'ast BlockNode) {
        walk_block_node(self, context, block_node);
    }
    fn visit_statement_node(
        &mut self,
        context: &Self::Context,
        statement_node: &'ast StatementNode,
    ) {
        walk_statement_node(self, context, statement_node);
    }
    fn visit_statement_kind(
        &mut self,
        context: &Self::Context,
        statement_kind: &'ast StatementKind,
    ) {
        walk_statement_kind(self, context, statement_kind);
    }
    fn visit_assign_node(&mut self, context: &Self::Context, assign_node: &'ast AssignNode) {
        walk_assign_node(self, context, assign_node);
    }
    fn visit_assign_kind(&mut self, context: &Self::Context, assign_kind: &'ast AssignKind) {
        walk_assign_kind(self, context, assign_kind);
    }
    fn visit_let_node(&mut self, context: &Self::Context, let_node: &'ast LetNode) {
        walk_let_node(self, context, let_node);
    }
    fn visit_pattern_node(&mut self, context: &Self::Context, pattern_node: &'ast PatternNode) {
        walk_pattern_node(self, context, pattern_node);
    }
    fn visit_pattern_kind(&mut self, context: &Self::Context, pattern_kind: &'ast PatternKind) {
        walk_pattern_kind(self, context, pattern_kind);
    }
    fn visit_literal_node(&mut self, context: &Self::Context, literal_node: &'ast LiteralNode) {
        walk_literal_node(self, context, literal_node);
    }
    fn visit_pattern_named_struct_node(
        &mut self,
        context: &Self::Context,
        pattern_named_struct_node: &'ast PatternNamedStructNode,
    ) {
        walk_pattern_named_struct_node(self, context, pattern_named_struct_node);
    }
    fn visit_pattern_named_struct_field_node(
        &mut self,
        context: &Self::Context,
        pattern_named_struct_field_node: &'ast PatternNamedStructFieldNode,
    ) {
        walk_pattern_named_struct_field_node(self, context, pattern_named_struct_field_node);
    }
    fn visit_pattern_unnamed_struct_node(
        &mut self,
        context: &Self::Context,
        pattern_unnamed_struct_node: &'ast PatternUnnamedStructNode,
    ) {
        walk_pattern_unnamed_struct_node(self, context, pattern_unnamed_struct_node);
    }
    fn visit_type_alias_node(
        &mut self,
        context: &Self::Context,
        type_alias_node: &'ast TypeAliasNode,
    ) {
        walk_type_alias_node(self, context, type_alias_node);
    }
    fn visit_expression_node(
        &mut self,
        context: &Self::Context,
        expression_node: &'ast ExpressionNode,
    ) {
        walk_expression_node(self, context, expression_node);
    }
    fn visit_expression_kind(
        &mut self,
        context: &Self::Context,
        expression_kind: &'ast ExpressionKind,
    ) {
        walk_expression_kind(self, context, expression_kind);
    }
    fn visit_let_expression_node(
        &mut self,
        context: &Self::Context,
        let_expression_node: &'ast LetExpressionNode,
    ) {
        walk_let_expression_node(self, context, let_expression_node);
    }
    fn visit_condition_node(
        &mut self,
        context: &Self::Context,
        condition_node: &'ast ConditionNode,
    ) {
        walk_condition_node(self, context, condition_node);
    }
    fn visit_condition_branch(
        &mut self,
        context: &Self::Context,
        condition_branch: &'ast ConditionBranch,
    ) {
        walk_condition_branch(self, context, condition_branch);
    }
    fn visit_loop_node(&mut self, context: &Self::Context, loop_node: &'ast LoopNode) {
        walk_loop_node(self, context, loop_node);
    }
    fn visit_while_node(&mut self, context: &Self::Context, while_node: &'ast WhileNode) {
        walk_while_node(self, context, while_node);
    }
    fn visit_for_node(&mut self, context: &Self::Context, for_node: &'ast ForNode) {
        walk_for_node(self, context, for_node);
    }
    fn visit_match_node(&mut self, context: &Self::Context, match_node: &'ast MatchNode) {
        walk_match_node(self, context, match_node);
    }
    fn visit_match_branch(&mut self, context: &Self::Context, match_branch: &'ast MatchBranch) {
        walk_match_branch(self, context, match_branch);
    }
    fn visit_closure_node(&mut self, context: &Self::Context, closure_node: &'ast ClosureNode) {
        walk_closure_node(self, context, closure_node);
    }
    fn visit_closure_parameter_node(
        &mut self,
        context: &Self::Context,
        closure_parameter_node: &'ast ClosureParameterNode,
    ) {
        walk_closure_parameter_node(self, context, closure_parameter_node);
    }
    fn visit_tuple_node(&mut self, context: &Self::Context, tuple_node: &'ast TupleNode) {
        walk_tuple_node(self, context, tuple_node);
    }
    fn visit_index_node(&mut self, context: &Self::Context, index_node: &'ast IndexNode) {
        walk_index_node(self, context, index_node);
    }
    fn visit_binary_expression_node(
        &mut self,
        context: &Self::Context,
        binary_expression_node: &'ast BinaryExpressionNode,
    ) {
        walk_binary_expression_node(self, context, binary_expression_node);
    }
    fn visit_field_node(&mut self, context: &Self::Context, field_node: &'ast FieldNode) {
        walk_field_node(self, context, field_node);
    }
    fn visit_function_call_node(
        &mut self,
        context: &Self::Context,
        function_call_node: &'ast FunctionCallNode,
    ) {
        walk_function_call_node(self, context, function_call_node);
    }
    fn visit_method_call_node(
        &mut self,
        context: &Self::Context,
        method_call_node: &'ast MethodCallNode,
    ) {
        walk_method_call_node(self, context, method_call_node);
    }
    fn visit_argument_node(&mut self, context: &Self::Context, argument_node: &'ast ArgumentNode) {
        walk_argument_node(self, context, argument_node);
    }
    fn visit_binary_operator_kind(
        &mut self,
        context: &Self::Context,
        binary_operator_kind: &'ast BinaryOperatorKind,
    ) {
        walk_binary_operator_kind(self, context, binary_operator_kind);
    }
    fn visit_trait_node(&mut self, context: &Self::Context, trait_node: &'ast TraitNode) {
        walk_trait_node(self, context, trait_node);
    }
    fn visit_constant_node(&mut self, context: &Self::Context, constant_node: &'ast ConstantNode) {
        walk_constant_node(self, context, constant_node);
    }
    fn visit_implement_node(
        &mut self,
        context: &Self::Context,
        implement_node: &'ast ImplementNode,
    ) {
        walk_implement_node(self, context, implement_node);
    }
    fn visit_implement_item_node(
        &mut self,
        context: &Self::Context,
        implement_item_node: &'ast ImplementItemNode,
    ) {
        walk_implement_item_node(self, context, implement_item_node);
    }
    fn visit_implement_item_kind(
        &mut self,
        context: &Self::Context,
        implement_item_kind: &'ast ImplementItemKind,
    ) {
        walk_implement_item_kind(self, context, implement_item_kind);
    }
}
#[allow(unused_variables)]
pub fn walk_package_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    package_node: &'ast PackageNode,
) {
    for attributes in package_node.attributes.iter() {
        visitor.visit_attribute_node(context, attributes);
    }
    for items in package_node.items.iter() {
        visitor.visit_item_node(context, items);
    }
}
#[allow(unused_variables)]
pub fn walk_attribute_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    attribute_node: &'ast AttributeNode,
) {
    visitor.visit_path_node(context, &attribute_node.path);
    for args in attribute_node.args.iter() {
        visitor.visit_attribute_argument_node(context, args);
    }
}
#[allow(unused_variables)]
pub fn walk_attribute_argument_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    attribute_argument_node: &'ast AttributeArgumentNode,
) {
    visitor.visit_ident_node(context, &attribute_argument_node.ident);
    if let Some(ref value) = attribute_argument_node.value {
        visitor.visit_expression_node(context, value);
    }
}
#[allow(unused_variables)]
pub fn walk_path_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    path_node: &'ast PathNode,
) {
    for segments in path_node.segments.iter() {
        visitor.visit_ident_node(context, segments);
    }
}
#[allow(unused_variables)]
pub fn walk_ident_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    ident_node: &'ast IdentNode,
) {
}
#[allow(unused_variables)]
pub fn walk_item_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    item_node: &'ast ItemNode,
) {
    for attributes in item_node.attributes.iter() {
        visitor.visit_attribute_node(context, attributes);
    }
    visitor.visit_visibility_kind(context, &item_node.visibility);
    visitor.visit_item_kind(context, &item_node.kind);
}
#[allow(unused_variables)]
fn walk_item_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    item_kind: &'ast ItemKind,
) {
    match item_kind {
        ItemKind::Mod(node) => {
            visitor.visit_mod_node(context, node);
        }
        ItemKind::Use(node) => {
            visitor.visit_use_node(context, node);
        }
        ItemKind::Enum(node) => {
            visitor.visit_enum_node(context, node);
        }
        ItemKind::Struct(node) => {
            visitor.visit_struct_node(context, node);
        }
        ItemKind::Function(node) => {
            visitor.visit_function_node(context, node);
        }
        ItemKind::TypeAlias(node) => {
            visitor.visit_type_alias_node(context, node);
        }
        ItemKind::Trait(node) => {
            visitor.visit_trait_node(context, node);
        }
        ItemKind::Constant(node) => {
            visitor.visit_constant_node(context, node);
        }
        ItemKind::Implement(node) => {
            visitor.visit_implement_node(context, node);
        }
    }
}
#[allow(unused_variables)]
pub fn walk_mod_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    mod_node: &'ast ModNode,
) {
    for attributes in mod_node.attributes.iter() {
        visitor.visit_attribute_node(context, attributes);
    }
    for items in mod_node.items.iter() {
        visitor.visit_item_node(context, items);
    }
}
#[allow(unused_variables)]
pub fn walk_use_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    use_node: &'ast UseNode,
) {
    visitor.visit_path_node(context, &use_node.path);
}
#[allow(unused_variables)]
fn walk_visibility_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    visibility_kind: &'ast VisibilityKind,
) {
    match visibility_kind {
        VisibilityKind::Current => {}
        VisibilityKind::Public => {}
        VisibilityKind::Restricted(node) => {
            visitor.visit_path_node(context, node);
        }
    }
}
#[allow(unused_variables)]
pub fn walk_struct_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    struct_node: &'ast StructNode,
) {
    visitor.visit_ident_node(context, &struct_node.ident);
    for generics in struct_node.generics.iter() {
        visitor.visit_generic_node(context, generics);
    }
    if let Some(ref fields) = struct_node.fields {
        visitor.visit_struct_field_kind(context, fields);
    }
}
#[allow(unused_variables)]
pub fn walk_generic_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    generic_node: &'ast GenericNode,
) {
    visitor.visit_ident_node(context, &generic_node.ident);
    for traits in generic_node.traits.iter() {
        visitor.visit_path_node(context, traits);
    }
    if let Some(ref default) = generic_node.default {
        visitor.visit_path_node(context, default);
    }
}
#[allow(unused_variables)]
fn walk_struct_field_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    struct_field_kind: &'ast StructFieldKind,
) {
    match struct_field_kind {
        StructFieldKind::Unnamed(nodes) => {
            for node in nodes {
                visitor.visit_unnamed_struct_field(context, node);
            }
        }
        StructFieldKind::Named(nodes) => {
            for node in nodes {
                visitor.visit_named_struct_field(context, node);
            }
        }
    }
}
#[allow(unused_variables)]
pub fn walk_unnamed_struct_field<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    unnamed_struct_field: &'ast UnnamedStructField,
) {
    visitor.visit_visibility_kind(context, &unnamed_struct_field.visibility);
    visitor.visit_type_node(context, &unnamed_struct_field.ty);
}
#[allow(unused_variables)]
pub fn walk_named_struct_field<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    named_struct_field: &'ast NamedStructField,
) {
    visitor.visit_visibility_kind(context, &named_struct_field.visibility);
    visitor.visit_ident_node(context, &named_struct_field.ident);
    visitor.visit_type_node(context, &named_struct_field.ty);
}
#[allow(unused_variables)]
fn walk_immutability_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    immutability_kind: &'ast ImmutabilityKind,
) {
    match immutability_kind {
        ImmutabilityKind::Nope => {}
        ImmutabilityKind::Yes => {}
    }
}
#[allow(unused_variables)]
pub fn walk_type_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    type_node: &'ast TypeNode,
) {
    visitor.visit_immutability_kind(context, &type_node.immutability);
    visitor.visit_type_kind(context, &type_node.kind);
}
#[allow(unused_variables)]
fn walk_type_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    type_kind: &'ast TypeKind,
) {
    match type_kind {
        TypeKind::Tuple(nodes) => {
            for node in nodes {
                visitor.visit_type_kind(context, node);
            }
        }
        TypeKind::Path(node) => {
            visitor.visit_path_node(context, node);
        }
        TypeKind::Generic(node) => {
            visitor.visit_generic_type_node(context, node);
        }
    }
}
#[allow(unused_variables)]
pub fn walk_generic_type_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    generic_type_node: &'ast GenericTypeNode,
) {
    visitor.visit_path_node(context, &generic_type_node.path);
    for parameters in generic_type_node.parameters.iter() {
        visitor.visit_type_kind(context, parameters);
    }
}
#[allow(unused_variables)]
pub fn walk_enum_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    enum_node: &'ast EnumNode,
) {
    visitor.visit_ident_node(context, &enum_node.ident);
    for generics in enum_node.generics.iter() {
        visitor.visit_generic_node(context, generics);
    }
    for variants in enum_node.variants.iter() {
        visitor.visit_enum_variant_node(context, variants);
    }
}
#[allow(unused_variables)]
pub fn walk_enum_variant_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    enum_variant_node: &'ast EnumVariantNode,
) {
    visitor.visit_ident_node(context, &enum_variant_node.ident);
    if let Some(ref kind) = enum_variant_node.kind {
        visitor.visit_enum_variant_kind(context, kind);
    }
}
#[allow(unused_variables)]
fn walk_enum_variant_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    enum_variant_kind: &'ast EnumVariantKind,
) {
    match enum_variant_kind {
        EnumVariantKind::Unnamed(nodes) => {
            for node in nodes {
                visitor.visit_type_node(context, node);
            }
        }
        EnumVariantKind::Named(nodes) => {
            for node in nodes {
                visitor.visit_enum_named_variant_node(context, node);
            }
        }
    }
}
#[allow(unused_variables)]
pub fn walk_enum_named_variant_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    enum_named_variant_node: &'ast EnumNamedVariantNode,
) {
    visitor.visit_ident_node(context, &enum_named_variant_node.ident);
    visitor.visit_type_node(context, &enum_named_variant_node.ty);
}
#[allow(unused_variables)]
pub fn walk_function_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    function_node: &'ast FunctionNode,
) {
    visitor.visit_ident_node(context, &function_node.ident);
    for generics in function_node.generics.iter() {
        visitor.visit_generic_node(context, generics);
    }
    if let Some(ref self_type) = function_node.self_type {
        visitor.visit_immutability_kind(context, self_type);
    }
    for parameters in function_node.parameters.iter() {
        visitor.visit_function_parameter_node(context, parameters);
    }
    if let Some(ref return_type) = function_node.return_type {
        visitor.visit_type_node(context, return_type);
    }
    if let Some(ref block) = function_node.block {
        visitor.visit_block_node(context, block);
    }
}
#[allow(unused_variables)]
pub fn walk_function_parameter_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    function_parameter_node: &'ast FunctionParameterNode,
) {
    visitor.visit_ident_node(context, &function_parameter_node.argument_label);
    if let Some(ref parameter_label) = function_parameter_node.parameter_label {
        visitor.visit_ident_node(context, parameter_label);
    }
    visitor.visit_type_node(context, &function_parameter_node.ty);
}
#[allow(unused_variables)]
pub fn walk_block_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    block_node: &'ast BlockNode,
) {
    for statements in block_node.statements.iter() {
        visitor.visit_statement_node(context, statements);
    }
}
#[allow(unused_variables)]
pub fn walk_statement_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    statement_node: &'ast StatementNode,
) {
    visitor.visit_statement_kind(context, &statement_node.kind);
}
#[allow(unused_variables)]
fn walk_statement_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    statement_kind: &'ast StatementKind,
) {
    match statement_kind {
        StatementKind::Semicolon => {}
        StatementKind::Break => {}
        StatementKind::Continue => {}
        StatementKind::Return(node) => {
            if let Some(node) = node {
                visitor.visit_expression_node(context, node);
            }
        }
        StatementKind::Item(node) => {
            visitor.visit_item_node(context, node);
        }
        StatementKind::Let(node) => {
            visitor.visit_let_node(context, node);
        }
        StatementKind::Assign(node) => {
            visitor.visit_assign_node(context, node);
        }
        StatementKind::Expression(node) => {
            visitor.visit_expression_node(context, node);
        }
    }
}
#[allow(unused_variables)]
pub fn walk_assign_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    assign_node: &'ast AssignNode,
) {
    visitor.visit_assign_kind(context, &assign_node.kind);
    visitor.visit_expression_node(context, &assign_node.lhs);
    visitor.visit_expression_node(context, &assign_node.rhs);
}
#[allow(unused_variables)]
fn walk_assign_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    assign_kind: &'ast AssignKind,
) {
    match assign_kind {
        AssignKind::Assign => {}
        AssignKind::Add => {}
        AssignKind::Sub => {}
        AssignKind::Exp => {}
        AssignKind::Mul => {}
        AssignKind::Div => {}
        AssignKind::Mod => {}
        AssignKind::And => {}
        AssignKind::Or => {}
        AssignKind::BitAnd => {}
        AssignKind::BitOr => {}
        AssignKind::BitXor => {}
        AssignKind::BitLeft => {}
        AssignKind::BitRight => {}
    }
}
#[allow(unused_variables)]
pub fn walk_let_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    let_node: &'ast LetNode,
) {
    visitor.visit_immutability_kind(context, &let_node.immutability);
    visitor.visit_pattern_node(context, &let_node.pattern);
    if let Some(ref ty) = let_node.ty {
        visitor.visit_type_node(context, ty);
    }
    if let Some(ref value) = let_node.value {
        visitor.visit_expression_node(context, value);
    }
}
#[allow(unused_variables)]
pub fn walk_pattern_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    pattern_node: &'ast PatternNode,
) {
    visitor.visit_pattern_kind(context, &pattern_node.kind);
}
#[allow(unused_variables)]
fn walk_pattern_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    pattern_kind: &'ast PatternKind,
) {
    match pattern_kind {
        PatternKind::Wildcard => {}
        PatternKind::Rest => {}
        PatternKind::Literal(node) => {
            visitor.visit_literal_node(context, node);
        }
        PatternKind::Path(node) => {
            visitor.visit_path_node(context, node);
        }
        PatternKind::NamedStruct(node) => {
            visitor.visit_pattern_named_struct_node(context, node);
        }
        PatternKind::UnnamedStruct(node) => {
            visitor.visit_pattern_unnamed_struct_node(context, node);
        }
        PatternKind::Slice(nodes) => {
            for node in nodes {
                visitor.visit_pattern_node(context, node);
            }
        }
    }
}
#[allow(unused_variables)]
pub fn walk_literal_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    literal_node: &'ast LiteralNode,
) {
}
#[allow(unused_variables)]
pub fn walk_pattern_named_struct_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    pattern_named_struct_node: &'ast PatternNamedStructNode,
) {
    visitor.visit_path_node(context, &pattern_named_struct_node.path);
    for fields in pattern_named_struct_node.fields.iter() {
        visitor.visit_pattern_named_struct_field_node(context, fields);
    }
}
#[allow(unused_variables)]
pub fn walk_pattern_named_struct_field_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    pattern_named_struct_field_node: &'ast PatternNamedStructFieldNode,
) {
    visitor.visit_path_node(context, &pattern_named_struct_field_node.path);
    if let Some(ref pattern) = pattern_named_struct_field_node.pattern {
        visitor.visit_pattern_node(context, pattern);
    }
}
#[allow(unused_variables)]
pub fn walk_pattern_unnamed_struct_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    pattern_unnamed_struct_node: &'ast PatternUnnamedStructNode,
) {
    if let Some(ref path) = pattern_unnamed_struct_node.path {
        visitor.visit_path_node(context, path);
    }
    for fields in pattern_unnamed_struct_node.fields.iter() {
        visitor.visit_pattern_node(context, fields);
    }
}
#[allow(unused_variables)]
pub fn walk_type_alias_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    type_alias_node: &'ast TypeAliasNode,
) {
    visitor.visit_ident_node(context, &type_alias_node.ident);
    if let Some(ref ty) = type_alias_node.ty {
        visitor.visit_type_node(context, ty);
    }
}
#[allow(unused_variables)]
pub fn walk_expression_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    expression_node: &'ast ExpressionNode,
) {
    visitor.visit_expression_kind(context, &expression_node.kind);
}
#[allow(unused_variables)]
fn walk_expression_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    expression_kind: &'ast ExpressionKind,
) {
    match expression_kind {
        ExpressionKind::Let(node) => {
            visitor.visit_let_expression_node(context, node);
        }
        ExpressionKind::Negate(node) => {
            visitor.visit_expression_node(context, node);
        }
        ExpressionKind::Not(node) => {
            visitor.visit_expression_node(context, node);
        }
        ExpressionKind::Literal(node) => {
            visitor.visit_literal_node(context, node);
        }
        ExpressionKind::Conditional(node) => {
            visitor.visit_condition_node(context, node);
        }
        ExpressionKind::Loop(node) => {
            visitor.visit_loop_node(context, node);
        }
        ExpressionKind::While(node) => {
            visitor.visit_while_node(context, node);
        }
        ExpressionKind::For(node) => {
            visitor.visit_for_node(context, node);
        }
        ExpressionKind::Match(node) => {
            visitor.visit_match_node(context, node);
        }
        ExpressionKind::Path(node) => {
            visitor.visit_path_node(context, node);
        }
        ExpressionKind::FunctionCall(node) => {
            visitor.visit_function_call_node(context, node);
        }
        ExpressionKind::Closure(node) => {
            visitor.visit_closure_node(context, node);
        }
        ExpressionKind::Block(node) => {
            visitor.visit_block_node(context, node);
        }
        ExpressionKind::Tuple(node) => {
            visitor.visit_tuple_node(context, node);
        }
        ExpressionKind::Array(nodes) => {
            for node in nodes {
                visitor.visit_expression_node(context, node);
            }
        }
        ExpressionKind::Try(node) => {
            visitor.visit_expression_node(context, node);
        }
        ExpressionKind::Await(node) => {
            visitor.visit_expression_node(context, node);
        }
        ExpressionKind::Field(node) => {
            visitor.visit_field_node(context, node);
        }
        ExpressionKind::Index(node) => {
            visitor.visit_index_node(context, node);
        }
        ExpressionKind::MethodCall(node) => {
            visitor.visit_method_call_node(context, node);
        }
        ExpressionKind::Binary(node) => {
            visitor.visit_binary_expression_node(context, node);
        }
    }
}
#[allow(unused_variables)]
pub fn walk_let_expression_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    let_expression_node: &'ast LetExpressionNode,
) {
    visitor.visit_pattern_node(context, &let_expression_node.pattern);
    visitor.visit_expression_node(context, &let_expression_node.value);
}
#[allow(unused_variables)]
pub fn walk_condition_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    condition_node: &'ast ConditionNode,
) {
    for branches in condition_node.branches.iter() {
        visitor.visit_condition_branch(context, branches);
    }
    if let Some(ref other) = condition_node.other {
        visitor.visit_block_node(context, other);
    }
}
#[allow(unused_variables)]
pub fn walk_condition_branch<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    condition_branch: &'ast ConditionBranch,
) {
    visitor.visit_expression_node(context, &condition_branch.expression);
    visitor.visit_block_node(context, &condition_branch.block);
}
#[allow(unused_variables)]
pub fn walk_loop_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    loop_node: &'ast LoopNode,
) {
    visitor.visit_block_node(context, &loop_node.block);
}
#[allow(unused_variables)]
pub fn walk_while_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    while_node: &'ast WhileNode,
) {
    visitor.visit_condition_branch(context, &while_node.branch);
}
#[allow(unused_variables)]
pub fn walk_for_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    for_node: &'ast ForNode,
) {
    visitor.visit_pattern_node(context, &for_node.pattern);
    visitor.visit_expression_node(context, &for_node.iter);
    visitor.visit_block_node(context, &for_node.block);
}
#[allow(unused_variables)]
pub fn walk_match_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    match_node: &'ast MatchNode,
) {
    visitor.visit_expression_node(context, &match_node.expression);
    for branches in match_node.branches.iter() {
        visitor.visit_match_branch(context, branches);
    }
}
#[allow(unused_variables)]
pub fn walk_match_branch<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    match_branch: &'ast MatchBranch,
) {
    visitor.visit_pattern_node(context, &match_branch.pattern);
    visitor.visit_block_node(context, &match_branch.block);
}
#[allow(unused_variables)]
pub fn walk_closure_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    closure_node: &'ast ClosureNode,
) {
    for parameters in closure_node.parameters.iter() {
        visitor.visit_closure_parameter_node(context, parameters);
    }
    if let Some(ref return_type) = closure_node.return_type {
        visitor.visit_type_node(context, return_type);
    }
    visitor.visit_block_node(context, &closure_node.block);
}
#[allow(unused_variables)]
pub fn walk_closure_parameter_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    closure_parameter_node: &'ast ClosureParameterNode,
) {
    visitor.visit_ident_node(context, &closure_parameter_node.ident);
    if let Some(ref ty) = closure_parameter_node.ty {
        visitor.visit_type_node(context, ty);
    }
}
#[allow(unused_variables)]
pub fn walk_tuple_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    tuple_node: &'ast TupleNode,
) {
    for arguments in tuple_node.arguments.iter() {
        visitor.visit_expression_node(context, arguments);
    }
}
#[allow(unused_variables)]
pub fn walk_index_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    index_node: &'ast IndexNode,
) {
    visitor.visit_expression_node(context, &index_node.expression);
    visitor.visit_expression_node(context, &index_node.index);
}
#[allow(unused_variables)]
pub fn walk_binary_expression_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    binary_expression_node: &'ast BinaryExpressionNode,
) {
    visitor.visit_binary_operator_kind(context, &binary_expression_node.kind);
    visitor.visit_expression_node(context, &binary_expression_node.lhs);
    visitor.visit_expression_node(context, &binary_expression_node.rhs);
}
#[allow(unused_variables)]
pub fn walk_field_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    field_node: &'ast FieldNode,
) {
    visitor.visit_expression_node(context, &field_node.expression);
    visitor.visit_ident_node(context, &field_node.field);
}
#[allow(unused_variables)]
pub fn walk_function_call_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    function_call_node: &'ast FunctionCallNode,
) {
    visitor.visit_expression_node(context, &function_call_node.expression);
    for arguments in function_call_node.arguments.iter() {
        visitor.visit_argument_node(context, arguments);
    }
}
#[allow(unused_variables)]
pub fn walk_method_call_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    method_call_node: &'ast MethodCallNode,
) {
    visitor.visit_ident_node(context, &method_call_node.ident);
    for arguments in method_call_node.arguments.iter() {
        visitor.visit_argument_node(context, arguments);
    }
}
#[allow(unused_variables)]
pub fn walk_argument_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    argument_node: &'ast ArgumentNode,
) {
    if let Some(ref ident) = argument_node.ident {
        visitor.visit_ident_node(context, ident);
    }
    visitor.visit_expression_node(context, &argument_node.expression);
}
#[allow(unused_variables)]
fn walk_binary_operator_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    binary_operator_kind: &'ast BinaryOperatorKind,
) {
    match binary_operator_kind {
        BinaryOperatorKind::Add => {}
        BinaryOperatorKind::Sub => {}
        BinaryOperatorKind::Mul => {}
        BinaryOperatorKind::Exp => {}
        BinaryOperatorKind::Div => {}
        BinaryOperatorKind::Mod => {}
        BinaryOperatorKind::BitAnd => {}
        BinaryOperatorKind::BitOr => {}
        BinaryOperatorKind::BitXor => {}
        BinaryOperatorKind::BitLeft => {}
        BinaryOperatorKind::BitRight => {}
        BinaryOperatorKind::Equal => {}
        BinaryOperatorKind::NotEqual => {}
        BinaryOperatorKind::GreaterThan => {}
        BinaryOperatorKind::LessThan => {}
        BinaryOperatorKind::GreaterThanOrEqual => {}
        BinaryOperatorKind::LessThanOrEqual => {}
        BinaryOperatorKind::And => {}
        BinaryOperatorKind::Or => {}
    }
}
#[allow(unused_variables)]
pub fn walk_trait_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    trait_node: &'ast TraitNode,
) {
    visitor.visit_ident_node(context, &trait_node.ident);
    for generics in trait_node.generics.iter() {
        visitor.visit_generic_node(context, generics);
    }
    for inheritances in trait_node.inheritances.iter() {
        visitor.visit_path_node(context, inheritances);
    }
    for items in trait_node.items.iter() {
        visitor.visit_implement_item_node(context, items);
    }
}
#[allow(unused_variables)]
pub fn walk_constant_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    constant_node: &'ast ConstantNode,
) {
    visitor.visit_pattern_node(context, &constant_node.pattern);
    visitor.visit_type_node(context, &constant_node.ty);
    if let Some(ref expression) = constant_node.expression {
        visitor.visit_expression_node(context, expression);
    }
}
#[allow(unused_variables)]
pub fn walk_implement_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    implement_node: &'ast ImplementNode,
) {
    for generics in implement_node.generics.iter() {
        visitor.visit_generic_node(context, generics);
    }
    if let Some(ref trait_ident) = implement_node.trait_ident {
        visitor.visit_path_node(context, trait_ident);
    }
    visitor.visit_path_node(context, &implement_node.target);
    for target_generics in implement_node.target_generics.iter() {
        visitor.visit_generic_node(context, target_generics);
    }
    for items in implement_node.items.iter() {
        visitor.visit_implement_item_node(context, items);
    }
}
#[allow(unused_variables)]
pub fn walk_implement_item_node<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    implement_item_node: &'ast ImplementItemNode,
) {
    for attributes in implement_item_node.attributes.iter() {
        visitor.visit_attribute_node(context, attributes);
    }
    visitor.visit_implement_item_kind(context, &implement_item_node.kind);
}
#[allow(unused_variables)]
fn walk_implement_item_kind<'ast, V: Visit<'ast>>(
    visitor: &mut V,
    context: &V::Context,
    implement_item_kind: &'ast ImplementItemKind,
) {
    match implement_item_kind {
        ImplementItemKind::Type(node) => {
            visitor.visit_type_alias_node(context, node);
        }
        ImplementItemKind::Constant(node) => {
            visitor.visit_constant_node(context, node);
        }
        ImplementItemKind::Function(node) => {
            visitor.visit_function_node(context, node);
        }
    }
}
