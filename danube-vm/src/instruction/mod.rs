use crate::Cursor;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Halting,

    ConstInt8(Cursor<i64>, i8),
    ConstInt16(Cursor<i64>, i16),
    ConstInt32(Cursor<i64>, i32),
    ConstInt64(Cursor<i64>, i64),

    ConstFloat32(Cursor<f64>, f32),
    ConstFloat64(Cursor<f64>, f64),

    Jump(Cursor<i64>),
    JumpBack(Cursor<i64>),
    JumpFront(Cursor<i64>),

    AddInt(Cursor<i64>, Cursor<i64>, Cursor<i64>),
    SubInt(Cursor<i64>, Cursor<i64>, Cursor<i64>),
    MulInt(Cursor<i64>, Cursor<i64>, Cursor<i64>),
    DivInt(Cursor<i64>, Cursor<i64>, Cursor<i64>),

    AddFloat(Cursor<f64>, Cursor<f64>, Cursor<f64>),
    SubFloat(Cursor<f64>, Cursor<f64>, Cursor<f64>),
    MulFloat(Cursor<f64>, Cursor<f64>, Cursor<f64>),
    DivFloat(Cursor<f64>, Cursor<f64>, Cursor<f64>),

    Illegal(u8),
}
