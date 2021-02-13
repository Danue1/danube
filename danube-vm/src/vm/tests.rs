use crate::*;

#[test]
fn create_vm() {
    let vm = vm!();
    assert_eq!(vm.register_list, RegisterList::<i64>::new());
    assert_eq!(vm.float_register_list, RegisterList::<f64>::new());
    assert_eq!(vm.program_counter, 0);
    assert_eq!(vm.program, vec![]);
}

#[test]
fn opcode_halting() {
    let vm = vm! {
        hlt;
        run();
    };
    assert_eq!(vm.program_counter, 1);
}

#[test]
fn opcode_jump() {
    let mut vm = vm! {
        consti8 #0, [1];
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
        consti8 #0, [5];
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
        consti8 #0, [5];
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
    let vm = vm! {
        // CONSTI8 -12
        consti8 #0, [244];
        run();
    };
    assert_eq!(vm.register_list[0], -12);
}

#[test]
fn opcode_const_int16() {
    let vm = vm! {
        // CONSTI16 #0 500
        consti16 #0, [1, 244];
        run();
    };
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn opcode_const_int32() {
    let vm = vm! {
        // CONSTI32 #0 500
        consti32 #0, [0, 0, 1, 244];
        run();
    };
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn opcode_const_int64() {
    let vm = vm! {
        // CONSTI64 #0 500
        consti64 #0, [0, 0, 0, 0, 0, 0, 1, 244];
        run();
    };
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn opcode_const_float32() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
}

#[test]
fn opcode_const_float64() {
    let vm = vm! {
        // CONSTF64 #0 1.5
        constf64 #0, [63, 248, 0, 0, 0, 0, 0, 0];
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
}

#[test]
fn opcode_add_int() {
    let vm = vm! {
        consti8 #0, [7];
        consti8 #1, [14];
        addi #0, #1, #2;
        run();
    };
    assert_eq!(vm.register_list[0], 7);
    assert_eq!(vm.register_list[1], 14);
    assert_eq!(vm.register_list[2], 21);
}

#[test]
fn opcode_sub_int() {
    let vm = vm! {
        // CONSTI8 -12
        consti8 #0, [244];
        // CONSTI8 16
        consti8 #1, [144];
        subi #0, #1, #2;
        run();
    };
    assert_eq!(vm.register_list[0], -12);
    assert_eq!(vm.register_list[1], -112);
    assert_eq!(vm.register_list[2], 100);
}

#[test]
fn opcode_mul_int() {
    let vm = vm! {
        consti8 #0, [3];
        consti8 #1, [4];
        muli #0, #1, #2;
        run();
    };
    assert_eq!(vm.register_list[0], 3);
    assert_eq!(vm.register_list[1], 4);
    assert_eq!(vm.register_list[2], 12);
}

#[test]
fn opcode_div_int() {
    let vm = vm! {
        consti8 #0, [5];
        consti8 #1, [4];
        divi #0, #1, #2;
        run();
    };
    assert_eq!(vm.register_list[0], 5);
    assert_eq!(vm.register_list[1], 4);
    assert_eq!(vm.register_list[2], 1);
}

#[test]
fn opcode_add_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        addf #0, #1, #2;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], 4.0);
}

#[test]
fn opcode_sub_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        subf #0, #1, #2;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], -1.0);
}

#[test]
fn opcode_mul_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        mulf #0, #1, #2;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], 3.75);
}

#[test]
fn opcode_div_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        divf #0, #1, #2;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], 0.6);
}

#[test]
fn opcode_illegal() {
    let vm = vm! {
        ilg(200);
        run();
    };
    assert_eq!(vm.program_counter, 1);
}
