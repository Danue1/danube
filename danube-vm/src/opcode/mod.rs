#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    Halting,

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

    AddFloat,
    SubFloat,
    MulFloat,
    DivFloat,

    Jump,
    JumpBack,
    JumpFront,

    Illegal = 255,
}

pub const HALTING: u8 = Opcode::Halting as u8;

pub const JUMP: u8 = Opcode::Jump as u8;
pub const JUMP_BACK: u8 = Opcode::JumpBack as u8;
pub const JUMP_FRONT: u8 = Opcode::JumpFront as u8;

pub const CONST_INT8: u8 = Opcode::ConstInt8 as u8;
pub const CONST_INT16: u8 = Opcode::ConstInt16 as u8;
pub const CONST_INT32: u8 = Opcode::ConstInt32 as u8;
pub const CONST_INT64: u8 = Opcode::ConstInt64 as u8;

pub const CONST_FLOAT32: u8 = Opcode::ConstFloat32 as u8;
pub const CONST_FLOAT64: u8 = Opcode::ConstFloat64 as u8;

pub const ADD_INT: u8 = Opcode::AddInt as u8;
pub const SUB_INT: u8 = Opcode::SubInt as u8;
pub const MUL_INT: u8 = Opcode::MulInt as u8;
pub const DIV_INT: u8 = Opcode::DivInt as u8;

pub const ADD_FLOAT: u8 = Opcode::AddFloat as u8;
pub const SUB_FLOAT: u8 = Opcode::SubFloat as u8;
pub const MUL_FLOAT: u8 = Opcode::MulFloat as u8;
pub const DIV_FLOAT: u8 = Opcode::DivFloat as u8;

pub const ILLEGAL: u8 = Opcode::Illegal as u8;

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        match opcode {
            HALTING => Opcode::Halting,

            JUMP => Opcode::Jump,
            JUMP_BACK => Opcode::JumpBack,
            JUMP_FRONT => Opcode::JumpFront,

            CONST_INT8 => Opcode::ConstInt8,
            CONST_INT16 => Opcode::ConstInt16,
            CONST_INT32 => Opcode::ConstInt32,
            CONST_INT64 => Opcode::ConstInt64,

            CONST_FLOAT32 => Opcode::ConstFloat32,
            CONST_FLOAT64 => Opcode::ConstFloat64,

            ADD_INT => Opcode::AddInt,
            SUB_INT => Opcode::SubInt,
            MUL_INT => Opcode::MulInt,
            DIV_INT => Opcode::DivInt,

            ADD_FLOAT => Opcode::AddFloat,
            SUB_FLOAT => Opcode::SubFloat,
            MUL_FLOAT => Opcode::MulFloat,
            DIV_FLOAT => Opcode::DivFloat,

            _ => Opcode::Illegal,
        }
    }
}

impl From<Opcode> for &'static str {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::Halting => "HLT",

            Opcode::Jump => "JMP",
            Opcode::JumpBack => "JMPB",
            Opcode::JumpFront => "JMPF",

            Opcode::ConstFloat32 => "CONSTF32",
            Opcode::ConstFloat64 => "CONSTF64",

            Opcode::ConstInt8 => "CONSTI8",
            Opcode::ConstInt16 => "CONSTI16",
            Opcode::ConstInt32 => "CONSTI32",
            Opcode::ConstInt64 => "CONSTI64",

            Opcode::AddInt => "ADDI",
            Opcode::SubInt => "SUBI",
            Opcode::MulInt => "MULI",
            Opcode::DivInt => "DIVI",

            Opcode::AddFloat => "ADDF",
            Opcode::SubFloat => "SUBF",
            Opcode::MulFloat => "MULF",
            Opcode::DivFloat => "DIVF",

            Opcode::Illegal => "ILG",
        }
    }
}
