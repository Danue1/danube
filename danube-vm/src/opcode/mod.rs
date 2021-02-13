use std::borrow::Cow;

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

    AddFloat,
    SubFloat,
    MulFloat,
    DivFloat,

    Illegal(u8),
}

pub const HALTING: u8 = 0;

pub const JUMP: u8 = 1;
pub const JUMP_BACK: u8 = 2;
pub const JUMP_FRONT: u8 = 3;

pub const CONST_INT8: u8 = 4;
pub const CONST_INT16: u8 = 5;
pub const CONST_INT32: u8 = 6;
pub const CONST_INT64: u8 = 7;

pub const CONST_FLOAT32: u8 = 8;
pub const CONST_FLOAT64: u8 = 9;

pub const ADD_INT: u8 = 10;
pub const SUB_INT: u8 = 11;
pub const MUL_INT: u8 = 12;
pub const DIV_INT: u8 = 13;

pub const ADD_FLOAT: u8 = 14;
pub const SUB_FLOAT: u8 = 15;
pub const MUL_FLOAT: u8 = 16;
pub const DIV_FLOAT: u8 = 17;

pub const ILLEGAL: u8 = 255;

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        use Opcode::*;

        match opcode {
            HALTING => Halting,

            JUMP => Jump,
            JUMP_BACK => JumpBack,
            JUMP_FRONT => JumpFront,

            CONST_INT8 => ConstInt8,
            CONST_INT16 => ConstInt16,
            CONST_INT32 => ConstInt32,
            CONST_INT64 => ConstInt64,

            CONST_FLOAT32 => ConstFloat32,
            CONST_FLOAT64 => ConstFloat64,

            ADD_INT => AddInt,
            SUB_INT => SubInt,
            MUL_INT => MulInt,
            DIV_INT => DivInt,

            ADD_FLOAT => AddFloat,
            SUB_FLOAT => SubFloat,
            MUL_FLOAT => MulFloat,
            DIV_FLOAT => DivFloat,

            _ => Illegal(opcode),
        }
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        use Opcode::*;

        match opcode {
            Halting => HALTING,

            Jump => JUMP,
            JumpBack => JUMP_BACK,
            JumpFront => JUMP_FRONT,

            ConstInt8 => CONST_INT8,
            ConstInt16 => CONST_INT16,
            ConstInt32 => CONST_INT32,
            ConstInt64 => CONST_INT64,

            ConstFloat32 => CONST_FLOAT32,
            ConstFloat64 => CONST_FLOAT64,

            AddInt => ADD_INT,
            SubInt => SUB_INT,
            MulInt => MUL_INT,
            DivInt => DIV_INT,

            AddFloat => ADD_FLOAT,
            SubFloat => SUB_FLOAT,
            MulFloat => MUL_FLOAT,
            DivFloat => DIV_FLOAT,

            Illegal(_) => ILLEGAL,
        }
    }
}

pub const OPCODE_HALTING: &'static str = "HLT";

pub const OPCODE_JUMP: &'static str = "JMP";
pub const OPCODE_JUMP_BACK: &'static str = "JMPB";
pub const OPCODE_JUMP_FRONT: &'static str = "JMPF";

pub const OPCODE_CONST_INT8: &'static str = "CONSTF32";
pub const OPCODE_CONST_INT16: &'static str = "CONSTF64";
pub const OPCODE_CONST_INT32: &'static str = "CONSTI8";
pub const OPCODE_CONST_INT64: &'static str = "CONSTI16";
pub const OPCODE_CONST_FLOAT32: &'static str = "CONSTI32";
pub const OPCODE_CONST_FLOAT64: &'static str = "CONSTI64";

pub const OPCODE_ADD_INT: &'static str = "ADDI";
pub const OPCODE_SUB_INT: &'static str = "SUBI";
pub const OPCODE_MUL_INT: &'static str = "MULI";
pub const OPCODE_DIV_INT: &'static str = "DIVI";

pub const OPCODE_ADD_FLOAT: &'static str = "ADDF";
pub const OPCODE_SUB_FLOAT: &'static str = "SUBF";
pub const OPCODE_MUL_FLOAT: &'static str = "MULF";
pub const OPCODE_DIV_FLOAT: &'static str = "DIVF";

impl From<Opcode> for Cow<'static, str> {
    fn from(opcode: Opcode) -> Self {
        use Opcode::*;

        let opcode = match opcode {
            Halting => OPCODE_HALTING,

            Jump => OPCODE_JUMP,
            JumpBack => OPCODE_JUMP_BACK,
            JumpFront => OPCODE_JUMP_FRONT,

            ConstFloat32 => OPCODE_CONST_INT8,
            ConstFloat64 => OPCODE_CONST_INT16,

            ConstInt8 => OPCODE_CONST_INT32,
            ConstInt16 => OPCODE_CONST_INT64,
            ConstInt32 => OPCODE_CONST_FLOAT32,
            ConstInt64 => OPCODE_CONST_FLOAT64,

            AddInt => OPCODE_ADD_INT,
            SubInt => OPCODE_SUB_INT,
            MulInt => OPCODE_MUL_INT,
            DivInt => OPCODE_DIV_INT,

            AddFloat => OPCODE_ADD_FLOAT,
            SubFloat => OPCODE_SUB_FLOAT,
            MulFloat => OPCODE_MUL_FLOAT,
            DivFloat => OPCODE_DIV_FLOAT,

            Illegal(opcode) => return Cow::Owned(format!("ILG({})", opcode)),
        };

        Cow::Borrowed(opcode)
    }
}
