use crate::*;

#[test]
fn instruction_const_int8() {
    let vm = vm! {
        // CONSTI8 -12
        consti8 #0, [244];
        run();
    };
    assert_eq!(vm.register_list[0], -12);
}

#[test]
fn instruction_const_int16() {
    let vm = vm! {
        // CONSTI16 #0 500
        consti16 #0, [1, 244];
        run();
    };
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn instruction_const_int32() {
    let vm = vm! {
        // CONSTI32 #0 500
        consti32 #0, [0, 0, 1, 244];
        run();
    };
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn instruction_const_int64() {
    let vm = vm! {
        // CONSTI64 #0 500
        consti64 #0, [0, 0, 0, 0, 0, 0, 1, 244];
        run();
    };
    assert_eq!(vm.register_list[0], 500);
}

#[test]
fn instruction_const_float32() {
    let vm = vm! {
        // CONSTF32 #0 1.5
        constf32 #0, [63, 192, 0, 0];
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
}

#[test]
fn instruction_const_float64() {
    let vm = vm! {
        // CONSTF64 #0 1.5
        constf64 #0, [63, 248, 0, 0, 0, 0, 0, 0];
        run();
    };
    assert_eq!(vm.float_register_list[0], 1.5);
}
