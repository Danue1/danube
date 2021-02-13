mod macros;
#[cfg(test)]
mod tests;

use crate::Opcode;

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

    pub fn execute_instruction(&mut self) -> Option<u8> {
        if self.program_counter >= self.program.len() {
            return Some(1);
        }

        match self.decode_opcode() {
            Opcode::Halting => {
                debug_info!("Halting encountered.");
                return Some(0);
            }

            Opcode::Jump => {
                let register1 = self.register_list[self.next_1_byte() as usize];
                self.program_counter = register1 as usize;
            }
            Opcode::JumpBack => {
                let register1 = self.register_list[self.next_1_byte() as usize];
                self.program_counter -= register1 as usize;
            }
            Opcode::JumpFront => {
                let register1 = self.register_list[self.next_1_byte() as usize];
                self.program_counter += register1 as usize;
            }

            Opcode::ConstInt8 => {
                let register = self.next_1_byte() as usize;
                self.register_list[register] = self.next_1_byte() as i64;
            }
            Opcode::ConstInt16 => {
                let register = self.next_1_byte() as usize;
                self.register_list[register] = self.next_2_bytes() as i64;
            }
            Opcode::ConstInt32 => {
                let register = self.next_1_byte() as usize;
                self.register_list[register] = self.next_4_bytes() as i64;
            }
            Opcode::ConstInt64 => {
                let register = self.next_1_byte() as usize;
                self.register_list[register] = self.next_8_bytes() as i64;
            }

            Opcode::ConstFloat32 => {
                let register = self.next_1_byte() as usize;
                self.float_register_list[register] = f32::from_bits(self.next_4_bytes()) as f64;
            }
            Opcode::ConstFloat64 => {
                let register = self.next_1_byte() as usize;
                self.float_register_list[register] = f64::from_bits(self.next_8_bytes());
            }

            Opcode::AddInt => {
                let register1 = self.register_list[self.next_1_byte() as usize];
                let register2 = self.register_list[self.next_1_byte() as usize];
                self.register_list[self.next_1_byte() as usize] = register1 + register2;
            }
            Opcode::SubInt => {
                let register1 = self.register_list[self.next_1_byte() as usize];
                let register2 = self.register_list[self.next_1_byte() as usize];
                self.register_list[self.next_1_byte() as usize] = register1 - register2;
            }
            Opcode::MulInt => {
                let register1 = self.register_list[self.next_1_byte() as usize];
                let register2 = self.register_list[self.next_1_byte() as usize];
                self.register_list[self.next_1_byte() as usize] = register1 * register2;
            }
            Opcode::DivInt => {
                let register1 = self.register_list[self.next_1_byte() as usize];
                let register2 = self.register_list[self.next_1_byte() as usize];
                self.register_list[self.next_1_byte() as usize] = register1 / register2;
            }

            Opcode::AddFloat => {
                let register1 = self.float_register_list[self.next_1_byte() as usize];
                let register2 = self.float_register_list[self.next_1_byte() as usize];
                self.float_register_list[self.next_1_byte() as usize] = register1 + register2;
            }
            Opcode::SubFloat => {
                let register1 = self.float_register_list[self.next_1_byte() as usize];
                let register2 = self.float_register_list[self.next_1_byte() as usize];
                self.float_register_list[self.next_1_byte() as usize] = register1 - register2;
            }
            Opcode::MulFloat => {
                let register1 = self.float_register_list[self.next_1_byte() as usize];
                let register2 = self.float_register_list[self.next_1_byte() as usize];
                self.float_register_list[self.next_1_byte() as usize] = register1 * register2;
            }
            Opcode::DivFloat => {
                let register1 = self.float_register_list[self.next_1_byte() as usize];
                let register2 = self.float_register_list[self.next_1_byte() as usize];
                self.float_register_list[self.next_1_byte() as usize] = register1 / register2;
            }

            Opcode::Illegal => {
                error!("Unrecognized opcode found! Terminating!");
                return Some(1);
            }
        }

        None
    }

    pub fn append_program(&mut self, program: &[u8]) {
        self.program.extend(program);
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(program_counter!(self));
        self.program_counter += 1;
        return opcode;
    }
}

impl VM {
    fn next_1_byte(&mut self) -> u8 {
        let result = program_counter!(self);
        self.program_counter += 1;
        return result;
    }

    fn next_2_bytes(&mut self) -> u16 {
        let result = ((program_counter!(self) as u16) << 8)
            | ((program_counter!(self, 1) as u16) << (8 * 0));
        self.program_counter += 2;
        return result;
    }

    fn next_4_bytes(&mut self) -> u32 {
        let result = ((program_counter!(self) as u32) << (8 * 3))
            | ((program_counter!(self, 1) as u32) << (8 * 2))
            | ((program_counter!(self, 2) as u32) << (8 * 1))
            | ((program_counter!(self, 3) as u32) << (8 * 0));
        self.program_counter += 4;
        return result;
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
        return result;
    }
}
