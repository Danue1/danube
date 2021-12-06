use crate::*;

#[test]
fn instruction_compare_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 1.5
        constf32 #1, [63, 192, 0, 0];
        cmpf #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 1.5);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 15);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        cmpf #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 15);
}

#[test]
fn instruction_compare_zero_float() {
    let vm = vm! {
        // CONSTF32 #0 0
        constf32 #0, [0, 0, 0, 0];
        cmpzf #0;
        run();
    };
    assert_eq!(vm.float_register_list[0], 0.0);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 8);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        cmpzf #0;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 8);
}

#[test]
fn instruction_compare_not_zero_float() {
    let vm = vm! {
        // CONSTF32 #0 0
        constf32 #0, [0, 0, 0, 0];
        cmpnzf #0;
        run();
    };
    assert_eq!(vm.float_register_list[0], 0.0);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 8);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        cmpnzf #0;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 8);
}

#[test]
fn instruction_compare_greater_than_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 0.5
        constf32 #1, [63, 0, 0, 0];
        gtf #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 0.5);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 15);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 1.5
        constf32 #1, [63, 192, 0, 0];
        gtf #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 1.5);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 15);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        gtf #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 15);
}

#[test]
fn instruction_compare_greater_than_or_equal_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 0.5
        constf32 #1, [63, 0, 0, 0];
        gtef #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 0.5);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 15);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 1.5
        constf32 #1, [63, 192, 0, 0];
        gtef #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 1.5);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 15);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        gtef #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 15);
}

#[test]
fn instruction_compare_less_than_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 0.5
        constf32 #1, [63, 0, 0, 0];
        ltf #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 0.5);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 15);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 1.5
        constf32 #1, [63, 192, 0, 0];
        ltf #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 1.5);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 15);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        ltf #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 15);
}

#[test]
fn instruction_compare_less_than_or_equal_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 0.5
        constf32 #1, [63, 0, 0, 0];
        ltef #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 0.5);
    assert_eq!(vm.equal_flag, false);
    assert_eq!(vm.program_counter, 15);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 1.5
        constf32 #1, [63, 192, 0, 0];
        ltef #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 1.5);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 15);

    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        ltef #0, #1;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.equal_flag, true);
    assert_eq!(vm.program_counter, 15);
}
