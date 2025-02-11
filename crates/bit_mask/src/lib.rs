use std::marker::PhantomData;

// from https://github.com/rust-lang/rust-analyzer/blob/35181e167efb94d5090df588e6af9f93250421f3/crates/parser/src/token_set.rs#L5-L7.
pub struct BitMask<const N: usize, const MAX: usize>([u64; N], PhantomData<[u64; MAX]>);

impl<const N: usize, const MAX: usize> BitMask<N, MAX> {
    pub const EMPTY: Self = Self([0; N], PhantomData);

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
