pub struct Ticker {
    index: usize,
}

impl Ticker {
    #[inline]
    pub const fn new() -> Self {
        Self { index: 0 }
    }

    #[inline]
    pub const fn changed(&self) -> bool {
        self.index != 0
    }

    pub const fn mark(&mut self) {
        self.index += 1;
    }
}
