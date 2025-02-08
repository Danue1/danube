pub mod path_type;

pub use path_type::*;

ast_node! {
    enum Type {
        Path(PathType),
    }
}
