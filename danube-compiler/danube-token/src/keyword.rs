use crate::SymbolInterner;
use std::collections::HashMap;

/// Define reserved keywords.
///
/// ex)
/// ```rust
/// use crate::{Symbol, SymbolIndex};
///
/// pub const IF = Symbol(SymbolIndex { index: 0 });
/// ```
macro_rules! reserve {
    (
        Keywords {
            $($keyword:ident: $string:expr,)+
        }
        Symbols {
            $($symbol:ident,)+
        }
    ) => {
        #[allow(non_upper_case_globals)]
        pub mod keywords {
            use crate::{Symbol, SymbolIndex};

            const_keywords!(@step 0; $($keyword: $string,)+);
        }

        #[allow(non_upper_case_globals)]
        pub mod symbols {
            use crate::{Symbol, SymbolIndex};

            const_symbols!(@step 64; $($symbol;)+);
        }

        impl Default for SymbolInterner {
            fn default() -> Self {
                #[allow(non_upper_case_globals)]
                mod keyword_literals {
                    $(pub const $keyword: &'static str = $string;)+
                }

                #[allow(non_upper_case_globals)]
                mod symbol_literals {
                    $(pub const $symbol: &'static str = stringify!($symbol);)+
                }

                const CURRENT_KEYWORD_COUNT: usize = count_keywords!(@step 0; $($keyword;)+);
                const MAXIMUM_KEYWORD_COUNT: usize = 64;
                const DUMMY: &'static str = "";

                let mut strings = Vec::new();
                let mut symbols = HashMap::new();

                $(strings.push(keyword_literals::$keyword.to_owned());)+
                for _ in CURRENT_KEYWORD_COUNT..MAXIMUM_KEYWORD_COUNT {
                    strings.push(DUMMY.to_owned());
                }
                $(strings.push(symbol_literals::$symbol.to_owned());)+

                $(symbols.insert(keyword_literals::$keyword.to_owned(), keywords::$keyword);)+
                $(symbols.insert(symbol_literals::$symbol.to_owned(), symbols::$symbol);)+

                SymbolInterner {
                    strings,
                    symbols,
                }
            }
        }
    };
}

macro_rules! const_keywords {
    (@step $_index:expr;) => {
        //
    };

    (@step $index:expr; $keyword:ident: $string:expr, $($tail_keyword:ident: $tail_string:expr,)*) => {
        pub const $keyword: Symbol = Symbol(SymbolIndex { index: $index });
        const_keywords!(@step $index + 1; $($tail_keyword: $tail_string,)*);
    };
}

macro_rules! const_symbols {
    (@step $_index:expr;) => {
        //
    };

    (@step $index:expr; $symbol:ident; $($tail_symbol:ident;)*) => {
        pub const $symbol: Symbol = Symbol(SymbolIndex { index: $index });
        const_symbols!(@step $index + 1; $($tail_symbol;)*);
    };
}

macro_rules! count_keywords {
    (@step $index:expr;) => {
        $index
    };

    (@step $index:expr; $_keyword:ident; $($tail_keyword:ident;)*) => {
        count_keywords!(@step $index + 1; $($tail_keyword;)*)
    }
}

reserve! {
    Keywords {
        // Special keywords
        Empty: "",
        Placeholder: "_",

        // Stable keywords
        As: "as",
        Async: "async",
        Await: "await",
        Break: "break",
        Const: "const",
        Continue: "continue",
        Else: "else",
        Enum: "enum",
        False: "false",
        Fn: "fn",
        For: "for",
        If: "if",
        Impl: "impl",
        In: "in",
        Let: "let",
        Loop: "loop",
        Match: "match",
        Mut: "mut",
        Pub: "pub",
        Package: "package",
        Return: "return",
        SelfUpper: "Self",
        SelfLower: "self",
        Static: "static",
        Struct: "struct",
        Super: "super",
        Trait: "trait",
        True: "true",
        Type: "type",
        Use: "use",
        Where: "where",
        While: "while",

        // Unstable keywords
        Yield: "yield",

        // Contextual keywords
        Default: "default",
    }

    Symbols {
        bool,
        char,
        Err,
        f8,
        f16,
        f32,
        f64,
        i8,
        i16,
        i32,
        i64,
        isize,
        None,
        Ok,
        Some,
        string,
        u8,
        u16,
        u32,
        u64,
        usize,
    }
}
