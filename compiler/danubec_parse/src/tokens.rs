use danubec_syntax::SyntaxKind;
use std::marker::PhantomData;

const TOKEN_COUNT: usize = SyntaxKind::ERROR as usize + 1;
const BIT_MASK_SIZE: usize = TOKEN_COUNT / 64 + 1;

// from https://github.com/rust-lang/rust-analyzer/blob/35181e167efb94d5090df588e6af9f93250421f3/crates/parser/src/token_set.rs#L5-L7.
pub struct BitMask<const N: usize, const MAX: usize>([u64; N], PhantomData<[u64; MAX]>);

impl<const N: usize, const MAX: usize> BitMask<N, MAX> {
    pub const fn new(items: &[usize]) -> Self {
        let mut set = [0; N];
        let mut index = 0;
        while index < items.len() {
            let item = items[index];
            debug_assert!(item < MAX, "Expected a token `SyntaxKind` in the lexer");
            index += 1;

            let index = item / 64;
            debug_assert!(index < N, "Expected a token `SyntaxKind` in the lexer");

            set[index] |= 1 << (item % 64);
        }

        Self(set, PhantomData)
    }

    pub const fn concat(self, other: Self) -> Self {
        let mut set = [0; N];
        let mut index = 0;
        while index < N {
            set[index] = self.0[index] | other.0[index];
            index += 1;
        }

        Self(set, PhantomData)
    }

    pub const fn contains(&self, item: usize) -> bool {
        debug_assert!(item < MAX, "Expected a token `SyntaxKind` in the lexer");

        let index = item / 64;
        debug_assert!(index < N, "Expected a token `SyntaxKind` in the lexer");

        let mask = 1 << (item % 64);

        self.0[index] & mask != 0
    }
}

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
    ($($kind:ident),+ $(,)?) => {{
        crate::tokens::Tokens::new(&[$(danubec_syntax::SyntaxKind::$kind as usize,)+])
    }};
}
