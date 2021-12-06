use crate::{Cursor, BIT};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RegisterList<T: Sized>([T; BIT]);

impl RegisterList<i64> {
    #[inline]
    pub const fn new() -> Self {
        RegisterList([0; BIT])
    }
}

impl RegisterList<f64> {
    #[inline]
    pub const fn new() -> Self {
        RegisterList([0.0; BIT])
    }
}

impl std::ops::Index<Cursor<i64>> for RegisterList<i64> {
    type Output = i64;

    #[inline]
    fn index(&self, cursor: Cursor<i64>) -> &Self::Output {
        &self.0[cursor.0]
    }
}

impl std::ops::Index<Cursor<f64>> for RegisterList<f64> {
    type Output = f64;

    #[inline]
    fn index(&self, cursor: Cursor<f64>) -> &Self::Output {
        &self.0[cursor.0]
    }
}

impl std::ops::IndexMut<Cursor<i64>> for RegisterList<i64> {
    #[inline]
    fn index_mut(&mut self, cursor: Cursor<i64>) -> &mut Self::Output {
        &mut self.0[cursor.0]
    }
}

impl std::ops::IndexMut<Cursor<f64>> for RegisterList<f64> {
    #[inline]
    fn index_mut(&mut self, cursor: Cursor<f64>) -> &mut Self::Output {
        &mut self.0[cursor.0]
    }
}

#[cfg(test)]
impl<T> std::ops::Index<usize> for RegisterList<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
