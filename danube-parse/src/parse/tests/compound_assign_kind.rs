use super::Context;
use danube_ast::AssignKind;

macro_rules! assigned_kind {
    ($($expr:expr => $kind:ident,)+) => {
        $(
            assert_eq!(
                Ok(AssignKind::$kind),
                Context::new($expr).parse_assign_kind()
            );
        )+
    };
}

#[test]
fn simple() {
    assigned_kind! {
        "+=" => Add,
        "-=" => Sub,
        "**=" => Exp,
        "*=" => Mul,
        "/=" => Div,
        "%=" => Mod,
        "&&=" => And,
        "||=" => Or,

        "&=" => BitAnd,
        "|=" => BitOr,
        "^=" => BitXor,
        "~=" => BitNot,
        "<<=" => BitLeft,
        ">>=" => BitRight,
    };
}
