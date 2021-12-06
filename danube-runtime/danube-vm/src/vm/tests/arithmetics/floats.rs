use crate::*;

#[test]
fn instruction_add_float() {
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
fn instruction_sub_float() {
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
fn instruction_mul_float() {
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
fn instruction_div_float() {
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
fn instruction_mod_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        modf #0, #1, #2;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], 1.5);
}

#[test]
fn instruction_exp_float() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        // CONSTF32 #1 2.5
        constf32 #1, [64, 32, 0, 0];
        expf #0, #1, #2;
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
    assert_eq!(vm.float_register_list[1], 2.5);
    assert_eq!(vm.float_register_list[2], 2.7556759606310752);
}
