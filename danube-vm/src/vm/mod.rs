mod macros;
#[cfg(test)]
mod tests;

use crate::{Cursor, Instruction, Opcode, RegisterList};

#[derive(Debug)]
pub struct VM {
    register_list: RegisterList<i64>,
    float_register_list: RegisterList<f64>,
    program_counter: usize,
    program: Vec<u8>,
    equal_flag: bool,
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
            register_list: RegisterList::<i64>::new(),
            float_register_list: RegisterList::<f64>::new(),
            program_counter: 0,
            program: vec![],
            equal_flag: false,
        }
    }

    #[inline]
    pub fn append_program(&mut self, program: &[u8]) {
        self.program.extend(program);
    }

    #[inline]
    pub fn run(&mut self) -> u8 {
        loop {
            if let Some(code) = self.execute_instruction() {
                return code;
            }
        }
    }

    #[inline]
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

            Instruction::ConstInt8(cursor, constant) => {
                self.register_list[cursor] = constant.into();
            }
            Instruction::ConstInt16(cursor, constant) => {
                self.register_list[cursor] = constant.into();
            }
            Instruction::ConstInt32(cursor, constant) => {
                self.register_list[cursor] = constant.into();
            }
            Instruction::ConstInt64(cursor, constant) => {
                self.register_list[cursor] = constant.into();
            }

            Instruction::ConstFloat32(cursor, constant) => {
                self.float_register_list[cursor] = constant as f64;
            }
            Instruction::ConstFloat64(cursor, constant) => {
                self.float_register_list[cursor] = constant as f64;
            }

            Instruction::AddInt(cursor1, cursor2, cursor3) => {
                let register1 = self.register_list[cursor1];
                let register2 = self.register_list[cursor2];
                self.register_list[cursor3] = register1 + register2;
            }
            Instruction::SubInt(cursor1, cursor2, cursor3) => {
                let register1 = self.register_list[cursor1];
                let register2 = self.register_list[cursor2];
                self.register_list[cursor3] = register1 - register2;
            }
            Instruction::MulInt(cursor1, cursor2, cursor3) => {
                let register1 = self.register_list[cursor1];
                let register2 = self.register_list[cursor2];
                self.register_list[cursor3] = register1 * register2;
            }
            Instruction::DivInt(cursor1, cursor2, cursor3) => {
                let register1 = self.register_list[cursor1];
                let register2 = self.register_list[cursor2];
                self.register_list[cursor3] = register1 / register2;
            }
            Instruction::ModInt(cursor1, cursor2, cursor3) => {
                let register1 = self.register_list[cursor1];
                let register2 = self.register_list[cursor2];
                self.register_list[cursor3] = register1 % register2;
            }
            Instruction::ExpInt(cursor1, cursor2, cursor3) => {
                use std::convert::TryInto;

                let register1 = self.register_list[cursor1];
                let register2 = self.register_list[cursor2];
                self.register_list[cursor3] = register1.pow(register2.try_into().unwrap());
            }

            Instruction::AddFloat(cursor1, cursor2, cursor3) => {
                let register1 = self.float_register_list[cursor1];
                let register2 = self.float_register_list[cursor2];
                self.float_register_list[cursor3] = register1 + register2;
            }
            Instruction::SubFloat(cursor1, cursor2, cursor3) => {
                let register1 = self.float_register_list[cursor1];
                let register2 = self.float_register_list[cursor2];
                self.float_register_list[cursor3] = register1 - register2;
            }
            Instruction::MulFloat(cursor1, cursor2, cursor3) => {
                let register1 = self.float_register_list[cursor1];
                let register2 = self.float_register_list[cursor2];
                self.float_register_list[cursor3] = register1 * register2;
            }
            Instruction::DivFloat(cursor1, cursor2, cursor3) => {
                let register1 = self.float_register_list[cursor1];
                let register2 = self.float_register_list[cursor2];
                self.float_register_list[cursor3] = register1 / register2;
            }
            Instruction::ModFloat(cursor1, cursor2, cursor3) => {
                let register1 = self.float_register_list[cursor1];
                let register2 = self.float_register_list[cursor2];
                self.float_register_list[cursor3] = register1 % register2;
            }
            Instruction::ExpFloat(cursor1, cursor2, cursor3) => {
                let register1 = self.float_register_list[cursor1];
                let register2 = self.float_register_list[cursor2];
                self.float_register_list[cursor3] = register1.powf(register2);
            }

            Instruction::CompareInt(cursor1, cursor2) => {
                self.equal_flag = self.register_list[cursor1] == self.register_list[cursor2];
            }
            Instruction::CompareZeroInt(cursor) => {
                self.equal_flag = self.register_list[cursor] == 0;
            }
            Instruction::CompareNotZeroInt(cursor) => {
                self.equal_flag = self.register_list[cursor] != 0;
            }
            Instruction::GreaterThanInt(cursor1, cursor2) => {
                self.equal_flag = self.register_list[cursor1] > self.register_list[cursor2];
            }
            Instruction::GreaterThanOrEqualInt(cursor1, cursor2) => {
                self.equal_flag = self.register_list[cursor1] >= self.register_list[cursor2];
            }
            Instruction::LessThanInt(cursor1, cursor2) => {
                self.equal_flag = self.register_list[cursor1] < self.register_list[cursor2];
            }
            Instruction::LessThanOrEqualInt(cursor1, cursor2) => {
                self.equal_flag = self.register_list[cursor1] <= self.register_list[cursor2];
            }

            Instruction::CompareFloat(cursor1, cursor2) => {
                self.equal_flag =
                    self.float_register_list[cursor1] == self.float_register_list[cursor2];
            }
            Instruction::CompareZeroFloat(cursor) => {
                self.equal_flag = self.float_register_list[cursor] == 0.0;
            }
            Instruction::CompareNotZeroFloat(cursor) => {
                self.equal_flag = self.float_register_list[cursor] != 0.0;
            }
            Instruction::GreaterThanFloat(cursor1, cursor2) => {
                self.equal_flag =
                    self.float_register_list[cursor1] > self.float_register_list[cursor2];
            }
            Instruction::GreaterThanOrEqualFloat(cursor1, cursor2) => {
                self.equal_flag =
                    self.float_register_list[cursor1] >= self.float_register_list[cursor2];
            }
            Instruction::LessThanFloat(cursor1, cursor2) => {
                self.equal_flag =
                    self.float_register_list[cursor1] < self.float_register_list[cursor2];
            }
            Instruction::LessThanOrEqualFloat(cursor1, cursor2) => {
                self.equal_flag =
                    self.float_register_list[cursor1] <= self.float_register_list[cursor2];
            }

            Instruction::Jump(cursor) => {
                self.program_counter = self.register_list[cursor] as usize;
            }
            Instruction::JumpBack(cursor) => {
                self.program_counter -= self.register_list[cursor] as usize;
            }
            Instruction::JumpFront(cursor) => {
                self.program_counter += self.register_list[cursor] as usize;
            }
            Instruction::JumpEqual(cursor) => {
                if self.equal_flag {
                    self.program_counter = self.register_list[cursor] as usize;
                }
            }
            Instruction::JumpNotEqual(cursor) => {
                if !self.equal_flag {
                    self.program_counter = self.register_list[cursor] as usize;
                }
            }

            Instruction::Illegal(opcode) => {
                error!("Unrecognized OPCODE({}) found! Terminating!", opcode);
                return Some(1);
            }
        }

        None
    }
}

impl VM {
    fn next_instruction(&mut self) -> Instruction {
        match self.next_opcode() {
            Ok(Opcode::Halting) => Instruction::Halting,

            Ok(Opcode::ConstInt8) => {
                Instruction::ConstInt8(self.next_int_cursor(), self.next_1_byte() as i8)
            }
            Ok(Opcode::ConstInt16) => {
                Instruction::ConstInt16(self.next_int_cursor(), self.next_2_bytes() as i16)
            }
            Ok(Opcode::ConstInt32) => {
                Instruction::ConstInt32(self.next_int_cursor(), self.next_4_bytes() as i32)
            }
            Ok(Opcode::ConstInt64) => {
                Instruction::ConstInt64(self.next_int_cursor(), self.next_8_bytes() as i64)
            }

            Ok(Opcode::ConstFloat32) => Instruction::ConstFloat32(
                self.next_float_cursor(),
                f32::from_bits(self.next_4_bytes()),
            ),
            Ok(Opcode::ConstFloat64) => Instruction::ConstFloat64(
                self.next_float_cursor(),
                f64::from_bits(self.next_8_bytes()),
            ),

            Ok(Opcode::AddInt) => Instruction::AddInt(
                self.next_int_cursor(),
                self.next_int_cursor(),
                self.next_int_cursor(),
            ),
            Ok(Opcode::SubInt) => Instruction::SubInt(
                self.next_int_cursor(),
                self.next_int_cursor(),
                self.next_int_cursor(),
            ),
            Ok(Opcode::MulInt) => Instruction::MulInt(
                self.next_int_cursor(),
                self.next_int_cursor(),
                self.next_int_cursor(),
            ),
            Ok(Opcode::DivInt) => Instruction::DivInt(
                self.next_int_cursor(),
                self.next_int_cursor(),
                self.next_int_cursor(),
            ),
            Ok(Opcode::ModInt) => Instruction::ModInt(
                self.next_int_cursor(),
                self.next_int_cursor(),
                self.next_int_cursor(),
            ),
            Ok(Opcode::ExpInt) => Instruction::ExpInt(
                self.next_int_cursor(),
                self.next_int_cursor(),
                self.next_int_cursor(),
            ),

            Ok(Opcode::AddFloat) => Instruction::AddFloat(
                self.next_float_cursor(),
                self.next_float_cursor(),
                self.next_float_cursor(),
            ),
            Ok(Opcode::SubFloat) => Instruction::SubFloat(
                self.next_float_cursor(),
                self.next_float_cursor(),
                self.next_float_cursor(),
            ),
            Ok(Opcode::MulFloat) => Instruction::MulFloat(
                self.next_float_cursor(),
                self.next_float_cursor(),
                self.next_float_cursor(),
            ),
            Ok(Opcode::DivFloat) => Instruction::DivFloat(
                self.next_float_cursor(),
                self.next_float_cursor(),
                self.next_float_cursor(),
            ),
            Ok(Opcode::ModFloat) => Instruction::ModFloat(
                self.next_float_cursor(),
                self.next_float_cursor(),
                self.next_float_cursor(),
            ),
            Ok(Opcode::ExpFloat) => Instruction::ExpFloat(
                self.next_float_cursor(),
                self.next_float_cursor(),
                self.next_float_cursor(),
            ),

            Ok(Opcode::CompareInt) => {
                Instruction::CompareInt(self.next_int_cursor(), self.next_int_cursor())
            }
            Ok(Opcode::CompareZeroInt) => Instruction::CompareZeroInt(self.next_int_cursor()),
            Ok(Opcode::CompareNotZeroInt) => Instruction::CompareNotZeroInt(self.next_int_cursor()),
            Ok(Opcode::GreaterThanInt) => {
                Instruction::GreaterThanInt(self.next_int_cursor(), self.next_int_cursor())
            }
            Ok(Opcode::GreaterThanOrEqualInt) => {
                Instruction::GreaterThanOrEqualInt(self.next_int_cursor(), self.next_int_cursor())
            }
            Ok(Opcode::LessThanInt) => {
                Instruction::LessThanInt(self.next_int_cursor(), self.next_int_cursor())
            }
            Ok(Opcode::LessThanOrEqualInt) => {
                Instruction::LessThanOrEqualInt(self.next_int_cursor(), self.next_int_cursor())
            }

            Ok(Opcode::CompareFloat) => {
                Instruction::CompareFloat(self.next_float_cursor(), self.next_float_cursor())
            }
            Ok(Opcode::CompareZeroFloat) => Instruction::CompareZeroFloat(self.next_float_cursor()),
            Ok(Opcode::CompareNotZeroFloat) => {
                Instruction::CompareNotZeroFloat(self.next_float_cursor())
            }
            Ok(Opcode::GreaterThanFloat) => {
                Instruction::GreaterThanFloat(self.next_float_cursor(), self.next_float_cursor())
            }
            Ok(Opcode::GreaterThanOrEqualFloat) => Instruction::GreaterThanOrEqualFloat(
                self.next_float_cursor(),
                self.next_float_cursor(),
            ),
            Ok(Opcode::LessThanFloat) => {
                Instruction::LessThanFloat(self.next_float_cursor(), self.next_float_cursor())
            }
            Ok(Opcode::LessThanOrEqualFloat) => Instruction::LessThanOrEqualFloat(
                self.next_float_cursor(),
                self.next_float_cursor(),
            ),

            Ok(Opcode::Jump) => Instruction::Jump(self.next_int_cursor()),
            Ok(Opcode::JumpBack) => Instruction::JumpBack(self.next_int_cursor()),
            Ok(Opcode::JumpFront) => Instruction::JumpFront(self.next_int_cursor()),
            Ok(Opcode::JumpEqual) => Instruction::JumpEqual(self.next_int_cursor()),
            Ok(Opcode::JumpNotEqual) => Instruction::JumpNotEqual(self.next_int_cursor()),

            Err(opcode) => Instruction::Illegal(opcode),
        }
    }

    fn next_opcode(&mut self) -> Result<Opcode, u8> {
        use std::convert::TryInto;

        let opcode = program_counter!(self).try_into();
        self.program_counter += 1;
        opcode
    }

    #[inline]
    fn next_int_cursor(&mut self) -> Cursor<i64> {
        Cursor::<i64>::new(self.next_1_byte())
    }

    #[inline]
    fn next_float_cursor(&mut self) -> Cursor<f64> {
        Cursor::<f64>::new(self.next_1_byte())
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
