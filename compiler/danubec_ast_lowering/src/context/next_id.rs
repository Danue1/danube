use super::Symbol;

pub struct NextId {
    next_id: usize,
}

impl NextId {
    pub const fn new() -> Self {
        NextId { next_id: 0 }
    }

    pub fn next(&mut self) -> Symbol {
        let id = self.next_id;
        self.next_id += 1;

        Symbol::from_usize(id)
    }
}
