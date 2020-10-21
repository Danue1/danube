use super::*;

pub(super) fn parse_illegal(s: LexSpan) -> LexResult<()> {
    map(take(1usize), |_| ())(s)
}
