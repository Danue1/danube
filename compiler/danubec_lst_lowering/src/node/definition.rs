use super::{lower_expression, lower_identifier, lower_statement, lower_type, lower_visibility};
use danubec_diagnostic::Diagnostic;
use danubec_middle::{ast, lst};
use danubec_symbol::Symbol;

pub fn lower_definition(definition: lst::Definition) -> Result<ast::Definition, Diagnostic> {
    let visibility = definition.visibility();
    let kind = opt!(definition.kind(), "ICE: DefinitionKind not found");
    let kind = lower_definition_kind(kind, visibility)?;

    Ok(ast::Definition { kind })
}

pub fn lower_definition_kind(
    kind: lst::DefinitionKind,
    visibility: Option<lst::Visibility>,
) -> Result<ast::DefinitionKind, Diagnostic> {
    match kind {
        lst::DefinitionKind::Const(const_definition) => {
            let visibility = lower_visibility(visibility)?;
            let (ident, ty, expression) = lower_const_definition(const_definition)?;

            Ok(ast::DefinitionKind::Const {
                visibility,
                ident,
                ty,
                expression,
            })
        }
        lst::DefinitionKind::Enum(enum_definition) => {
            let visibility = lower_visibility(visibility)?;
            let (ident, type_parameters, predicates, variants) =
                lower_enum_definition(enum_definition)?;

            Ok(ast::DefinitionKind::Enum {
                visibility,
                ident,
                type_parameters,
                predicates,
                variants,
            })
        }
        lst::DefinitionKind::Function(function_definition) => {
            let visibility = lower_visibility(visibility)?;
            let (ident, type_parameters, parameters, return_type, predicates, body) =
                lower_function_definition(function_definition)?;

            Ok(ast::DefinitionKind::Function(ast::FunctionDef {
                visibility,
                ident,
                type_parameters,
                parameters,
                return_type,
                predicates,
                body,
            }))
        }
        lst::DefinitionKind::Impl(impl_definition) => {
            let (type_parameters, trait_type, target_type, predicates, definitions) =
                lower_impl_definition(impl_definition)?;

            Ok(ast::DefinitionKind::Impl {
                type_parameters,
                trait_type,
                target_type,
                predicates,
                definitions,
            })
        }
        lst::DefinitionKind::Module(module_definition) => {
            let visibility = lower_visibility(visibility)?;
            let (ident, definitions) = lower_module_definition(module_definition)?;

            Ok(ast::DefinitionKind::Module {
                visibility,
                ident,
                definitions,
            })
        }
        lst::DefinitionKind::Static(static_definition) => {
            let visibility = lower_visibility(visibility)?;
            let (ident, ty, expression) = lower_static_definition(static_definition)?;

            Ok(ast::DefinitionKind::Static {
                visibility,
                ident,
                ty,
                expression,
            })
        }
        lst::DefinitionKind::Struct(struct_definition) => {
            let visibility = lower_visibility(visibility)?;
            let (ident, type_parameters, predicates, kind) =
                lower_struct_definition(struct_definition)?;

            Ok(ast::DefinitionKind::Struct {
                visibility,
                ident,
                type_parameters,
                predicates,
                kind,
            })
        }
        lst::DefinitionKind::Trait(trait_definition) => {
            let visibility = lower_visibility(visibility)?;
            let (ident, type_parameters, predicates, definitions) =
                lower_trait_definition(trait_definition)?;

            Ok(ast::DefinitionKind::Trait {
                visibility,
                ident,
                type_parameters,
                predicates,
                definitions,
            })
        }
        lst::DefinitionKind::Type(type_definition) => {
            let visibility = lower_visibility(visibility)?;
            let (ident, type_parameters, ty) = lower_type_definition(type_definition)?;

            Ok(ast::DefinitionKind::Type {
                visibility,
                ident,
                type_parameters,
                ty,
            })
        }
        lst::DefinitionKind::Use(use_definition) => {
            let visibility = lower_visibility(visibility)?;

            let tree = opt!(use_definition.tree(), "ICE: Tree not found");
            let tree = lower_use_tree(tree)?;

            Ok(ast::DefinitionKind::Use { visibility, tree })
        }
    }
}

pub fn lower_const_definition(
    const_definition: lst::ConstDefinition,
) -> Result<(Symbol, ast::Type, ast::Expression), Diagnostic> {
    let identifier = opt!(const_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let ty = opt!(const_definition.ty(), "ICE: Type not found");
    let ty = lower_type(ty)?;

    let expression = opt!(const_definition.expression(), "ICE: Expression not found");
    let expression = lower_expression(expression)?;

    Ok((symbol, ty, expression))
}

pub fn lower_enum_definition(
    enum_definition: lst::EnumDefinition,
) -> Result<
    (
        Symbol,
        Vec<ast::TypeParameter>,
        Vec<ast::Predicate>,
        Vec<ast::EnumVariant>,
    ),
    Diagnostic,
> {
    let identifier = opt!(enum_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let mut type_parameters = vec![];
    for type_parameter in enum_definition.type_parameters() {
        type_parameters.push(lower_type_parameter(type_parameter)?);
    }

    let mut predicates = vec![];
    if let Some(where_clause) = enum_definition.where_clause() {
        for type_constraint in where_clause.type_constraints() {
            predicates.push(lower_type_constraint(type_constraint)?);
        }
    }

    let mut variants = vec![];
    for variant in enum_definition.variants() {
        variants.push(lower_enum_variant(variant)?);
    }

    Ok((symbol, type_parameters, predicates, variants))
}

pub fn lower_function_definition(
    function_definition: lst::FunctionDefinition,
) -> Result<
    (
        Symbol,
        Vec<ast::TypeParameter>,
        Vec<ast::Parameter>,
        Option<ast::Type>,
        Vec<ast::Predicate>,
        Vec<ast::Statement>,
    ),
    Diagnostic,
> {
    let identifier = opt!(function_definition.name(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let mut type_parameters = vec![];
    for type_parameter in function_definition.type_parameters() {
        type_parameters.push(lower_type_parameter(type_parameter)?);
    }

    let mut parameters = vec![];
    for parameter in function_definition.parameters() {
        let identifier = opt!(parameter.identifier(), "ICE: Identifier not found");
        let ident = lower_identifier(identifier)?;

        let ty = opt!(parameter.ty(), "ICE: Type not found");
        let ty = lower_type(ty)?;

        parameters.push(ast::Parameter { ident, ty });
    }

    let return_ty = if let Some(return_ty) = function_definition.return_type() {
        Some(lower_type(return_ty)?)
    } else {
        None
    };

    let mut predicates = vec![];
    if let Some(where_clause) = function_definition.where_clause() {
        for type_constraint in where_clause.type_constraints() {
            predicates.push(lower_type_constraint(type_constraint)?);
        }
    }

    let mut body = vec![];
    if let Some(block_expression) = function_definition.body() {
        for statement in block_expression.statements() {
            body.push(lower_statement(statement)?);
        }
    }

    Ok((
        symbol,
        type_parameters,
        parameters,
        return_ty,
        predicates,
        body,
    ))
}

pub fn lower_impl_definition(
    impl_definition: lst::ImplDefinition,
) -> Result<
    (
        Vec<ast::TypeParameter>,
        Option<ast::Type>,
        ast::Type,
        Vec<ast::Predicate>,
        Vec<ast::ImplItem>,
    ),
    Diagnostic,
> {
    let mut type_parameters = vec![];
    for type_parameter in impl_definition.type_parameters() {
        type_parameters.push(lower_type_parameter(type_parameter)?);
    }

    let (trait_type, target_type) = match impl_definition.target_type() {
        Some(target_type) => {
            let trait_type = opt!(impl_definition.ty(), "ICE: Type not found");
            let trait_type = lower_type(trait_type)?;

            let target_type = opt!(target_type.ty(), "ICE: Target type not found");
            let target_type = lower_type(target_type)?;

            (Some(trait_type), target_type)
        }
        None => {
            let target_type = opt!(impl_definition.ty(), "ICE: Type not found");
            let target_type = lower_type(target_type)?;

            (None, target_type)
        }
    };

    let mut predicates = vec![];
    if let Some(where_clause) = impl_definition.where_clause() {
        for type_constraint in where_clause.type_constraints() {
            predicates.push(lower_type_constraint(type_constraint)?);
        }
    }

    let mut items = vec![];
    for item in impl_definition.items() {
        items.push(lower_impl_item(item)?);
    }

    Ok((type_parameters, trait_type, target_type, predicates, items))
}

pub fn lower_module_definition(
    module_definition: lst::ModuleDefinition,
) -> Result<(Symbol, Vec<ast::Definition>), Diagnostic> {
    let identifier = opt!(module_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let mut definitions = vec![];
    for definition in module_definition.definitions() {
        let definition = lower_definition(definition)?;
        definitions.push(definition);
    }

    Ok((symbol, definitions))
}

pub fn lower_static_definition(
    static_definition: lst::StaticDefinition,
) -> Result<(Symbol, ast::Type, ast::Expression), Diagnostic> {
    let identifier = opt!(static_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let ty = opt!(static_definition.ty(), "ICE: Type not found");
    let ty = lower_type(ty)?;

    let expression = opt!(static_definition.expression(), "ICE: Expression not found");
    let expression = lower_expression(expression)?;

    Ok((symbol, ty, expression))
}

pub fn lower_struct_definition(
    struct_definition: lst::StructDefinition,
) -> Result<
    (
        Symbol,
        Vec<ast::TypeParameter>,
        Vec<ast::Predicate>,
        ast::StructKind,
    ),
    Diagnostic,
> {
    let identifier = opt!(struct_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let mut type_parameters = vec![];
    for type_parameter in struct_definition.type_parameters() {
        type_parameters.push(lower_type_parameter(type_parameter)?);
    }

    let mut predicates = vec![];
    if let Some(where_clause) = struct_definition.where_clause() {
        for type_constraint in where_clause.type_constraints() {
            predicates.push(lower_type_constraint(type_constraint)?);
        }
    }

    let kind = opt!(struct_definition.kind(), "ICE: StructKind not found");
    let kind = lower_struct_kind(kind)?;

    Ok((symbol, type_parameters, predicates, kind))
}

pub fn lower_type_definition(
    type_definition: lst::TypeDefinition,
) -> Result<(Symbol, Vec<ast::TypeParameter>, ast::Type), Diagnostic> {
    let identifier = opt!(type_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let mut type_parameters = vec![];
    for type_parameter in type_definition.type_parameters() {
        type_parameters.push(lower_type_parameter(type_parameter)?);
    }

    let ty = opt!(type_definition.ty(), "ICE: Type not found");
    let ty = lower_type(ty)?;

    Ok((symbol, type_parameters, ty))
}

pub fn lower_trait_definition(
    trait_definition: lst::TraitDefinition,
) -> Result<
    (
        Symbol,
        Vec<ast::TypeParameter>,
        Vec<ast::Predicate>,
        Vec<ast::TraitItem>,
    ),
    Diagnostic,
> {
    let identifier = opt!(trait_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let mut type_parameters = vec![];
    for type_parameter in trait_definition.type_parameters() {
        type_parameters.push(lower_type_parameter(type_parameter)?);
    }

    let mut predicates = vec![];
    if let Some(where_clause) = trait_definition.where_clause() {
        for type_constraint in where_clause.type_constraints() {
            predicates.push(lower_type_constraint(type_constraint)?);
        }
    }

    let mut items = vec![];
    for item in trait_definition.items() {
        items.push(lower_trait_item(item)?);
    }

    Ok((symbol, type_parameters, predicates, items))
}

pub fn lower_struct_kind(struct_kind: lst::StructBodyKind) -> Result<ast::StructKind, Diagnostic> {
    match struct_kind {
        lst::StructBodyKind::Named(named) => {
            let mut fields = vec![];
            for field in named.fields() {
                let identifier = opt!(field.identifier(), "ICE: Identifier not found");
                let symbol = lower_identifier(identifier)?;

                let ty = opt!(field.ty(), "ICE: Type not found");
                let ty = lower_type(ty)?;

                fields.push((symbol, ty));
            }

            Ok(ast::StructKind::Named(fields))
        }
        lst::StructBodyKind::Unnamed(unnamed) => {
            let mut fields = vec![];
            for field in unnamed.fields() {
                let ty = opt!(field.ty(), "ICE: Type not found");
                let ty = lower_type(ty)?;

                fields.push(ty);
            }

            Ok(ast::StructKind::Unnamed(fields))
        }
    }
}

pub fn lower_type_parameter(
    type_parameter: lst::TypeParameter,
) -> Result<ast::TypeParameter, Diagnostic> {
    let identifier = opt!(type_parameter.identifier(), "ICE: Identifier not found");
    let ident = lower_identifier(identifier)?;

    let mut types = vec![];
    for ty in type_parameter.types() {
        types.push(lower_type(ty)?);
    }

    Ok(ast::TypeParameter { ident, types })
}

pub fn lower_type_constraint(
    type_constraint: lst::TypeConstraint,
) -> Result<ast::Predicate, Diagnostic> {
    let ty = opt!(type_constraint.lhs(), "ICE: Type not found");
    let ty = lower_type(ty)?;

    let mut bounds = vec![];
    if let Some(type_constraint_parameter) = type_constraint.rhs() {
        for ty in type_constraint_parameter.types() {
            bounds.push(lower_type(ty)?);
        }
    }

    Ok(ast::Predicate { ty, bounds })
}

pub fn lower_enum_variant(enum_variant: lst::EnumVariant) -> Result<ast::EnumVariant, Diagnostic> {
    let identifier = opt!(enum_variant.identifier(), "ICE: Identifier not found");
    let ident = lower_identifier(identifier)?;

    let kind = match enum_variant.kind() {
        Some(lst::EnumVariantKind::Named(named)) => {
            ast::EnumVariantKind::Named(lower_enum_variant_named(named)?)
        }
        Some(lst::EnumVariantKind::Unnamed(unnamed)) => {
            ast::EnumVariantKind::Unnamed(lower_enum_variant_unnamed(unnamed)?)
        }
        Some(lst::EnumVariantKind::Sequence(sequence)) => {
            ast::EnumVariantKind::Sequence(lower_enum_variant_sequence(sequence)?)
        }
        None => ast::EnumVariantKind::Unit,
    };

    Ok(ast::EnumVariant { ident, kind })
}

pub fn lower_enum_variant_named(
    enum_variant_named: lst::EnumVariantNamed,
) -> Result<Vec<(Symbol, ast::Type)>, Diagnostic> {
    let mut fields = vec![];
    for field in enum_variant_named.fields() {
        let identifier = opt!(field.identifier(), "ICE: Identifier not found");
        let symbol = lower_identifier(identifier)?;

        let ty = opt!(field.ty(), "ICE: Type not found");
        let ty = lower_type(ty)?;

        fields.push((symbol, ty));
    }

    Ok(fields)
}

pub fn lower_enum_variant_unnamed(
    enum_variant_unnamed: lst::EnumVariantUnnamed,
) -> Result<Vec<ast::Type>, Diagnostic> {
    let mut fields = vec![];
    for field in enum_variant_unnamed.fields() {
        let ty = opt!(field.ty(), "ICE: Type not found");
        fields.push(lower_type(ty)?);
    }

    Ok(fields)
}

pub fn lower_enum_variant_sequence(
    enum_variant_sequence: lst::EnumVariantSequence,
) -> Result<ast::Expression, Diagnostic> {
    let expression = opt!(
        enum_variant_sequence.expression(),
        "ICE: Expression not found"
    );
    let expression = lower_expression(expression)?;

    Ok(expression)
}

pub fn lower_impl_item(associated_item: lst::AssociatedItem) -> Result<ast::ImplItem, Diagnostic> {
    let visibility = lower_visibility(associated_item.visibility())?;

    let kind = opt!(associated_item.kind(), "ICE: AssociatedItemKind not found");
    let kind = lower_impl_item_kind(kind, visibility)?;

    Ok(kind)
}

pub fn lower_impl_item_kind(
    kind: lst::AssociatedItemKind,
    visibility: ast::Visibility,
) -> Result<ast::ImplItem, Diagnostic> {
    match kind {
        lst::AssociatedItemKind::Const(const_definition) => {
            let (ident, ty, expression) = lower_impl_const_item(const_definition)?;

            Ok(ast::ImplItem::Const {
                visibility,
                ident,
                ty,
                expression,
            })
        }
        lst::AssociatedItemKind::Function(function_definition) => {
            let (ident, type_parameters, parameters, return_type, predicates, body) =
                lower_impl_function_item(function_definition)?;

            Ok(ast::ImplItem::Function(ast::FunctionDef {
                visibility,
                ident,
                type_parameters,
                parameters,
                return_type,
                predicates,
                body,
            }))
        }
        lst::AssociatedItemKind::Type(type_definition) => {
            let (ident, type_parameters, bounds, ty) = lower_impl_type_item(type_definition)?;

            Ok(ast::ImplItem::Type {
                visibility,
                ident,
                type_parameters,
                bounds,
                ty,
            })
        }
    }
}

pub fn lower_impl_const_item(
    const_definition: lst::ConstDefinition,
) -> Result<(Symbol, ast::Type, ast::Expression), Diagnostic> {
    let identifier = opt!(const_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let ty = opt!(const_definition.ty(), "ICE: Type not found");
    let ty = lower_type(ty)?;

    let expression = opt!(const_definition.expression(), "ICE: Expression not found");
    let expression = lower_expression(expression)?;

    Ok((symbol, ty, expression))
}

pub fn lower_impl_type_item(
    type_definition: lst::TypeDefinition,
) -> Result<
    (
        Symbol,
        Vec<ast::TypeParameter>,
        Vec<ast::Predicate>,
        ast::Type,
    ),
    Diagnostic,
> {
    let identifier = opt!(type_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let mut type_parameters = vec![];
    for type_parameter in type_definition.type_parameters() {
        type_parameters.push(lower_type_parameter(type_parameter)?);
    }

    let mut bounds = vec![];
    if let Some(where_clause) = type_definition.where_clause() {
        for type_constraint in where_clause.type_constraints() {
            bounds.push(lower_type_constraint(type_constraint)?);
        }
    }

    let ty = opt!(type_definition.ty(), "ICE: Type not found");
    let ty = lower_type(ty)?;

    Ok((symbol, type_parameters, bounds, ty))
}

pub fn lower_impl_function_item(
    function_definition: lst::FunctionDefinition,
) -> Result<
    (
        Symbol,
        Vec<ast::TypeParameter>,
        Vec<ast::Parameter>,
        Option<ast::Type>,
        Vec<ast::Predicate>,
        Vec<ast::Statement>,
    ),
    Diagnostic,
> {
    let identifier = opt!(function_definition.name(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let mut type_parameters = vec![];
    for type_parameter in function_definition.type_parameters() {
        type_parameters.push(lower_type_parameter(type_parameter)?);
    }

    let mut parameters = vec![];
    for parameter in function_definition.parameters() {
        let identifier = opt!(parameter.identifier(), "ICE: Identifier not found");
        let ident = lower_identifier(identifier)?;

        let ty = opt!(parameter.ty(), "ICE: Type not found");
        let ty = lower_type(ty)?;

        parameters.push(ast::Parameter { ident, ty });
    }

    let return_ty = if let Some(return_ty) = function_definition.return_type() {
        Some(lower_type(return_ty)?)
    } else {
        None
    };

    let mut predicates = vec![];
    if let Some(where_clause) = function_definition.where_clause() {
        for type_constraint in where_clause.type_constraints() {
            predicates.push(lower_type_constraint(type_constraint)?);
        }
    }

    let mut body = vec![];
    if let Some(block_expression) = function_definition.body() {
        for statement in block_expression.statements() {
            body.push(lower_statement(statement)?);
        }
    }

    Ok((
        symbol,
        type_parameters,
        parameters,
        return_ty,
        predicates,
        body,
    ))
}

pub fn lower_trait_item(
    associated_item: lst::AssociatedItem,
) -> Result<ast::TraitItem, Diagnostic> {
    let visibility = lower_visibility(associated_item.visibility())?;

    let kind = opt!(associated_item.kind(), "ICE: AssociatedItemKind not found");
    let kind = lower_trait_item_kind(kind, visibility)?;

    Ok(kind)
}

pub fn lower_trait_item_kind(
    kind: lst::AssociatedItemKind,
    visibility: ast::Visibility,
) -> Result<ast::TraitItem, Diagnostic> {
    match kind {
        lst::AssociatedItemKind::Const(const_definition) => {
            let (ident, ty, expression) = lower_trait_const_item(const_definition)?;

            Ok(ast::TraitItem::Const {
                visibility,
                ident,
                ty,
                expression,
            })
        }
        lst::AssociatedItemKind::Function(function_definition) => {
            let (ident, type_parameters, parameters, return_type, predicates, body) =
                lower_trait_function_item(function_definition)?;

            Ok(ast::TraitItem::Function(ast::FunctionDef {
                visibility,
                ident,
                type_parameters,
                parameters,
                return_type,
                predicates,
                body,
            }))
        }
        lst::AssociatedItemKind::Type(type_definition) => {
            let (ident, type_parameters, bounds, ty) = lower_trait_type_item(type_definition)?;

            Ok(ast::TraitItem::Type {
                visibility,
                ident,
                type_parameters,
                bounds,
                ty,
            })
        }
    }
}

pub fn lower_trait_const_item(
    const_definition: lst::ConstDefinition,
) -> Result<(Symbol, ast::Type, Option<ast::Expression>), Diagnostic> {
    let identifier = opt!(const_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let ty = opt!(const_definition.ty(), "ICE: Type not found");
    let ty = lower_type(ty)?;

    let expression = if let Some(expression) = const_definition.expression() {
        Some(lower_expression(expression)?)
    } else {
        None
    };

    Ok((symbol, ty, expression))
}

pub fn lower_trait_function_item(
    function_definition: lst::FunctionDefinition,
) -> Result<
    (
        Symbol,
        Vec<ast::TypeParameter>,
        Vec<ast::Parameter>,
        Option<ast::Type>,
        Vec<ast::Predicate>,
        Vec<ast::Statement>,
    ),
    Diagnostic,
> {
    let identifier = opt!(function_definition.name(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let mut type_parameters = vec![];
    for type_parameter in function_definition.type_parameters() {
        type_parameters.push(lower_type_parameter(type_parameter)?);
    }

    let mut parameters = vec![];
    for parameter in function_definition.parameters() {
        let identifier = opt!(parameter.identifier(), "ICE: Identifier not found");
        let ident = lower_identifier(identifier)?;

        let ty = opt!(parameter.ty(), "ICE: Type not found");
        let ty = lower_type(ty)?;

        parameters.push(ast::Parameter { ident, ty });
    }

    let return_ty = if let Some(return_ty) = function_definition.return_type() {
        Some(lower_type(return_ty)?)
    } else {
        None
    };

    let mut predicates = vec![];
    if let Some(where_clause) = function_definition.where_clause() {
        for type_constraint in where_clause.type_constraints() {
            predicates.push(lower_type_constraint(type_constraint)?);
        }
    }

    let mut body = vec![];
    if let Some(block_expression) = function_definition.body() {
        for statement in block_expression.statements() {
            body.push(lower_statement(statement)?);
        }
    }

    Ok((
        symbol,
        type_parameters,
        parameters,
        return_ty,
        predicates,
        body,
    ))
}

pub fn lower_trait_type_item(
    type_definition: lst::TypeDefinition,
) -> Result<
    (
        Symbol,
        Vec<ast::TypeParameter>,
        Vec<ast::Predicate>,
        Option<ast::Type>,
    ),
    Diagnostic,
> {
    let identifier = opt!(type_definition.identifier(), "ICE: Identifier not found");
    let symbol = lower_identifier(identifier)?;

    let mut type_parameters = vec![];
    for type_parameter in type_definition.type_parameters() {
        type_parameters.push(lower_type_parameter(type_parameter)?);
    }

    let mut bounds = vec![];
    if let Some(where_clause) = type_definition.where_clause() {
        for type_constraint in where_clause.type_constraints() {
            bounds.push(lower_type_constraint(type_constraint)?);
        }
    }

    let ty = if let Some(ty) = type_definition.ty() {
        Some(lower_type(ty)?)
    } else {
        None
    };

    Ok((symbol, type_parameters, bounds, ty))
}

pub fn lower_use_tree(use_tree: lst::UseTree) -> Result<ast::UseTree, Diagnostic> {
    let path = if let Some(path) = use_tree.path() {
        lower_use_tree_path(path)?
    } else {
        ast::Path::empty()
    };

    let kind = opt!(use_tree.kind(), "ICE: UseTreeKind not found");
    let kind = lower_use_tree_kind(kind)?;

    Ok(ast::UseTree { path, kind })
}

pub fn lower_use_tree_path(use_tree_path: lst::Path) -> Result<ast::Path, Diagnostic> {
    let mut segments = vec![];
    for segment in use_tree_path.segments() {
        segments.push(lower_use_tree_path_segment(segment)?);
    }

    Ok(ast::Path { segments })
}

pub fn lower_use_tree_path_segment(
    segment: lst::PathSegment,
) -> Result<ast::PathSegment, Diagnostic> {
    let identifier = opt!(segment.identifier(), "ICE: Identifier not found");
    let ident = lower_identifier(identifier)?;

    if segment.type_argument().is_some() {
        error!("Type arguments are not supported in use statements");
    }

    Ok(ast::PathSegment {
        ident,
        types: vec![],
    })
}

pub fn lower_use_tree_kind(kind: lst::UseTreeKind) -> Result<ast::UseTreeKind, Diagnostic> {
    match kind {
        lst::UseTreeKind::Barrel(_) => Ok(ast::UseTreeKind::Barrel),
        lst::UseTreeKind::Ident(ident) => {
            let identifier = opt!(ident.identifier(), "ICE: Identifier not found");
            let ident = lower_identifier(identifier)?;

            Ok(ast::UseTreeKind::Alias(ident))
        }
        lst::UseTreeKind::Nested(nested) => {
            let mut trees = vec![];
            for tree in nested.trees() {
                trees.push(lower_use_tree(tree)?);
            }

            Ok(ast::UseTreeKind::Nested(trees))
        }
    }
}
