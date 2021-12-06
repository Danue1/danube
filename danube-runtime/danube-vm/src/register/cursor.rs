#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cursor<T: Sized>(pub(crate) usize, std::marker::PhantomData<T>);

impl Cursor<i64> {
    #[inline]
    pub const fn new(cursor: u8) -> Self {
        Cursor(cursor as usize, std::marker::PhantomData)
    }
}

impl Cursor<f64> {
    #[inline]
    pub const fn new(cursor: u8) -> Self {
        Cursor(cursor as usize, std::marker::PhantomData)
    }
}
