use danube_lex::lex;

macro_rules! comment {
    ($($expr:expr,)+) => {
        $(
            assert_eq!(Ok(vec![]), lex($expr));
        )+
    };
}

#[test]
fn simple() {
    comment!["//", "//Hello, World!", "// Hello, World!",];
}
