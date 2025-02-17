pub mod definition;
pub mod expression;
pub mod literal;
pub mod path;
pub mod statement;
pub mod r#type;
pub mod visibility;

pub use definition::*;
pub use expression::*;
pub use literal::*;
pub use path::*;
pub use r#type::*;
pub use statement::*;
pub use visibility::*;

pub struct Root {
    pub definitions: Vec<Definition>,
}
