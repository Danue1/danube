pub trait Index: Copy + PartialEq + Eq + std::hash::Hash + 'static {
    fn from_usize(index: usize) -> Self;

    fn as_usize(self) -> usize;

    #[inline]
    fn index(self) -> usize {
        self.as_usize()
    }
}

pub struct Arena<I, T> {
    raw: Vec<T>,
    _marker: std::marker::PhantomData<I>,
}

impl<I, T> Arena<I, T> {
    #[inline]
    pub const fn new() -> Self {
        Self {
            raw: vec![],
            _marker: std::marker::PhantomData,
        }
    }

    pub fn alloc(&mut self, item: T) -> I
    where
        I: crate::Index,
    {
        let index = self.next_index();
        self.raw.push(item);

        index
    }

    #[inline]
    pub fn next_index(&self) -> I
    where
        I: crate::Index,
    {
        I::from_usize(self.raw.len())
    }
}

impl<I, T> std::ops::Index<I> for Arena<I, T>
where
    I: crate::Index,
{
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.raw[index.index()]
    }
}

#[macro_export]
macro_rules! new_arena {
    ($index:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $index(usize);

        impl $index {
            pub fn new() -> Self {
                static mut COUNTER: usize = 0;

                unsafe {
                    let index = COUNTER;
                    COUNTER += 1;

                    Self(index)
                }
            }
        }

        impl $crate::Index for $index {
            #[inline]
            fn from_usize(index: usize) -> Self {
                Self(index)
            }

            #[inline]
            fn as_usize(self) -> usize {
                self.0
            }
        }
    };
}
