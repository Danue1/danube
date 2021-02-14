use crate::*;

#[test]
fn instruction_jump() {
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
fn instruction_jump_back() {
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
fn instruction_jump_front() {
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
fn instruction_jump_equal() {
    let mut vm = vm! {
        consti8 #0, [2];
        consti8 #1, [2];
        consti8 #2, [0];
        cmpi #0, #1;
        jmpe #2;
        consti8 #0, [1];
        run(5);
    };
    assert_eq!(vm.register_list[0], 2);
    assert_eq!(vm.register_list[1], 2);
    assert_eq!(vm.register_list[2], 0);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 0);

    vm.run_once();
    assert_eq!(vm.program_counter, 3);
}

#[ignore]
#[test]
fn instruction_jump_not_equal() {
    let mut vm = vm! {
        consti8 #0, [2];
        consti8 #1, [3];
        consti8 #2, [0];
        cmpi #0, #1;
        jmpne #2;
        consti8 #0, [1];
        run(5);
    };
    assert_eq!(vm.register_list[0], 2);
    assert_eq!(vm.register_list[1], 3);
    assert_eq!(vm.register_list[2], 0);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 0);

    vm.run_once();
    assert_eq!(vm.program_counter, 3);
}
