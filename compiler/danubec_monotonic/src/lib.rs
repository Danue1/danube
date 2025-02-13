#[macro_export]
macro_rules! new_monotonic {
    ($symbol:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $symbol(usize);

        impl $symbol {
            pub fn new() -> Self {
                use std::sync::atomic::{AtomicUsize, Ordering};

                static COUNTER: AtomicUsize = AtomicUsize::new(0);

                Self(COUNTER.fetch_add(1, Ordering::SeqCst))
            }

            #[inline]
            pub const fn as_usize(self) -> usize {
                self.0
            }
        }
    };
}
