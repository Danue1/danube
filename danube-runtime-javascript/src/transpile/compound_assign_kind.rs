use super::*;

pub(super) fn transpile_compound_assign_kind(
  node: &CompoundAssignKind,
  c: &mut Context,
) {
  let _ = match node {
    CompoundAssignKind::AddAssign => write!(c, "+="),
    CompoundAssignKind::SubAssign => write!(c, "-="),
    CompoundAssignKind::ExpAssign => write!(c, "**="),
    CompoundAssignKind::MulAssign => write!(c, "*="),
    CompoundAssignKind::DivAssign => write!(c, "/="),
    CompoundAssignKind::ModAssign => write!(c, "%="),
    CompoundAssignKind::AndAssign => write!(c, "&&="),
    CompoundAssignKind::OrAssign => write!(c, "||="),

    CompoundAssignKind::BitAndAssign => write!(c, "&="),
    CompoundAssignKind::BitOrAssign => write!(c, "|="),
    CompoundAssignKind::BitXorAssign => write!(c, "^="),
    CompoundAssignKind::BitLeftAssign => write!(c, "<<="),
    CompoundAssignKind::BitRightAssign => write!(c, ">>="),
  };
}
