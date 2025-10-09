use crate::{
    event::{Event, EventStream, Marker},
    token_stream::TokenStream,
};
use danubec_diagnostic::Diagnostic;
use danubec_syntax::SyntaxKind::{self, *};

pub(crate) struct Parser<'source> {
    tokens: TokenStream<'source>,
    events: EventStream,
    diagnostic: Diagnostic,
}

impl<'source> Parser<'source> {
    pub(crate) fn new(tokens: &'source [SyntaxKind], diagnostic: Diagnostic) -> Self {
        Self {
            tokens: TokenStream::new(tokens),
            events: EventStream::new(),
            diagnostic,
        }
    }

    #[inline]
    fn start(&mut self) -> Marker {
        self.events.reserve()
    }

    #[inline]
    fn complete(&mut self, m: Marker, kind: SyntaxKind) {
        self.events.complete(m, kind);
    }

    fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.at(kind) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn bump(&mut self) {
        self.events.token();
        self.tokens.bump();
    }

    #[inline]
    fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    #[inline]
    fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        self.nth(n) == kind
    }

    #[inline]
    fn nth_at_(&self, n: usize, kind: SyntaxKind) -> bool {
        self.nth_(n) == kind
    }

    #[inline]
    fn nth(&self, n: usize) -> SyntaxKind {
        self.tokens.nth(n)
    }

    #[inline]
    fn nth_(&self, n: usize) -> SyntaxKind {
        self.tokens.nth_(n)
    }

    #[inline]
    fn report(&mut self, ms: Vec<Marker>, report: miette::Report) {
        self.diagnostic.report(report);
        for m in ms {
            self.complete(m, ERROR_NODE);
        }
    }

    #[inline]
    fn recover(&mut self, m: Marker, kinds: &[SyntaxKind]) {
        loop {
            let kind = self.nth(0);
            if kind == END_OF_FILE {
                break;
            }
            if kinds.contains(&kind) {
                break;
            }
            self.bump();
        }

        self.complete(m, ERROR_NODE);
    }

    #[inline]
    pub fn finish(self) -> (Vec<Event>, Diagnostic) {
        (self.events.finalize(), self.diagnostic)
    }
}

macro_rules! expect {
    ($p:expr, $kind:expr, [$($m:expr),*], $($tt:tt)*) => {
        if !$p.eat($kind) {
            return $p.report(vec![$($m),*], miette!(concat!("Expected ", $("`", $tt, "`",)", "*)));
        }
    };
}

macro_rules! cut {
    ($p:expr, $($tt:tt)+) => {
        if !at!($p, $($tt)+) {
            return;
        }
    };
}

macro_rules! at {
    ($p:expr, $($tt:tt)+) => {{
        $($tt)+.into_iter()
            .enumerate()
            .all(|(index, kind)| $p.nth_at(index, kind))
    }};
}

macro_rules! current {
    ($p:expr, $($tt:tt)+) => {
        $($tt)+.into_iter().any(|kind| $p.at(kind))
    };
}

pub(crate) fn krate(p: &mut Parser) {
    let m = p.start();

    top_level_attributes(p);
    definitions(p);

    p.complete(m, KRATE_NODE);
}

pub(crate) fn top_level_attributes(p: &mut Parser) {
    while at!(p, [HASH, EXCLAMATION]) {
        top_level_attribute(p);
    }
}

pub(crate) fn top_level_attribute(p: &mut Parser) {
    cut!(p, [HASH, EXCLAMATION]);

    let m = p.start();

    p.bump();
    p.bump();
    expect!(p, LEFT_BRACKET, [m], '[');
    attribute_argument(p);
    expect!(p, RIGHT_BRACKET, [m], ']');

    p.complete(m, TOP_LEVEL_ATTRIBUTE_NODE);
}

pub(crate) fn attributes(p: &mut Parser) {
    while p.at(HASH) {
        attribute(p);
    }
}

pub(crate) fn attribute(p: &mut Parser) {
    let m = p.start();

    expect!(p, HASH, [m], '#');
    expect!(p, LEFT_BRACKET, [m], '[');
    attribute_argument(p);
    expect!(p, RIGHT_BRACKET, [m], ']');

    p.complete(m, ATTRIBUTE_NODE);
}

pub(crate) fn attribute_argument(p: &mut Parser) {
    match p.nth(0) {
        kind if kind.at_literal() => {
            let m = p.start();

            expression(p);

            p.complete(m, ATTRIBUTE_ARGUMENT_NODE);
        }
        _ if at_path(p) => {
            let m = p.start();

            path(p);

            if p.eat(LEFT_PAREN) {
                while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
                    attribute_argument(p);
                    if !p.eat(COMMA) {
                        break;
                    }
                }
                expect!(p, RIGHT_PAREN, [m], ')');

                return p.complete(m, NESTED_ATTRIBUTE_ARGUMENT_NODE);
            }

            if p.eat(EQUAL) {
                expression(p);
            }

            p.complete(m, KEY_VALUE_ATTRIBUTE_ARGUMENT_NODE);
        }
        _ => p.report(
            vec![],
            miette!("Expected attribute argument: identifier, literal, key-value pair"),
        ),
    }
}

pub(crate) fn definitions(p: &mut Parser) {
    while !p.at(END_OF_FILE) {
        definition(p);
    }
}

const DEFINITION_START: [SyntaxKind; 12] = [
    HASH, PUB, FN, STRUCT, ENUM, USE, MOD, TRAIT, CONST, STATIC, TYPE, IMPL,
];

pub(crate) fn definition(p: &mut Parser) {
    let m = p.start();

    attributes(p);
    visibility_modifier(p);
    match p.nth(0) {
        kind if matches!(kind, FN) => function_definition(p, m),
        kind if matches!(kind, STRUCT) => struct_definition(p, m),
        kind if matches!(kind, ENUM) => enum_definition(p, m),
        kind if matches!(kind, USE) => use_definition(p, m),
        kind if matches!(kind, MOD) => module_definition(p, m),
        kind if matches!(kind, TRAIT) => trait_definition(p, m),
        kind if matches!(kind, CONST) => constant_definition(p, m),
        kind if matches!(kind, STATIC) => static_definition(p, m),
        kind if matches!(kind, TYPE) => type_definition(p, m),
        kind if matches!(kind, IMPL) => implement_definition(p, m),
        _ => {
            p.report(vec![], miette!("Expected definition"));
            p.recover(m, &DEFINITION_START);
        }
    }
}

pub(crate) fn visibility_modifier(p: &mut Parser) {
    cut!(p, [PUB]);

    let m = p.start();

    p.bump(); // eat 'pub'

    if p.eat(LEFT_PAREN) {
        if !current!(p, [CRATE, SUPER, SELF]) {
            return p.report(
                vec![m],
                miette!("Expected visibility modifier: `crate`, `super` or `self`"),
            );
        }
        path(p);
    }

    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, VISIBILITY_NODE);
}

pub(crate) fn function_definition(p: &mut Parser, m: Marker) {
    expect!(p, FN, [m], "function definition");
    identifier(p);
    function_parameters(p);

    if p.eat(HYPHEN) {
        expect!(p, RIGHT_CHEVRON, [m], '>');
        type_expression(p);
    }

    function_body(p);

    p.complete(m, FUNCTION_DEFINITION_NODE);
}

pub(crate) fn struct_definition(p: &mut Parser, m: Marker) {
    expect!(p, STRUCT, [m], "struct definition");
    identifier(p);
    struct_definition_body(p);

    p.complete(m, STRUCT_DEFINITION_NODE);
}

pub(crate) fn enum_definition(p: &mut Parser, m: Marker) {
    expect!(p, ENUM, [m], "enum definition");
    identifier(p);
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        enum_variant(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, ENUM_DEFINITION_NODE);
}

pub(crate) fn use_definition(p: &mut Parser, m: Marker) {
    expect!(p, USE, [m], "use definition");
    use_tree(p);
    expect!(p, SEMICOLON, [m], ';');

    p.complete(m, USE_DEFINITION_NODE);
}

pub(crate) fn module_definition(p: &mut Parser, m: Marker) {
    expect!(p, MOD, [m], "module definition");
    identifier(p);

    let m1 = p.start();

    if p.eat(SEMICOLON) {
        p.complete(m1, MODULE_DEFINITION_OUTLINE_NODE);
        p.complete(m, MODULE_DEFINITION_NODE);
        return;
    }

    expect!(p, LEFT_BRACE, [m1], '{');

    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        if !current!(p, DEFINITION_START) {
            p.report(
                vec![],
                miette!("Expected definition: function, struct, enum, etc."),
            );
            p.complete(m, ERROR_NODE);
            return;
        }
        definition(p);
    }
    if !p.eat(RIGHT_BRACE) {
        p.report(vec![], miette!("Expected `}}`"));
        p.complete(m1, ERROR_NODE);
        p.complete(m, MODULE_DEFINITION_NODE);
        return;
    }

    p.complete(m1, MODULE_DEFINITION_INLINE_NODE);
    p.complete(m, MODULE_DEFINITION_NODE);
}

pub(crate) fn trait_definition(p: &mut Parser, m: Marker) {
    expect!(p, TRAIT, [m], "trait definition");
    identifier(p);
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        associated_item(p);
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, TRAIT_DEFINITION_NODE);
}

pub(crate) fn constant_definition(p: &mut Parser, m: Marker) {
    expect!(p, CONST, [m], "constant definition");
    identifier(p);
    if p.eat(COLON) {
        type_expression(p);
    }
    if p.eat(EQUAL) {
        expression(p);
    }
    expect!(p, SEMICOLON, [m], ';');

    p.complete(m, CONSTANT_DEFINITION_NODE);
}

pub(crate) fn static_definition(p: &mut Parser, m: Marker) {
    expect!(p, STATIC, [m], "static definition");
    identifier(p);
    expect!(p, COLON, [m], ':');
    type_expression(p);
    expect!(p, EQUAL, [m], '=');
    expression(p);
    expect!(p, SEMICOLON, [m], ';');

    p.complete(m, STATIC_DEFINITION_NODE);
}

pub(crate) fn type_definition(p: &mut Parser, m: Marker) {
    expect!(p, TYPE, [m], "type definition");
    identifier(p);
    if p.eat(COLON) {
        type_expression(p);
    }
    if p.eat(EQUAL) {
        type_expression(p);
    }
    expect!(p, SEMICOLON, [m], ';');

    p.complete(m, TYPE_DEFINITION_NODE);
}

pub(crate) fn implement_definition(p: &mut Parser, m: Marker) {
    expect!(p, IMPL, [m], "implement definition");
    type_expression(p);
    if p.eat(FOR) {
        type_expression(p);
    }
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        associated_item(p);
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, IMPLEMENT_DEFINITION_NODE);
}

pub(crate) fn use_tree(p: &mut Parser) {
    let m = p.start();

    if at_root_path(p) {
        let m1 = p.start();

        p.bump(); // eat ':'
        p.bump(); // eat ':'

        p.complete(m1, PATH_SEGMENT_ROOT_NODE);
    }

    use_tree_kind(p);

    p.complete(m, USE_TREE_NODE);
}

pub(crate) fn use_tree_kind(p: &mut Parser) {
    let m = p.start();

    if p.at(LEFT_BRACE) {
        return use_tree_nested(p, m);
    }

    if p.at(ASTERISK) {
        return use_tree_glob(p, m);
    }

    use_tree_element(p, m);
}

pub(crate) fn use_tree_nested(p: &mut Parser, m: Marker) {
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        use_tree(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, USE_TREE_NESTED_NODE);
}

pub(crate) fn use_tree_glob(p: &mut Parser, m: Marker) {
    expect!(p, ASTERISK, [m], '*');

    p.complete(m, USE_TREE_GLOB_NODE);
}

pub(crate) fn use_tree_element(p: &mut Parser, m: Marker) {
    path(p);

    if at_path_separator(p) && p.nth_at(2, LEFT_BRACE) {
        use_tree_trailing_nested(p);
    } else if at_path_separator(p) && p.nth_at(2, ASTERISK) {
        use_tree_trailing_glob(p);
    } else if p.at(AS) {
        use_tree_rename(p);
    }

    p.complete(m, USE_TREE_ELEMENT_NODE);
}

pub(crate) fn use_tree_trailing_nested(p: &mut Parser) {
    let m = p.start();

    if !at_path_separator(p) {
        return p.report(vec![m], miette!("Expected `::`"));
    }

    expect!(p, COLON, [m], ':');
    expect!(p, COLON, [m], ':');
    use_tree_nested(p, m);
}

pub(crate) fn use_tree_trailing_glob(p: &mut Parser) {
    let m = p.start();

    if !(at_path_separator(p) && p.nth_at(2, ASTERISK)) {
        return p.report(vec![m], miette!("Expected `::*`"));
    }

    p.bump(); // eat ':'
    p.bump(); // eat ':'
    p.bump(); // eat '*'

    p.complete(m, USE_TREE_GLOB_NODE);
}

pub(crate) fn use_tree_rename(p: &mut Parser) {
    let m = p.start();

    expect!(p, AS, [m], "as");
    identifier(p);

    p.complete(m, USE_TREE_RENAME_NODE);
}

pub(crate) fn associated_item(p: &mut Parser) {
    let m = p.start();

    attributes(p);

    let m1 = p.start();

    match p.nth(0) {
        kind if matches!(kind, FN) => function_definition(p, m1),
        kind if matches!(kind, CONST) => constant_definition(p, m1),
        kind if matches!(kind, TYPE) => type_definition(p, m1),
        _ => return p.report(vec![m1, m], miette!("Expected trait item")),
    }

    p.complete(m, ASSOCIATED_DEFINITION_NODE);
}

pub(crate) fn function_parameters(p: &mut Parser) {
    expect!(p, LEFT_PAREN, [], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        function_parameter(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [], ')');
}

pub(crate) fn function_parameter(p: &mut Parser) {
    let m = p.start();

    attributes(p);

    if !at_path(p) {
        return p.report(vec![m], miette!("Expected function parameter name"));
    }
    pattern(p);
    expect!(p, COLON, [m], ':');
    type_expression(p);

    p.complete(m, FUNCTION_PARAMETER_NODE);
}

pub(crate) fn function_body(p: &mut Parser) {
    let m = p.start();

    if p.eat(SEMICOLON) {
        return p.complete(m, FUNCTION_BODY_UNIT_NODE);
    }

    if p.at(LEFT_BRACE) {
        block_expression(p);
        return p.complete(m, FUNCTION_BODY_BLOCK_NODE);
    }

    p.report(vec![m], miette!("Expected function body: `{{` or `;`"));
}

pub(crate) fn struct_definition_body(p: &mut Parser) {
    if p.at(SEMICOLON) {
        let m = p.start();
        p.bump(); // eat ';'
        p.complete(m, STRUCT_BODY_UNIT_NODE);
        return;
    }

    if p.at(LEFT_BRACE) {
        struct_named_body(p);
        return;
    }

    if p.at(LEFT_PAREN) {
        struct_unnamed_body(p);
        return;
    }

    p.report(vec![], miette!("Expected struct body: `{{`, `(`, or `;`"));
}

pub(crate) fn struct_named_body(p: &mut Parser) {
    let m = p.start();

    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        struct_named_field(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, STRUCT_BODY_NAMED_NODE);
}

pub(crate) fn struct_named_field(p: &mut Parser) {
    let m = p.start();

    attributes(p);
    visibility_modifier(p);
    identifier(p);
    expect!(p, COLON, [m], ':');
    type_expression(p);

    p.complete(m, STRUCT_BODY_NAMED_FIELD_NODE);
}

pub(crate) fn struct_unnamed_body(p: &mut Parser) {
    let m = p.start();

    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        struct_unnamed_field(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');
    expect!(p, SEMICOLON, [m], ';');

    p.complete(m, STRUCT_BODY_UNNAMED_NODE);
}

pub(crate) fn struct_unnamed_field(p: &mut Parser) {
    let m = p.start();

    attributes(p);
    visibility_modifier(p);
    type_expression(p);

    p.complete(m, STRUCT_BODY_UNNAMED_FIELD_NODE);
}

pub(crate) fn enum_variant(p: &mut Parser) {
    let m = p.start();

    attributes(p);
    identifier(p);
    if p.at(EQUAL) {
        enum_variant_scalar(p, m);
    } else if p.at(LEFT_BRACE) {
        enum_variant_named(p, m);
    } else if p.at(LEFT_PAREN) {
        enum_variant_unnamed(p, m);
    } else {
        p.complete(m, ENUM_VARIANT_UNIT_NODE);
    }
}

pub(crate) fn enum_variant_scalar(p: &mut Parser, m: Marker) {
    expect!(p, EQUAL, [m], '=');
    expression(p);

    p.complete(m, ENUM_VARIANT_SCALAR_NODE);
}

pub(crate) fn enum_variant_named(p: &mut Parser, m: Marker) {
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        enum_variant_named_field(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, ENUM_VARIANT_NAMED_NODE);
}

pub(crate) fn enum_variant_named_field(p: &mut Parser) {
    let m = p.start();

    attributes(p);
    identifier(p);
    expect!(p, COLON, [m], ':');
    type_expression(p);

    p.complete(m, ENUM_VARIANT_NAMED_FIELD_NODE);
}

pub(crate) fn enum_variant_unnamed(p: &mut Parser, m: Marker) {
    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        type_expression(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, ENUM_VARIANT_UNNAMED_NODE);
}

pub(crate) fn type_expression(p: &mut Parser) {
    let m = p.start();

    match p.nth(0) {
        kind if matches!(kind, LEFT_BRACKET) => slice_type_expression(p, m),
        kind if matches!(kind, LEFT_PAREN) => tuple_type_expression(p, m),
        kind if matches!(kind, MUT) => mutable_type_expression(p, m),
        _ if at_path(p) => path_type_expression(p, m),
        _ => p.report(
            vec![m],
            miette!("Expected type expression: path, literal, etc."),
        ),
    }
}

pub(crate) fn path_type_expression(p: &mut Parser, m: Marker) {
    path(p);

    if p.at(COLON) && p.nth_at_(1, COLON) && p.nth_at(2, LEFT_CHEVRON) {
        p.bump(); // eat ':'
        p.bump(); // eat ':'
        p.bump(); // eat '<'

        while !current!(p, [RIGHT_CHEVRON, END_OF_FILE]) {
            type_expression(p);
            if !p.eat(COMMA) {
                break;
            }
        }
        expect!(p, RIGHT_CHEVRON, [m], '>');
    }

    p.complete(m, PATH_TYPE_NODE);
}

pub(crate) fn slice_type_expression(p: &mut Parser, m: Marker) {
    expect!(p, LEFT_BRACKET, [m], '[');
    type_expression(p);
    expect!(p, RIGHT_BRACKET, [m], ']');

    p.complete(m, SLICE_TYPE_NODE);
}

pub(crate) fn tuple_type_expression(p: &mut Parser, m: Marker) {
    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        type_expression(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, TUPLE_TYPE_NODE);
}

pub(crate) fn mutable_type_expression(p: &mut Parser, m: Marker) {
    expect!(p, MUT, [m], "mut");
    type_expression(p);

    p.complete(m, MUTABLE_TYPE_NODE);
}

pub(crate) fn pattern(p: &mut Parser) {
    let m = p.start();

    if p.at(EXCLAMATION) {
        return never_pattern(p, m);
    }
    if p.at(PLACEHOLDER) {
        return placeholder_pattern(p, m);
    }
    if at_path(p) {
        return path_pattern(p, m);
    }
    if p.at(MUT) {
        return mutable_pattern(p, m);
    }
    if p.at(LEFT_PAREN) {
        return tuple_pattern(p, m);
    }
    if p.at(LEFT_BRACKET) {
        return array_pattern(p, m);
    }
    if p.nth(0).at_literal() {
        return literal_pattern(p, m);
    }

    p.report(vec![m], miette!("Expected pattern"));
}

pub(crate) fn never_pattern(p: &mut Parser, m: Marker) {
    expect!(p, EXCLAMATION, [m], '!');

    p.complete(m, NEVER_PATTERN_NODE);
}

pub(crate) fn placeholder_pattern(p: &mut Parser, m: Marker) {
    expect!(p, PLACEHOLDER, [m], '_');

    p.complete(m, PLACEHOLDER_PATTERN_NODE);
}

pub(crate) fn path_pattern(p: &mut Parser, m: Marker) {
    path(p);

    if p.at(LEFT_BRACE) {
        named_pattern(p, m);
    } else if p.at(LEFT_PAREN) {
        unnamed_pattern(p, m);
    } else {
        p.complete(m, PATH_PATTERN_NODE);
    }
}

pub(crate) fn named_pattern(p: &mut Parser, m: Marker) {
    if !at_path(p) {
        return p.report(vec![m], miette!("Expected path"));
    }

    path(p);
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        named_pattern_field(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, NAMED_PATTERN_NODE);
}

pub(crate) fn named_pattern_field(p: &mut Parser) {
    if !at_identifier(p) {
        return p.report(vec![], miette!("Expected identifier"));
    }

    let m = p.start();

    identifier(p);
    expect!(p, COLON, [m], ':');
    pattern(p);

    p.complete(m, NAMED_PATTERN_FIELD_NODE);
}

pub(crate) fn unnamed_pattern(p: &mut Parser, m: Marker) {
    if !at_path(p) {
        return p.report(vec![m], miette!("Expected path"));
    }

    path(p);
    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        pattern(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, UNNAMED_PATTERN_NODE);
}

pub(crate) fn mutable_pattern(p: &mut Parser, m: Marker) {
    expect!(p, MUT, [m], "mut");
    pattern(p);

    p.complete(m, MUTABLE_PATTERN_NODE);
}

pub(crate) fn tuple_pattern(p: &mut Parser, m: Marker) {
    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        pattern(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, TUPLE_PATTERN_NODE);
}

pub(crate) fn array_pattern(p: &mut Parser, m: Marker) {
    expect!(p, LEFT_BRACKET, [m], '[');
    while !current!(p, [RIGHT_BRACKET, END_OF_FILE]) {
        pattern(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACKET, [m], ']');

    p.complete(m, ARRAY_PATTERN_NODE);
}

pub(crate) fn literal_pattern(p: &mut Parser, m: Marker) {
    if !p.nth(0).at_literal() {
        return p.report(vec![m], miette!("Expected literal"));
    }

    expression(p);

    p.complete(m, LITERAL_PATTERN_NODE);
}

pub(crate) fn at_path(p: &Parser) -> bool {
    at_identifier(p) || at_root_path(p) || matches!(p.nth(0), SELF | SUPER | CRATE)
}

pub(crate) fn at_identifier(p: &Parser) -> bool {
    p.at(IDENTIFIER) || p.at(RAW_IDENTIFIER_START)
}

#[inline]
fn at_root_path(p: &Parser) -> bool {
    at_path_separator(p)
}

pub(crate) fn at_path_separator(p: &Parser) -> bool {
    p.at(COLON) && p.nth_at_(1, COLON)
}

pub(crate) fn path(p: &mut Parser) {
    let m = p.start();

    if !at_path(p) {
        return p.report(vec![m], miette!("Expected path"));
    }

    if at_root_path(p) {
        let m1 = p.start();

        p.bump(); // eat ':'
        p.bump(); // eat ':'

        p.complete(m1, PATH_SEGMENT_ROOT_NODE);
    }

    let m2 = p.start();
    match p.nth(0) {
        kind if kind.at_identifier() => {
            identifier(p);
            p.complete(m2, PATH_SEGMENT_IDENTIFIER_NODE);
        }
        kind if matches!(kind, SELF) => {
            p.bump(); // eat 'self'
            p.complete(m2, PATH_SEGMENT_SELF_NODE);
        }
        kind if matches!(kind, SUPER) => {
            p.bump(); // eat 'super'
            p.complete(m2, PATH_SEGMENT_SUPER_NODE);
        }
        kind if matches!(kind, CRATE) => {
            p.bump(); // eat 'crate'
            p.complete(m2, PATH_SEGMENT_KRATE_NODE);
        }
        _ => return p.report(vec![m2, m], miette!("Expected identifier")),
    }

    while at_path_separator(p) && !matches!(p.nth(2), ASTERISK | LEFT_CHEVRON | LEFT_BRACE) {
        p.bump(); // eat ':'
        p.bump(); // eat ':'

        let m3 = p.start();
        match p.nth(0) {
            kind if kind.at_identifier() => {
                identifier(p);
                p.complete(m3, PATH_SEGMENT_IDENTIFIER_NODE);
            }
            kind if matches!(kind, SELF) => {
                p.bump(); // eat 'self'
                p.complete(m3, PATH_SEGMENT_SELF_NODE);
            }
            kind if matches!(kind, SUPER) => {
                p.bump(); // eat 'super'
                p.complete(m3, PATH_SEGMENT_SUPER_NODE);
            }
            kind if matches!(kind, CRATE) => {
                p.bump(); // eat 'crate'
                p.complete(m3, PATH_SEGMENT_KRATE_NODE);
            }
            _ => return p.report(vec![m3, m], miette!("Expected identifier")),
        }
    }

    p.complete(m, PATH_NODE);
}

pub(crate) fn identifier(p: &mut Parser) {
    let m = p.start();
    if p.at(IDENTIFIER) {
        let m1 = p.start();

        p.bump(); // eat identifier
        p.complete(m1, IDENTIFIER_SEGMENT);
        p.complete(m, IDENTIFIER_NODE);
        return;
    }

    if p.eat(RAW_IDENTIFIER_START) {
        let m2 = p.start();

        expect!(p, IDENTIFIER, [m2, m], "identifier");
        p.complete(m2, IDENTIFIER_SEGMENT);
        p.complete(m, IDENTIFIER_NODE);
        return;
    }

    p.report(vec![m], miette!("Expected identifier"));
}

pub(crate) fn expression(p: &mut Parser) {
    if p.nth(0).at_literal() {
        return literal_expression(p);
    }
    if p.at(HASH) || p.at(LEFT_BRACE) {
        return block_expression(p);
    }
    if p.at(LEFT_PAREN) {
        return tuple_expression(p);
    }
    if at_path(p) {
        return path_expression(p);
    }

    p.report(
        vec![],
        miette!("Expected expression: literal, path, function call, etc."),
    );
}

pub(crate) fn literal_expression(p: &mut Parser) {
    let m = p.start();

    literal(p);

    p.complete(m, LITERAL_EXPRESSION_NODE);
}

pub(crate) fn block_expression(p: &mut Parser) {
    if !current!(p, [HASH, LEFT_BRACE]) {
        return p.report(vec![], miette!("Expected block expression"));
    }

    let m = p.start();

    attributes(p);
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        statement(p);
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, BLOCK_EXPRESSION_NODE);
}

pub(crate) fn tuple_expression(p: &mut Parser) {
    let m = p.start();

    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        expression(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, TUPLE_EXPRESSION_NODE);
}

pub(crate) fn path_expression(p: &mut Parser) {
    let m = p.start();

    path(p);

    p.complete(m, PATH_EXPRESSION_NODE);
}

pub(crate) fn statement(p: &mut Parser) {
    let m = p.start();

    if p.eat(SEMICOLON) {
        return p.complete(m, SEMICOLON_STATEMENT_NODE);
    }

    if p.eat(LET) {
        pattern(p);

        if p.eat(COLON) {
            type_expression(p);
        }

        expect!(p, EQUAL, [m], '=');
        expression(p);
        expect!(p, SEMICOLON, [m], ';');

        return p.complete(m, LET_STATEMENT_NODE);
    }

    if !p.at(HASH) {
        expression(p);
        p.eat(SEMICOLON);

        return p.complete(m, EXPRESSION_STATEMENT_NODE);
    }

    attributes(p);

    if p.at(LEFT_BRACE) {
        block_expression(p);
        return p.complete(m, EXPRESSION_STATEMENT_NODE);
    }

    visibility_modifier(p);
    definition(p);

    p.complete(m, DEFINITION_STATEMENT_NODE);
}

pub(crate) fn literal(p: &mut Parser) {
    match p.nth(0) {
        kind if matches!(kind, TRUE | FALSE) => boolean_literal(p),
        kind if matches!(kind, CHARACTER_START) => character_literal(p),
        kind if matches!(kind, STRING_START) => string_literal(p),
        kind if matches!(kind, RAW_STRING_START) => raw_string_literal(p),
        kind if matches!(kind, INTEGER_SEGMENT) => numeric_literal(p),
        kind if matches!(kind, BINARY_START) => binary_literal(p),
        kind if matches!(kind, OCTAL_START) => octal_literal(p),
        kind if matches!(kind, HEX_START) => hex_literal(p),
        _ => p.report(vec![], miette!("Expected literal")),
    }
}

pub(crate) fn boolean_literal(p: &mut Parser) {
    if !current!(p, [TRUE, FALSE]) {
        return p.report(vec![], miette!("Expected boolean literal"));
    }

    let m = p.start();
    p.bump(); // eat 'true' or 'false'

    p.complete(m, BOOLEAN_LITERAL_NODE);
}

pub(crate) fn character_literal(p: &mut Parser) {
    let m = p.start();

    expect!(p, CHARACTER_START, [m], '\'');

    if p.at(CHARACTER_SEGMENT) {
        p.bump(); // eat character segment
    } else if p.at(ESCAPE_START) {
        p.bump(); // eat '\'
        expect!(p, ESCAPE_SEGMENT, [m], "escape sequence");
    } else {
        return p.report(
            vec![m],
            miette!("Expected character segment or escape sequence"),
        );
    }

    expect!(p, CHARACTER_END, [m], '\'');

    p.complete(m, CHARACTER_LITERAL_NODE);
}

pub(crate) fn string_literal(p: &mut Parser) {
    let m = p.start();

    expect!(p, STRING_START, [m], '"');

    while !current!(p, [STRING_END, END_OF_FILE]) {
        let m1 = p.start();

        if p.eat(STRING_SEGMENT) {
            p.complete(m1, STRING_LITERAL_SEGMENT_NODE);

            continue;
        }

        if p.eat(INTERPOLATION_START) {
            expression(p);
            expect!(p, INTERPOLATION_END, [m1, m], '}');

            p.complete(m1, STRING_LITERAL_INTERPOLATION_NODE);

            continue;
        }

        if p.eat(ESCAPE_START) {
            expect!(p, ESCAPE_START, [m1, m], '\\');

            p.complete(m1, STRING_LITERAL_ESCAPE_NODE);

            continue;
        }

        if p.eat(UNICODE_START) {
            while p.eat(UNICODE_SEGMENT) {
                if !p.eat(NUMERIC_SEPARATOR) {
                    break;
                }
            }
            expect!(p, UNICODE_END, [m1, m], '}');

            p.complete(m1, STRING_LITERAL_UNICODE_NODE);

            continue;
        }

        m1.expire();

        break;
    }

    expect!(p, STRING_END, [m], '"');

    p.complete(m, STRING_LITERAL_NODE);
}

pub(crate) fn raw_string_literal(p: &mut Parser) {
    let m = p.start();

    expect!(p, RAW_STRING_START, [m], "string start");
    expect!(p, RAW_STRING_SEGMENT, [m], "string segment");
    expect!(p, RAW_STRING_END, [m], "string end");

    p.complete(m, STRING_LITERAL_NODE);
}

pub(crate) fn numeric_literal(p: &mut Parser) {
    let m = p.start();

    expect!(p, INTEGER_SEGMENT, [m], "numeric literal");
    if p.eat(NUMERIC_SEPARATOR) {
        while p.eat(INTEGER_SEGMENT) {
            if !p.eat(NUMERIC_SEPARATOR) {
                break;
            }
        }
    }

    if !current!(p, [FRACTION_START, EXPONENT_START]) {
        p.complete(m, INTEGER_LITERAL_NODE);
        return;
    }

    if p.eat(FRACTION_START) {
        expect!(p, FRACTION_SEGMENT, [m], "fraction segment");
        while p.eat(NUMERIC_SEPARATOR) {
            if !p.eat(FRACTION_SEGMENT) {
                break;
            }
        }
    }

    if p.eat(EXPONENT_START) {
        p.eat(EXPONENT_SIGN);
        expect!(p, EXPONENT_SEGMENT, [m], "exponent segment");
        while p.eat(NUMERIC_SEPARATOR) {
            if !p.eat(EXPONENT_SEGMENT) {
                break;
            }
        }
    }

    p.complete(m, FLOAT_LITERAL_NODE);
}

pub(crate) fn binary_literal(p: &mut Parser) {
    let m = p.start();

    expect!(p, BINARY_START, [m], "0b");
    expect!(p, BINARY_SEGMENT, [m], "binary literal");
    while p.eat(NUMERIC_SEPARATOR) {
        if !p.eat(BINARY_SEGMENT) {
            break;
        }
    }

    p.complete(m, BINARY_NUMERIC_LITERAL_NODE);
}

pub(crate) fn octal_literal(p: &mut Parser) {
    let m = p.start();

    expect!(p, OCTAL_START, [m], "0o");
    expect!(p, OCTAL_SEGMENT, [m], "octal literal");
    while p.eat(NUMERIC_SEPARATOR) {
        if !p.eat(OCTAL_SEGMENT) {
            break;
        }
    }

    p.complete(m, OCTAL_NUMERIC_LITERAL_NODE);
}

pub(crate) fn hex_literal(p: &mut Parser) {
    let m = p.start();

    expect!(p, HEX_START, [m], "0x");
    expect!(p, HEX_SEGMENT, [m], "hexadecimal literal");
    while p.eat(NUMERIC_SEPARATOR) {
        if !p.eat(HEX_SEGMENT) {
            break;
        }
    }

    p.complete(m, HEX_NUMERIC_LITERAL_NODE);
}
