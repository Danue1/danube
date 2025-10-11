use danubec_diagnostic::Diagnostic;
use danubec_syntax::{AstNode, Span};
use std::collections::HashMap;

macro_rules! lower {
    ($node:ident . $field:ident() . $lower:ident ( $diagnostic:ident )) => {{
        let node = $node.$field().ok_or_else(|| {
            $diagnostic.report(miette!(concat!("Expected ", stringify!($field))));
        })?;
        $lower(node, $diagnostic)?
    }};
}

macro_rules! lower_opt {
    ($node:ident . $field:ident() . $lower:ident ( $diagnostic:ident )) => {{
        if let Some(node) = $node.$field() {
            Some($lower(node, $diagnostic)?)
        } else {
            None
        }
    }};
}

macro_rules! many {
    ($node:ident . $field:ident() . $lower:ident ( $diagnostic:ident )) => {{
        let mut items = vec![];
        for item in $node.$field() {
            match $lower(item, $diagnostic) {
                Ok(item) => items.push(item),
                Err(_) => continue,
            }
        }
        items
    }};
}

pub fn lower_krate(
    krate: danubec_syntax::Krate,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Krate, ()> {
    let attributes = many!(krate.attributes().lower_top_level_attribute(diagnostic));
    let definitions = many!(krate.definitions().lower_definition(diagnostic));

    Ok(danubec_ast::Krate {
        attributes,
        definitions,
        children: HashMap::new(),
    })
}

pub fn lower_top_level_attribute(
    attribute: danubec_syntax::TopLevelAttribute,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::TopLevelAttribute, ()> {
    let argument = lower!(attribute.argument().lower_attribute_argument(diagnostic));

    Ok(danubec_ast::TopLevelAttribute {
        argument,
        span: Span::new(attribute.syntax()),
    })
}

pub fn lower_attribute(
    attribute: danubec_syntax::Attribute,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Attribute, ()> {
    let argument = lower!(attribute.argument().lower_attribute_argument(diagnostic));

    Ok(danubec_ast::Attribute {
        argument,
        span: Span::new(attribute.syntax()),
    })
}

pub fn lower_attribute_argument(
    argument: danubec_syntax::AttributeArgument,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::AttributeArgument, ()> {
    let kind = match argument {
        danubec_syntax::AttributeArgument::Expression(expr) => {
            let value = lower_expression_attribute_argument(expr, diagnostic)?;

            danubec_ast::AttributeArgumentKind::Expression { value }
        }
        danubec_syntax::AttributeArgument::KeyValue(kv) => {
            let (key, value) = lower_key_value_attribute_argument(kv, diagnostic)?;

            danubec_ast::AttributeArgumentKind::KeyValue { key, value }
        }
        danubec_syntax::AttributeArgument::Nested(nested) => {
            let (path, arguments) = lower_nested_attribute_argument(nested, diagnostic)?;

            danubec_ast::AttributeArgumentKind::Nested { path, arguments }
        }
    };

    Ok(danubec_ast::AttributeArgument { kind })
}

pub fn lower_expression_attribute_argument(
    expr: danubec_syntax::ExpressionAttributeArgument,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Expression, ()> {
    let expr = lower!(expr.value().lower_expression(diagnostic));

    Ok(expr)
}

pub fn lower_key_value_attribute_argument(
    kv: danubec_syntax::KeyValueAttributeArgument,
    diagnostic: &mut Diagnostic,
) -> Result<(danubec_ast::Path, Option<danubec_ast::Expression>), ()> {
    let key = lower!(kv.key().lower_path(diagnostic));
    let value = lower_opt!(kv.value().lower_expression(diagnostic));

    Ok((key, value))
}

pub fn lower_nested_attribute_argument(
    nested: danubec_syntax::NestedAttributeArgument,
    diagnostic: &mut Diagnostic,
) -> Result<(danubec_ast::Path, Vec<danubec_ast::AttributeArgument>), ()> {
    let path = lower!(nested.path().lower_path(diagnostic));
    let arguments = many!(nested.arguments().lower_attribute_argument(diagnostic));

    Ok((path, arguments))
}

pub fn lower_definition(
    definition: danubec_syntax::Definition,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Definition, ()> {
    let attributes = many!(definition.attributes().lower_attribute(diagnostic));

    let visibility = lower_opt!(definition.visibility().lower_visibility(diagnostic));
    let visibility = visibility.unwrap_or(danubec_ast::Visibility::Private);

    let kind = lower!(definition.kind().lower_definition_kind(diagnostic));

    Ok(danubec_ast::Definition {
        attributes,
        visibility,
        kind,
        span: Span::new(definition.syntax()),
    })
}

pub fn lower_definition_kind(
    kind: danubec_syntax::DefinitionKind,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::DefinitionKind, ()> {
    match kind {
        danubec_syntax::DefinitionKind::Function(fn_) => {
            let name = lower!(fn_.name().lower_identifier(diagnostic));

            let type_parameters = many!(fn_.type_parameters().lower_type_parameter(diagnostic));

            let parameters = many!(fn_.parameters().lower_function_parameter(diagnostic));

            let return_type = lower_opt!(fn_.return_type().lower_type_expression(diagnostic));

            let type_bounds = many!(fn_.type_bounds().lower_type_bound(diagnostic));

            let body = lower!(fn_.body().lower_function_body_kind(diagnostic));

            Ok(danubec_ast::DefinitionKind::Function {
                name,
                type_parameters,
                parameters,
                return_type,
                type_bounds,
                body,
            })
        }
        danubec_syntax::DefinitionKind::Struct(struct_) => {
            let name = lower!(struct_.name().lower_identifier(diagnostic));

            let type_parameters = many!(struct_.type_parameters().lower_type_parameter(diagnostic));

            let type_bounds = many!(struct_.type_bounds().lower_type_bound(diagnostic));

            let body = lower!(struct_.body().lower_struct_body(diagnostic));

            Ok(danubec_ast::DefinitionKind::Struct {
                name,
                type_parameters,
                type_bounds,
                body,
            })
        }
        danubec_syntax::DefinitionKind::Enum(enm) => {
            let name = lower!(enm.name().lower_identifier(diagnostic));

            let type_parameters = many!(enm.type_parameters().lower_type_parameter(diagnostic));

            let type_bounds = many!(enm.type_bounds().lower_type_bound(diagnostic));

            let variants = many!(enm.variants().lower_enum_variant(diagnostic));

            Ok(danubec_ast::DefinitionKind::Enum {
                name,
                type_parameters,
                type_bounds,
                variants,
            })
        }
        danubec_syntax::DefinitionKind::Use(use_) => {
            let tree = lower!(use_.tree().lower_use_tree(diagnostic));

            Ok(danubec_ast::DefinitionKind::Use { tree })
        }
        danubec_syntax::DefinitionKind::Module(module) => {
            let name = lower!(module.name().lower_identifier(diagnostic));

            let kind = lower!(module.kind().lower_module_definition_kind(diagnostic));

            Ok(danubec_ast::DefinitionKind::Module { name, kind })
        }
        danubec_syntax::DefinitionKind::Trait(trait_) => {
            let name = lower!(trait_.name().lower_identifier(diagnostic));

            let type_parameters = many!(trait_.type_parameters().lower_type_parameter(diagnostic));

            let type_bounds = many!(trait_.type_bounds().lower_type_bound(diagnostic));

            let definitions = many!(trait_.definitions().lower_associated_definition(diagnostic));

            Ok(danubec_ast::DefinitionKind::Trait {
                name,
                type_parameters,
                type_bounds,
                definitions,
            })
        }
        danubec_syntax::DefinitionKind::Constant(constant) => {
            let name = lower!(constant.name().lower_identifier(diagnostic));

            let r#type = lower!(constant.r#type().lower_type_expression(diagnostic));

            let initializer = lower!(constant.initializer().lower_expression(diagnostic));

            Ok(danubec_ast::DefinitionKind::Constant {
                name,
                r#type,
                initializer,
            })
        }
        danubec_syntax::DefinitionKind::Static(static_) => {
            let name = lower!(static_.name().lower_identifier(diagnostic));

            let r#type = lower!(static_.r#type().lower_type_expression(diagnostic));

            let initializer = lower!(static_.initializer().lower_expression(diagnostic));

            Ok(danubec_ast::DefinitionKind::Static {
                name,
                r#type,
                initializer,
            })
        }
        danubec_syntax::DefinitionKind::Type(type_) => {
            let name = lower!(type_.name().lower_identifier(diagnostic));

            let type_parameters = many!(type_.type_parameters().lower_type_parameter(diagnostic));

            let type_bounds = many!(type_.type_bounds().lower_type_bound(diagnostic));

            let initializer = lower_opt!(type_.initializer().lower_type_expression(diagnostic));

            Ok(danubec_ast::DefinitionKind::Type {
                name,
                type_parameters,
                type_bounds,
                initializer,
            })
        }
        danubec_syntax::DefinitionKind::Implement(implement) => {
            let type_parameters =
                many!(implement.type_parameters().lower_type_parameter(diagnostic));

            let mut types = implement
                .syntax()
                .children()
                .filter_map(danubec_syntax::TypeExpression::cast);
            let (trait_type, target_type) = match implement.r#for() {
                Some(_) => {
                    let trait_type = types.next().ok_or_else(|| {
                        diagnostic.report(miette!("Expected trait type in implement"));
                    })?;
                    let trait_type = lower_type_expression(trait_type, diagnostic)?;

                    let target_type = types.next().ok_or_else(|| {
                        diagnostic.report(miette!("Expected target type in implement"));
                    })?;
                    let target_type = lower_type_expression(target_type, diagnostic)?;

                    (Some(trait_type), target_type)
                }
                None => {
                    let target_type = types.next().ok_or_else(|| {
                        diagnostic.report(miette!("Expected target type in implement"));
                    })?;
                    let target_type = lower_type_expression(target_type, diagnostic)?;

                    (None, target_type)
                }
            };

            let type_bounds = many!(implement.type_bounds().lower_type_bound(diagnostic));

            let definitions = many!(
                implement
                    .definitions()
                    .lower_associated_definition(diagnostic)
            );

            Ok(danubec_ast::DefinitionKind::Implement {
                type_parameters,
                trait_type,
                target_type,
                type_bounds,
                definitions,
            })
        }
    }
}

pub fn lower_associated_definition(
    definition: danubec_syntax::AssociatedDefinition,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::AssociatedDefinition, ()> {
    let attributes = many!(definition.attributes().lower_attribute(diagnostic));

    let visibility = lower_opt!(definition.visibility().lower_visibility(diagnostic));
    let visibility = visibility.unwrap_or(danubec_ast::Visibility::Private);

    let kind = lower!(
        definition
            .kind()
            .lower_associated_definition_kind(diagnostic)
    );

    Ok(danubec_ast::AssociatedDefinition {
        attributes,
        visibility,
        kind,
        span: Span::new(definition.syntax()),
    })
}

pub fn lower_associated_definition_kind(
    kind: danubec_syntax::AssociatedDefinitionKind,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::AssociatedDefinitionKind, ()> {
    match kind {
        danubec_syntax::AssociatedDefinitionKind::Function(fn_) => {
            let name = lower!(fn_.name().lower_identifier(diagnostic));

            let type_parameters = many!(fn_.type_parameters().lower_type_parameter(diagnostic));

            let parameters = many!(fn_.parameters().lower_function_parameter(diagnostic));

            let return_type = lower_opt!(fn_.return_type().lower_type_expression(diagnostic));

            let type_bounds = many!(fn_.type_bounds().lower_type_bound(diagnostic));

            let body = lower!(fn_.body().lower_function_body_kind(diagnostic));

            Ok(danubec_ast::AssociatedDefinitionKind::Function {
                name,
                type_parameters,
                parameters,
                return_type,
                type_bounds,
                body,
            })
        }
        danubec_syntax::AssociatedDefinitionKind::Constant(constant) => {
            let name = lower!(constant.name().lower_identifier(diagnostic));

            let r#type = lower_opt!(constant.r#type().lower_type_expression(diagnostic));

            let initializer = lower_opt!(constant.initializer().lower_expression(diagnostic));

            Ok(danubec_ast::AssociatedDefinitionKind::Constant {
                name,
                r#type,
                initializer,
            })
        }
        danubec_syntax::AssociatedDefinitionKind::Type(type_) => {
            let name = lower!(type_.name().lower_identifier(diagnostic));

            let type_parameters = many!(type_.type_parameters().lower_type_parameter(diagnostic));

            let type_bounds = many!(type_.type_bounds().lower_type_bound(diagnostic));

            let initializer = lower_opt!(type_.initializer().lower_type_expression(diagnostic));

            Ok(danubec_ast::AssociatedDefinitionKind::Type {
                name,
                type_parameters,
                type_bounds,
                initializer,
            })
        }
    }
}

pub fn lower_function_body_kind(
    body: danubec_syntax::FunctionBodyKind,
    diagnostic: &mut Diagnostic,
) -> Result<Option<Vec<danubec_ast::Statement>>, ()> {
    match body {
        danubec_syntax::FunctionBodyKind::Block(block) => {
            let (_, statements) = lower!(block.body().lower_block_expression(diagnostic));

            Ok(Some(statements))
        }
        danubec_syntax::FunctionBodyKind::Unit(_) => Ok(None),
    }
}

pub fn lower_use_tree(
    tree: danubec_syntax::UseTree,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::UseTree, ()> {
    let root = tree.root().is_some();

    let kind = lower!(tree.kind().lower_use_tree_kind(diagnostic));

    Ok(danubec_ast::UseTree {
        root,
        kind,
        span: Span::new(tree.syntax()),
    })
}

pub fn lower_use_tree_kind(
    kind: danubec_syntax::UseTreeKind,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::UseTreeKind, ()> {
    match kind {
        danubec_syntax::UseTreeKind::Nested(nested) => {
            let trees = many!(nested.trees().lower_use_tree(diagnostic));

            Ok(danubec_ast::UseTreeKind::Nested { trees })
        }
        danubec_syntax::UseTreeKind::Glob(_) => Ok(danubec_ast::UseTreeKind::Glob),
        danubec_syntax::UseTreeKind::Element(element) => {
            let path = lower!(element.path().lower_path(diagnostic));

            let trailing = lower_opt!(element.trailing().lower_use_tree_trailing(diagnostic));
            let trailing = trailing.unwrap_or(danubec_ast::UseTreeTrailing::Identifier);

            Ok(danubec_ast::UseTreeKind::Element { path, trailing })
        }
    }
}

pub fn lower_use_tree_trailing(
    trailing: danubec_syntax::UseTreeTrailing,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::UseTreeTrailing, ()> {
    match trailing {
        danubec_syntax::UseTreeTrailing::Nested(nested) => {
            let trees = many!(nested.trees().lower_use_tree(diagnostic));

            Ok(danubec_ast::UseTreeTrailing::Nested { trees })
        }
        danubec_syntax::UseTreeTrailing::Glob(_) => Ok(danubec_ast::UseTreeTrailing::Glob),
        danubec_syntax::UseTreeTrailing::Rename(rename) => {
            let name = lower!(rename.identifier().lower_identifier(diagnostic));

            Ok(danubec_ast::UseTreeTrailing::Rename { name })
        }
    }
}

pub fn lower_struct_body(
    body: danubec_syntax::StructBody,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::StructBody, ()> {
    match body {
        danubec_syntax::StructBody::Unit(_) => Ok(danubec_ast::StructBody::Unit),
        danubec_syntax::StructBody::Named(named) => {
            let fields = many!(named.fields().lower_struct_named_field(diagnostic));

            Ok(danubec_ast::StructBody::Named(fields))
        }
        danubec_syntax::StructBody::Unnamed(unnamed) => {
            let fields = many!(unnamed.fields().lower_struct_unnamed_field(diagnostic));

            Ok(danubec_ast::StructBody::Unnamed(fields))
        }
    }
}

pub fn lower_struct_named_field(
    field: danubec_syntax::StructNamedField,
    diagnostic: &mut Diagnostic,
) -> Result<
    (
        danubec_ast::Visibility,
        danubec_ast::Identifier,
        danubec_ast::TypeExpression,
    ),
    (),
> {
    let visibility = lower_opt!(field.visibility().lower_visibility(diagnostic));
    let visibility = visibility.unwrap_or(danubec_ast::Visibility::Private);

    let name = lower!(field.name().lower_identifier(diagnostic));

    let r#type = lower!(field.r#type().lower_type_expression(diagnostic));

    Ok((visibility, name, r#type))
}

pub fn lower_struct_unnamed_field(
    field: danubec_syntax::StructUnnamedField,
    diagnostic: &mut Diagnostic,
) -> Result<(danubec_ast::Visibility, danubec_ast::TypeExpression), ()> {
    let visibility = lower_opt!(field.visibility().lower_visibility(diagnostic));
    let visibility = visibility.unwrap_or(danubec_ast::Visibility::Private);

    let r#type = lower!(field.r#type().lower_type_expression(diagnostic));

    Ok((visibility, r#type))
}

pub fn lower_module_definition_kind(
    kind: danubec_syntax::ModuleDefinitionKind,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::ModuleDefinitionKind, ()> {
    match kind {
        danubec_syntax::ModuleDefinitionKind::Inline(inline) => {
            let definitions = many!(inline.definitions().lower_definition(diagnostic));

            Ok(danubec_ast::ModuleDefinitionKind::Inline { definitions })
        }
        danubec_syntax::ModuleDefinitionKind::External(_) => {
            Ok(danubec_ast::ModuleDefinitionKind::External)
        }
    }
}

pub fn lower_enum_variant(
    variant: danubec_syntax::EnumVariant,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::EnumVariant, ()> {
    let attributes = many!(variant.attributes().lower_attribute(diagnostic));

    let name = lower!(variant.name().lower_identifier(diagnostic));

    let kind = lower_opt!(variant.kind().lower_enum_variant_kind(diagnostic));
    let kind = kind.unwrap_or(danubec_ast::EnumVariantKind::Unit);

    Ok(danubec_ast::EnumVariant {
        attributes,
        name,
        kind,
        span: Span::new(variant.syntax()),
    })
}

pub fn lower_enum_variant_kind(
    kind: danubec_syntax::EnumVariantKind,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::EnumVariantKind, ()> {
    match kind {
        danubec_syntax::EnumVariantKind::Unit(_) => Ok(danubec_ast::EnumVariantKind::Unit),
        danubec_syntax::EnumVariantKind::Scalar(scalar) => {
            let value = lower!(scalar.value().lower_expression(diagnostic));

            Ok(danubec_ast::EnumVariantKind::Scalar(value))
        }
        danubec_syntax::EnumVariantKind::Named(named) => {
            let fields = many!(named.fields().lower_enum_variant_named_field(diagnostic));

            Ok(danubec_ast::EnumVariantKind::Named(fields))
        }
        danubec_syntax::EnumVariantKind::Unnamed(unnamed) => {
            let fields = many!(
                unnamed
                    .fields()
                    .lower_enum_variant_unnamed_field(diagnostic)
            );

            Ok(danubec_ast::EnumVariantKind::Unnamed(fields))
        }
    }
}

pub fn lower_enum_variant_named_field(
    field: danubec_syntax::EnumVariantNamedField,
    diagnostic: &mut Diagnostic,
) -> Result<
    (
        Vec<danubec_ast::Attribute>,
        danubec_ast::Identifier,
        danubec_ast::TypeExpression,
    ),
    (),
> {
    let attributes = many!(field.attributes().lower_attribute(diagnostic));

    let name = lower!(field.name().lower_identifier(diagnostic));

    let r#type = lower!(field.r#type().lower_type_expression(diagnostic));

    Ok((attributes, name, r#type))
}

pub fn lower_enum_variant_unnamed_field(
    field: danubec_syntax::EnumVariantUnnamedField,
    diagnostic: &mut Diagnostic,
) -> Result<(Vec<danubec_ast::Attribute>, danubec_ast::TypeExpression), ()> {
    let attributes = many!(field.attributes().lower_attribute(diagnostic));

    let r#type = lower!(field.r#type().lower_type_expression(diagnostic));

    Ok((attributes, r#type))
}

pub fn lower_visibility(
    visibility: danubec_syntax::Visibility,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Visibility, ()> {
    if visibility.krate().is_some() {
        return Ok(danubec_ast::Visibility::Krate);
    }
    if visibility.super_().is_some() {
        return Ok(danubec_ast::Visibility::Super);
    }
    if visibility.self_().is_some() {
        return Ok(danubec_ast::Visibility::Self_);
    }

    diagnostic.report(miette!("Unknown visibility"));
    Err(())
}

pub fn lower_type_parameter(
    parameter: danubec_syntax::TypeParameter,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::TypeParameter, ()> {
    let r#type = lower!(parameter.r#type().lower_type_expression(diagnostic));

    let constraints = many!(parameter.constraints().lower_constraint(diagnostic));

    Ok(danubec_ast::TypeParameter {
        r#type,
        constraints,
        span: Span::new(parameter.syntax()),
    })
}

pub fn lower_type_bound(
    bound: danubec_syntax::TypeBound,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::TypeBound, ()> {
    let r#type = lower!(bound.r#type().lower_type_expression(diagnostic));

    let constraints = many!(bound.constraints().lower_constraint(diagnostic));

    Ok(danubec_ast::TypeBound {
        r#type,
        constraints,
        span: Span::new(bound.syntax()),
    })
}

pub fn lower_function_parameter(
    parameter: danubec_syntax::FunctionParameter,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::FunctionParameter, ()> {
    let attributes = many!(parameter.attributes().lower_attribute(diagnostic));

    let pattern = lower!(parameter.pattern().lower_pattern(diagnostic));

    let r#type = lower!(parameter.r#type().lower_type_expression(diagnostic));

    Ok(danubec_ast::FunctionParameter {
        attributes,
        pattern,
        r#type,
        span: Span::new(parameter.syntax()),
    })
}

pub fn lower_constraint(
    constraint: danubec_syntax::TypeParameterConstraint,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::TypeExpression, ()> {
    let r#type = lower!(constraint.r#type().lower_type_expression(diagnostic));

    Ok(r#type)
}

pub fn lower_expression(
    expr: danubec_syntax::Expression,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Expression, ()> {
    let kind = match &expr {
        danubec_syntax::Expression::Break(_) => danubec_ast::ExpressionKind::Break,
        danubec_syntax::Expression::Continue(_) => danubec_ast::ExpressionKind::Continue,
        danubec_syntax::Expression::Return(expr) => {
            let value = lower_opt!(expr.expression().lower_expression(diagnostic));
            let value = value.map(Box::new);

            danubec_ast::ExpressionKind::Return { value }
        }
        danubec_syntax::Expression::For(expr) => {
            let pattern = lower!(expr.pattern().lower_pattern(diagnostic));

            let iterable = lower!(expr.iterable().lower_expression(diagnostic));
            let iterable = Box::new(iterable);

            let (_, body) = lower!(expr.body().lower_block_expression(diagnostic));

            danubec_ast::ExpressionKind::For {
                pattern,
                iterable,
                body,
            }
        }
        danubec_syntax::Expression::While(while_) => {
            let condition = lower!(while_.condition().lower_expression(diagnostic));
            let condition = Box::new(condition);

            let (_, body) = lower!(while_.body().lower_block_expression(diagnostic));

            danubec_ast::ExpressionKind::While { condition, body }
        }
        danubec_syntax::Expression::Loop(loop_) => {
            let (_, body) = lower!(loop_.body().lower_block_expression(diagnostic));

            danubec_ast::ExpressionKind::Loop { body }
        }
        danubec_syntax::Expression::If(if_) => {
            let condition = lower!(if_.condition().lower_expression(diagnostic));
            let condition = Box::new(condition);

            let (_, then_branch) = lower!(if_.then_branch().lower_block_expression(diagnostic));

            let else_branch = lower_opt!(if_.else_branch().lower_expression(diagnostic));
            let else_branch = else_branch.map(Box::new);

            danubec_ast::ExpressionKind::If {
                condition,
                then_branch,
                else_branch,
            }
        }
        danubec_syntax::Expression::Match(match_) => {
            let expression = lower!(match_.expression().lower_expression(diagnostic));
            let expression = Box::new(expression);

            let arms = many!(match_.arms().lower_match_arm(diagnostic));

            danubec_ast::ExpressionKind::Match { expression, arms }
        }
        danubec_syntax::Expression::Let(let_) => {
            let pattern = lower!(let_.pattern().lower_pattern(diagnostic));

            let r#type = lower_opt!(let_.r#type().lower_type_expression(diagnostic));

            let initializer = lower_opt!(let_.initializer().lower_expression(diagnostic));
            let initializer = initializer.map(Box::new);

            danubec_ast::ExpressionKind::Let {
                pattern,
                r#type,
                initializer,
            }
        }
        danubec_syntax::Expression::Array(array) => {
            let elements = many!(array.elements().lower_expression(diagnostic));

            danubec_ast::ExpressionKind::Array { elements }
        }
        danubec_syntax::Expression::Tuple(tuple) => {
            let elements = many!(tuple.elements().lower_expression(diagnostic));

            danubec_ast::ExpressionKind::Tuple { elements }
        }
        danubec_syntax::Expression::Block(block) => {
            let (attributes, statements) = lower_block_expression(block.clone(), diagnostic)?;

            danubec_ast::ExpressionKind::Block {
                attributes,
                statements,
            }
        }
        danubec_syntax::Expression::Literal(literal) => {
            let value = lower!(literal.literal().lower_literal(diagnostic));

            danubec_ast::ExpressionKind::Literal { value }
        }
        danubec_syntax::Expression::Path(path) => {
            let path = lower!(path.path().lower_path(diagnostic));

            danubec_ast::ExpressionKind::Path { path }
        }
        danubec_syntax::Expression::Unary(unary) => {
            let operator = lower!(unary.operator().lower_unary_operator(diagnostic));
            let operand = lower!(unary.operand().lower_expression(diagnostic));
            let operand = Box::new(operand);

            danubec_ast::ExpressionKind::Unary { operator, operand }
        }
        danubec_syntax::Expression::Binary(binary) => {
            let left = lower!(binary.left().lower_expression(diagnostic));
            let left = Box::new(left);

            let operator = lower!(binary.operator().lower_binary_operator(diagnostic));

            let right = lower!(binary.right().lower_expression(diagnostic));
            let right = Box::new(right);

            danubec_ast::ExpressionKind::Binary {
                left,
                operator,
                right,
            }
        }
        danubec_syntax::Expression::Assignment(assignment) => {
            let left = lower!(assignment.left().lower_expression(diagnostic));
            let left = Box::new(left);

            let operator = lower!(assignment.operator().lower_assignment_operator(diagnostic));

            let right = lower!(assignment.right().lower_expression(diagnostic));
            let right = Box::new(right);

            danubec_ast::ExpressionKind::Assignment {
                left,
                operator,
                right,
            }
        }
        danubec_syntax::Expression::FunctionCall(function_call) => {
            let callee = lower!(function_call.callee().lower_expression(diagnostic));
            let callee = Box::new(callee);

            let type_arguments = many!(
                function_call
                    .type_arguments()
                    .lower_type_argument(diagnostic)
            );

            let arguments = many!(function_call.arguments().lower_expression(diagnostic));

            danubec_ast::ExpressionKind::FunctionCall {
                callee,
                type_arguments,
                arguments,
            }
        }
        danubec_syntax::Expression::MethodCall(method_call) => {
            let receiver = lower!(method_call.receiver().lower_expression(diagnostic));
            let receiver = Box::new(receiver);

            let method = lower!(method_call.method().lower_identifier(diagnostic));

            let type_arguments =
                many!(method_call.type_arguments().lower_type_argument(diagnostic));

            let arguments = many!(method_call.arguments().lower_expression(diagnostic));

            danubec_ast::ExpressionKind::MethodCall {
                receiver,
                method,
                type_arguments,
                arguments,
            }
        }
        danubec_syntax::Expression::Field(field) => {
            let receiver = lower!(field.receiver().lower_expression(diagnostic));
            let receiver = Box::new(receiver);

            let field = lower!(field.field().lower_identifier(diagnostic));

            danubec_ast::ExpressionKind::Field { receiver, field }
        }
        danubec_syntax::Expression::Index(index) => {
            let collection = lower!(index.receiver().lower_expression(diagnostic));
            let collection = Box::new(collection);

            let index = lower!(index.index().lower_expression(diagnostic));
            let index = Box::new(index);

            danubec_ast::ExpressionKind::Index { collection, index }
        }
        danubec_syntax::Expression::Await(await_) => {
            let expression = lower!(await_.expression().lower_expression(diagnostic));
            let expression = Box::new(expression);

            danubec_ast::ExpressionKind::Await { expression }
        }
        danubec_syntax::Expression::Range(range) => {
            let range = match range {
                danubec_syntax::RangeExpression::FromTo(from_to) => {
                    let start = lower!(from_to.start().lower_expression(diagnostic));
                    let start = Box::new(start);

                    let end = lower!(from_to.end().lower_expression(diagnostic));
                    let end = Box::new(end);

                    danubec_ast::RangeExpression::FromTo { start, end }
                }
                danubec_syntax::RangeExpression::From(from_) => {
                    let start = lower!(from_.start().lower_expression(diagnostic));
                    let start = Box::new(start);

                    danubec_ast::RangeExpression::From { start }
                }
                danubec_syntax::RangeExpression::To(to) => {
                    let end = lower!(to.end().lower_expression(diagnostic));
                    let end = Box::new(end);

                    danubec_ast::RangeExpression::To { end }
                }
                danubec_syntax::RangeExpression::Full(_) => danubec_ast::RangeExpression::Full,
                danubec_syntax::RangeExpression::FromToInclusive(from_to) => {
                    let start = lower!(from_to.start().lower_expression(diagnostic));
                    let start = Box::new(start);

                    let end = lower!(from_to.end().lower_expression(diagnostic));
                    let end = Box::new(end);

                    danubec_ast::RangeExpression::FromToInclusive { start, end }
                }
                danubec_syntax::RangeExpression::ToInclusive(to) => {
                    let end = lower!(to.end().lower_expression(diagnostic));
                    let end = Box::new(end);

                    danubec_ast::RangeExpression::ToInclusive { end }
                }
            };

            danubec_ast::ExpressionKind::Range { range }
        }
        danubec_syntax::Expression::Struct(struct_expr) => {
            let path = lower!(struct_expr.path().lower_path_expression(diagnostic));

            let fields = many!(
                struct_expr
                    .fields()
                    .lower_struct_expression_field(diagnostic)
            );

            danubec_ast::ExpressionKind::Struct { path, fields }
        }
        danubec_syntax::Expression::Try(try_) => {
            let expression = lower!(try_.expression().lower_expression(diagnostic));
            let expression = Box::new(expression);

            danubec_ast::ExpressionKind::Try { expression }
        }
        danubec_syntax::Expression::Yield(yield_) => {
            let expression = lower_opt!(yield_.expression().lower_expression(diagnostic));
            let expression = expression.map(Box::new);

            danubec_ast::ExpressionKind::Yield { expression }
        }
    };

    Ok(danubec_ast::Expression {
        kind,
        span: Span::new(expr.syntax()),
    })
}

pub fn lower_literal_expression(
    literal: danubec_syntax::LiteralExpression,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Literal, ()> {
    let value = lower!(literal.literal().lower_literal(diagnostic));

    Ok(value)
}

pub fn lower_struct_expression_field(
    field: danubec_syntax::StructExpressionField,
    diagnostic: &mut Diagnostic,
) -> Result<(danubec_ast::Identifier, danubec_ast::Expression), ()> {
    let name = lower!(field.name().lower_identifier(diagnostic));
    let value = lower!(field.value().lower_expression(diagnostic));

    Ok((name, value))
}

pub fn lower_match_arm(
    arm: danubec_syntax::MatchArm,
    diagnostic: &mut Diagnostic,
) -> Result<(danubec_ast::Pattern, danubec_ast::Expression), ()> {
    let pattern = lower!(arm.pattern().lower_pattern(diagnostic));

    let expression = lower!(arm.expression().lower_expression(diagnostic));

    Ok((pattern, expression))
}

pub fn lower_block_expression(
    expr: danubec_syntax::BlockExpression,
    diagnostic: &mut Diagnostic,
) -> Result<(Vec<danubec_ast::Attribute>, Vec<danubec_ast::Statement>), ()> {
    let attributes = many!(expr.attributes().lower_attribute(diagnostic));

    let statements = many!(expr.statements().lower_statement(diagnostic));

    Ok((attributes, statements))
}

pub fn lower_statement(
    statement: danubec_syntax::Statement,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Statement, ()> {
    let kind = match &statement {
        danubec_syntax::Statement::Definition(definition) => {
            let definition = lower!(definition.definition().lower_definition(diagnostic));

            danubec_ast::StatementKind::Definition { definition }
        }
        danubec_syntax::Statement::Expression(expr) => {
            let value = lower!(expr.expression().lower_expression(diagnostic));

            danubec_ast::StatementKind::Expression { value }
        }
        danubec_syntax::Statement::Semicolon(_) => danubec_ast::StatementKind::Semicolon,
        danubec_syntax::Statement::Let(let_) => {
            let pattern = lower!(let_.pattern().lower_pattern(diagnostic));

            let r#type = lower_opt!(let_.r#type().lower_type_expression(diagnostic));

            let initializer = lower_opt!(let_.initializer().lower_expression(diagnostic));

            danubec_ast::StatementKind::Let {
                pattern,
                r#type,
                initializer,
            }
        }
    };

    Ok(danubec_ast::Statement {
        kind,
        span: Span::new(statement.syntax()),
    })
}

pub fn lower_pattern(
    pattern: danubec_syntax::Pattern,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Pattern, ()> {
    let kind = match &pattern {
        danubec_syntax::Pattern::Never(_) => danubec_ast::PatternKind::Never,
        danubec_syntax::Pattern::Placeholder(_) => danubec_ast::PatternKind::Placeholder,
        danubec_syntax::Pattern::Path(path) => {
            let path = lower!(path.path().lower_path(diagnostic));

            danubec_ast::PatternKind::Path { path }
        }
        danubec_syntax::Pattern::Mutable(mutable) => {
            let pattern = lower!(mutable.pattern().lower_pattern(diagnostic));
            let pattern = Box::new(pattern);

            danubec_ast::PatternKind::Mutable { pattern }
        }
        danubec_syntax::Pattern::Tuple(tuple) => {
            let elements = many!(tuple.elements().lower_pattern(diagnostic));

            danubec_ast::PatternKind::Tuple { elements }
        }
        danubec_syntax::Pattern::Array(array) => {
            let elements = many!(array.elements().lower_pattern(diagnostic));

            danubec_ast::PatternKind::Array { elements }
        }
        danubec_syntax::Pattern::Literal(literal) => {
            let value = lower!(literal.literal().lower_literal_expression(diagnostic));

            danubec_ast::PatternKind::Literal { value }
        }
        danubec_syntax::Pattern::Range(range) => {
            let range = match range {
                danubec_syntax::RangePattern::FromTo(from_to) => {
                    let start = lower!(from_to.start().lower_pattern(diagnostic));
                    let start = Box::new(start);

                    let end = lower!(from_to.end().lower_pattern(diagnostic));
                    let end = Box::new(end);

                    danubec_ast::RangePattern::FromTo { start, end }
                }
                danubec_syntax::RangePattern::FromToInclusive(from_to) => {
                    let start = lower!(from_to.start().lower_pattern(diagnostic));
                    let start = Box::new(start);

                    let end = lower!(from_to.end().lower_pattern(diagnostic));
                    let end = Box::new(end);

                    danubec_ast::RangePattern::FromToInclusive { start, end }
                }
                danubec_syntax::RangePattern::From(from_) => {
                    let start = lower!(from_.start().lower_pattern(diagnostic));
                    let start = Box::new(start);

                    danubec_ast::RangePattern::From { start }
                }
                danubec_syntax::RangePattern::To(to) => {
                    let end = lower!(to.end().lower_pattern(diagnostic));
                    let end = Box::new(end);

                    danubec_ast::RangePattern::To { end }
                }
                danubec_syntax::RangePattern::ToInclusive(to) => {
                    let end = lower!(to.end().lower_pattern(diagnostic));
                    let end = Box::new(end);

                    danubec_ast::RangePattern::ToInclusive { end }
                }
            };

            danubec_ast::PatternKind::Range { range }
        }
        danubec_syntax::Pattern::At(at) => {
            let mut patterns = at
                .syntax()
                .children()
                .filter_map(danubec_syntax::Pattern::cast);

            let name = patterns.next().ok_or_else(|| {
                diagnostic.report(miette!("Expected pattern binding in @ pattern"));
            })?;
            let name = lower_pattern_binding(name, diagnostic)?;

            let pattern = patterns.next().ok_or_else(|| {
                diagnostic.report(miette!("Expected pattern in @ pattern"));
            })?;
            let pattern = lower_pattern(pattern, diagnostic)?;
            let pattern = Box::new(pattern);

            danubec_ast::PatternKind::At { name, pattern }
        }
        danubec_syntax::Pattern::Or(or) => {
            let patterns = many!(or.patterns().lower_pattern(diagnostic));

            danubec_ast::PatternKind::Or { patterns }
        }
        danubec_syntax::Pattern::Named(named) => {
            let path = lower!(named.path().lower_path_pattern(diagnostic));
            let fields = many!(named.fields().lower_named_pattern_field(diagnostic));

            danubec_ast::PatternKind::Named { path, fields }
        }
        danubec_syntax::Pattern::Unnamed(unnamed) => {
            let elements = many!(unnamed.elements().lower_pattern(diagnostic));

            danubec_ast::PatternKind::Unnamed { elements }
        }
    };

    Ok(danubec_ast::Pattern {
        kind,
        span: Span::new(pattern.syntax()),
    })
}

pub fn lower_pattern_binding(
    pattern: danubec_syntax::Pattern,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Identifier, ()> {
    if let danubec_syntax::Pattern::Path(path) = pattern {
        let path = lower!(path.path().lower_path(diagnostic));
        if let Some(danubec_ast::PathSegmentKind::Identifier(name)) =
            path.segments.into_iter().map(|s| s.kind).next()
        {
            return Ok(name);
        }
    }

    diagnostic.report(miette!("Pattern binding must be an identifier"));
    Err(())
}

pub fn lower_named_pattern_field(
    field: danubec_syntax::NamedPatternField,
    diagnostic: &mut Diagnostic,
) -> Result<(danubec_ast::Identifier, danubec_ast::Pattern), ()> {
    let name = lower!(field.name().lower_identifier(diagnostic));
    let pattern = lower!(field.pattern().lower_pattern(diagnostic));

    Ok((name, pattern))
}

pub fn lower_path_pattern(
    path: danubec_syntax::PathPattern,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Path, ()> {
    let path = lower!(path.path().lower_path(diagnostic));

    Ok(path)
}

pub fn lower_type_argument(
    argument: danubec_syntax::TypeArgument,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::TypeExpression, ()> {
    let expr = lower!(argument.r#type().lower_type_expression(diagnostic));

    Ok(expr)
}

pub fn lower_type_expression(
    expr: danubec_syntax::TypeExpression,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::TypeExpression, ()> {
    let kind = match &expr {
        danubec_syntax::TypeExpression::Never(_) => danubec_ast::TypeExpressionKind::Never,
        danubec_syntax::TypeExpression::Mutable(mutable) => {
            let inner = lower!(mutable.r#type().lower_type_expression(diagnostic));

            danubec_ast::TypeExpressionKind::Mutable {
                inner: Box::new(inner),
            }
        }
        danubec_syntax::TypeExpression::Path(path) => {
            let path = lower!(path.path().lower_path(diagnostic));

            danubec_ast::TypeExpressionKind::Path { path }
        }
        danubec_syntax::TypeExpression::Tuple(tuple) => {
            let elements = many!(tuple.arguments().lower_type_argument(diagnostic));

            danubec_ast::TypeExpressionKind::Tuple { elements }
        }
        danubec_syntax::TypeExpression::Slice(slice) => {
            let element = lower!(slice.r#type().lower_type_expression(diagnostic));

            danubec_ast::TypeExpressionKind::Slice {
                element: Box::new(element),
            }
        }
    };

    Ok(danubec_ast::TypeExpression {
        kind,
        span: Span::new(expr.syntax()),
    })
}

pub fn lower_path_expression(
    path: danubec_syntax::PathExpression,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Path, ()> {
    let path = lower!(path.path().lower_path(diagnostic));

    Ok(path)
}

pub fn lower_path(
    path: danubec_syntax::Path,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Path, ()> {
    let segments = many!(path.segments().lower_path_segment(diagnostic));

    Ok(danubec_ast::Path {
        segments,
        span: Span::new(path.syntax()),
    })
}

pub fn lower_path_segment(
    segment: danubec_syntax::PathSegment,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::PathSegment, ()> {
    let kind = match &segment {
        danubec_syntax::PathSegment::Root(_) => danubec_ast::PathSegmentKind::Root,
        danubec_syntax::PathSegment::Self_(_) => danubec_ast::PathSegmentKind::Self_,
        danubec_syntax::PathSegment::Super_(_) => danubec_ast::PathSegmentKind::Super_,
        danubec_syntax::PathSegment::Krate(_) => danubec_ast::PathSegmentKind::Krate,
        danubec_syntax::PathSegment::Identifier(identifier) => {
            let name = lower!(identifier.identifier().lower_identifier(diagnostic));

            danubec_ast::PathSegmentKind::Identifier(name)
        }
    };

    Ok(danubec_ast::PathSegment {
        kind,
        span: Span::new(segment.syntax()),
    })
}

pub fn lower_identifier(
    identifier: danubec_syntax::Identifier,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Identifier, ()> {
    let name = match identifier.segment().and_then(|s| s.identifier()) {
        Some(segment) => segment.text().to_owned(),
        None => {
            diagnostic.report(miette!("Expected identifier segment"));
            return Err(());
        }
    };

    Ok(danubec_ast::Identifier {
        name: symbol!(&name),
        span: Span::new(identifier.syntax()),
    })
}

pub fn lower_unary_operator(
    operator: danubec_syntax::UnaryOperator,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::UnaryOperator, ()> {
    use danubec_syntax::{AstNode, SyntaxKind::*};

    match operator.syntax().first_token().map(|n| n.kind()) {
        Some(MUT) => Ok(danubec_ast::UnaryOperator::Mutable),
        Some(PLUS) => Ok(danubec_ast::UnaryOperator::Positive),
        Some(HYPHEN) => Ok(danubec_ast::UnaryOperator::Negate),
        Some(EXCLAMATION) => Ok(danubec_ast::UnaryOperator::Not),
        Some(TILDE) => Ok(danubec_ast::UnaryOperator::BitwiseNot),
        _ => {
            diagnostic.report(miette!("Unknown unary operator"));
            Err(())
        }
    }
}

pub fn lower_binary_operator(
    operator: danubec_syntax::BinaryOperator,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::BinaryOperator, ()> {
    use danubec_syntax::{AstNode, SyntaxKind::*};

    match operator.syntax().first_token().map(|n| n.kind()) {
        Some(PIPE__PIPE) => Ok(danubec_ast::BinaryOperator::LogicalOr),
        Some(AMPERSAND__AMPERSAND) => Ok(danubec_ast::BinaryOperator::LogicalAnd),
        Some(PIPE) => Ok(danubec_ast::BinaryOperator::BitwiseOr),
        Some(CARET) => Ok(danubec_ast::BinaryOperator::BitwiseXor),
        Some(AMPERSAND) => Ok(danubec_ast::BinaryOperator::BitwiseAnd),
        Some(LEFT_CHEVRON__LEFT_CHEVRON__PIPE) => {
            Ok(danubec_ast::BinaryOperator::SaturatingLeftShift)
        }
        Some(LEFT_CHEVRON__LEFT_CHEVRON) => Ok(danubec_ast::BinaryOperator::LeftShift),
        Some(LEFT_CHEVRON__EQUAL) => Ok(danubec_ast::BinaryOperator::LessOrEqual),
        Some(LEFT_CHEVRON) => Ok(danubec_ast::BinaryOperator::Less),
        Some(RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON) => {
            Ok(danubec_ast::BinaryOperator::RightShiftUnsigned)
        }
        Some(RIGHT_CHEVRON__RIGHT_CHEVRON) => Ok(danubec_ast::BinaryOperator::RightShift),
        Some(RIGHT_CHEVRON__EQUAL) => Ok(danubec_ast::BinaryOperator::GreaterOrEqual),
        Some(PLUS__PIPE) => Ok(danubec_ast::BinaryOperator::SaturatingAdd),
        Some(PLUS__PERCENT) => Ok(danubec_ast::BinaryOperator::WrappingAdd),
        Some(PLUS) => Ok(danubec_ast::BinaryOperator::Add),
        Some(HYPHEN__PERCENT) => Ok(danubec_ast::BinaryOperator::WrappingSubtract),
        Some(HYPHEN__PIPE) => Ok(danubec_ast::BinaryOperator::SaturatingSubtract),
        Some(HYPHEN) => Ok(danubec_ast::BinaryOperator::Subtract),
        Some(ASTERISK__PIPE) => Ok(danubec_ast::BinaryOperator::SaturatingMultiply),
        Some(ASTERISK__PERCENT) => Ok(danubec_ast::BinaryOperator::WrappingMultiply),
        Some(ASTERISK__ASTERISK__PIPE) => Ok(danubec_ast::BinaryOperator::SaturatingExponent),
        Some(ASTERISK__ASTERISK__PERCENT) => Ok(danubec_ast::BinaryOperator::WrappingExponent),
        Some(ASTERISK__ASTERISK) => Ok(danubec_ast::BinaryOperator::Exponent),
        Some(ASTERISK) => Ok(danubec_ast::BinaryOperator::Multiply),
        Some(SLASH) => Ok(danubec_ast::BinaryOperator::Divide),
        Some(PERCENT) => Ok(danubec_ast::BinaryOperator::Remainder),
        Some(EQUAL__EQUAL) => Ok(danubec_ast::BinaryOperator::Equal),
        Some(EXCLAMATION__EQUAL) => Ok(danubec_ast::BinaryOperator::NotEqual),
        _ => {
            diagnostic.report(miette!("Unknown binary operator"));
            Err(())
        }
    }
}

pub fn lower_assignment_operator(
    operator: danubec_syntax::AssignmentOperator,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::AssignmentOperator, ()> {
    use danubec_syntax::{AstNode, SyntaxKind::*};

    match operator.syntax().first_child().map(|n| n.kind()) {
        Some(EQUAL) => Ok(danubec_ast::AssignmentOperator::Assign),
        Some(PLUS__EQUAL) => Ok(danubec_ast::AssignmentOperator::Add),
        Some(PLUS__PIPE__EQUAL) => Ok(danubec_ast::AssignmentOperator::SaturatingAdd),
        Some(PLUS__PERCENT__EQUAL) => Ok(danubec_ast::AssignmentOperator::WrappingAdd),
        Some(HYPHEN__EQUAL) => Ok(danubec_ast::AssignmentOperator::Subtract),
        Some(HYPHEN__PIPE__EQUAL) => Ok(danubec_ast::AssignmentOperator::SaturatingSubtract),
        Some(HYPHEN__PERCENT__EQUAL) => Ok(danubec_ast::AssignmentOperator::WrappingSubtract),
        Some(ASTERISK__EQUAL) => Ok(danubec_ast::AssignmentOperator::Multiply),
        Some(ASTERISK__PIPE__EQUAL) => Ok(danubec_ast::AssignmentOperator::SaturatingMultiply),
        Some(ASTERISK__PERCENT__EQUAL) => Ok(danubec_ast::AssignmentOperator::WrappingMultiply),
        Some(ASTERISK__ASTERISK__EQUAL) => Ok(danubec_ast::AssignmentOperator::Exponent),
        Some(ASTERISK__ASTERISK__PIPE__EQUAL) => {
            Ok(danubec_ast::AssignmentOperator::SaturatingExponent)
        }
        Some(ASTERISK__ASTERISK__PERCENT__EQUAL) => {
            Ok(danubec_ast::AssignmentOperator::WrappingExponent)
        }
        Some(SLASH__EQUAL) => Ok(danubec_ast::AssignmentOperator::Divide),
        Some(PERCENT__EQUAL) => Ok(danubec_ast::AssignmentOperator::Remainder),
        Some(CARET__EQUAL) => Ok(danubec_ast::AssignmentOperator::BitwiseXor),
        Some(AMPERSAND__EQUAL) => Ok(danubec_ast::AssignmentOperator::BitwiseAnd),
        Some(AMPERSAND__AMPERSAND__EQUAL) => Ok(danubec_ast::AssignmentOperator::LogicalAnd),
        Some(PIPE__EQUAL) => Ok(danubec_ast::AssignmentOperator::BitwiseOr),
        Some(PIPE__PIPE__EQUAL) => Ok(danubec_ast::AssignmentOperator::LogicalOr),
        Some(LEFT_CHEVRON__LEFT_CHEVRON__EQUAL) => Ok(danubec_ast::AssignmentOperator::LeftShift),
        Some(LEFT_CHEVRON__LEFT_CHEVRON__PIPE__EQUAL) => {
            Ok(danubec_ast::AssignmentOperator::SaturatingLeftShift)
        }
        Some(RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL) => {
            Ok(danubec_ast::AssignmentOperator::RightShift)
        }
        Some(RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL) => {
            Ok(danubec_ast::AssignmentOperator::RightShiftUnsigned)
        }
        _ => {
            diagnostic.report(miette!("Unknown assignment operator"));
            Err(())
        }
    }
}

pub fn lower_literal(
    literal: danubec_syntax::Literal,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::Literal, ()> {
    let kind = match literal.clone() {
        danubec_syntax::Literal::Boolean(bool_) => lower_boolean_literal(bool_, diagnostic)?,
        danubec_syntax::Literal::Character(char_) => lower_character_literal(char_, diagnostic)?,
        danubec_syntax::Literal::Integer(integer) => lower_integer_literal(integer, diagnostic)?,
        danubec_syntax::Literal::Float(float) => lower_float_literal(float, diagnostic)?,
        danubec_syntax::Literal::String(string) => lower_string_literal(string, diagnostic)?,
        danubec_syntax::Literal::Binary(binary) => lower_binary_literal(binary, diagnostic)?,
        danubec_syntax::Literal::Octal(octal) => lower_octal_literal(octal, diagnostic)?,
        danubec_syntax::Literal::Hex(hex) => lower_hex_literal(hex, diagnostic)?,
    };

    Ok(danubec_ast::Literal {
        kind,
        span: Span::new(literal.syntax()),
    })
}

pub fn lower_boolean_literal(
    bool_: danubec_syntax::BooleanLiteral,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::LiteralKind, ()> {
    use danubec_syntax::{AstNode, SyntaxKind::*};

    let value = match bool_.syntax().first_token().map(|n| n.kind()) {
        Some(TRUE) => true,
        Some(FALSE) => false,
        _ => {
            diagnostic.report(miette!("Invalid boolean literal"));
            return Err(());
        }
    };

    Ok(danubec_ast::LiteralKind::Boolean { value })
}

pub fn lower_character_literal(
    char_: danubec_syntax::CharacterLiteral,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::LiteralKind, ()> {
    let value = lower!(char_.kind().lower_character_literal_kind(diagnostic));

    Ok(danubec_ast::LiteralKind::Character { value })
}

pub fn lower_character_literal_kind(
    kind: danubec_syntax::CharacterLiteralKind,
    diagnostic: &mut Diagnostic,
) -> Result<char, ()> {
    match kind {
        danubec_syntax::CharacterLiteralKind::One(one) => {
            let text = one.syntax().text().to_string();
            if let Some(ch) = text.chars().next() {
                Ok(ch)
            } else {
                diagnostic.report(miette!("Invalid character literal"));
                Err(())
            }
        }
        danubec_syntax::CharacterLiteralKind::Escape(escape) => {
            let segment = escape.segment().ok_or_else(|| {
                diagnostic.report(miette!("Expected escape sequence in character literal"));
            })?;
            match segment.text().chars().next() {
                Some('\\') => Ok('\\'),
                Some('\'') => Ok('\''),
                Some('"') => Ok('"'),
                Some('n') => Ok('\n'),
                Some('t') => Ok('\t'),
                _ => {
                    diagnostic.report(miette!("Invalid escape sequence in character literal"));
                    return Err(());
                }
            }
        }
        danubec_syntax::CharacterLiteralKind::Unicode(unicode) => {
            let code_point: String = unicode.segments().map(|s| s.text().to_owned()).fold(
                String::new(),
                |mut sum, code| {
                    sum.push_str(&code);
                    sum
                },
            );
            let code_point = u32::from_str_radix(&code_point, 16).map_err(|_| {
                diagnostic.report(miette!("Invalid unicode code point in character literal"));
            })?;
            let c = std::char::from_u32(code_point).ok_or_else(|| {
                diagnostic.report(miette!("Invalid unicode code point in character literal"));
            })?;

            Ok(c)
        }
    }
}

pub fn lower_integer_literal(
    integer: danubec_syntax::IntegerLiteral,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::LiteralKind, ()> {
    let value = integer.syntax().to_string();
    let value = value.trim().replace('_', "").parse().map_err(|_| {
        diagnostic.report(miette!("Invalid integer literal"));
    })?;

    Ok(danubec_ast::LiteralKind::Integer { value })
}

pub fn lower_float_literal(
    float: danubec_syntax::FloatLiteral,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::LiteralKind, ()> {
    let value = float.syntax().to_string();
    let value = value.trim().replace('_', "").parse().map_err(|_| {
        diagnostic.report(miette!("Invalid float literal"));
    })?;

    Ok(danubec_ast::LiteralKind::Float { value })
}

pub fn lower_string_literal(
    string: danubec_syntax::StringLiteral,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::LiteralKind, ()> {
    let segments = many!(string.segments().lower_string_segment(diagnostic));

    Ok(danubec_ast::LiteralKind::String { segments })
}

pub fn lower_string_segment(
    segment: danubec_syntax::StringSegment,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::StringSegment, ()> {
    match segment {
        danubec_syntax::StringSegment::Text(text) => {
            let value = text.syntax().text().to_string();

            Ok(danubec_ast::StringSegment::Text { value })
        }
        danubec_syntax::StringSegment::Escape(escape) => {
            let segment = escape.segment().ok_or_else(|| {
                diagnostic.report(miette!("Expected escape sequence in string literal"));
            })?;
            let value = match segment.text().chars().next() {
                Some('\\') => '\\',
                Some('\'') => '\'',
                Some('"') => '"',
                Some('n') => '\n',
                Some('t') => '\t',
                _ => {
                    diagnostic.report(miette!("Invalid escape sequence in string literal"));
                    return Err(());
                }
            };

            Ok(danubec_ast::StringSegment::Escape { value })
        }
        danubec_syntax::StringSegment::Unicode(unicode) => {
            let code_point: String = unicode.segments().map(|s| s.text().to_owned()).fold(
                String::new(),
                |mut sum, code| {
                    sum.push_str(&code);
                    sum
                },
            );
            let code_point = u32::from_str_radix(&code_point, 16).map_err(|_| {
                diagnostic.report(miette!("Invalid unicode code point in string literal"));
            })?;
            let value = std::char::from_u32(code_point).ok_or_else(|| {
                diagnostic.report(miette!("Invalid unicode code point in string literal"));
            })?;

            Ok(danubec_ast::StringSegment::Unicode { value })
        }
        danubec_syntax::StringSegment::Interpolation(interpolation) => {
            let expression = lower!(interpolation.expression().lower_expression(diagnostic));

            Ok(danubec_ast::StringSegment::Interpolation { expression })
        }
    }
}

pub fn lower_binary_literal(
    binary: danubec_syntax::BinaryLiteral,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::LiteralKind, ()> {
    let value = binary.syntax().to_string();
    let value = i128::from_str_radix(&value.trim().replace('_', ""), 2).map_err(|_| {
        diagnostic.report(miette!("Invalid binary literal"));
    })?;

    Ok(danubec_ast::LiteralKind::Integer { value })
}

pub fn lower_octal_literal(
    octal: danubec_syntax::OctalLiteral,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::LiteralKind, ()> {
    let value = octal.syntax().to_string();
    let value = i128::from_str_radix(&value.trim().replace('_', ""), 8).map_err(|_| {
        diagnostic.report(miette!("Invalid octal literal"));
    })?;

    Ok(danubec_ast::LiteralKind::Integer { value })
}

pub fn lower_hex_literal(
    hex: danubec_syntax::HexLiteral,
    diagnostic: &mut Diagnostic,
) -> Result<danubec_ast::LiteralKind, ()> {
    let value = hex.syntax().to_string();
    let value = i128::from_str_radix(&value.trim().replace('_', ""), 16).map_err(|_| {
        diagnostic.report(miette!("Invalid hex literal"));
    })?;

    Ok(danubec_ast::LiteralKind::Integer { value })
}
