use crate::*;

#[derive(Debug, Default)]
pub struct Resolver {
    next_id: Id,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            next_id: Default::default(),
        }
    }

    pub fn next_id(&mut self) -> Id {
        let next_id = self.next_id.as_usize().checked_add(1).unwrap();
        self.next_id = Id::from_usize(next_id);
        self.next_id
    }
}
