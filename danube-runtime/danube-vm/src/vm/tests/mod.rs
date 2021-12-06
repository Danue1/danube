mod arithmetics;
mod conditions;
mod consts;
mod jumps;

use crate::*;

#[test]
fn create_vm() {
    let vm = vm!();
    assert_eq!(vm.register_list, RegisterList::<i64>::new());
    assert_eq!(vm.float_register_list, RegisterList::<f64>::new());
    assert_eq!(vm.program_counter, 0);
    assert_eq!(vm.program, vec![]);
    assert_eq!(vm.equal_flag, false);
}

#[test]
fn instruction_halting() {
    let vm = vm! {
        hlt;
        run();
    };
    assert_eq!(vm.program_counter, 1);
}

#[test]
fn instruction_no_op() {
    let vm = vm! {
        noop;
        run();
    };
    assert_eq!(vm.program_counter, 1);
}

#[test]
fn instruction_illegal() {
    let vm = vm! {
        ilg(200);
        run();
    };
    assert_eq!(vm.program_counter, 1);
}
