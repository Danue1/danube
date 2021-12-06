use crate::*;

#[test]
fn instruction_add_int() {
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
fn instruction_sub_int() {
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
fn instruction_mul_int() {
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
fn instruction_div_int() {
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
fn instruction_mod_int() {
    let vm = vm! {
        consti8 #0, [5];
        consti8 #1, [4];
        modi #0, #1, #2;
        run();
    };
    assert_eq!(vm.register_list[0], 5);
    assert_eq!(vm.register_list[1], 4);
    assert_eq!(vm.register_list[2], 1);
}

#[test]
fn instruction_exp_int() {
    let vm = vm! {
        consti8 #0, [5];
        consti8 #1, [4];
        expi #0, #1, #2;
        run();
    };
    assert_eq!(vm.register_list[0], 5);
    assert_eq!(vm.register_list[1], 4);
    assert_eq!(vm.register_list[2], 625);
}
