use crate::ast::*;
use crate::Context;
#[allow(unused_variables)]
pub trait Fold<'ast>: Sized
where
    Self::Context: Context,
{
    type Context;
    fn fold_package_node(&mut self, context: &Self::Context, package_node: &'ast mut PackageNode) {
        walk_package_node(self, context, package_node);
    }
    fn fold_attribute_node(
        &mut self,
        context: &Self::Context,
        attribute_node: &'ast mut AttributeNode,
    ) {
        walk_attribute_node(self, context, attribute_node);
    }
    fn fold_attribute_argument_node(
        &mut self,
        context: &Self::Context,
        attribute_argument_node: &'ast mut AttributeArgumentNode,
    ) {
        walk_attribute_argument_node(self, context, attribute_argument_node);
    }
    fn fold_path_node(&mut self, context: &Self::Context, path_node: &'ast mut PathNode) {
        walk_path_node(self, context, path_node);
    }
    fn fold_ident_node(&mut self, context: &Self::Context, ident_node: &'ast mut IdentNode) {
        walk_ident_node(self, context, ident_node);
    }
    fn fold_item_node(&mut self, context: &Self::Context, item_node: &'ast mut ItemNode) {
        walk_item_node(self, context, item_node);
    }
    fn fold_item_kind(&mut self, context: &Self::Context, item_kind: &'ast mut ItemKind) {
        walk_item_kind(self, context, item_kind);
    }
    fn fold_mod_node(&mut self, context: &Self::Context, mod_node: &'ast mut ModNode) {
        walk_mod_node(self, context, mod_node);
    }
    fn fold_use_node(&mut self, context: &Self::Context, use_node: &'ast mut UseNode) {
        walk_use_node(self, context, use_node);
    }
    fn fold_visibility_kind(
        &mut self,
        context: &Self::Context,
        visibility_kind: &'ast mut VisibilityKind,
    ) {
        walk_visibility_kind(self, context, visibility_kind);
    }
    fn fold_struct_node(&mut self, context: &Self::Context, struct_node: &'ast mut StructNode) {
        walk_struct_node(self, context, struct_node);
    }
    fn fold_generic_node(&mut self, context: &Self::Context, generic_node: &'ast mut GenericNode) {
        walk_generic_node(self, context, generic_node);
    }
    fn fold_struct_field_kind(
        &mut self,
        context: &Self::Context,
        struct_field_kind: &'ast mut StructFieldKind,
    ) {
        walk_struct_field_kind(self, context, struct_field_kind);
    }
    fn fold_unnamed_struct_field(
        &mut self,
        context: &Self::Context,
        unnamed_struct_field: &'ast mut UnnamedStructField,
    ) {
        walk_unnamed_struct_field(self, context, unnamed_struct_field);
    }
    fn fold_named_struct_field(
        &mut self,
        context: &Self::Context,
        named_struct_field: &'ast mut NamedStructField,
    ) {
        walk_named_struct_field(self, context, named_struct_field);
    }
    fn fold_immutability_kind(
        &mut self,
        context: &Self::Context,
        immutability_kind: &'ast mut ImmutabilityKind,
    ) {
        walk_immutability_kind(self, context, immutability_kind);
    }
    fn fold_type_node(&mut self, context: &Self::Context, type_node: &'ast mut TypeNode) {
        walk_type_node(self, context, type_node);
    }
    fn fold_type_kind(&mut self, context: &Self::Context, type_kind: &'ast mut TypeKind) {
        walk_type_kind(self, context, type_kind);
    }
    fn fold_generic_type_node(
        &mut self,
        context: &Self::Context,
        generic_type_node: &'ast mut GenericTypeNode,
    ) {
        walk_generic_type_node(self, context, generic_type_node);
    }
    fn fold_enum_node(&mut self, context: &Self::Context, enum_node: &'ast mut EnumNode) {
        walk_enum_node(self, context, enum_node);
    }
    fn fold_enum_variant_node(
        &mut self,
        context: &Self::Context,
        enum_variant_node: &'ast mut EnumVariantNode,
    ) {
        walk_enum_variant_node(self, context, enum_variant_node);
    }
    fn fold_enum_variant_kind(
        &mut self,
        context: &Self::Context,
        enum_variant_kind: &'ast mut EnumVariantKind,
    ) {
        walk_enum_variant_kind(self, context, enum_variant_kind);
    }
    fn fold_enum_named_variant_node(
        &mut self,
        context: &Self::Context,
        enum_named_variant_node: &'ast mut EnumNamedVariantNode,
    ) {
        walk_enum_named_variant_node(self, context, enum_named_variant_node);
    }
    fn fold_function_node(
        &mut self,
        context: &Self::Context,
        function_node: &'ast mut FunctionNode,
    ) {
        walk_function_node(self, context, function_node);
    }
    fn fold_function_parameter_node(
        &mut self,
        context: &Self::Context,
        function_parameter_node: &'ast mut FunctionParameterNode,
    ) {
        walk_function_parameter_node(self, context, function_parameter_node);
    }
    fn fold_block_node(&mut self, context: &Self::Context, block_node: &'ast mut BlockNode) {
        walk_block_node(self, context, block_node);
    }
    fn fold_statement_node(
        &mut self,
        context: &Self::Context,
        statement_node: &'ast mut StatementNode,
    ) {
        walk_statement_node(self, context, statement_node);
    }
    fn fold_statement_kind(
        &mut self,
        context: &Self::Context,
        statement_kind: &'ast mut StatementKind,
    ) {
        walk_statement_kind(self, context, statement_kind);
    }
    fn fold_assign_node(&mut self, context: &Self::Context, assign_node: &'ast mut AssignNode) {
        walk_assign_node(self, context, assign_node);
    }
    fn fold_assign_kind(&mut self, context: &Self::Context, assign_kind: &'ast mut AssignKind) {
        walk_assign_kind(self, context, assign_kind);
    }
    fn fold_let_node(&mut self, context: &Self::Context, let_node: &'ast mut LetNode) {
        walk_let_node(self, context, let_node);
    }
    fn fold_pattern_node(&mut self, context: &Self::Context, pattern_node: &'ast mut PatternNode) {
        walk_pattern_node(self, context, pattern_node);
    }
    fn fold_pattern_kind(&mut self, context: &Self::Context, pattern_kind: &'ast mut PatternKind) {
        walk_pattern_kind(self, context, pattern_kind);
    }
    fn fold_literal_node(&mut self, context: &Self::Context, literal_node: &'ast mut LiteralNode) {
        walk_literal_node(self, context, literal_node);
    }
    fn fold_pattern_named_struct_node(
        &mut self,
        context: &Self::Context,
        pattern_named_struct_node: &'ast mut PatternNamedStructNode,
    ) {
        walk_pattern_named_struct_node(self, context, pattern_named_struct_node);
    }
    fn fold_pattern_named_struct_field_node(
        &mut self,
        context: &Self::Context,
        pattern_named_struct_field_node: &'ast mut PatternNamedStructFieldNode,
    ) {
        walk_pattern_named_struct_field_node(self, context, pattern_named_struct_field_node);
    }
    fn fold_pattern_unnamed_struct_node(
        &mut self,
        context: &Self::Context,
        pattern_unnamed_struct_node: &'ast mut PatternUnnamedStructNode,
    ) {
        walk_pattern_unnamed_struct_node(self, context, pattern_unnamed_struct_node);
    }
    fn fold_type_alias_node(
        &mut self,
        context: &Self::Context,
        type_alias_node: &'ast mut TypeAliasNode,
    ) {
        walk_type_alias_node(self, context, type_alias_node);
    }
    fn fold_expression_node(
        &mut self,
        context: &Self::Context,
        expression_node: &'ast mut ExpressionNode,
    ) {
        walk_expression_node(self, context, expression_node);
    }
    fn fold_expression_kind(
        &mut self,
        context: &Self::Context,
        expression_kind: &'ast mut ExpressionKind,
    ) {
        walk_expression_kind(self, context, expression_kind);
    }
    fn fold_let_expression_node(
        &mut self,
        context: &Self::Context,
        let_expression_node: &'ast mut LetExpressionNode,
    ) {
        walk_let_expression_node(self, context, let_expression_node);
    }
    fn fold_condition_node(
        &mut self,
        context: &Self::Context,
        condition_node: &'ast mut ConditionNode,
    ) {
        walk_condition_node(self, context, condition_node);
    }
    fn fold_condition_branch(
        &mut self,
        context: &Self::Context,
        condition_branch: &'ast mut ConditionBranch,
    ) {
        walk_condition_branch(self, context, condition_branch);
    }
    fn fold_loop_node(&mut self, context: &Self::Context, loop_node: &'ast mut LoopNode) {
        walk_loop_node(self, context, loop_node);
    }
    fn fold_while_node(&mut self, context: &Self::Context, while_node: &'ast mut WhileNode) {
        walk_while_node(self, context, while_node);
    }
    fn fold_for_node(&mut self, context: &Self::Context, for_node: &'ast mut ForNode) {
        walk_for_node(self, context, for_node);
    }
    fn fold_match_node(&mut self, context: &Self::Context, match_node: &'ast mut MatchNode) {
        walk_match_node(self, context, match_node);
    }
    fn fold_match_branch(&mut self, context: &Self::Context, match_branch: &'ast mut MatchBranch) {
        walk_match_branch(self, context, match_branch);
    }
    fn fold_closure_node(&mut self, context: &Self::Context, closure_node: &'ast mut ClosureNode) {
        walk_closure_node(self, context, closure_node);
    }
    fn fold_closure_parameter_node(
        &mut self,
        context: &Self::Context,
        closure_parameter_node: &'ast mut ClosureParameterNode,
    ) {
        walk_closure_parameter_node(self, context, closure_parameter_node);
    }
    fn fold_tuple_node(&mut self, context: &Self::Context, tuple_node: &'ast mut TupleNode) {
        walk_tuple_node(self, context, tuple_node);
    }
    fn fold_index_node(&mut self, context: &Self::Context, index_node: &'ast mut IndexNode) {
        walk_index_node(self, context, index_node);
    }
    fn fold_binary_expression_node(
        &mut self,
        context: &Self::Context,
        binary_expression_node: &'ast mut BinaryExpressionNode,
    ) {
        walk_binary_expression_node(self, context, binary_expression_node);
    }
    fn fold_field_node(&mut self, context: &Self::Context, field_node: &'ast mut FieldNode) {
        walk_field_node(self, context, field_node);
    }
    fn fold_function_call_node(
        &mut self,
        context: &Self::Context,
        function_call_node: &'ast mut FunctionCallNode,
    ) {
        walk_function_call_node(self, context, function_call_node);
    }
    fn fold_method_call_node(
        &mut self,
        context: &Self::Context,
        method_call_node: &'ast mut MethodCallNode,
    ) {
        walk_method_call_node(self, context, method_call_node);
    }
    fn fold_argument_node(
        &mut self,
        context: &Self::Context,
        argument_node: &'ast mut ArgumentNode,
    ) {
        walk_argument_node(self, context, argument_node);
    }
    fn fold_binary_operator_kind(
        &mut self,
        context: &Self::Context,
        binary_operator_kind: &'ast mut BinaryOperatorKind,
    ) {
        walk_binary_operator_kind(self, context, binary_operator_kind);
    }
    fn fold_trait_node(&mut self, context: &Self::Context, trait_node: &'ast mut TraitNode) {
        walk_trait_node(self, context, trait_node);
    }
    fn fold_constant_node(
        &mut self,
        context: &Self::Context,
        constant_node: &'ast mut ConstantNode,
    ) {
        walk_constant_node(self, context, constant_node);
    }
    fn fold_implement_node(
        &mut self,
        context: &Self::Context,
        implement_node: &'ast mut ImplementNode,
    ) {
        walk_implement_node(self, context, implement_node);
    }
    fn fold_implement_item_node(
        &mut self,
        context: &Self::Context,
        implement_item_node: &'ast mut ImplementItemNode,
    ) {
        walk_implement_item_node(self, context, implement_item_node);
    }
    fn fold_implement_item_kind(
        &mut self,
        context: &Self::Context,
        implement_item_kind: &'ast mut ImplementItemKind,
    ) {
        walk_implement_item_kind(self, context, implement_item_kind);
    }
}
#[allow(unused_variables)]
pub fn walk_package_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    package_node: &'ast mut PackageNode,
) where
    F::Context: Context,
{
    for attributes in package_node.attributes.iter_mut() {
        folder.fold_attribute_node(context, attributes);
    }
    for items in package_node.items.iter_mut() {
        folder.fold_item_node(context, items);
    }
}
#[allow(unused_variables)]
pub fn walk_attribute_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    attribute_node: &'ast mut AttributeNode,
) where
    F::Context: Context,
{
    folder.fold_path_node(context, &mut attribute_node.path);
    for args in attribute_node.args.iter_mut() {
        folder.fold_attribute_argument_node(context, args);
    }
}
#[allow(unused_variables)]
pub fn walk_attribute_argument_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    attribute_argument_node: &'ast mut AttributeArgumentNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut attribute_argument_node.ident);
    if let Some(ref mut value) = attribute_argument_node.value {
        folder.fold_expression_node(context, value);
    }
}
#[allow(unused_variables)]
pub fn walk_path_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    path_node: &'ast mut PathNode,
) where
    F::Context: Context,
{
    for segments in path_node.segments.iter_mut() {
        folder.fold_ident_node(context, segments);
    }
}
#[allow(unused_variables)]
pub fn walk_ident_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    ident_node: &'ast mut IdentNode,
) where
    F::Context: Context,
{
}
#[allow(unused_variables)]
pub fn walk_item_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    item_node: &'ast mut ItemNode,
) where
    F::Context: Context,
{
    for attributes in item_node.attributes.iter_mut() {
        folder.fold_attribute_node(context, attributes);
    }
    folder.fold_visibility_kind(context, &mut item_node.visibility);
    folder.fold_item_kind(context, &mut item_node.kind);
}
#[allow(unused_variables)]
fn walk_item_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    item_kind: &'ast mut ItemKind,
) where
    F::Context: Context,
{
    match item_kind {
        ItemKind::Mod(node) => {
            folder.fold_mod_node(context, node);
        }
        ItemKind::Use(node) => {
            folder.fold_use_node(context, node);
        }
        ItemKind::Enum(node) => {
            folder.fold_enum_node(context, node);
        }
        ItemKind::Struct(node) => {
            folder.fold_struct_node(context, node);
        }
        ItemKind::Function(node) => {
            folder.fold_function_node(context, node);
        }
        ItemKind::TypeAlias(node) => {
            folder.fold_type_alias_node(context, node);
        }
        ItemKind::Trait(node) => {
            folder.fold_trait_node(context, node);
        }
        ItemKind::Constant(node) => {
            folder.fold_constant_node(context, node);
        }
        ItemKind::Implement(node) => {
            folder.fold_implement_node(context, node);
        }
    }
}
#[allow(unused_variables)]
pub fn walk_mod_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    mod_node: &'ast mut ModNode,
) where
    F::Context: Context,
{
    for attributes in mod_node.attributes.iter_mut() {
        folder.fold_attribute_node(context, attributes);
    }
    folder.fold_ident_node(context, &mut mod_node.ident);
    for items in mod_node.items.iter_mut() {
        folder.fold_item_node(context, items);
    }
}
#[allow(unused_variables)]
pub fn walk_use_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    use_node: &'ast mut UseNode,
) where
    F::Context: Context,
{
    folder.fold_path_node(context, &mut use_node.path);
}
#[allow(unused_variables)]
fn walk_visibility_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    visibility_kind: &'ast mut VisibilityKind,
) where
    F::Context: Context,
{
    match visibility_kind {
        VisibilityKind::Current => {}
        VisibilityKind::Public => {}
        VisibilityKind::Restricted(node) => {
            folder.fold_path_node(context, node);
        }
    }
}
#[allow(unused_variables)]
pub fn walk_struct_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    struct_node: &'ast mut StructNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut struct_node.ident);
    for generics in struct_node.generics.iter_mut() {
        folder.fold_generic_node(context, generics);
    }
    if let Some(ref mut fields) = struct_node.fields {
        folder.fold_struct_field_kind(context, fields);
    }
}
#[allow(unused_variables)]
pub fn walk_generic_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    generic_node: &'ast mut GenericNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut generic_node.ident);
    for traits in generic_node.traits.iter_mut() {
        folder.fold_path_node(context, traits);
    }
    if let Some(ref mut default) = generic_node.default {
        folder.fold_path_node(context, default);
    }
}
#[allow(unused_variables)]
fn walk_struct_field_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    struct_field_kind: &'ast mut StructFieldKind,
) where
    F::Context: Context,
{
    match struct_field_kind {
        StructFieldKind::Unnamed(nodes) => {
            for node in nodes.iter_mut() {
                folder.fold_unnamed_struct_field(context, node);
            }
        }
        StructFieldKind::Named(nodes) => {
            for node in nodes.iter_mut() {
                folder.fold_named_struct_field(context, node);
            }
        }
    }
}
#[allow(unused_variables)]
pub fn walk_unnamed_struct_field<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    unnamed_struct_field: &'ast mut UnnamedStructField,
) where
    F::Context: Context,
{
    folder.fold_visibility_kind(context, &mut unnamed_struct_field.visibility);
    folder.fold_type_node(context, &mut unnamed_struct_field.ty);
}
#[allow(unused_variables)]
pub fn walk_named_struct_field<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    named_struct_field: &'ast mut NamedStructField,
) where
    F::Context: Context,
{
    folder.fold_visibility_kind(context, &mut named_struct_field.visibility);
    folder.fold_ident_node(context, &mut named_struct_field.ident);
    folder.fold_type_node(context, &mut named_struct_field.ty);
}
#[allow(unused_variables)]
fn walk_immutability_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    immutability_kind: &'ast mut ImmutabilityKind,
) where
    F::Context: Context,
{
    match immutability_kind {
        ImmutabilityKind::Nope => {}
        ImmutabilityKind::Yes => {}
    }
}
#[allow(unused_variables)]
pub fn walk_type_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    type_node: &'ast mut TypeNode,
) where
    F::Context: Context,
{
    folder.fold_immutability_kind(context, &mut type_node.immutability);
    folder.fold_type_kind(context, &mut type_node.kind);
}
#[allow(unused_variables)]
fn walk_type_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    type_kind: &'ast mut TypeKind,
) where
    F::Context: Context,
{
    match type_kind {
        TypeKind::Tuple(nodes) => {
            for node in nodes.iter_mut() {
                folder.fold_type_kind(context, node);
            }
        }
        TypeKind::Path(node) => {
            folder.fold_path_node(context, node);
        }
        TypeKind::Generic(node) => {
            folder.fold_generic_type_node(context, node);
        }
    }
}
#[allow(unused_variables)]
pub fn walk_generic_type_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    generic_type_node: &'ast mut GenericTypeNode,
) where
    F::Context: Context,
{
    folder.fold_path_node(context, &mut generic_type_node.path);
    for parameters in generic_type_node.parameters.iter_mut() {
        folder.fold_type_kind(context, parameters);
    }
}
#[allow(unused_variables)]
pub fn walk_enum_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    enum_node: &'ast mut EnumNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut enum_node.ident);
    for generics in enum_node.generics.iter_mut() {
        folder.fold_generic_node(context, generics);
    }
    for variants in enum_node.variants.iter_mut() {
        folder.fold_enum_variant_node(context, variants);
    }
}
#[allow(unused_variables)]
pub fn walk_enum_variant_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    enum_variant_node: &'ast mut EnumVariantNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut enum_variant_node.ident);
    if let Some(ref mut kind) = enum_variant_node.kind {
        folder.fold_enum_variant_kind(context, kind);
    }
}
#[allow(unused_variables)]
fn walk_enum_variant_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    enum_variant_kind: &'ast mut EnumVariantKind,
) where
    F::Context: Context,
{
    match enum_variant_kind {
        EnumVariantKind::Unnamed(nodes) => {
            for node in nodes.iter_mut() {
                folder.fold_type_node(context, node);
            }
        }
        EnumVariantKind::Named(nodes) => {
            for node in nodes.iter_mut() {
                folder.fold_enum_named_variant_node(context, node);
            }
        }
    }
}
#[allow(unused_variables)]
pub fn walk_enum_named_variant_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    enum_named_variant_node: &'ast mut EnumNamedVariantNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut enum_named_variant_node.ident);
    folder.fold_type_node(context, &mut enum_named_variant_node.ty);
}
#[allow(unused_variables)]
pub fn walk_function_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    function_node: &'ast mut FunctionNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut function_node.ident);
    for generics in function_node.generics.iter_mut() {
        folder.fold_generic_node(context, generics);
    }
    if let Some(ref mut self_type) = function_node.self_type {
        folder.fold_immutability_kind(context, self_type);
    }
    for parameters in function_node.parameters.iter_mut() {
        folder.fold_function_parameter_node(context, parameters);
    }
    if let Some(ref mut return_type) = function_node.return_type {
        folder.fold_type_node(context, return_type);
    }
    if let Some(ref mut block) = function_node.block {
        folder.fold_block_node(context, block);
    }
}
#[allow(unused_variables)]
pub fn walk_function_parameter_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    function_parameter_node: &'ast mut FunctionParameterNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut function_parameter_node.argument_label);
    if let Some(ref mut parameter_label) = function_parameter_node.parameter_label {
        folder.fold_ident_node(context, parameter_label);
    }
    folder.fold_type_node(context, &mut function_parameter_node.ty);
}
#[allow(unused_variables)]
pub fn walk_block_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    block_node: &'ast mut BlockNode,
) where
    F::Context: Context,
{
    for statements in block_node.statements.iter_mut() {
        folder.fold_statement_node(context, statements);
    }
}
#[allow(unused_variables)]
pub fn walk_statement_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    statement_node: &'ast mut StatementNode,
) where
    F::Context: Context,
{
    folder.fold_statement_kind(context, &mut statement_node.kind);
}
#[allow(unused_variables)]
fn walk_statement_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    statement_kind: &'ast mut StatementKind,
) where
    F::Context: Context,
{
    match statement_kind {
        StatementKind::Semicolon => {}
        StatementKind::Break => {}
        StatementKind::Continue => {}
        StatementKind::Return(node) => {
            if let Some(node) = node {
                folder.fold_expression_node(context, node);
            }
        }
        StatementKind::Item(node) => {
            folder.fold_item_node(context, node);
        }
        StatementKind::Let(node) => {
            folder.fold_let_node(context, node);
        }
        StatementKind::Assign(node) => {
            folder.fold_assign_node(context, node);
        }
        StatementKind::Expression(node) => {
            folder.fold_expression_node(context, node);
        }
    }
}
#[allow(unused_variables)]
pub fn walk_assign_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    assign_node: &'ast mut AssignNode,
) where
    F::Context: Context,
{
    folder.fold_assign_kind(context, &mut assign_node.kind);
    folder.fold_expression_node(context, &mut assign_node.lhs);
    folder.fold_expression_node(context, &mut assign_node.rhs);
}
#[allow(unused_variables)]
fn walk_assign_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    assign_kind: &'ast mut AssignKind,
) where
    F::Context: Context,
{
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
pub fn walk_let_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    let_node: &'ast mut LetNode,
) where
    F::Context: Context,
{
    folder.fold_immutability_kind(context, &mut let_node.immutability);
    folder.fold_pattern_node(context, &mut let_node.pattern);
    if let Some(ref mut ty) = let_node.ty {
        folder.fold_type_node(context, ty);
    }
    if let Some(ref mut value) = let_node.value {
        folder.fold_expression_node(context, value);
    }
}
#[allow(unused_variables)]
pub fn walk_pattern_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    pattern_node: &'ast mut PatternNode,
) where
    F::Context: Context,
{
    folder.fold_pattern_kind(context, &mut pattern_node.kind);
}
#[allow(unused_variables)]
fn walk_pattern_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    pattern_kind: &'ast mut PatternKind,
) where
    F::Context: Context,
{
    match pattern_kind {
        PatternKind::Wildcard => {}
        PatternKind::Rest => {}
        PatternKind::Literal(node) => {
            folder.fold_literal_node(context, node);
        }
        PatternKind::Path(node) => {
            folder.fold_path_node(context, node);
        }
        PatternKind::NamedStruct(node) => {
            folder.fold_pattern_named_struct_node(context, node);
        }
        PatternKind::UnnamedStruct(node) => {
            folder.fold_pattern_unnamed_struct_node(context, node);
        }
        PatternKind::Slice(nodes) => {
            for node in nodes.iter_mut() {
                folder.fold_pattern_node(context, node);
            }
        }
    }
}
#[allow(unused_variables)]
pub fn walk_literal_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    literal_node: &'ast mut LiteralNode,
) where
    F::Context: Context,
{
}
#[allow(unused_variables)]
pub fn walk_pattern_named_struct_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    pattern_named_struct_node: &'ast mut PatternNamedStructNode,
) where
    F::Context: Context,
{
    folder.fold_path_node(context, &mut pattern_named_struct_node.path);
    for fields in pattern_named_struct_node.fields.iter_mut() {
        folder.fold_pattern_named_struct_field_node(context, fields);
    }
}
#[allow(unused_variables)]
pub fn walk_pattern_named_struct_field_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    pattern_named_struct_field_node: &'ast mut PatternNamedStructFieldNode,
) where
    F::Context: Context,
{
    folder.fold_path_node(context, &mut pattern_named_struct_field_node.path);
    if let Some(ref mut pattern) = pattern_named_struct_field_node.pattern {
        folder.fold_pattern_node(context, pattern);
    }
}
#[allow(unused_variables)]
pub fn walk_pattern_unnamed_struct_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    pattern_unnamed_struct_node: &'ast mut PatternUnnamedStructNode,
) where
    F::Context: Context,
{
    if let Some(ref mut path) = pattern_unnamed_struct_node.path {
        folder.fold_path_node(context, path);
    }
    for fields in pattern_unnamed_struct_node.fields.iter_mut() {
        folder.fold_pattern_node(context, fields);
    }
}
#[allow(unused_variables)]
pub fn walk_type_alias_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    type_alias_node: &'ast mut TypeAliasNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut type_alias_node.ident);
    if let Some(ref mut ty) = type_alias_node.ty {
        folder.fold_type_node(context, ty);
    }
}
#[allow(unused_variables)]
pub fn walk_expression_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    expression_node: &'ast mut ExpressionNode,
) where
    F::Context: Context,
{
    folder.fold_expression_kind(context, &mut expression_node.kind);
}
#[allow(unused_variables)]
fn walk_expression_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    expression_kind: &'ast mut ExpressionKind,
) where
    F::Context: Context,
{
    match expression_kind {
        ExpressionKind::Let(node) => {
            folder.fold_let_expression_node(context, node);
        }
        ExpressionKind::Negate(node) => {
            folder.fold_expression_node(context, node);
        }
        ExpressionKind::Not(node) => {
            folder.fold_expression_node(context, node);
        }
        ExpressionKind::Literal(node) => {
            folder.fold_literal_node(context, node);
        }
        ExpressionKind::Conditional(node) => {
            folder.fold_condition_node(context, node);
        }
        ExpressionKind::Loop(node) => {
            folder.fold_loop_node(context, node);
        }
        ExpressionKind::While(node) => {
            folder.fold_while_node(context, node);
        }
        ExpressionKind::For(node) => {
            folder.fold_for_node(context, node);
        }
        ExpressionKind::Match(node) => {
            folder.fold_match_node(context, node);
        }
        ExpressionKind::Path(node) => {
            folder.fold_path_node(context, node);
        }
        ExpressionKind::FunctionCall(node) => {
            folder.fold_function_call_node(context, node);
        }
        ExpressionKind::Closure(node) => {
            folder.fold_closure_node(context, node);
        }
        ExpressionKind::Block(node) => {
            folder.fold_block_node(context, node);
        }
        ExpressionKind::Tuple(node) => {
            folder.fold_tuple_node(context, node);
        }
        ExpressionKind::Array(nodes) => {
            for node in nodes.iter_mut() {
                folder.fold_expression_node(context, node);
            }
        }
        ExpressionKind::Try(node) => {
            folder.fold_expression_node(context, node);
        }
        ExpressionKind::Await(node) => {
            folder.fold_expression_node(context, node);
        }
        ExpressionKind::Field(node) => {
            folder.fold_field_node(context, node);
        }
        ExpressionKind::Index(node) => {
            folder.fold_index_node(context, node);
        }
        ExpressionKind::MethodCall(node) => {
            folder.fold_method_call_node(context, node);
        }
        ExpressionKind::Binary(node) => {
            folder.fold_binary_expression_node(context, node);
        }
    }
}
#[allow(unused_variables)]
pub fn walk_let_expression_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    let_expression_node: &'ast mut LetExpressionNode,
) where
    F::Context: Context,
{
    folder.fold_pattern_node(context, &mut let_expression_node.pattern);
    folder.fold_expression_node(context, &mut let_expression_node.value);
}
#[allow(unused_variables)]
pub fn walk_condition_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    condition_node: &'ast mut ConditionNode,
) where
    F::Context: Context,
{
    for branches in condition_node.branches.iter_mut() {
        folder.fold_condition_branch(context, branches);
    }
    if let Some(ref mut other) = condition_node.other {
        folder.fold_block_node(context, other);
    }
}
#[allow(unused_variables)]
pub fn walk_condition_branch<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    condition_branch: &'ast mut ConditionBranch,
) where
    F::Context: Context,
{
    folder.fold_expression_node(context, &mut condition_branch.expression);
    folder.fold_block_node(context, &mut condition_branch.block);
}
#[allow(unused_variables)]
pub fn walk_loop_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    loop_node: &'ast mut LoopNode,
) where
    F::Context: Context,
{
    folder.fold_block_node(context, &mut loop_node.block);
}
#[allow(unused_variables)]
pub fn walk_while_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    while_node: &'ast mut WhileNode,
) where
    F::Context: Context,
{
    folder.fold_condition_branch(context, &mut while_node.branch);
}
#[allow(unused_variables)]
pub fn walk_for_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    for_node: &'ast mut ForNode,
) where
    F::Context: Context,
{
    folder.fold_pattern_node(context, &mut for_node.pattern);
    folder.fold_expression_node(context, &mut for_node.iter);
    folder.fold_block_node(context, &mut for_node.block);
}
#[allow(unused_variables)]
pub fn walk_match_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    match_node: &'ast mut MatchNode,
) where
    F::Context: Context,
{
    folder.fold_expression_node(context, &mut match_node.expression);
    for branches in match_node.branches.iter_mut() {
        folder.fold_match_branch(context, branches);
    }
}
#[allow(unused_variables)]
pub fn walk_match_branch<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    match_branch: &'ast mut MatchBranch,
) where
    F::Context: Context,
{
    folder.fold_pattern_node(context, &mut match_branch.pattern);
    folder.fold_block_node(context, &mut match_branch.block);
}
#[allow(unused_variables)]
pub fn walk_closure_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    closure_node: &'ast mut ClosureNode,
) where
    F::Context: Context,
{
    for parameters in closure_node.parameters.iter_mut() {
        folder.fold_closure_parameter_node(context, parameters);
    }
    if let Some(ref mut return_type) = closure_node.return_type {
        folder.fold_type_node(context, return_type);
    }
    folder.fold_block_node(context, &mut closure_node.block);
}
#[allow(unused_variables)]
pub fn walk_closure_parameter_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    closure_parameter_node: &'ast mut ClosureParameterNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut closure_parameter_node.ident);
    if let Some(ref mut ty) = closure_parameter_node.ty {
        folder.fold_type_node(context, ty);
    }
}
#[allow(unused_variables)]
pub fn walk_tuple_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    tuple_node: &'ast mut TupleNode,
) where
    F::Context: Context,
{
    for arguments in tuple_node.arguments.iter_mut() {
        folder.fold_expression_node(context, arguments);
    }
}
#[allow(unused_variables)]
pub fn walk_index_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    index_node: &'ast mut IndexNode,
) where
    F::Context: Context,
{
    folder.fold_expression_node(context, &mut index_node.expression);
    folder.fold_expression_node(context, &mut index_node.index);
}
#[allow(unused_variables)]
pub fn walk_binary_expression_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    binary_expression_node: &'ast mut BinaryExpressionNode,
) where
    F::Context: Context,
{
    folder.fold_binary_operator_kind(context, &mut binary_expression_node.kind);
    folder.fold_expression_node(context, &mut binary_expression_node.lhs);
    folder.fold_expression_node(context, &mut binary_expression_node.rhs);
}
#[allow(unused_variables)]
pub fn walk_field_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    field_node: &'ast mut FieldNode,
) where
    F::Context: Context,
{
    folder.fold_expression_node(context, &mut field_node.expression);
    folder.fold_ident_node(context, &mut field_node.field);
}
#[allow(unused_variables)]
pub fn walk_function_call_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    function_call_node: &'ast mut FunctionCallNode,
) where
    F::Context: Context,
{
    folder.fold_expression_node(context, &mut function_call_node.expression);
    for arguments in function_call_node.arguments.iter_mut() {
        folder.fold_argument_node(context, arguments);
    }
}
#[allow(unused_variables)]
pub fn walk_method_call_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    method_call_node: &'ast mut MethodCallNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut method_call_node.ident);
    for arguments in method_call_node.arguments.iter_mut() {
        folder.fold_argument_node(context, arguments);
    }
}
#[allow(unused_variables)]
pub fn walk_argument_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    argument_node: &'ast mut ArgumentNode,
) where
    F::Context: Context,
{
    if let Some(ref mut ident) = argument_node.ident {
        folder.fold_ident_node(context, ident);
    }
    folder.fold_expression_node(context, &mut argument_node.expression);
}
#[allow(unused_variables)]
fn walk_binary_operator_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    binary_operator_kind: &'ast mut BinaryOperatorKind,
) where
    F::Context: Context,
{
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
pub fn walk_trait_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    trait_node: &'ast mut TraitNode,
) where
    F::Context: Context,
{
    folder.fold_ident_node(context, &mut trait_node.ident);
    for generics in trait_node.generics.iter_mut() {
        folder.fold_generic_node(context, generics);
    }
    for inheritances in trait_node.inheritances.iter_mut() {
        folder.fold_path_node(context, inheritances);
    }
    for items in trait_node.items.iter_mut() {
        folder.fold_implement_item_node(context, items);
    }
}
#[allow(unused_variables)]
pub fn walk_constant_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    constant_node: &'ast mut ConstantNode,
) where
    F::Context: Context,
{
    folder.fold_pattern_node(context, &mut constant_node.pattern);
    folder.fold_type_node(context, &mut constant_node.ty);
    if let Some(ref mut expression) = constant_node.expression {
        folder.fold_expression_node(context, expression);
    }
}
#[allow(unused_variables)]
pub fn walk_implement_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    implement_node: &'ast mut ImplementNode,
) where
    F::Context: Context,
{
    for generics in implement_node.generics.iter_mut() {
        folder.fold_generic_node(context, generics);
    }
    if let Some(ref mut trait_ident) = implement_node.trait_ident {
        folder.fold_path_node(context, trait_ident);
    }
    folder.fold_path_node(context, &mut implement_node.target);
    for target_generics in implement_node.target_generics.iter_mut() {
        folder.fold_generic_node(context, target_generics);
    }
    for items in implement_node.items.iter_mut() {
        folder.fold_implement_item_node(context, items);
    }
}
#[allow(unused_variables)]
pub fn walk_implement_item_node<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    implement_item_node: &'ast mut ImplementItemNode,
) where
    F::Context: Context,
{
    for attributes in implement_item_node.attributes.iter_mut() {
        folder.fold_attribute_node(context, attributes);
    }
    folder.fold_implement_item_kind(context, &mut implement_item_node.kind);
}
#[allow(unused_variables)]
fn walk_implement_item_kind<'ast, F: Fold<'ast>>(
    folder: &mut F,
    context: &F::Context,
    implement_item_kind: &'ast mut ImplementItemKind,
) where
    F::Context: Context,
{
    match implement_item_kind {
        ImplementItemKind::Type(node) => {
            folder.fold_type_alias_node(context, node);
        }
        ImplementItemKind::Constant(node) => {
            folder.fold_constant_node(context, node);
        }
        ImplementItemKind::Function(node) => {
            folder.fold_function_node(context, node);
        }
    }
}
