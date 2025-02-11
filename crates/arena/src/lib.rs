pub trait Index: Copy + PartialEq + Eq + std::hash::Hash {
    fn from_usize(index: usize) -> Self;

    fn as_usize(self) -> usize;

    #[inline]
    fn index(self) -> usize {
        self.as_usize()
    }
}

pub struct Arena<I, T> {
    items: Vec<T>,
    _phantom: std::marker::PhantomData<I>,
}

impl<I, T> Arena<I, T> {
    pub const fn new() -> Self {
        Arena {
            items: vec![],
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn alloc(&mut self, item: T) -> I
    where
        I: Index,
    {
        let id = I::from_usize(self.items.len());
        self.items.push(item);

        id
    }

    #[inline]
    pub fn get(&self, id: I) -> Option<&T>
    where
        I: Index,
    {
        self.items.get(id.as_usize())
    }
}

impl<I, T> std::ops::Index<I> for Arena<I, T>
where
    I: Index,
{
    type Output = T;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        &self.items[index.index()]
    }
}

#[macro_export]
macro_rules! new_index {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(usize);

        impl $crate::Index for $name {
            #[inline]
            fn from_usize(index: usize) -> Self {
                $name(index)
            }

            #[inline]
            fn as_usize(self) -> usize {
                self.0
            }
        }
    };
}
