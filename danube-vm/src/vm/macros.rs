#[macro_export]
macro_rules! vm {
    ($($t:tt)*) => {{
        #[allow(unused_mut)]
        let mut vm = VM::new();
        $crate::internal_vm!(vm, $($t)*);
        vm
    }};
}

#[macro_export]
macro_rules! internal_vm {
    ($vm:ident,) => {};

    ($vm:ident, ilg($operand1:expr); $($t:tt)*) => {
        $crate::asm!($vm, $operand1);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, hlt; $($t:tt)*) => {
        hlt!($vm);
        internal_vm!($vm, $($t)*)
    };

    ($vm:ident, jmp #$register1:expr; $($t:tt)*) => {
        jmp!($vm, #$register1);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, jmpb #$register1:expr; $($t:tt)*) => {
        jmpb!($vm, #$register1);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, jmpf #$register1:expr; $($t:tt)*) => {
        jmpf!($vm, #$register1);
        internal_vm!($vm, $($t)*)
    };

    ($vm:ident, add #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        add!($vm, #$register1, #$register2, #$register3);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, sub #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        sub!($vm, #$register1, #$register2, #$register3);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, mul #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        mul!($vm, #$register1, #$register2, #$register3);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, div #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        div!($vm, #$register1, #$register2, #$register3);
        internal_vm!($vm, $($t)*)
    };

    ($vm:ident, addf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        addf!($vm, #$register1, #$register2, #$register3);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, subf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        subf!($vm, #$register1, #$register2, #$register3);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, mulf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        mulf!($vm, #$register1, #$register2, #$register3);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, divf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        divf!($vm, #$register1, #$register2, #$register3);
        internal_vm!($vm, $($t)*)
    };

    ($vm:ident, load8 #$register1:expr, [$operand1:expr]; $($t:tt)*) => {
        load8!($vm, #$register1, [$operand1]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, load16 #$register1:expr, [$operand1:expr, $operand2:expr]; $($t:tt)*) => {
        load16!($vm, #$register1, [$operand1, $operand2]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, load32 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr]; $($t:tt)*) => {
        load32!($vm, #$register1, [$operand1, $operand2, $operand3, $operand4]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, load64 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr, $operand5:expr, $operand6:expr, $operand7:expr, $operand8:expr]; $($t:tt)*) => {
        load64!($vm, #$register1, [$operand1, $operand2, $operand3, $operand4, $operand5, $operand6, $operand7, $operand8]);
        internal_vm!($vm, $($t)*)
    };

    ($vm:ident, loadf32 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr]; $($t:tt)*) => {
        loadf32!($vm, #$register1, [$operand1, $operand2, $operand3, $operand4]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, loadf64 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr, $operand5:expr, $operand6:expr, $operand7:expr, $operand8:expr]; $($t:tt)*) => {
        loadf64!($vm, #$register1, [$operand1, $operand2, $operand3, $operand4, $operand5, $operand6, $operand7, $operand8]);
        internal_vm!($vm, $($t)*)
    };
}

#[macro_export]
macro_rules! asm {
    ($vm:ident, $opcode:expr) => {
        $vm.append_program(&[$opcode]);
    };
}

macro_rules! instruction_opcode {
    ($($macro_name:ident => $opcode:ident),+ $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($vm:ident) => {
                    $vm.append_program(&[$opcode]);
                };
            }
        )+
    };
}

macro_rules! instruction_register {
    ($($macro_name:ident => $opcode:ident),+ $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($vm:ident, #$register1:expr) => {
                    $vm.append_program(&[$opcode, $register1]);
                };
            }
        )+
    };
}

macro_rules! instruction_register_register_register {
    ($($macro_name:ident => $opcode:ident),+ $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($vm:ident, #$register1:expr, #$register2:expr, #$register3:expr) => {
                    $vm.append_program(&[$opcode, $register1, $register2, $register3]);
                };
            }
        )+
    };
}

macro_rules! instruction_register_1byte {
    ($($macro_name:ident => $opcode:ident),+ $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($vm:ident, #$register1:expr, [$operand1:expr]) => {
                    $vm.append_program(&[$opcode, $register1, $operand1]);
                };
            }
        )+
    };
}

macro_rules! instruction_register_2bytes {
    ($($macro_name:ident => $opcode:ident),+ $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($vm:ident, #$register1:expr, [$operand1:expr, $operand2:expr]) => {
                    $vm.append_program(&[$opcode, $register1, $operand1, $operand2]);
                };
            }
        )+
    };
}

macro_rules! instruction_register_3bytes {
    ($($macro_name:ident => $opcode:ident),+ $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($vm:ident, #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr]) => {
                    $vm.append_program(&[$opcode, $register1, $operand1, $operand2, $operand3, $operand4]);
                };
            }
        )+
    };
}

macro_rules! instruction_register_4bytes {
    ($($macro_name:ident => $opcode:ident),+ $(,)?) => {
        $(
            #[macro_export]
            macro_rules! $macro_name {
                ($vm:ident, #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr, $operand5:expr, $operand6:expr, $operand7:expr, $operand8:expr]) => {
                    $vm.append_program(&[$opcode, $register1, $operand1, $operand2, $operand3, $operand4, $operand5, $operand6, $operand7, $operand8]);
                };
            }
        )+
    };
}

instruction_opcode! {
    hlt => HALTING,
}

instruction_register! {
    jmp => JUMP,
    jmpb => JUMP_BACK,
    jmpf => JUMP_FRONT,
}

instruction_register_register_register! {
    add => ADD,
    sub => SUB,
    mul => MUL,
    div => DIV,

    addf => ADD_FLOAT,
    subf => SUB_FLOAT,
    mulf => MUL_FLOAT,
    divf => DIV_FLOAT,
}

instruction_register_1byte! {
    load8 => LOAD8,
}

instruction_register_2bytes! {
    load16 => LOAD16,
}

instruction_register_3bytes! {
    load32 => LOAD32,
    loadf32 => LOAD_FLOAT32,
}

instruction_register_4bytes! {
    load64 => LOAD64,
    loadf64 => LOAD_FLOAT64,
}
