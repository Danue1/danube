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
    ($vm:ident,) => { };

    ($vm:ident, ilg($operand1:expr); $($t:tt)*) => {
        $vm.append_program(&[$operand1]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, hlt; $($t:tt)*) => {
        $vm.append_program(&[HALTING]);
        internal_vm!($vm, $($t)*)
    };

    ($vm:ident, jmp #$register1:expr; $($t:tt)*) => {
        $vm.append_program(&[JUMP, $register1]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, jmpb #$register1:expr; $($t:tt)*) => {
        $vm.append_program(&[JUMP_BACK, $register1]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, jmpf #$register1:expr; $($t:tt)*) => {
        $vm.append_program(&[JUMP_FRONT, $register1]);
        internal_vm!($vm, $($t)*)
    };

    ($vm:ident, add #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[ADD, $register1, $register2, $register3]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, sub #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[SUB, $register1, $register2, $register3]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, mul #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[MUL, $register1, $register2, $register3]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, div #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[DIV, $register1, $register2, $register3]);
        internal_vm!($vm, $($t)*)
    };

    ($vm:ident, addf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[ADD_FLOAT, $register1, $register2, $register3]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, subf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[SUB_FLOAT, $register1, $register2, $register3]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, mulf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[MUL_FLOAT, $register1, $register2, $register3]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, divf #$register1:expr, #$register2:expr, #$register3:expr; $($t:tt)*) => {
        $vm.append_program(&[DIV_FLOAT, $register1, $register2, $register3]);
        internal_vm!($vm, $($t)*)
    };

    ($vm:ident, load8 #$register1:expr, [$operand1:expr]; $($t:tt)*) => {
        $vm.append_program(&[LOAD8, $register1, $operand1]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, load16 #$register1:expr, [$operand1:expr, $operand2:expr]; $($t:tt)*) => {
        $vm.append_program(&[LOAD16, $register1, $operand1, $operand2]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, load32 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr]; $($t:tt)*) => {
        $vm.append_program(&[LOAD32, $register1, $operand1, $operand2, $operand3, $operand4]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, load64 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr, $operand5:expr, $operand6:expr, $operand7:expr, $operand8:expr]; $($t:tt)*) => {
        $vm.append_program(&[LOAD64, $register1, $operand1, $operand2, $operand3, $operand4, $operand5, $operand6, $operand7, $operand8]);
        internal_vm!($vm, $($t)*)
    };

    ($vm:ident, loadf32 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr]; $($t:tt)*) => {
        $vm.append_program(&[LOAD_FLOAT32, $register1, $operand1, $operand2, $operand3, $operand4]);
        internal_vm!($vm, $($t)*)
    };
    ($vm:ident, loadf64 #$register1:expr, [$operand1:expr, $operand2:expr, $operand3:expr, $operand4:expr, $operand5:expr, $operand6:expr, $operand7:expr, $operand8:expr]; $($t:tt)*) => {
        $vm.append_program(&[LOAD_FLOAT64, $register1, $operand1, $operand2, $operand3, $operand4, $operand5, $operand6, $operand7, $operand8]);
        internal_vm!($vm, $($t)*)
    };
}
