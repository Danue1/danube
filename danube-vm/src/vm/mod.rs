mod macros;
#[cfg(test)]
mod tests;

use crate::{Instruction, Opcode};

const BIT: usize = 64;

#[derive(Debug)]
pub struct VM {
    register_list: [i64; BIT],
    float_register_list: [f64; BIT],
    program_counter: usize,
    program: Vec<u8>,
}

macro_rules! debug_info {
    ($($args:tt)*) => {
        if cfg!(debug_assertions) {
            println!($($args)*);
        }
    };
}

macro_rules! error {
    ($($args:tt)*) => {
        println!($($args)*);
    };
}

macro_rules! program_counter {
    ($self:ident $(, $counter:expr)?) => {
        $self.program[$self.program_counter $(+ $counter)?]
    };
}

impl VM {
    #[inline]
    pub const fn new() -> Self {
        VM {
            register_list: [0; BIT],
            float_register_list: [0.0; BIT],
            program_counter: 0,
            program: vec![],
        }
    }

    pub fn run(&mut self) -> u8 {
        match self.execute_instruction() {
            Some(code) => code,
            None => self.run(),
        }
    }

    pub fn run_once(&mut self) -> Option<u8> {
        self.execute_instruction()
    }

    fn execute_instruction(&mut self) -> Option<u8> {
        if self.program_counter >= self.program.len() {
            return Some(1);
        }

        match self.next_instruction() {
            Instruction::Halting => {
                debug_info!("Halting encountered.");
                return Some(0);
            }

            Instruction::Jump(register) => {
                self.program_counter = *register;
            }
            Instruction::JumpBack(register) => {
                self.program_counter -= *register;
            }
            Instruction::JumpFront(register) => {
                self.program_counter += *register;
            }

            Instruction::ConstInt8(register, constant) => {
                self.register_list[*register] = constant.into();
            }
            Instruction::ConstInt16(register, constant) => {
                self.register_list[*register] = constant.into();
            }
            Instruction::ConstInt32(register, constant) => {
                self.register_list[*register] = constant.into();
            }
            Instruction::ConstInt64(register, constant) => {
                self.register_list[*register] = constant.into();
            }

            Instruction::ConstFloat32(register, constant) => {
                self.float_register_list[*register] = constant as f64;
            }
            Instruction::ConstFloat64(register, constant) => {
                self.float_register_list[*register] = constant as f64;
            }

            Instruction::AddInt(register1, register2, register3) => {
                let register1 = self.register_list[*register1];
                let register2 = self.register_list[*register2];
                self.register_list[*register3] = register1 + register2;
            }
            Instruction::SubInt(register1, register2, register3) => {
                let register1 = self.register_list[*register1];
                let register2 = self.register_list[*register2];
                self.register_list[*register3] = register1 - register2;
            }
            Instruction::MulInt(register1, register2, register3) => {
                let register1 = self.register_list[*register1];
                let register2 = self.register_list[*register2];
                self.register_list[*register3] = register1 * register2;
            }
            Instruction::DivInt(register1, register2, register3) => {
                let register1 = self.register_list[*register1];
                let register2 = self.register_list[*register2];
                self.register_list[*register3] = register1 / register2;
            }

            Instruction::AddFloat(register1, register2, register3) => {
                let register1 = self.float_register_list[*register1];
                let register2 = self.float_register_list[*register2];
                self.float_register_list[*register3] = register1 + register2;
            }
            Instruction::SubFloat(register1, register2, register3) => {
                let register1 = self.float_register_list[*register1];
                let register2 = self.float_register_list[*register2];
                self.float_register_list[*register3] = register1 - register2;
            }
            Instruction::MulFloat(register1, register2, register3) => {
                let register1 = self.float_register_list[*register1];
                let register2 = self.float_register_list[*register2];
                self.float_register_list[*register3] = register1 * register2;
            }
            Instruction::DivFloat(register1, register2, register3) => {
                let register1 = self.float_register_list[*register1];
                let register2 = self.float_register_list[*register2];
                self.float_register_list[*register3] = register1 / register2;
            }

            Instruction::Illegal(opcode) => {
                error!("Unrecognized OPCODE({}) found! Terminating!", opcode);
                return Some(1);
            }
        }

        None
    }

    pub fn append_program(&mut self, program: &[u8]) {
        self.program.extend(program);
    }

    fn next_instruction(&mut self) -> Instruction {
        match self.next_opcode() {
            Opcode::Halting => Instruction::Halting,
            Opcode::Jump => {
                Instruction::Jump(self.register_list[self.next_1_byte() as usize].into())
            }
            Opcode::JumpBack => {
                Instruction::JumpBack(self.register_list[self.next_1_byte() as usize].into())
            }
            Opcode::JumpFront => {
                Instruction::JumpFront(self.register_list[self.next_1_byte() as usize].into())
            }
            Opcode::ConstInt8 => {
                Instruction::ConstInt8(self.next_1_byte().into(), self.next_1_byte() as i8)
            }
            Opcode::ConstInt16 => {
                Instruction::ConstInt16(self.next_1_byte().into(), self.next_2_bytes() as i16)
            }
            Opcode::ConstInt32 => {
                Instruction::ConstInt32(self.next_1_byte().into(), self.next_4_bytes() as i32)
            }
            Opcode::ConstInt64 => {
                Instruction::ConstInt64(self.next_1_byte().into(), self.next_8_bytes() as i64)
            }
            Opcode::ConstFloat32 => Instruction::ConstFloat32(
                self.next_1_byte().into(),
                f32::from_bits(self.next_4_bytes()),
            ),
            Opcode::ConstFloat64 => Instruction::ConstFloat64(
                self.next_1_byte().into(),
                f64::from_bits(self.next_8_bytes()),
            ),
            Opcode::AddInt => Instruction::AddInt(
                self.next_1_byte().into(),
                self.next_1_byte().into(),
                self.next_1_byte().into(),
            ),
            Opcode::SubInt => Instruction::SubInt(
                self.next_1_byte().into(),
                self.next_1_byte().into(),
                self.next_1_byte().into(),
            ),
            Opcode::MulInt => Instruction::MulInt(
                self.next_1_byte().into(),
                self.next_1_byte().into(),
                self.next_1_byte().into(),
            ),
            Opcode::DivInt => Instruction::DivInt(
                self.next_1_byte().into(),
                self.next_1_byte().into(),
                self.next_1_byte().into(),
            ),
            Opcode::AddFloat => Instruction::AddFloat(
                self.next_1_byte().into(),
                self.next_1_byte().into(),
                self.next_1_byte().into(),
            ),
            Opcode::SubFloat => Instruction::SubFloat(
                self.next_1_byte().into(),
                self.next_1_byte().into(),
                self.next_1_byte().into(),
            ),
            Opcode::MulFloat => Instruction::MulFloat(
                self.next_1_byte().into(),
                self.next_1_byte().into(),
                self.next_1_byte().into(),
            ),
            Opcode::DivFloat => Instruction::DivFloat(
                self.next_1_byte().into(),
                self.next_1_byte().into(),
                self.next_1_byte().into(),
            ),
            Opcode::Illegal(opcode) => Instruction::Illegal(opcode),
        }
    }

    fn next_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(program_counter!(self));
        self.program_counter += 1;
        opcode
    }

    fn next_1_byte(&mut self) -> u8 {
        let result = program_counter!(self);
        self.program_counter += 1;
        result
    }

    fn next_2_bytes(&mut self) -> u16 {
        let result = ((program_counter!(self) as u16) << 8)
            | ((program_counter!(self, 1) as u16) << (8 * 0));
        self.program_counter += 2;
        result
    }

    fn next_4_bytes(&mut self) -> u32 {
        let result = ((program_counter!(self) as u32) << (8 * 3))
            | ((program_counter!(self, 1) as u32) << (8 * 2))
            | ((program_counter!(self, 2) as u32) << (8 * 1))
            | ((program_counter!(self, 3) as u32) << (8 * 0));
        self.program_counter += 4;
        result
    }

    fn next_8_bytes(&mut self) -> u64 {
        let result = ((program_counter!(self) as u64) << (8 * 7))
            | ((program_counter!(self, 1) as u64) << (8 * 6))
            | ((program_counter!(self, 2) as u64) << (8 * 5))
            | ((program_counter!(self, 3) as u64) << (8 * 4))
            | ((program_counter!(self, 4) as u64) << (8 * 3))
            | ((program_counter!(self, 5) as u64) << (8 * 2))
            | ((program_counter!(self, 6) as u64) << (8 * 1))
            | ((program_counter!(self, 7) as u64) << (8 * 0));
        self.program_counter += 8;
        result
    }
}
