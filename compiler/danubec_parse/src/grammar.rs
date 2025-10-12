use crate::{
    event::{CompleteMarker, Event, EventStream, Marker},
    token_stream::TokenStream,
    tokens::Tokens,
};
use danubec_diagnostic::Diagnostic;
use danubec_syntax::SyntaxKind::{self, *};

pub(crate) struct Context<'source> {
    tokens: TokenStream<'source>,
    events: EventStream,
    diagnostic: &'source mut Diagnostic,
}

impl<'source> Context<'source> {
    pub(crate) fn new(tokens: &'source [SyntaxKind], diagnostic: &'source mut Diagnostic) -> Self {
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
    fn complete(&mut self, m: Marker, kind: SyntaxKind) -> CompleteMarker {
        self.events.complete(m, kind)
    }

    #[inline]
    fn expire(&mut self) -> CompleteMarker {
        self.events.expire()
    }

    #[inline]
    fn precede(&mut self, cm: CompleteMarker) -> Marker {
        self.events.precede(cm)
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
    fn report(&mut self, ms: Vec<Marker>, report: miette::Report) -> CompleteMarker {
        self.diagnostic.report(report);

        let mut cm = None;
        for m in ms {
            cm = Some(self.complete(m, ERROR_NODE));
        }

        match cm {
            Some(cm) => cm,
            None => self.expire(),
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
    pub fn finish(self) -> Vec<Event> {
        self.events.finalize()
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
            return $p.expire();
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

pub(crate) fn root(p: &mut Context) {
    let m = p.start();

    top_level_attributes(p);
    definitions(p);

    p.complete(m, ROOT_NODE);
}

pub(crate) fn top_level_attributes(p: &mut Context) {
    while at!(p, [HASH, EXCLAMATION]) {
        top_level_attribute(p);
    }
}

pub(crate) fn top_level_attribute(p: &mut Context) -> CompleteMarker {
    cut!(p, [HASH, EXCLAMATION]);

    let m = p.start();

    p.bump();
    p.bump();
    expect!(p, LEFT_BRACKET, [m], '[');
    attribute_argument(p);
    expect!(p, RIGHT_BRACKET, [m], ']');

    p.complete(m, TOP_LEVEL_ATTRIBUTE_NODE)
}

pub(crate) fn attributes(p: &mut Context) {
    while p.at(HASH) {
        attribute(p);
    }
}

pub(crate) fn attribute(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, HASH, [m], '#');
    expect!(p, LEFT_BRACKET, [m], '[');
    attribute_argument(p);
    expect!(p, RIGHT_BRACKET, [m], ']');

    p.complete(m, ATTRIBUTE_NODE)
}

pub(crate) fn attribute_argument(p: &mut Context) -> CompleteMarker {
    match p.nth(0) {
        kind if kind.at_literal() => {
            let m = p.start();

            expression(p);

            p.complete(m, ATTRIBUTE_ARGUMENT_NODE)
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

            p.complete(m, KEY_VALUE_ATTRIBUTE_ARGUMENT_NODE)
        }
        _ => p.report(
            vec![],
            miette!("Expected attribute argument: identifier, literal, key-value pair"),
        ),
    }
}

pub(crate) fn definitions(p: &mut Context) {
    while !p.at(END_OF_FILE) {
        definition(p);
    }
}

const DEFINITION_START: [SyntaxKind; 12] = [
    HASH, PUB, FN, STRUCT, ENUM, USE, MOD, TRAIT, CONST, STATIC, TYPE, IMPL,
];

pub(crate) fn definition(p: &mut Context) {
    let m = p.start();
    let m1 = p.start();

    attributes(p);
    visibility_modifier(p);

    match p.nth(0) {
        kind if matches!(kind, FN) => {
            function_definition(p, m1);
        }
        kind if matches!(kind, STRUCT) => {
            struct_definition(p, m1);
        }
        kind if matches!(kind, ENUM) => {
            enum_definition(p, m1);
        }
        kind if matches!(kind, USE) => {
            use_definition(p, m1);
        }
        kind if matches!(kind, MOD) => {
            module_definition(p, m1);
        }
        kind if matches!(kind, TRAIT) => {
            trait_definition(p, m1);
        }
        kind if matches!(kind, CONST) => {
            constant_definition(p, m1);
        }
        kind if matches!(kind, STATIC) => {
            static_definition(p, m1);
        }
        kind if matches!(kind, TYPE) => {
            type_definition(p, m1);
        }
        kind if matches!(kind, IMPL) => {
            implement_definition(p, m1);
        }
        _ => {
            p.report(vec![], miette!("Expected definition"));
            p.recover(m1, &DEFINITION_START);
            return;
        }
    };

    p.complete(m, DEFINITION_NODE);
}

pub(crate) fn visibility_modifier(p: &mut Context) -> CompleteMarker {
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

    p.complete(m, VISIBILITY_NODE)
}

pub(crate) fn function_definition(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, FN, [m], "function definition");
    identifier(p);
    function_parameters(p);

    if p.eat(HYPHEN) {
        expect!(p, RIGHT_CHEVRON, [m], '>');
        type_expression(p);
    }

    function_body(p);

    p.complete(m, FUNCTION_DEFINITION_NODE)
}

pub(crate) fn struct_definition(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, STRUCT, [m], "struct definition");
    identifier(p);
    struct_definition_body(p);

    p.complete(m, STRUCT_DEFINITION_NODE)
}

pub(crate) fn enum_definition(p: &mut Context, m: Marker) -> CompleteMarker {
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

    p.complete(m, ENUM_DEFINITION_NODE)
}

pub(crate) fn use_definition(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, USE, [m], "use definition");
    use_tree(p);
    expect!(p, SEMICOLON, [m], ';');

    p.complete(m, USE_DEFINITION_NODE)
}

pub(crate) fn module_definition(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, MOD, [m], "module definition");
    identifier(p);

    let m1 = p.start();

    if p.eat(SEMICOLON) {
        p.complete(m1, MODULE_DEFINITION_EXTERNAL_NODE);
        return p.complete(m, MODULE_DEFINITION_NODE);
    }

    expect!(p, LEFT_BRACE, [m1], '{');

    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        if !current!(p, DEFINITION_START) {
            p.report(
                vec![],
                miette!("Expected definition: function, struct, enum, etc."),
            );
            return p.complete(m, ERROR_NODE);
        }
        definition(p);
    }
    if !p.eat(RIGHT_BRACE) {
        p.report(vec![], miette!("Expected `}}`"));
        p.complete(m1, ERROR_NODE);
        return p.complete(m, MODULE_DEFINITION_NODE);
    }

    p.complete(m1, MODULE_DEFINITION_INLINE_NODE);
    p.complete(m, MODULE_DEFINITION_NODE)
}

pub(crate) fn trait_definition(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, TRAIT, [m], "trait definition");
    identifier(p);
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        associated_item(p);
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, TRAIT_DEFINITION_NODE)
}

pub(crate) fn constant_definition(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, CONST, [m], "constant definition");
    identifier(p);
    if p.eat(COLON) {
        type_expression(p);
    }
    if p.eat(EQUAL) {
        expression(p);
    }
    expect!(p, SEMICOLON, [m], ';');

    p.complete(m, CONSTANT_DEFINITION_NODE)
}

pub(crate) fn static_definition(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, STATIC, [m], "static definition");
    identifier(p);
    expect!(p, COLON, [m], ':');
    type_expression(p);
    expect!(p, EQUAL, [m], '=');
    expression(p);
    expect!(p, SEMICOLON, [m], ';');

    p.complete(m, STATIC_DEFINITION_NODE)
}

pub(crate) fn type_definition(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, TYPE, [m], "type definition");
    identifier(p);
    if p.eat(COLON) {
        type_expression(p);
    }
    if p.eat(EQUAL) {
        type_expression(p);
    }
    expect!(p, SEMICOLON, [m], ';');

    p.complete(m, TYPE_DEFINITION_NODE)
}

pub(crate) fn implement_definition(p: &mut Context, m: Marker) -> CompleteMarker {
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

    p.complete(m, IMPLEMENT_DEFINITION_NODE)
}

pub(crate) fn use_tree(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    if at_root_path(p) {
        let m1 = p.start();

        p.bump(); // eat ':'
        p.bump(); // eat ':'

        p.complete(m1, PATH_SEGMENT_ROOT_NODE);
    }

    use_tree_kind(p);

    p.complete(m, USE_TREE_NODE)
}

pub(crate) fn use_tree_kind(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    if p.at(LEFT_BRACE) {
        return use_tree_nested(p, m);
    }

    if p.at(ASTERISK) {
        return use_tree_glob(p, m);
    }

    return use_tree_element(p, m);
}

pub(crate) fn use_tree_nested(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        use_tree(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, USE_TREE_LIST_NODE)
}

pub(crate) fn use_tree_glob(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, ASTERISK, [m], '*');

    p.complete(m, USE_TREE_GLOB_NODE)
}

pub(crate) fn use_tree_element(p: &mut Context, m: Marker) -> CompleteMarker {
    path(p);

    if at_path_separator(p) && p.nth_at(2, LEFT_BRACE) {
        use_tree_trailing_nested(p);
    } else if at_path_separator(p) && p.nth_at(2, ASTERISK) {
        use_tree_trailing_glob(p);
    } else if p.at(AS) {
        use_tree_rename(p);
    }

    p.complete(m, USE_TREE_ELEMENT_NODE)
}

pub(crate) fn use_tree_trailing_nested(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    if !at_path_separator(p) {
        return p.report(vec![m], miette!("Expected `::`"));
    }

    expect!(p, COLON, [m], ':');
    expect!(p, COLON, [m], ':');
    use_tree_nested(p, m)
}

pub(crate) fn use_tree_trailing_glob(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    if !(at_path_separator(p) && p.nth_at(2, ASTERISK)) {
        return p.report(vec![m], miette!("Expected `::*`"));
    }

    p.bump(); // eat ':'
    p.bump(); // eat ':'
    p.bump(); // eat '*'

    p.complete(m, USE_TREE_GLOB_NODE)
}

pub(crate) fn use_tree_rename(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, AS, [m], "as");
    identifier(p);

    p.complete(m, USE_TREE_RENAME_NODE)
}

pub(crate) fn associated_item(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    attributes(p);

    let m1 = p.start();

    match p.nth(0) {
        kind if matches!(kind, FN) => function_definition(p, m1),
        kind if matches!(kind, CONST) => constant_definition(p, m1),
        kind if matches!(kind, TYPE) => type_definition(p, m1),
        _ => return p.report(vec![m1, m], miette!("Expected trait item")),
    };

    p.complete(m, ASSOCIATED_DEFINITION_NODE)
}

pub(crate) fn function_parameters(p: &mut Context) -> CompleteMarker {
    expect!(p, LEFT_PAREN, [], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        function_parameter(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [], ')');
    p.expire()
}

pub(crate) fn function_parameter(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    attributes(p);

    if !at_path(p) {
        return p.report(vec![m], miette!("Expected function parameter name"));
    }
    pattern(p);
    expect!(p, COLON, [m], ':');
    type_expression(p);

    p.complete(m, FUNCTION_PARAMETER_NODE)
}

pub(crate) fn function_body(p: &mut Context) {
    let m = p.start();

    if p.eat(SEMICOLON) {
        p.complete(m, FUNCTION_BODY_UNIT_NODE);
        return;
    }

    if p.at(LEFT_BRACE) {
        block_expression(p);
        p.complete(m, FUNCTION_BODY_BLOCK_NODE);
        return;
    }

    p.report(vec![m], miette!("Expected function body: `{{` or `;`"));
}

pub(crate) fn struct_definition_body(p: &mut Context) {
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

pub(crate) fn struct_named_body(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        struct_named_field(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, STRUCT_BODY_NAMED_NODE)
}

pub(crate) fn struct_named_field(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    attributes(p);
    visibility_modifier(p);
    identifier(p);
    expect!(p, COLON, [m], ':');
    type_expression(p);

    p.complete(m, STRUCT_BODY_NAMED_FIELD_NODE)
}

pub(crate) fn struct_unnamed_body(p: &mut Context) -> CompleteMarker {
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

    p.complete(m, STRUCT_BODY_UNNAMED_NODE)
}

pub(crate) fn struct_unnamed_field(p: &mut Context) {
    let m = p.start();

    attributes(p);
    visibility_modifier(p);
    type_expression(p);

    p.complete(m, STRUCT_BODY_UNNAMED_FIELD_NODE);
}

pub(crate) fn enum_variant(p: &mut Context) {
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

pub(crate) fn enum_variant_scalar(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, EQUAL, [m], '=');
    expression(p);

    p.complete(m, ENUM_VARIANT_SCALAR_NODE)
}

pub(crate) fn enum_variant_named(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        enum_variant_named_field(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, ENUM_VARIANT_NAMED_NODE)
}

pub(crate) fn enum_variant_named_field(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    attributes(p);
    identifier(p);
    expect!(p, COLON, [m], ':');
    type_expression(p);

    p.complete(m, ENUM_VARIANT_NAMED_FIELD_NODE)
}

pub(crate) fn enum_variant_unnamed(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        type_expression(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, ENUM_VARIANT_UNNAMED_NODE)
}

pub(crate) fn type_expression(p: &mut Context) -> CompleteMarker {
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

pub(crate) fn path_type_expression(p: &mut Context, m: Marker) -> CompleteMarker {
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

    p.complete(m, PATH_TYPE_NODE)
}

pub(crate) fn slice_type_expression(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, LEFT_BRACKET, [m], '[');
    type_expression(p);
    expect!(p, RIGHT_BRACKET, [m], ']');

    p.complete(m, SLICE_TYPE_NODE)
}

pub(crate) fn tuple_type_expression(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        type_expression(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, TUPLE_TYPE_NODE)
}

pub(crate) fn mutable_type_expression(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, MUT, [m], "mut");
    type_expression(p);

    p.complete(m, MUTABLE_TYPE_NODE)
}

pub(crate) fn pattern(p: &mut Context) -> CompleteMarker {
    pattern_bp(p, 0)
}

pub(crate) fn pattern_bp(p: &mut Context, bp: usize) -> CompleteMarker {
    let mut lhs = primary_pattern(p);

    loop {
        let r_bp = match p.nth(0) {
            kind if matches!(kind, AT) && 3 >= bp => 2,
            kind if matches!(kind, PIPE) && 1 >= bp => 0,
            _ => break,
        };

        let m = p.precede(lhs);

        lhs = infix_pattern(p, m, r_bp);
    }

    lhs
}

pub(crate) fn primary_pattern(p: &mut Context) -> CompleteMarker {
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

    p.report(vec![m], miette!("Expected pattern"))
}

pub(crate) fn infix_pattern(p: &mut Context, m: Marker, bp: usize) -> CompleteMarker {
    if p.at(AT) {
        return at_pattern(p, m, bp);
    }
    if p.at(PIPE) {
        return or_pattern(p, m);
    }

    p.report(vec![m], miette!("Expected infix pattern operator"))
}

pub(crate) fn at_pattern(p: &mut Context, m: Marker, bp: usize) -> CompleteMarker {
    expect!(p, AT, [m], '@');
    pattern_bp(p, bp);

    p.complete(m, AT_PATTERN_NODE)
}

pub(crate) fn or_pattern(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, PIPE, [m], '|');
    primary_pattern(p);

    while p.eat(PIPE) {
        primary_pattern(p);
    }

    p.complete(m, OR_PATTERN_NODE)
}

pub(crate) fn never_pattern(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, EXCLAMATION, [m], '!');

    p.complete(m, NEVER_PATTERN_NODE)
}

pub(crate) fn placeholder_pattern(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, PLACEHOLDER, [m], '_');

    p.complete(m, PLACEHOLDER_PATTERN_NODE)
}

pub(crate) fn path_pattern(p: &mut Context, m: Marker) -> CompleteMarker {
    path(p);

    if p.at(LEFT_BRACE) {
        named_pattern(p, m)
    } else if p.at(LEFT_PAREN) {
        unnamed_pattern(p, m)
    } else {
        p.complete(m, PATH_PATTERN_NODE)
    }
}

pub(crate) fn named_pattern(p: &mut Context, m: Marker) -> CompleteMarker {
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

    p.complete(m, NAMED_PATTERN_NODE)
}

pub(crate) fn named_pattern_field(p: &mut Context) -> CompleteMarker {
    if !at_identifier(p) {
        return p.report(vec![], miette!("Expected identifier"));
    }

    let m = p.start();

    identifier(p);
    expect!(p, COLON, [m], ':');
    pattern(p);

    p.complete(m, NAMED_PATTERN_FIELD_NODE)
}

pub(crate) fn unnamed_pattern(p: &mut Context, m: Marker) -> CompleteMarker {
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

    p.complete(m, UNNAMED_PATTERN_NODE)
}

pub(crate) fn mutable_pattern(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, MUT, [m], "mut");
    pattern(p);

    p.complete(m, MUTABLE_PATTERN_NODE)
}

pub(crate) fn tuple_pattern(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        pattern(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, TUPLE_PATTERN_NODE)
}

pub(crate) fn array_pattern(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, LEFT_BRACKET, [m], '[');
    while !current!(p, [RIGHT_BRACKET, END_OF_FILE]) {
        pattern(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACKET, [m], ']');

    p.complete(m, ARRAY_PATTERN_NODE)
}

pub(crate) fn literal_pattern(p: &mut Context, m: Marker) -> CompleteMarker {
    if !p.nth(0).at_literal() {
        return p.report(vec![m], miette!("Expected literal"));
    }

    literal_expression(p);

    p.complete(m, LITERAL_PATTERN_NODE)
}

pub(crate) fn at_path(p: &Context) -> bool {
    at_identifier(p) || at_root_path(p) || matches!(p.nth(0), SELF | SUPER | CRATE)
}

pub(crate) fn at_identifier(p: &Context) -> bool {
    p.at(IDENTIFIER) || p.at(RAW_IDENTIFIER_START)
}

#[inline]
fn at_root_path(p: &Context) -> bool {
    at_path_separator(p)
}

pub(crate) fn at_path_separator(p: &Context) -> bool {
    p.at(COLON) && p.nth_at_(1, COLON)
}

pub(crate) fn path(p: &mut Context) -> CompleteMarker {
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

    path_segment(p);

    while at_path_separator(p) && !matches!(p.nth(2), ASTERISK | LEFT_CHEVRON | LEFT_BRACE) {
        p.bump(); // eat ':'
        p.bump(); // eat ':'

        path_segment(p);
    }

    p.complete(m, PATH_NODE)
}

pub(crate) fn path_segment(p: &mut Context) {
    let m = p.start();

    match p.nth(0) {
        kind if kind.at_identifier() => {
            identifier(p);
            p.complete(m, PATH_SEGMENT_IDENTIFIER_NODE);
        }
        kind if matches!(kind, SELF) => {
            p.bump(); // eat 'self'
            p.complete(m, PATH_SEGMENT_SELF_NODE);
        }
        kind if matches!(kind, SUPER) => {
            p.bump(); // eat 'super'
            p.complete(m, PATH_SEGMENT_SUPER_NODE);
        }
        kind if matches!(kind, CRATE) => {
            p.bump(); // eat 'crate'
            p.complete(m, PATH_SEGMENT_KRATE_NODE);
        }
        _ => {
            p.report(vec![m], miette!("Expected identifier"));
        }
    }
}

pub(crate) fn identifier(p: &mut Context) -> CompleteMarker {
    let m = p.start();
    if p.at(IDENTIFIER) {
        let m1 = p.start();

        p.bump(); // eat identifier
        p.complete(m1, IDENTIFIER_SEGMENT);
        return p.complete(m, IDENTIFIER_NODE);
    }

    if p.eat(RAW_IDENTIFIER_START) {
        let m2 = p.start();

        expect!(p, IDENTIFIER, [m2, m], "identifier");
        p.complete(m2, IDENTIFIER_SEGMENT);
        return p.complete(m, IDENTIFIER_NODE);
    }

    p.report(vec![m], miette!("Expected identifier"))
}

const LITERAL_FIRST: Tokens = tokens![
    TRUE,
    FALSE,
    CHARACTER_START,
    STRING_START,
    RAW_STRING_START,
    INTEGER_SEGMENT,
    BINARY_START,
    OCTAL_START,
    HEX_START,
];

const PATH_FIRST: Tokens = tokens![COLON, IDENTIFIER, RAW_IDENTIFIER_START, SELF, SUPER, CRATE];

const UNARY_FIRST: Tokens = tokens![MUT, PLUS, HYPHEN, EXCLAMATION, TILDE];

const EXPRESSION_FIRST: Tokens = tokens![
    BREAK,
    CONTINUE,
    RETURN,
    FOR,
    WHILE,
    LOOP,
    IF,
    MATCH,
    LET,
    LEFT_BRACKET,
    LEFT_PAREN,
    LEFT_BRACE,
    HASH,
];

const PRIMARY_FIRST: Tokens = LITERAL_FIRST.concat(PATH_FIRST).concat(EXPRESSION_FIRST);

const PREFIX_EXPRESSION_FIRST: Tokens = UNARY_FIRST.concat(PRIMARY_FIRST);

#[inline]
pub(crate) fn expression(p: &mut Context) {
    expression_bp(p, 0);
}

pub(crate) fn expression_bp(p: &mut Context, bp: usize) {
    let mut lhs = if UNARY_FIRST.contains(p.nth(0)) {
        unary_expression(p)
    } else if PRIMARY_FIRST.contains(p.nth(0)) {
        primary_expression(p)
    } else {
        p.report(vec![], miette!("Expected expression"))
    };
    ();

    loop {
        let (r_bp, kind, count) = match bp_infix_expression(p) {
            Some(((l_bp, r_bp), (kind, count))) if l_bp >= bp => (r_bp, kind, count),
            _ => break,
        };

        let m = p.precede(lhs);

        {
            let m1 = p.start();
            for _ in 0..count {
                p.bump(); // eat operator
            }
            let cm = p.complete(m1, kind);
            let parent = if kind.at_assign_operator() {
                Some(ASSIGNMENT_OPERATOR_NODE)
            } else if kind.at_binary_operator() {
                Some(BINARY_OPERATOR_NODE)
            } else {
                None
            };
            if let Some(parent) = parent {
                let m = p.precede(cm);
                p.complete(m, parent);
            }
        }

        lhs = infix_expression(p, m, r_bp, kind);
    }
}

pub(crate) fn unary_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    {
        let m1 = p.start();

        p.bump(); // eat unary operator
        p.complete(m1, UNARY_OPERATOR_NODE);
    }

    expression(p);

    p.complete(m, UNARY_EXPRESSION_NODE)
}

pub(crate) fn bp_infix_expression(
    p: &mut Context,
) -> Option<((usize, usize), (SyntaxKind, usize))> {
    match (p.nth(0), p.nth(1), p.nth(2), p.nth(3)) {
        // Assignment operators
        (PLUS, EQUAL, _, _) => Some(((0, 1), (PLUS__EQUAL, 2))), // +=
        (PLUS, PIPE, EQUAL, _) => Some(((0, 1), (PLUS__PIPE__EQUAL, 3))), // +|=
        (PLUS, PERCENT, EQUAL, _) => Some(((0, 1), (PLUS__PERCENT__EQUAL, 3))), // +%=
        (HYPHEN, EQUAL, _, _) => Some(((0, 1), (HYPHEN__EQUAL, 2))), // -=
        (HYPHEN, PIPE, EQUAL, _) => Some(((0, 1), (HYPHEN__PIPE__EQUAL, 3))), // -|=
        (HYPHEN, PERCENT, EQUAL, _) => Some(((0, 1), (HYPHEN__PERCENT__EQUAL, 3))), // -%=
        (ASTERISK, EQUAL, _, _) => Some(((0, 1), (ASTERISK__EQUAL, 2))), // *=
        (ASTERISK, PIPE, EQUAL, _) => Some(((0, 1), (ASTERISK__PIPE__EQUAL, 3))), // *|=
        (ASTERISK, PERCENT, EQUAL, _) => Some(((0, 1), (ASTERISK__PERCENT__EQUAL, 3))), // *%=
        (ASTERISK, ASTERISK, EQUAL, _) => Some(((0, 1), (ASTERISK__ASTERISK__EQUAL, 3))), // **=
        (ASTERISK, ASTERISK, PIPE, EQUAL) => Some(((0, 1), (ASTERISK__ASTERISK__PIPE__EQUAL, 4))), // **|=
        (ASTERISK, ASTERISK, PERCENT, EQUAL) => {
            Some(((0, 1), (ASTERISK__ASTERISK__PERCENT__EQUAL, 4)))
        } // **%=
        (SLASH, EQUAL, _, _) => Some(((0, 1), (SLASH__EQUAL, 2))), // /=
        (PERCENT, EQUAL, _, _) => Some(((0, 1), (PERCENT__EQUAL, 2))), // %=
        (CARET, EQUAL, _, _) => Some(((0, 1), (CARET__EQUAL, 2))), // ^=
        (AMPERSAND, EQUAL, _, _) => Some(((0, 1), (AMPERSAND__EQUAL, 2))), // &=
        (AMPERSAND, AMPERSAND, EQUAL, _) => Some(((0, 1), (AMPERSAND__AMPERSAND__EQUAL, 3))), // &&=
        (PIPE, EQUAL, _, _) => Some(((0, 1), (PIPE__EQUAL, 2))),   // |=
        (PIPE, PIPE, EQUAL, _) => Some(((0, 1), (PIPE__PIPE__EQUAL, 3))), // ||=
        (LEFT_CHEVRON, LEFT_CHEVRON, EQUAL, _) => {
            Some(((0, 1), (LEFT_CHEVRON__LEFT_CHEVRON__EQUAL, 3)))
        } // <<=
        (LEFT_CHEVRON, LEFT_CHEVRON, PIPE, EQUAL) => {
            Some(((0, 1), (LEFT_CHEVRON__LEFT_CHEVRON__PIPE__EQUAL, 4)))
        } // <<|=
        (RIGHT_CHEVRON, RIGHT_CHEVRON, EQUAL, _) => {
            Some(((0, 1), (RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL, 3)))
        } // >>=
        (RIGHT_CHEVRON, RIGHT_CHEVRON, RIGHT_CHEVRON, EQUAL) => Some((
            (0, 1),
            (RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON__EQUAL, 4),
        )), // >>>=

        // Binary operators
        (DOT, DOT, EQUAL, _) => Some(((2, 3), (DOT__DOT__EQUAL, 3))), // ..=
        (DOT, DOT, _, _) => Some(((2, 3), (DOT__DOT, 2))),            // ..

        (PIPE, PIPE, _, _) => Some(((4, 5), (PIPE__PIPE, 2))), // ||

        (AMPERSAND, AMPERSAND, _, _) => Some(((6, 7), (AMPERSAND__AMPERSAND, 2))), // &&

        (EQUAL, EQUAL, _, _) => Some(((8, 9), (EQUAL__EQUAL, 2))), // ==
        (EXCLAMATION, EQUAL, _, _) => Some(((8, 9), (EXCLAMATION__EQUAL, 2))), // !=

        (LEFT_CHEVRON, EQUAL, _, _) => Some(((10, 11), (LEFT_CHEVRON__EQUAL, 2))), // <=
        (RIGHT_CHEVRON, EQUAL, _, _) => Some(((10, 11), (RIGHT_CHEVRON__EQUAL, 2))), // >=

        (PIPE, _, _, _) => Some(((12, 13), (PIPE, 1))), // |

        (CARET, _, _, _) => Some(((14, 15), (CARET, 1))), // ^

        (AMPERSAND, _, _, _) => Some(((16, 17), (AMPERSAND, 1))), // &

        (LEFT_CHEVRON, LEFT_CHEVRON, PIPE, _) => {
            Some(((18, 19), (LEFT_CHEVRON__LEFT_CHEVRON__PIPE, 3)))
        } // <<|
        (LEFT_CHEVRON, LEFT_CHEVRON, _, _) => Some(((18, 19), (LEFT_CHEVRON__LEFT_CHEVRON, 2))), // <<
        (RIGHT_CHEVRON, RIGHT_CHEVRON, RIGHT_CHEVRON, _) => {
            Some(((18, 19), (RIGHT_CHEVRON__RIGHT_CHEVRON__RIGHT_CHEVRON, 3)))
        } // >>>
        (RIGHT_CHEVRON, RIGHT_CHEVRON, _, _) => Some(((18, 19), (RIGHT_CHEVRON__RIGHT_CHEVRON, 2))), // >>

        (PLUS, PIPE, _, _) => Some(((20, 21), (PLUS__PIPE, 2))), // +|
        (PLUS, PERCENT, _, _) => Some(((20, 21), (PLUS__PERCENT, 2))), // +%
        (PLUS, _, _, _) => Some(((20, 21), (PLUS, 1))),          // +
        (HYPHEN, PIPE, _, _) => Some(((20, 21), (HYPHEN__PIPE, 2))), // -|
        (HYPHEN, PERCENT, _, _) => Some(((20, 21), (HYPHEN__PERCENT, 2))), // -%
        (HYPHEN, _, _, _) => Some(((20, 21), (HYPHEN, 1))),      // -

        (ASTERISK, PIPE, _, _) => Some(((22, 23), (ASTERISK__PIPE, 2))), // *|
        (ASTERISK, PERCENT, _, _) => Some(((22, 23), (ASTERISK__PERCENT, 2))), // *%
        (ASTERISK, ASTERISK, PIPE, _) => Some(((22, 23), (ASTERISK__ASTERISK__PIPE, 3))), // **|
        (ASTERISK, ASTERISK, PERCENT, _) => Some(((22, 23), (ASTERISK__ASTERISK__PERCENT, 3))), // **%
        (ASTERISK, ASTERISK, _, _) => Some(((22, 23), (ASTERISK__ASTERISK, 2))), // **
        (ASTERISK, _, _, _) => Some(((22, 23), (ASTERISK, 1))),                  // *
        (SLASH, _, _, _) => Some(((22, 23), (SLASH, 1))),                        // /
        (PERCENT, _, _, _) => Some(((22, 23), (PERCENT, 1))),                    // %

        (QUESTION, _, _, _) => Some(((24, 25), (QUESTION, 1))), // ?

        (LEFT_BRACKET, _, _, _) => Some(((26, 27), (LEFT_BRACKET, 1))), // [
        (LEFT_PAREN, _, _, _) => Some(((26, 27), (LEFT_PAREN, 1))),     // (

        (DOT, _, _, _) => Some(((28, 29), (DOT, 1))), // .

        // Assignment operators
        (EQUAL, RIGHT_CHEVRON, _, _) => None,           // =>
        (EQUAL, _, _, _) => Some(((2, 3), (EQUAL, 1))), // =

        // Binary operators
        (LEFT_CHEVRON, _, _, _) => Some(((8, 9), (LEFT_CHEVRON, 1))), // <
        (RIGHT_CHEVRON, _, _, _) => Some(((8, 9), (RIGHT_CHEVRON, 1))), // >

        _ => None,
    }
}

pub(crate) fn infix_expression(
    p: &mut Context,
    m: Marker,
    bp: usize,
    kind: SyntaxKind,
) -> CompleteMarker {
    if kind.at_assign_operator() {
        return assignment_expression(p, m, bp);
    }
    if kind.at_binary_operator() {
        return binary_expression(p, m, bp);
    }
    if matches!(kind, LEFT_BRACKET) {
        return index_expression(p, m);
    }
    if matches!(kind, LEFT_PAREN) {
        return function_call_expression(p, m);
    }
    if matches!(kind, QUESTION) {
        return try_expression(p, m);
    }
    if matches!(kind, DOT) {
        if p.at(AWAIT) {
            return await_expression(p, m);
        }
        if p.at(YIELD) {
            return yield_expression(p, m);
        }
        if p.at(IDENTIFIER) || p.at(RAW_IDENTIFIER_START) {
            return field_expression(p, m);
        }
    }

    p.report(vec![m], miette!("Expected infix expression"))
}

pub(crate) fn assignment_expression(p: &mut Context, m: Marker, _bp: usize) -> CompleteMarker {
    expression_bp(p, 0);

    p.complete(m, ASSIGNMENT_EXPRESSION_NODE)
}

pub(crate) fn binary_expression(p: &mut Context, m: Marker, _bp: usize) -> CompleteMarker {
    expression_bp(p, 0);

    p.complete(m, BINARY_EXPRESSION_NODE)
}

pub(crate) fn index_expression(p: &mut Context, m: Marker) -> CompleteMarker {
    expression(p);
    expect!(p, RIGHT_BRACKET, [m], ']');

    p.complete(m, INDEX_EXPRESSION_NODE)
}

pub(crate) fn function_call_expression(p: &mut Context, m: Marker) -> CompleteMarker {
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        expression(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, FUNCTION_CALL_EXPRESSION_NODE)
}

pub(crate) fn try_expression(p: &mut Context, m: Marker) -> CompleteMarker {
    p.complete(m, TRY_EXPRESSION_NODE)
}

pub(crate) fn await_expression(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, AWAIT, [m], "await");

    p.complete(m, AWAIT_EXPRESSION_NODE)
}

pub(crate) fn yield_expression(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, YIELD, [m], "yield");

    p.complete(m, YIELD_EXPRESSION_NODE)
}

pub(crate) fn field_expression(p: &mut Context, m: Marker) -> CompleteMarker {
    identifier(p);
    if p.at(LEFT_PAREN) {
        return method_call_expression(p, m);
    }

    return p.complete(m, FIELD_EXPRESSION_NODE);
}

pub(crate) fn method_call_expression(p: &mut Context, m: Marker) -> CompleteMarker {
    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        expression(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, METHOD_CALL_EXPRESSION_NODE)
}

pub(crate) fn primary_expression(p: &mut Context) -> CompleteMarker {
    match p.nth(0) {
        kind if matches!(kind, BREAK) => break_expression(p),
        kind if matches!(kind, CONTINUE) => continue_expression(p),
        kind if matches!(kind, RETURN) => return_expression(p),
        kind if matches!(kind, FOR) => for_expression(p),
        kind if matches!(kind, WHILE) => while_expression(p),
        kind if matches!(kind, LOOP) => loop_expression(p),
        kind if matches!(kind, IF) => if_expression(p),
        kind if matches!(kind, MATCH) => match_expression(p),
        kind if matches!(kind, LET) => let_expression(p),
        kind if matches!(kind, LEFT_BRACKET) => array_expression(p),
        kind if matches!(kind, LEFT_PAREN) => tuple_expression(p),
        kind if matches!(kind, LEFT_BRACE | HASH) => block_expression(p),
        kind if matches!(
            kind,
            TRUE | FALSE
                | CHARACTER_START
                | STRING_START
                | RAW_STRING_START
                | INTEGER_SEGMENT
                | BINARY_START
                | OCTAL_START
                | HEX_START
        ) =>
        {
            literal_expression(p)
        }
        kind if matches!(
            kind,
            COLON | IDENTIFIER | RAW_IDENTIFIER_START | SELF | SUPER | CRATE
        ) =>
        {
            path_expression(p)
        }
        _ => p.report(vec![], miette!("Expected expression")),
    }
}

pub(crate) fn break_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, BREAK, [m], "break");
    if PREFIX_EXPRESSION_FIRST.contains(p.nth(0)) {
        expression(p);
    }

    p.complete(m, BREAK_EXPRESSION_NODE)
}

pub(crate) fn continue_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, CONTINUE, [m], "continue");

    p.complete(m, CONTINUE_EXPRESSION_NODE)
}

pub(crate) fn return_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, RETURN, [m], "return");
    if PREFIX_EXPRESSION_FIRST.contains(p.nth(0)) {
        expression(p);
    }

    p.complete(m, RETURN_EXPRESSION_NODE)
}

pub(crate) fn for_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, FOR, [m], "for");
    pattern(p);
    expect!(p, IN, [m], "in");
    expression(p);
    block_expression(p);

    p.complete(m, FOR_EXPRESSION_NODE)
}

pub(crate) fn while_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, WHILE, [m], "while");
    expression(p);
    block_expression(p);

    p.complete(m, WHILE_EXPRESSION_NODE)
}

pub(crate) fn loop_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, LOOP, [m], "loop");
    block_expression(p);

    p.complete(m, LOOP_EXPRESSION_NODE)
}

pub(crate) fn if_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, IF, [m], "if");
    expression(p);
    block_expression(p);

    if p.eat(ELSE) {
        if p.at(IF) {
            if_expression(p);
        } else {
            block_expression(p);
        }
    }

    p.complete(m, IF_EXPRESSION_NODE)
}

pub(crate) fn match_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, MATCH, [m], "match");
    expression(p);
    expect!(p, LEFT_BRACE, [m], '{');
    while !current!(p, [RIGHT_BRACE, END_OF_FILE]) {
        match_arm(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACE, [m], '}');

    p.complete(m, MATCH_EXPRESSION_NODE)
}

pub(crate) fn match_arm(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    pattern(p);
    expect!(p, EQUAL, [m], "=>");
    expect!(p, RIGHT_CHEVRON, [m], '>');
    expression(p);

    p.complete(m, MATCH_ARM_NODE)
}

pub(crate) fn let_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, LET, [m], "let");
    pattern(p);
    if p.eat(COLON) {
        type_expression(p);
    }
    if p.eat(EQUAL) {
        expression(p);
    }

    p.complete(m, LET_EXPRESSION_NODE)
}

pub(crate) fn array_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, LEFT_BRACKET, [m], '[');
    while !current!(p, [RIGHT_BRACKET, END_OF_FILE]) {
        expression(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_BRACKET, [m], ']');

    p.complete(m, ARRAY_EXPRESSION_NODE)
}

pub(crate) fn literal_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    literal(p);

    p.complete(m, LITERAL_EXPRESSION_NODE)
}

pub(crate) fn block_expression(p: &mut Context) -> CompleteMarker {
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

    p.complete(m, BLOCK_EXPRESSION_NODE)
}

pub(crate) fn tuple_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, LEFT_PAREN, [m], '(');
    while !current!(p, [RIGHT_PAREN, END_OF_FILE]) {
        expression(p);
        if !p.eat(COMMA) {
            break;
        }
    }
    expect!(p, RIGHT_PAREN, [m], ')');

    p.complete(m, TUPLE_EXPRESSION_NODE)
}

pub(crate) fn path_expression(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    path(p);

    p.complete(m, PATH_EXPRESSION_NODE)
}

pub(crate) fn statement(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    if p.eat(SEMICOLON) {
        return p.complete(m, SEMICOLON_STATEMENT_NODE);
    }

    if p.eat(LET) {
        pattern(p);
        if p.eat(COLON) {
            type_expression(p);
        }
        if p.eat(EQUAL) {
            expression(p);
        }
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

    p.complete(m, DEFINITION_STATEMENT_NODE)
}

pub(crate) fn literal(p: &mut Context) -> CompleteMarker {
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

pub(crate) fn boolean_literal(p: &mut Context) -> CompleteMarker {
    if !current!(p, [TRUE, FALSE]) {
        return p.report(vec![], miette!("Expected boolean literal"));
    }

    let m = p.start();
    p.bump(); // eat 'true' or 'false'

    p.complete(m, BOOLEAN_LITERAL_NODE)
}

pub(crate) fn character_literal(p: &mut Context) -> CompleteMarker {
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

    p.complete(m, CHARACTER_LITERAL_NODE)
}

pub(crate) fn string_literal(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, STRING_START, [m], '"');

    while !current!(p, [STRING_END, END_OF_FILE]) {
        let m1 = p.start();

        if p.eat(STRING_SEGMENT) {
            p.complete(m1, STRING_LITERAL_TEXT_NODE);

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

        m1.terminate();

        break;
    }

    expect!(p, STRING_END, [m], '"');

    p.complete(m, STRING_LITERAL_NODE)
}

pub(crate) fn raw_string_literal(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, RAW_STRING_START, [m], "string start");
    expect!(p, RAW_STRING_SEGMENT, [m], "string segment");
    expect!(p, RAW_STRING_END, [m], "string end");

    p.complete(m, STRING_LITERAL_NODE)
}

pub(crate) fn numeric_literal(p: &mut Context) -> CompleteMarker {
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
        return p.complete(m, INTEGER_LITERAL_NODE);
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

    p.complete(m, FLOAT_LITERAL_NODE)
}

pub(crate) fn binary_literal(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, BINARY_START, [m], "0b");
    expect!(p, BINARY_SEGMENT, [m], "binary literal");
    while p.eat(NUMERIC_SEPARATOR) {
        if !p.eat(BINARY_SEGMENT) {
            break;
        }
    }

    p.complete(m, BINARY_NUMERIC_LITERAL_NODE)
}

pub(crate) fn octal_literal(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, OCTAL_START, [m], "0o");
    expect!(p, OCTAL_SEGMENT, [m], "octal literal");
    while p.eat(NUMERIC_SEPARATOR) {
        if !p.eat(OCTAL_SEGMENT) {
            break;
        }
    }

    p.complete(m, OCTAL_NUMERIC_LITERAL_NODE)
}

pub(crate) fn hex_literal(p: &mut Context) -> CompleteMarker {
    let m = p.start();

    expect!(p, HEX_START, [m], "0x");
    expect!(p, HEX_SEGMENT, [m], "hexadecimal literal");
    while p.eat(NUMERIC_SEPARATOR) {
        if !p.eat(HEX_SEGMENT) {
            break;
        }
    }

    p.complete(m, HEX_NUMERIC_LITERAL_NODE)
}
