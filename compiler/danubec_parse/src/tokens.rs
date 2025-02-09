use danubec_syntax::SyntaxKind;

const TOKEN_COUNT: usize = SyntaxKind::UNEXPECTED as usize + 1;
const BIT_MASK_SIZE: usize = TOKEN_COUNT / 64 + 1;

pub struct Tokens(bit_mask::BitMask<BIT_MASK_SIZE, TOKEN_COUNT>);

impl Tokens {
    pub const fn new(items: &[usize]) -> Self {
        Self(bit_mask::BitMask::new(items))
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
        Tokens::new(&[$(SyntaxKind::$kind as usize,)+])
    }};
}
