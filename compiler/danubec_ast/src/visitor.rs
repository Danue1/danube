#[allow(unused)]
pub trait Visitor: Sized {
    fn visit_krate(&mut self, node: crate::Krate) {
        walk_krate(self, node);
    }

    fn visit_definition(&mut self, node: crate::Definition) {
        walk_definition(self, node);
    }

    #[inline]
    fn visit_identifier(&mut self, node: crate::Identifier) {
        //
    }

    fn visit_type_parameter(&mut self, node: crate::TypeParameter) {
        walk_type_parameter(self, node);
    }

    fn visit_type_bound(&mut self, node: crate::TypeBound) {
        walk_type_bound(self, node);
    }

    fn visit_type_expression(&mut self, node: crate::TypeExpression) {
        walk_type_expression(self, node);
    }

    fn visit_function_parameter(&mut self, node: crate::FunctionParameter) {
        walk_function_parameter(self, node);
    }

    fn visit_statement(&mut self, node: crate::Statement) {
        walk_statement(self, node);
    }

    fn visit_struct_body(&mut self, node: crate::StructBody) {
        walk_struct_body(self, node);
    }

    fn visit_enum_variant(&mut self, node: crate::EnumVariant) {
        walk_enum_variant(self, node);
    }

    fn visit_use_tree(&mut self, node: crate::UseTree) {
        walk_use_tree(self, node);
    }

    fn visit_associated_definition(&mut self, node: crate::AssociatedDefinition) {
        walk_associated_definition(self, node);
    }

    fn visit_expression(&mut self, node: crate::Expression) {
        walk_expression(self, node);
    }

    #[inline]
    fn visit_literal(&mut self, node: crate::Literal) {
        //
    }

    fn visit_path(&mut self, node: crate::Path) {
        walk_path(self, node);
    }

    fn visit_path_segment(&mut self, node: crate::PathSegment) {
        walk_path_segment(self, node);
    }

    fn visit_enum_variant_body(&mut self, node: crate::EnumVariantBody) {
        walk_enum_variant_body(self, node);
    }

    fn visit_pattern(&mut self, node: crate::Pattern) {
        walk_pattern(self, node);
    }

    fn visit_attribute(&mut self, node: crate::Attribute) {
        walk_attribute(self, node);
    }

    fn visit_attribute_argument(&mut self, node: crate::AttributeArgument) {
        walk_attribute_argument(self, node);
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

pub fn walk_krate<V: Visitor>(visitor: &mut V, node: crate::Krate) {
    visit_each!(visitor.visit_definition(node.definitions));
}

pub fn walk_definition<V: Visitor>(visitor: &mut V, node: crate::Definition) {
    visit_each!(visitor.visit_attribute(node.attributes));
    match node.kind {
        crate::DefinitionKind::Function {
            node_id,
            visibility,
            name,
            type_parameters,
            parameters,
            return_type,
            type_bounds,
            body,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            visit_each!(visitor.visit_function_parameter(parameters));
            visit_optional!(visitor.visit_type_expression(return_type));
            for (type_expression, type_bound) in type_bounds {
                visitor.visit_type_expression(type_expression);
                visitor.visit_type_bound(type_bound);
            }
            visitor.visit_expression(body);
        }
        crate::DefinitionKind::Struct {
            node_id,
            visibility,
            name,
            type_parameters,
            type_bounds,
            body,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            for (type_expression, type_bound) in type_bounds {
                visitor.visit_type_expression(type_expression);
                visitor.visit_type_bound(type_bound);
            }
            visit_optional!(visitor.visit_struct_body(body));
        }
        crate::DefinitionKind::Enum {
            node_id,
            visibility,
            name,
            type_parameters,
            type_bounds,
            variants,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            for (type_expression, type_bound) in type_bounds {
                visitor.visit_type_expression(type_expression);
                visitor.visit_type_bound(type_bound);
            }
            visit_each!(visitor.visit_enum_variant(variants));
        }
        crate::DefinitionKind::Use {
            node_id,
            visibility,
            tree,
        } => {
            visitor.visit_use_tree(tree);
        }
        crate::DefinitionKind::Module {
            node_id,
            visibility,
            name,
            definitions,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_definition(definitions));
        }
        crate::DefinitionKind::Trait {
            node_id,
            visibility,
            name,
            type_parameters,
            type_bounds,
            definitions,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            for (type_expression, type_bound) in type_bounds {
                visitor.visit_type_expression(type_expression);
                visitor.visit_type_bound(type_bound);
            }
            visit_each!(visitor.visit_associated_definition(definitions));
        }
        crate::DefinitionKind::Constant {
            node_id,
            visibility,
            name,
            r#type,
            value,
        } => {
            visitor.visit_identifier(name);
            visitor.visit_type_expression(r#type);
            visitor.visit_expression(value);
        }
        crate::DefinitionKind::Static {
            node_id,
            visibility,
            name,
            r#type,
            value,
        } => {
            visitor.visit_identifier(name);
            visitor.visit_type_expression(r#type);
            visitor.visit_expression(value);
        }
        crate::DefinitionKind::Type {
            node_id,
            visibility,
            name,
            type_parameters,
            type_bounds,
            expression,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            for (type_expression, type_bound) in type_bounds {
                visitor.visit_type_expression(type_expression);
                visitor.visit_type_bound(type_bound);
            }
            visitor.visit_type_expression(expression);
        }
        crate::DefinitionKind::Implement {
            type_parameters,
            trait_name,
            for_type,
            type_bounds,
            definitions,
        } => {
            visit_each!(visitor.visit_type_parameter(type_parameters));
            visit_optional!(visitor.visit_identifier(trait_name));
            visitor.visit_type_expression(for_type);
            for (type_expression, type_bound) in type_bounds {
                visitor.visit_type_expression(type_expression);
                visitor.visit_type_bound(type_bound);
            }
            visit_each!(visitor.visit_associated_definition(definitions));
        }
    }
}

pub fn walk_type_parameter<V: Visitor>(visitor: &mut V, node: crate::TypeParameter) {
    visitor.visit_identifier(node.name);
    visit_each!(visitor.visit_type_expression(node.bounds));
}

pub fn walk_type_bound<V: Visitor>(visitor: &mut V, node: crate::TypeBound) {
    visitor.visit_type_expression(node.r#type);
    visit_each!(visitor.visit_type_expression(node.constraints));
}

pub fn walk_type_expression<V: Visitor>(visitor: &mut V, node: crate::TypeExpression) {
    match node {
        crate::TypeExpression::Never => {}
        crate::TypeExpression::Path { path } => {
            visitor.visit_path(path);
        }
        crate::TypeExpression::Slice { element } => {
            visitor.visit_type_expression(*element);
        }
        crate::TypeExpression::Tuple { elements } => {
            visit_each!(visitor.visit_type_expression(elements));
        }
    }
}

pub fn walk_function_parameter<V: Visitor>(visitor: &mut V, node: crate::FunctionParameter) {
    visitor.visit_identifier(node.name);
    visitor.visit_type_expression(node.r#type);
}

pub fn walk_statement<V: Visitor>(visitor: &mut V, node: crate::Statement) {
    match node {
        crate::Statement::Definition { definition } => {
            visitor.visit_definition(definition);
        }
        crate::Statement::Let {
            pattern,
            r#type,
            expression,
        } => {
            visitor.visit_pattern(pattern);
            visit_optional!(visitor.visit_type_expression(r#type));
            visit_optional!(visitor.visit_expression(expression));
        }
        crate::Statement::Expression { expression } => {
            visitor.visit_expression(expression);
        }
        crate::Statement::Semicolon => {}
    }
}

pub fn walk_struct_body<V: Visitor>(visitor: &mut V, node: crate::StructBody) {
    match node {
        crate::StructBody::Named(fields) => {
            for (node_id, visibility, name, ty) in fields {
                visitor.visit_identifier(name);
                visitor.visit_type_expression(ty);
            }
        }
        crate::StructBody::Unnamed(fields) => {
            for (visibility, ty) in fields {
                visitor.visit_type_expression(ty);
            }
        }
    }
}

pub fn walk_enum_variant<V: Visitor>(visitor: &mut V, node: crate::EnumVariant) {
    visitor.visit_identifier(node.name);
    visit_optional!(visitor.visit_enum_variant_body(node.body));
}

pub fn walk_use_tree<V: Visitor>(visitor: &mut V, node: crate::UseTree) {
    visitor.visit_path(node.prefix);
    match node.kind {
        crate::UseTreeKind::Glob => {}
        crate::UseTreeKind::Terminal { alias } => {
            if let Some((node_id, alias)) = alias {
                visitor.visit_identifier(alias);
            }
        }
        crate::UseTreeKind::Nested { trees } => {
            visit_each!(visitor.visit_use_tree(trees));
        }
    }
}

pub fn walk_associated_definition<V: Visitor>(visitor: &mut V, node: crate::AssociatedDefinition) {
    visit_each!(visitor.visit_attribute(node.attributes));
    match node.kind {
        crate::AssociatedDefinitionKind::Function {
            name,
            type_parameters,
            parameters,
            return_type,
            type_bounds,
            body,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            visit_each!(visitor.visit_function_parameter(parameters));
            visit_optional!(visitor.visit_type_expression(return_type));
            for (type_expression, type_bound) in type_bounds {
                visitor.visit_type_expression(type_expression);
                visitor.visit_type_bound(type_bound);
            }
            visit_optional!(visitor.visit_expression(body));
        }
        crate::AssociatedDefinitionKind::Constant {
            name,
            r#type,
            value,
        } => {
            visitor.visit_identifier(name);
            visitor.visit_type_expression(r#type);
            visit_optional!(visitor.visit_expression(value));
        }
        crate::AssociatedDefinitionKind::Type {
            name,
            type_parameters,
            type_bounds,
            expression,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            for (type_expression, type_bound) in type_bounds {
                visitor.visit_type_expression(type_expression);
                visitor.visit_type_bound(type_bound);
            }
            visit_optional!(visitor.visit_type_expression(expression));
        }
    }
}

pub fn walk_expression<V: Visitor>(visitor: &mut V, node: crate::Expression) {
    match node.kind {
        crate::ExpressionKind::Break => {}
        crate::ExpressionKind::Continue => {}
        crate::ExpressionKind::Return { value } => {
            if let Some(value) = value {
                visitor.visit_expression(*value);
            }
        }
        crate::ExpressionKind::For {
            pattern,
            iterable,
            body,
        } => {
            visitor.visit_pattern(pattern);
            visitor.visit_expression(*iterable);
            visit_each!(visitor.visit_statement(body));
        }
        crate::ExpressionKind::While { condition, body } => {
            visitor.visit_expression(*condition);
            visit_each!(visitor.visit_statement(body));
        }
        crate::ExpressionKind::Loop { body } => {
            visit_each!(visitor.visit_statement(body));
        }
        crate::ExpressionKind::If {
            condition,
            then_branch,
            else_branch,
        } => {
            visitor.visit_expression(*condition);
            visit_each!(visitor.visit_statement(then_branch));
            if let Some(else_branch) = else_branch {
                visitor.visit_expression(*else_branch);
            }
        }
        crate::ExpressionKind::Match { expression, arms } => {
            visitor.visit_expression(*expression);
            for (pattern, body) in arms {
                visitor.visit_pattern(pattern);
                visitor.visit_expression(body);
            }
        }
        crate::ExpressionKind::Let {
            pattern,
            type_annotation,
            initializer,
        } => {
            visitor.visit_pattern(pattern);
            visit_optional!(visitor.visit_type_expression(type_annotation));
            if let Some(initializer) = initializer {
                visitor.visit_expression(*initializer);
            }
        }
        crate::ExpressionKind::Array { elements } => {
            visit_each!(visitor.visit_expression(elements));
        }
        crate::ExpressionKind::Block {
            attributes,
            statements,
        } => {
            visit_each!(visitor.visit_attribute(attributes));
            visit_each!(visitor.visit_statement(statements));
        }
        crate::ExpressionKind::Literal { value } => {
            visitor.visit_literal(value);
        }
        crate::ExpressionKind::Path { path } => {
            visitor.visit_path(path);
        }
        crate::ExpressionKind::Unary { operator, operand } => {
            visitor.visit_expression(*operand);
        }
        crate::ExpressionKind::Binary {
            left,
            operator,
            right,
        } => {
            visitor.visit_expression(*left);
            visitor.visit_expression(*right);
        }
        crate::ExpressionKind::Assignment {
            target,
            operator,
            value,
        } => {
            visitor.visit_expression(*target);
            visitor.visit_expression(*value);
        }
        crate::ExpressionKind::FunctionCall {
            callee,
            type_arguments,
            arguments,
        } => {
            visitor.visit_expression(*callee);
            visit_each!(visitor.visit_type_expression(type_arguments));
            visit_each!(visitor.visit_expression(arguments));
        }
        crate::ExpressionKind::MethodCall {
            node_id,
            receiver,
            identifier,
            type_arguments,
            arguments,
        } => {
            visitor.visit_expression(*receiver);
            visitor.visit_identifier(identifier);
            visit_each!(visitor.visit_type_expression(type_arguments));
            visit_each!(visitor.visit_expression(arguments));
        }
        crate::ExpressionKind::Field {
            node_id,
            receiver,
            field,
        } => {
            visitor.visit_expression(*receiver);
            visitor.visit_identifier(field);
        }
        crate::ExpressionKind::Index { collection, index } => {
            visitor.visit_expression(*collection);
            visitor.visit_expression(*index);
        }
        crate::ExpressionKind::Struct { path, fields, rest } => {
            visitor.visit_path(path);
            for (node_id, identifier, expression) in fields {
                visitor.visit_identifier(identifier);
                visitor.visit_expression(expression);
            }
            if let Some(rest) = rest {
                visitor.visit_expression(*rest);
            }
        }
        crate::ExpressionKind::Await { expression } => {
            visitor.visit_expression(*expression);
        }
        crate::ExpressionKind::Range {
            start,
            operator,
            end,
        } => {
            if let Some(start) = start {
                visitor.visit_expression(*start);
            }
            if let Some(end) = end {
                visitor.visit_expression(*end);
            }
        }
        crate::ExpressionKind::Try { expression } => {
            visitor.visit_expression(*expression);
        }
        crate::ExpressionKind::Yield { expression } => {
            visitor.visit_expression(*expression);
        }
    }
}

pub fn walk_path<V: Visitor>(visitor: &mut V, node: crate::Path) {
    visit_each!(visitor.visit_path_segment(node.segments));
}

pub fn walk_path_segment<V: Visitor>(visitor: &mut V, node: crate::PathSegment) {
    visitor.visit_identifier(node.name);
    visit_each!(visitor.visit_type_expression(node.type_arguments));
}

pub fn walk_enum_variant_body<V: Visitor>(visitor: &mut V, node: crate::EnumVariantBody) {
    match node {
        crate::EnumVariantBody::Named(fields) => {
            for (node_id, attributes, name, ty) in fields {
                visitor.visit_identifier(name);
                visitor.visit_type_expression(ty);
            }
        }
        crate::EnumVariantBody::Unnamed(fields) => {
            for (attributes, ty) in fields {
                visit_each!(visitor.visit_attribute(attributes));
                visitor.visit_type_expression(ty);
            }
        }
    }
}

pub fn walk_pattern<V: Visitor>(visitor: &mut V, node: crate::Pattern) {
    match node {
        crate::Pattern::Never => {}
        crate::Pattern::Placeholder => {}
        crate::Pattern::Path { path } => {
            visitor.visit_path(path);
        }
        crate::Pattern::Tuple { elements } => {
            visit_each!(visitor.visit_pattern(elements));
        }
        crate::Pattern::Array { elements } => {
            visit_each!(visitor.visit_pattern(elements));
        }
        crate::Pattern::Literal { value } => {
            visitor.visit_literal(value);
        }
        crate::Pattern::Rest { pattern } => {
            visitor.visit_pattern(*pattern);
        }
        crate::Pattern::Or { patterns } => {
            visit_each!(visitor.visit_pattern(patterns));
        }
        crate::Pattern::Named { path, fields } => {
            visitor.visit_path(path);
            for (node_id, identifier, pattern) in fields {
                visitor.visit_identifier(identifier);
                visitor.visit_pattern(pattern);
            }
        }
        crate::Pattern::Unnamed { elements } => {
            for (node_id, pattern) in elements {
                visitor.visit_pattern(pattern);
            }
        }
    }
}

pub fn walk_attribute<V: Visitor>(visitor: &mut V, node: crate::Attribute) {
    visitor.visit_path(node.path);
    visit_each!(visitor.visit_attribute_argument(node.arguments));
}

pub fn walk_attribute_argument<V: Visitor>(visitor: &mut V, node: crate::AttributeArgument) {
    match node {
        crate::AttributeArgument::Expression { value } => {
            visitor.visit_expression(value);
        }
        crate::AttributeArgument::KeyValue {
            node_id,
            key,
            value,
        } => {
            visitor.visit_path(key);
            visitor.visit_expression(value);
        }
        crate::AttributeArgument::Nested { path, arguments } => {
            visitor.visit_path(path);
            visit_each!(visitor.visit_attribute_argument(arguments));
        }
    }
}
