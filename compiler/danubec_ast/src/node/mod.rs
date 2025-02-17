pub mod definition;
pub mod path;
pub mod r#type;
pub mod visibility;

pub use definition::*;
pub use path::*;
pub use r#type::*;
pub use visibility::*;

use danubec_syntax::SyntaxNode;

pub struct Root {
    syntax: SyntaxNode,
    definitions: Vec<Definition>,
}
