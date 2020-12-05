use super::*;

impl crate::HirContext {
    pub(super) fn with_new_scope<T, F: FnOnce(&mut Self) -> T>(&mut self, f: F) -> T {
        let is_in_loop_condition = self.is_in_loop_condition;
        self.is_in_loop_condition = false;

        let ret = f(self);

        self.is_in_loop_condition = is_in_loop_condition;

        ret
    }
}
