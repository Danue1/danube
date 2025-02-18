use danubec_data_structure::BitMask;
use danubec_syntax::SyntaxKind;

const TOKEN_COUNT: usize = SyntaxKind::UNEXPECTED as usize + 1;
const BIT_MASK_SIZE: usize = TOKEN_COUNT / 64 + 1;

pub struct Tokens(BitMask<BIT_MASK_SIZE, TOKEN_COUNT>);

impl Tokens {
    pub const fn new(items: &[usize]) -> Self {
        Self(BitMask::new(items))
    }

    pub const fn concat(self, other: Self) -> Self {
        Self(self.0.concat(other.0))
    }

    pub const fn contains(&self, kind: SyntaxKind) -> bool {
        self.0.contains(kind as usize)
    }
}

#[macro_export]
macro_rules! tokens {
    ($($kind:ident,)+) => {{
        Tokens::new(&[$(danubec_syntax::SyntaxKind::$kind as usize,)+])
    }};
}
