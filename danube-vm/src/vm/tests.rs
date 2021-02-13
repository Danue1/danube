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
        load8 #0, [1];
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
        load8 #0, [5];
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
        load8 #0, [5];
        jmpf #0;
    };
    vm.run_once();
    assert_eq!(vm.register_list[0], 5);
    assert_eq!(vm.program_counter, 3);
    vm.run_once();
    assert_eq!(vm.program_counter, 10);
}

#[test]
fn opcode_load8() {
    let mut vm = vm! {
        load8 #0, [244];
    };
    vm.run();
    assert_eq!(vm.register_list[0], 244);
}

#[test]
fn opcode_load16() {
    let mut vm = vm! {
        // LOAD16 0 500
        load16 #0, [1, 244];
    };
    vm.run();
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn opcode_load32() {
    let mut vm = vm! {
        // LOAD32 0 500
        load32 #0, [0, 0, 1, 244];
    };
    vm.run();
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn opcode_load64() {
    let mut vm = vm! {
        // LOAD64 0 500
        load64 #0, [0, 0, 0, 0, 0, 0, 1, 244];
    };
    vm.run();
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn opcode_load_float32() {
    let mut vm = vm! {
        // LOADF32 0 1.5
        loadf32 #0, [63, 192, 0, 0];
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
}

#[test]
fn opcode_load_float64() {
    let mut vm = vm! {
        // LOADF64 0 1.5
        loadf64 #0, [63, 248, 0, 0, 0, 0, 0, 0];
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
}

#[test]
fn opcode_add() {
    let mut vm = vm! {
        load8 #0, [7];
        load8 #1, [14];
        add #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.register_list[0], 7);
    assert_eq!(vm.register_list[1], 14);
    assert_eq!(vm.register_list[2], 21);
}

#[test]
fn opcode_sub() {
    let mut vm = vm! {
        load8 #0, [244];
        load8 #1, [144];
        sub #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.register_list[0], 244);
    assert_eq!(vm.register_list[1], 144);
    assert_eq!(vm.register_list[2], 100);
}

#[test]
fn opcode_mul() {
    let mut vm = vm! {
        load8 #0, [3];
        load8 #1, [4];
        mul #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.register_list[0], 3);
    assert_eq!(vm.register_list[1], 4);
    assert_eq!(vm.register_list[2], 12);
}

#[test]
fn opcode_div() {
    let mut vm = vm! {
        load8 #0, [5];
        load8 #1, [4];
        div #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.register_list[0], 5);
    assert_eq!(vm.register_list[1], 4);
    assert_eq!(vm.register_list[2], 1);
}

#[test]
fn opcode_float_add() {
    let mut vm = vm! {
        // LOADF32 0 1.5
        loadf32 #0, [63, 192, 0, 0];
        loadf32 #1, [64, 32, 0, 0];
        addf #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], 4.0);
}

#[test]
fn opcode_float_sub() {
    let mut vm = vm! {
        // LOADF32 0 1.5
        loadf32 #0, [63, 192, 0, 0];
        loadf32 #1, [64, 32, 0, 0];
        subf #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], -1.0);
}

#[test]
fn opcode_float_mul() {
    let mut vm = vm! {
        // LOADF32 0 1.5
        loadf32 #0, [63, 192, 0, 0];
        loadf32 #1, [64, 32, 0, 0];
        mulf #0, #1, #2;
    };
    vm.run();
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], 3.75);
}

#[test]
fn opcode_float_div() {
    let mut vm = vm! {
        // LOADF32 0 1.5
        loadf32 #0, [63, 192, 0, 0];
        loadf32 #1, [64, 32, 0, 0];
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
