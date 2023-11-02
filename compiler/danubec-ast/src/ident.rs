#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Ident {
    pub text: String,
}

impl Ident {
    pub const fn new(text: String) -> Self {
        Self { text }
    }
}
