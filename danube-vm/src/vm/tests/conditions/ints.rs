use crate::*;

#[test]
fn instruction_compare_int() {
    let vm = vm! {
        consti8 #0, [2];
        consti8 #1, [2];
        cmpi #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 2);
    assert_eq!(vm.register_list[1], 2);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 9);

    let vm = vm! {
        consti8 #0, [2];
        consti8 #1, [3];
        cmpi #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 2);
    assert_eq!(vm.register_list[1], 3);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 9);
}

#[test]
fn instruction_compare_zero_int() {
    let vm = vm! {
        consti8 #0, [0];
        cmpzi #0;
        run();
    };
    assert_eq!(vm.register_list[0], 0);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 5);

    let vm = vm! {
        consti8 #0, [1];
        cmpzi #0;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 5);
}

#[test]
fn instruction_compare_not_zero_int() {
    let vm = vm! {
        consti8 #0, [0];
        cmpnzi #0;
        run();
    };
    assert_eq!(vm.register_list[0], 0);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 5);

    let vm = vm! {
        consti8 #0, [1];
        cmpnzi #0;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 5);
}

#[test]
fn instruction_compare_greater_than_int() {
    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [0];
        gti #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 0);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 9);

    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [1];
        gti #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 1);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 9);

    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [2];
        gti #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 2);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 9);
}

#[test]
fn instruction_compare_greater_than_or_equal_int() {
    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [0];
        gtei #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 0);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 9);

    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [1];
        gtei #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 1);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 9);

    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [2];
        gtei #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 2);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 9);
}

#[test]
fn instruction_compare_less_than_int() {
    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [0];
        lti #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 0);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 9);

    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [1];
        lti #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 1);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 9);

    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [2];
        lti #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 2);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 9);
}

#[test]
fn instruction_compare_less_than_or_equal_int() {
    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [0];
        ltei #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 0);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 9);

    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [1];
        ltei #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 1);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 9);

    let vm = vm! {
        consti8 #0, [1];
        consti8 #1, [2];
        ltei #0, #1;
        run();
    };
    assert_eq!(vm.register_list[0], 1);
    assert_eq!(vm.register_list[1], 2);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 9);
}
