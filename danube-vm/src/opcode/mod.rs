use Opcode::*;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    Halting,

    Jump,
    JumpBack,
    JumpFront,

    ConstInt8,
    ConstInt16,
    ConstInt32,
    ConstInt64,

    ConstFloat32,
    ConstFloat64,

    AddInt,
    SubInt,
    MulInt,
    DivInt,
    ModInt,
    ExpInt,

    AddFloat,
    SubFloat,
    MulFloat,
    DivFloat,
    ModFloat,
    ExpFloat,
}

pub const HALTING: u8 = Halting as u8;

pub const JUMP: u8 = Jump as u8;
pub const JUMP_BACK: u8 = JumpBack as u8;
pub const JUMP_FRONT: u8 = JumpFront as u8;

pub const CONST_INT8: u8 = ConstInt8 as u8;
pub const CONST_INT16: u8 = ConstInt16 as u8;
pub const CONST_INT32: u8 = ConstInt32 as u8;
pub const CONST_INT64: u8 = ConstInt64 as u8;

pub const CONST_FLOAT32: u8 = ConstFloat32 as u8;
pub const CONST_FLOAT64: u8 = ConstFloat64 as u8;

pub const ADD_INT: u8 = AddInt as u8;
pub const SUB_INT: u8 = SubInt as u8;
pub const MUL_INT: u8 = MulInt as u8;
pub const DIV_INT: u8 = DivInt as u8;
pub const MOD_INT: u8 = ModInt as u8;
pub const EXP_INT: u8 = ExpInt as u8;

pub const ADD_FLOAT: u8 = AddFloat as u8;
pub const SUB_FLOAT: u8 = SubFloat as u8;
pub const MUL_FLOAT: u8 = MulFloat as u8;
pub const DIV_FLOAT: u8 = DivFloat as u8;
pub const MOD_FLOAT: u8 = ModFloat as u8;
pub const EXP_FLOAT: u8 = ExpFloat as u8;

impl std::convert::TryFrom<u8> for Opcode {
    type Error = u8;

    fn try_from(opcode: u8) -> Result<Self, Self::Error> {
        match opcode {
            HALTING => Ok(Halting),

            JUMP => Ok(Jump),
            JUMP_BACK => Ok(JumpBack),
            JUMP_FRONT => Ok(JumpFront),

            CONST_INT8 => Ok(ConstInt8),
            CONST_INT16 => Ok(ConstInt16),
            CONST_INT32 => Ok(ConstInt32),
            CONST_INT64 => Ok(ConstInt64),

            CONST_FLOAT32 => Ok(ConstFloat32),
            CONST_FLOAT64 => Ok(ConstFloat64),

            ADD_INT => Ok(AddInt),
            SUB_INT => Ok(SubInt),
            MUL_INT => Ok(MulInt),
            DIV_INT => Ok(DivInt),
            MOD_INT => Ok(ModInt),
            EXP_INT => Ok(ExpInt),

            ADD_FLOAT => Ok(AddFloat),
            SUB_FLOAT => Ok(SubFloat),
            MUL_FLOAT => Ok(MulFloat),
            DIV_FLOAT => Ok(DivFloat),
            MOD_FLOAT => Ok(ModFloat),
            EXP_FLOAT => Ok(ExpFloat),

            _ => Err(opcode),
        }
    }
}
