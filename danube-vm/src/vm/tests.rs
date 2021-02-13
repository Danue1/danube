use super::*;
use crate::*;

#[test]
fn create_vm() {
    let vm = vm!();
    assert_eq!(vm.register_list, [0; BIT]);
    assert_eq!(vm.float_register_list, [0.0; BIT]);
    assert_eq!(vm.program_counter, 0);
    assert_eq!(vm.program, vec![]);
}

#[test]
fn opcode_halting() {
    let mut vm = vm! {
        hlt;
    };
    vm.run();
    assert_eq!(vm.program_counter, 1);
}

#[test]
fn opcode_jump() {
    let mut vm = vm! {
        constint8 #0, [1];
        jmp #0;
    };
    vm.run_once();
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.program_counter, 3);
    vm.run_once();
    assert_eq!(vm.program_counter, 1);
}

#[test]
fn opcode_jump_back() {
    let mut vm = vm! {
        constint8 #0, [5];
        jmpb #0;
    };
    vm.run_once();
    assert_eq!(vm.register_list[0], 5);
    assert_eq!(vm.program_counter, 3);
    vm.run_once();
    assert_eq!(vm.program_counter, 0);
}

#[test]
fn opcode_jump_front() {
    let mut vm = vm! {
        constint8 #0, [5];
        jmpf #0;
    };
    vm.run_once();
    assert_eq!(vm.register_list[0], 5);
    assert_eq!(vm.program_counter, 3);
    vm.run_once();
    assert_eq!(vm.program_counter, 10);
}

#[test]
fn opcode_const_int8() {
    let mut vm = vm! {
        constint8 #0, [244];
    };
    vm.run();
    assert_eq!(vm.register_list[0], 244);
}

#[test]
fn opcode_const_int16() {
    let mut vm = vm! {
        // CONST16 #0 500
        constint16 #0, [1, 244];
    };
    vm.run();
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn opcode_const_int32() {
    let mut vm = vm! {
        // CONST32 #0 500
        constint32 #0, [0, 0, 1, 244];
    };
    vm.run();
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn opcode_const_int64() {
    let mut vm = vm! {
        // CONST64 #0 500
        constint64 #0, [0, 0, 0, 0, 0, 0, 1, 244];
    };
    vm.run();
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn opcode_const_float32() {
    let mut vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
}

#[test]
fn opcode_const_float64() {
    let mut vm = vm! {
        // CONSTF64 #0 1.5
        constf64 #0, [63, 248, 0, 0, 0, 0, 0, 0];
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
}

#[test]
fn opcode_add_int() {
    let mut vm = vm! {
        constint8 #0, [7];
        constint8 #1, [14];
        addi #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.register_list[0], 7);
    assert_eq!(vm.register_list[1], 14);
    assert_eq!(vm.register_list[2], 21);
}

#[test]
fn opcode_sub_int() {
    let mut vm = vm! {
        constint8 #0, [244];
        constint8 #1, [144];
        subi #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.register_list[0], 244);
    assert_eq!(vm.register_list[1], 144);
    assert_eq!(vm.register_list[2], 100);
}

#[test]
fn opcode_mul_int() {
    let mut vm = vm! {
        constint8 #0, [3];
        constint8 #1, [4];
        muli #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.register_list[0], 3);
    assert_eq!(vm.register_list[1], 4);
    assert_eq!(vm.register_list[2], 12);
}

#[test]
fn opcode_div_int() {
    let mut vm = vm! {
        constint8 #0, [5];
        constint8 #1, [4];
        divi #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.register_list[0], 5);
    assert_eq!(vm.register_list[1], 4);
    assert_eq!(vm.register_list[2], 1);
}

#[test]
fn opcode_add_float() {
    let mut vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        addf #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], 4.0);
}

#[test]
fn opcode_sub_float() {
    let mut vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        subf #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], -1.0);
}

#[test]
fn opcode_mul_float() {
    let mut vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        mulf #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], 3.75);
}

#[test]
fn opcode_div_float() {
    let mut vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        divf #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], 0.6);
}

#[test]
fn opcode_illegal() {
    let mut vm = vm! {
        ilg(200);
    };
    vm.run();
    assert_eq!(vm.program_counter, 1);
}
