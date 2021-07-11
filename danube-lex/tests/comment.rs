use danube_lex::LexIter;

macro_rules! comment {
    ($($expr:expr,)+) => {
        $(
            assert_eq!(None, LexIter::new($expr).next());
        )+
    };
}

#[test]
fn simple() {
    comment!["//", "//Hello, World!", "// Hello, World!",];
}
