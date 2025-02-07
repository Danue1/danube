pub mod path_type;

pub use path_type::*;

crate::ast_node! {
    enum Type {
        Path(PathType),
    }
}
