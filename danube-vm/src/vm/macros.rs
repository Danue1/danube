#[macro_export]
macro_rules! vm {
    ($($t:tt)*) => {{
        #[allow(unused_mut)]
        let mut vm = VM::new();
        $crate::internal_vm!(vm $($t)*);
        vm
    }};
}

#[macro_export]
macro_rules! internal_vm {
    ($vm:ident) => { };

    ($vm:ident hlt; $($t:tt)*) => {
        $vm.append_program(&[HALTING]);
        internal_vm!($vm $($t)*)
    };

    ($vm:ident jmp #$register1:expr; $($t:tt)*) => {
        $vm.append_program(&[JUMP, $register1]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident jmpb #$register1:expr; $($t:tt)*) => {
        $vm.append_program(&[JUMP_BACK, $register1]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident jmpf #$register1:expr; $($t:tt)*) => {
        $vm.append_program(&[JUMP_FRONT, $register1]);
        internal_vm!($vm $($t)*)
    };

    ($vm:ident consti8 #$register1:expr, [$operand1:expr]; $($t:tt)*) => {
        $vm.append_program(&[CONST_INT8, $register1, $operand1]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident consti16 #$register1:expr, [$operand1:expr, $operand2:expr]; $($t:tt)*) => {
        $vm.append_program(&[CONST_INT16, $register1, $operand1, $operand2]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident consti32 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr]; $($t:tt)*) => {
        $vm.append_program(&[CONST_INT32, $register1, $operand1, $operand2, $operand3, $operand4]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident consti64 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr, $operand5:expr, $operand6:expr, $operand7:expr, $operand8:expr]; $($t:tt)*) => {
        $vm.append_program(&[CONST_INT64, $register1, $operand1, $operand2, $operand3, $operand4, $operand5, $operand6, $operand7, $operand8]);
        internal_vm!($vm $($t)*)
    };

    ($vm:ident constf32 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr]; $($t:tt)*) => {
        $vm.append_program(&[CONST_FLOAT32, $register1, $operand1, $operand2, $operand3, $operand4]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident constf64 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr, $operand5:expr, $operand6:expr, $operand7:expr, $operand8:expr]; $($t:tt)*) => {
        $vm.append_program(&[CONST_FLOAT64, $register1, $operand1, $operand2, $operand3, $operand4, $operand5, $operand6, $operand7, $operand8]);
        internal_vm!($vm $($t)*)
    };

    ($vm:ident addi #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[ADD_INT, $register1, $register2, $register3]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident subi #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[SUB_INT, $register1, $register2, $register3]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident muli #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[MUL_INT, $register1, $register2, $register3]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident divi #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[DIV_INT, $register1, $register2, $register3]);
        internal_vm!($vm $($t)*)
    };

    ($vm:ident addf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[ADD_FLOAT, $register1, $register2, $register3]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident subf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[SUB_FLOAT, $register1, $register2, $register3]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident mulf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[MUL_FLOAT, $register1, $register2, $register3]);
        internal_vm!($vm $($t)*)
    };
    ($vm:ident divf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[DIV_FLOAT, $register1, $register2, $register3]);
        internal_vm!($vm $($t)*)
    };

    ($vm:ident ilg($operand1:expr); $($t:tt)*) => {
        $vm.append_program(&[$operand1]);
        internal_vm!($vm $($t)*)
    };
}
