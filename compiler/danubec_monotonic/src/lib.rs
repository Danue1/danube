#![warn(clippy::all)]

#[macro_export]
macro_rules! new_monotonic {
    ($symbol:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $symbol(usize);

        impl $symbol {
            pub const ZERO: Self = Self(0);

            pub const MAX: Self = Self(usize::MAX);

            pub fn new() -> Self {
                use std::sync::atomic::{AtomicUsize, Ordering};

                static COUNTER: AtomicUsize = AtomicUsize::new(1);

                Self(COUNTER.fetch_add(1, Ordering::SeqCst))
            }

            #[inline]
            pub const fn as_usize(self) -> usize {
                self.0
            }
        }
    };
}
