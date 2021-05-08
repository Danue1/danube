#[derive(Debug, PartialEq)]
pub enum Error {
    Invalid,
    Illegal(usize, char),
    Need(usize, char),
}
