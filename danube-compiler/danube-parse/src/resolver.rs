use danube_ast::Id;

pub struct Resolver {
    index: u32,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver { index: 0 }
    }

    pub fn next_id(&mut self) -> Id {
        let index = self.index;
        self.index += 1;
        Id(index)
    }
}
