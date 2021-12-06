#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub value: String,
}

impl Identifier {
    #[inline(always)]
    pub fn new(value: &str) -> Self {
        Identifier {
            value: value.to_string(),
        }
    }
}
