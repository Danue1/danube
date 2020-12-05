mod add_item;
mod ident;
mod item;
mod program;
mod ty;
mod visibility;

use danube_parse::*;

pub fn hir(program: &Program) -> crate::Program {
    let context = crate::HirContext {
        resolver: Default::default(),
        items: Default::default(),
        bodies: Default::default(),
        is_in_loop_condition: false,
    };

    context.lower_program(program)
}
