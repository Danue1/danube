#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Instruction {
    Halting,

    ConstInt8(Register, i8),
    ConstInt16(Register, i16),
    ConstInt32(Register, i32),
    ConstInt64(Register, i64),

    ConstFloat32(Register, f32),
    ConstFloat64(Register, f64),

    Jump(Register),
    JumpBack(Register),
    JumpFront(Register),

    AddInt(Register, Register, Register),
    SubInt(Register, Register, Register),
    MulInt(Register, Register, Register),
    DivInt(Register, Register, Register),

    AddFloat(Register, Register, Register),
    SubFloat(Register, Register, Register),
    MulFloat(Register, Register, Register),
    DivFloat(Register, Register, Register),

    Illegal(u8),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Register(usize);

impl std::ops::Deref for Register {
    type Target = usize;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<i64> for Register {
    #[inline]
    fn from(register: i64) -> Self {
        Register(register as usize)
    }
}

impl From<u8> for Register {
    #[inline]
    fn from(register: u8) -> Self {
        Register(register as usize)
    }
}
