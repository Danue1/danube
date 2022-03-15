#![warn(clippy::all)]

#[macro_export]
macro_rules! newtype_index {
    (
        $(
            pub struct $name:ident(usize);
        )+
    ) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name(usize);

            impl $name {
                pub const DUMMY: Self = Self(0);
            }

            impl From<usize> for $name {
                fn from(index: usize) -> Self {
                    $name(index)
                }
            }
        )+
    };
}

#[derive(Debug, Default)]
pub struct Resolver<T: Default + From<usize>> {
    index: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Default + From<usize>> Resolver<T> {
    pub fn new() -> Self {
        Resolver::default()
    }

    pub fn next_id(&mut self) -> T {
        let index = self.index;
        self.index += 1;
        T::from(index)
    }
}
