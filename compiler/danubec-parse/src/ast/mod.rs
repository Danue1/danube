pub mod ast;
pub mod item;

pub trait Parse: Sized {
    fn parse(cursor: &mut crate::cursor::Cursor) -> Result<Self, crate::ParseError>;
}
