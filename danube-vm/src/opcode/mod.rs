#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Opcode {
    Load8,
    Load16,
    Load32,
    Load64,

    Add,
    Sub,
    Mul,
    Div,

    LoadFloat32,
    LoadFloat64,

    AddFloat,
    SubFloat,
    MulFloat,
    DivFloat,

    Jump,
    JumpBack,
    JumpFront,

    Halting,
    Illegal,
}

pub const LOAD8: u8 = Opcode::Load8 as u8;
pub const LOAD16: u8 = Opcode::Load16 as u8;
pub const LOAD32: u8 = Opcode::Load32 as u8;
pub const LOAD64: u8 = Opcode::Load64 as u8;

pub const ADD: u8 = Opcode::Add as u8;
pub const SUB: u8 = Opcode::Sub as u8;
pub const MUL: u8 = Opcode::Mul as u8;
pub const DIV: u8 = Opcode::Div as u8;

pub const LOAD_FLOAT32: u8 = Opcode::LoadFloat32 as u8;
pub const LOAD_FLOAT64: u8 = Opcode::LoadFloat64 as u8;

pub const ADD_FLOAT: u8 = Opcode::AddFloat as u8;
pub const SUB_FLOAT: u8 = Opcode::SubFloat as u8;
pub const MUL_FLOAT: u8 = Opcode::MulFloat as u8;
pub const DIV_FLOAT: u8 = Opcode::DivFloat as u8;

pub const JUMP: u8 = Opcode::Jump as u8;
pub const JUMP_BACK: u8 = Opcode::JumpBack as u8;
pub const JUMP_FRONT: u8 = Opcode::JumpFront as u8;

pub const HALTING: u8 = Opcode::Halting as u8;
pub const ILLEGAL: u8 = Opcode::Illegal as u8;

impl From<u8> for Opcode {
    fn from(opcode: u8) -> Self {
        match opcode {
            LOAD8 => Opcode::Load8,
            LOAD16 => Opcode::Load16,
            LOAD32 => Opcode::Load32,
            LOAD64 => Opcode::Load64,

            ADD => Opcode::Add,
            SUB => Opcode::Sub,
            MUL => Opcode::Mul,
            DIV => Opcode::Div,

            LOAD_FLOAT32 => Opcode::LoadFloat32,
            LOAD_FLOAT64 => Opcode::LoadFloat64,

            ADD_FLOAT => Opcode::AddFloat,
            SUB_FLOAT => Opcode::SubFloat,
            MUL_FLOAT => Opcode::MulFloat,
            DIV_FLOAT => Opcode::DivFloat,

            JUMP => Opcode::Jump,
            JUMP_BACK => Opcode::JumpBack,
            JUMP_FRONT => Opcode::JumpFront,

            HALTING => Opcode::Halting,
            _ => Opcode::Illegal,
        }
    }
}

impl From<Opcode> for &'static str {
    fn from(opcode: Opcode) -> Self {
        match opcode {
            Opcode::Load8 => "LOAD8",
            Opcode::Load16 => "LOAD16",
            Opcode::Load32 => "LOAD32",
            Opcode::Load64 => "LOAD64",

            Opcode::Add => "ADD",
            Opcode::Sub => "SUB",
            Opcode::Mul => "MUL",
            Opcode::Div => "DIV",

            Opcode::LoadFloat32 => "LOADF32",
            Opcode::LoadFloat64 => "LOADF64",

            Opcode::AddFloat => "ADDF",
            Opcode::SubFloat => "SUBF",
            Opcode::MulFloat => "MULF",
            Opcode::DivFloat => "DIVF",

            Opcode::Jump => "JMP",
            Opcode::JumpBack => "JMPB",
            Opcode::JumpFront => "JMPF",

            Opcode::Halting => "HLT",
            Opcode::Illegal => "ILG",
        }
    }
}
