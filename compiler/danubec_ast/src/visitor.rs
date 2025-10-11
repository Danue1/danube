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

    fn visit_use_tree_trailing(&mut self, node: crate::UseTreeTrailing) {
        walk_use_tree_trailing(self, node);
    }

    fn visit_associated_definition(&mut self, node: crate::AssociatedDefinition) {
        walk_associated_definition(self, node);
    }

    fn visit_expression(&mut self, node: crate::Expression) {
        walk_expression(self, node);
    }

    #[inline]
    fn visit_literal(&mut self, node: crate::Literal) {
        walk_literal(self, node);
    }

    fn visit_path(&mut self, node: crate::Path) {
        walk_path(self, node);
    }

    fn visit_path_segment(&mut self, node: crate::PathSegment) {
        walk_path_segment(self, node);
    }

    fn visit_enum_variant_kind(&mut self, node: crate::EnumVariantKind) {
        walk_enum_variant_kind(self, node);
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

    fn visit_range_expression(&mut self, node: crate::RangeExpression) {
        walk_range_expression(self, node);
    }

    fn visit_range_pattern(&mut self, node: crate::RangePattern) {
        walk_range_pattern(self, node);
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
            visit_each!(visitor.visit_type_bound(type_bounds));
            if let Some(body) = body {
                visit_each!(visitor.visit_statement(body));
            }
        }
        crate::DefinitionKind::Struct {
            name,
            type_parameters,
            type_bounds,
            body,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            visit_each!(visitor.visit_type_bound(type_bounds));
            visitor.visit_struct_body(body);
        }
        crate::DefinitionKind::Enum {
            name,
            type_parameters,
            type_bounds,
            variants,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            visit_each!(visitor.visit_type_bound(type_bounds));
            visit_each!(visitor.visit_enum_variant(variants));
        }
        crate::DefinitionKind::Use { tree } => visitor.visit_use_tree(tree),
        crate::DefinitionKind::Module { name, kind } => {
            visitor.visit_identifier(name);
            match kind {
                crate::ModuleDefinitionKind::Inline { definitions } => {
                    visit_each!(visitor.visit_definition(definitions));
                }
                crate::ModuleDefinitionKind::External => {}
            }
        }
        crate::DefinitionKind::Trait {
            name,
            type_parameters,
            type_bounds,
            definitions,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            visit_each!(visitor.visit_type_bound(type_bounds));
            visit_each!(visitor.visit_associated_definition(definitions));
        }
        crate::DefinitionKind::Constant {
            name,
            r#type,
            initializer: value,
        } => {
            visitor.visit_identifier(name);
            visitor.visit_type_expression(r#type);
            visitor.visit_expression(value);
        }
        crate::DefinitionKind::Static {
            name,
            r#type,
            initializer: value,
        } => {
            visitor.visit_identifier(name);
            visitor.visit_type_expression(r#type);
            visitor.visit_expression(value);
        }
        crate::DefinitionKind::Type {
            name,
            type_parameters,
            type_bounds,
            initializer,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            visit_each!(visitor.visit_type_bound(type_bounds));
            visit_optional!(visitor.visit_type_expression(initializer));
        }
        crate::DefinitionKind::Implement {
            type_parameters,
            trait_type,
            target_type,
            type_bounds,
            definitions,
        } => {
            visit_each!(visitor.visit_type_parameter(type_parameters));
            visit_optional!(visitor.visit_type_expression(trait_type));
            visitor.visit_type_expression(target_type);
            visit_each!(visitor.visit_type_bound(type_bounds));
            visit_each!(visitor.visit_associated_definition(definitions));
        }
    }
}

pub fn walk_type_parameter<V: Visitor>(visitor: &mut V, node: crate::TypeParameter) {
    visitor.visit_type_expression(node.r#type);
    visit_each!(visitor.visit_type_expression(node.constraints));
}

pub fn walk_type_bound<V: Visitor>(visitor: &mut V, node: crate::TypeBound) {
    visitor.visit_type_expression(node.r#type);
    visit_each!(visitor.visit_type_expression(node.constraints));
}

pub fn walk_type_expression<V: Visitor>(visitor: &mut V, node: crate::TypeExpression) {
    match node {
        crate::TypeExpression::Never => {}
        crate::TypeExpression::Mutable { inner } => visitor.visit_type_expression(*inner),
        crate::TypeExpression::Path { path } => visitor.visit_path(path),
        crate::TypeExpression::Slice { element } => visitor.visit_type_expression(*element),
        crate::TypeExpression::Tuple { elements } => {
            visit_each!(visitor.visit_type_expression(elements));
        }
    }
}

pub fn walk_function_parameter<V: Visitor>(visitor: &mut V, node: crate::FunctionParameter) {
    visit_each!(visitor.visit_attribute(node.attributes));
    visitor.visit_pattern(node.pattern);
    visitor.visit_type_expression(node.r#type);
}

pub fn walk_statement<V: Visitor>(visitor: &mut V, node: crate::Statement) {
    match node {
        crate::Statement::Definition { definition } => visitor.visit_definition(definition),
        crate::Statement::Let {
            pattern,
            r#type,
            initializer: expression,
        } => {
            visitor.visit_pattern(pattern);
            visit_optional!(visitor.visit_type_expression(r#type));
            visit_optional!(visitor.visit_expression(expression));
        }
        crate::Statement::Expression { value: expression } => visitor.visit_expression(expression),
        crate::Statement::Semicolon => {}
    }
}

pub fn walk_struct_body<V: Visitor>(visitor: &mut V, node: crate::StructBody) {
    match node {
        crate::StructBody::Unit => {
            //
        }
        crate::StructBody::Named(fields) => {
            for (visibility, name, ty) in fields {
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
    visitor.visit_enum_variant_kind(node.kind);
}

pub fn walk_use_tree<V: Visitor>(visitor: &mut V, node: crate::UseTree) {
    match node.kind {
        crate::UseTreeKind::Nested { trees } => {
            visit_each!(visitor.visit_use_tree(trees));
        }
        crate::UseTreeKind::Glob => {
            //
        }
        crate::UseTreeKind::Element { path, trailing } => {
            visitor.visit_path(path);
            visitor.visit_use_tree_trailing(trailing);
        }
    }
}

pub fn walk_use_tree_trailing<V: Visitor>(visitor: &mut V, node: crate::UseTreeTrailing) {
    match node {
        crate::UseTreeTrailing::Identifier => {
            //
        }
        crate::UseTreeTrailing::Nested { trees } => {
            visit_each!(visitor.visit_use_tree(trees));
        }
        crate::UseTreeTrailing::Glob => {
            //
        }
        crate::UseTreeTrailing::Rename { name } => visitor.visit_identifier(name),
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
            visit_each!(visitor.visit_type_bound(type_bounds));
            if let Some(body) = body {
                visit_each!(visitor.visit_statement(body));
            }
        }
        crate::AssociatedDefinitionKind::Constant {
            name,
            r#type,
            initializer,
        } => {
            visitor.visit_identifier(name);
            visit_optional!(visitor.visit_type_expression(r#type));
            visit_optional!(visitor.visit_expression(initializer));
        }
        crate::AssociatedDefinitionKind::Type {
            name,
            type_parameters,
            type_bounds,
            initializer,
        } => {
            visitor.visit_identifier(name);
            visit_each!(visitor.visit_type_parameter(type_parameters));
            visit_each!(visitor.visit_type_bound(type_bounds));
            visit_optional!(visitor.visit_type_expression(initializer));
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
            r#type: type_annotation,
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
        crate::ExpressionKind::Tuple { elements } => {
            visit_each!(visitor.visit_expression(elements));
        }
        crate::ExpressionKind::Block {
            attributes,
            statements,
        } => {
            visit_each!(visitor.visit_attribute(attributes));
            visit_each!(visitor.visit_statement(statements));
        }
        crate::ExpressionKind::Literal { value } => visitor.visit_literal(value),
        crate::ExpressionKind::Path { path } => visitor.visit_path(path),
        crate::ExpressionKind::Unary { operator, operand } => visitor.visit_expression(*operand),
        crate::ExpressionKind::Binary {
            left,
            operator,
            right,
        } => {
            visitor.visit_expression(*left);
            visitor.visit_expression(*right);
        }
        crate::ExpressionKind::Assignment {
            left,
            operator,
            right,
        } => {
            visitor.visit_expression(*left);
            visitor.visit_expression(*right);
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
            receiver,
            method,
            type_arguments,
            arguments,
        } => {
            visitor.visit_expression(*receiver);
            visitor.visit_identifier(method);
            visit_each!(visitor.visit_type_expression(type_arguments));
            visit_each!(visitor.visit_expression(arguments));
        }
        crate::ExpressionKind::Field { receiver, field } => {
            visitor.visit_expression(*receiver);
            visitor.visit_identifier(field);
        }
        crate::ExpressionKind::Index { collection, index } => {
            visitor.visit_expression(*collection);
            visitor.visit_expression(*index);
        }
        crate::ExpressionKind::Struct { path, fields } => {
            visitor.visit_path(path);
            for (identifier, expression) in fields {
                visitor.visit_identifier(identifier);
                visitor.visit_expression(expression);
            }
        }
        crate::ExpressionKind::Await { expression } => visitor.visit_expression(*expression),
        crate::ExpressionKind::Range { range } => visitor.visit_range_expression(range),
        crate::ExpressionKind::Try { expression } => visitor.visit_expression(*expression),
        crate::ExpressionKind::Yield { expression } => {
            if let Some(expression) = expression {
                visitor.visit_expression(*expression);
            }
        }
    }
}

pub fn walk_path<V: Visitor>(visitor: &mut V, node: crate::Path) {
    visit_each!(visitor.visit_path_segment(node.segments));
}

pub fn walk_path_segment<V: Visitor>(visitor: &mut V, node: crate::PathSegment) {
    match node.kind {
        crate::PathSegmentKind::Root => {}
        crate::PathSegmentKind::Self_ => {}
        crate::PathSegmentKind::Super_ => {}
        crate::PathSegmentKind::Krate => {}
        crate::PathSegmentKind::Identifier(name) => visitor.visit_identifier(name),
    }
}

pub fn walk_enum_variant_kind<V: Visitor>(visitor: &mut V, node: crate::EnumVariantKind) {
    match node {
        crate::EnumVariantKind::Unit => {
            //
        }
        crate::EnumVariantKind::Scalar(expression) => visitor.visit_expression(expression),
        crate::EnumVariantKind::Named(fields) => {
            for (attributes, name, ty) in fields {
                visitor.visit_identifier(name);
                visitor.visit_type_expression(ty);
            }
        }
        crate::EnumVariantKind::Unnamed(fields) => {
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
        crate::Pattern::Path { path } => visitor.visit_path(path),
        crate::Pattern::Mutable { pattern } => visitor.visit_pattern(*pattern),
        crate::Pattern::Tuple { elements } => {
            visit_each!(visitor.visit_pattern(elements));
        }
        crate::Pattern::Array { elements } => {
            visit_each!(visitor.visit_pattern(elements));
        }
        crate::Pattern::Literal { value } => visitor.visit_literal(value),
        crate::Pattern::Range { range } => visitor.visit_range_pattern(range),
        crate::Pattern::Rest { pattern } => visitor.visit_pattern(*pattern),
        crate::Pattern::At { name, pattern } => {
            visitor.visit_identifier(name);
            visitor.visit_pattern(*pattern);
        }
        crate::Pattern::Or { patterns } => {
            visit_each!(visitor.visit_pattern(patterns));
        }
        crate::Pattern::Named { path, fields } => {
            visitor.visit_path(path);
            for (identifier, pattern) in fields {
                visitor.visit_identifier(identifier);
                visitor.visit_pattern(pattern);
            }
        }
        crate::Pattern::Unnamed { elements } => {
            for (pattern) in elements {
                visitor.visit_pattern(pattern);
            }
        }
    }
}

pub fn walk_attribute<V: Visitor>(visitor: &mut V, node: crate::Attribute) {
    visitor.visit_attribute_argument(node.argument);
}

pub fn walk_attribute_argument<V: Visitor>(visitor: &mut V, node: crate::AttributeArgument) {
    match node.kind {
        crate::AttributeArgumentKind::Expression { value } => visitor.visit_expression(value),
        crate::AttributeArgumentKind::KeyValue { key, value } => {
            visitor.visit_path(key);
            if let Some(value) = value {
                visitor.visit_expression(value);
            }
        }
        crate::AttributeArgumentKind::Nested { path, arguments } => {
            visitor.visit_path(path);
            visit_each!(visitor.visit_attribute_argument(arguments));
        }
    }
}

pub fn walk_literal<V: Visitor>(visitor: &mut V, node: crate::Literal) {
    match node {
        crate::Literal::Boolean { value: _ } => {
            //
        }
        crate::Literal::Character { value: _ } => {
            //
        }
        crate::Literal::Float { value: _ } => {
            //
        }
        crate::Literal::Integer { value: _ } => {
            //
        }
        crate::Literal::String { segments } => {
            for segment in segments {
                match segment {
                    crate::StringSegment::Text { value: _ } => {
                        //
                    }
                    crate::StringSegment::Unicode { value: _ } => {
                        //
                    }
                    crate::StringSegment::Escape { value: _ } => {
                        //
                    }
                    crate::StringSegment::Interpolation { expression } => {
                        visitor.visit_expression(expression);
                    }
                }
            }
        }
    }
}

pub fn walk_range_expression<V: Visitor>(visitor: &mut V, node: crate::RangeExpression) {
    match node {
        crate::RangeExpression::FromTo { start, end } => {
            visitor.visit_expression(*start);
            visitor.visit_expression(*end);
        }
        crate::RangeExpression::From { start } => visitor.visit_expression(*start),
        crate::RangeExpression::To { end } => visitor.visit_expression(*end),
        crate::RangeExpression::Full => {
            //
        }
        crate::RangeExpression::FromToInclusive { start, end } => {
            visitor.visit_expression(*start);
            visitor.visit_expression(*end);
        }
        crate::RangeExpression::ToInclusive { end } => visitor.visit_expression(*end),
    }
}

pub fn walk_range_pattern<V: Visitor>(visitor: &mut V, node: crate::RangePattern) {
    match node {
        crate::RangePattern::FromTo { start, end } => {
            visitor.visit_pattern(*start);
            visitor.visit_pattern(*end);
        }
        crate::RangePattern::FromToInclusive { start, end } => {
            visitor.visit_pattern(*start);
            visitor.visit_pattern(*end);
        }
        crate::RangePattern::From { start } => visitor.visit_pattern(*start),
        crate::RangePattern::To { end } => visitor.visit_pattern(*end),
        crate::RangePattern::ToInclusive { end } => visitor.visit_pattern(*end),
    }
}
