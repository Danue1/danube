#![warn(clippy::all)]

#[macro_use]
extern crate danubec_diagnostic;

use danubec_data_structure::Directory;
use danubec_middle::{ast, Context};

pub struct CompileConfig {
    pub working_directory: String,
    pub crates: Vec<String>,
}

impl CompileConfig {
    pub fn compile(self) {
        let mut context = Context::new(self.working_directory);
        let mut directory = Directory::new();
        for krate in self.crates {
            load(&mut context, &mut directory, &krate);
        }

        dbg!(&directory);
    }
}

fn load(context: &mut Context, directory: &mut Directory<ast::Root>, krate: &str) {
    let pattern = format!("{}/{}/src/**/*.dnb", context.working_directory, krate);
    match glob::glob(&pattern) {
        Ok(paths) => {
            for path in paths.filter_map(Result::ok) {
                if let Ok(source) = std::fs::read_to_string(&path) {
                    let Ok(path) = path.strip_prefix(&context.working_directory) else {
                        error!(
                            context.diagnostic,
                            "ICE: failed to strip prefix ({})",
                            path.display()
                        );
                        continue;
                    };
                    let ast = danubec_parse::parse(&source);
                    directory
                        .insert(&path.to_path_buf(), ast)
                        .expect("failed to insert");
                } else {
                    error!(
                        context.diagnostic,
                        "ICE: failed to read file ({})",
                        path.display()
                    );
                }
            }
        }
        Err(_) => error!(context.diagnostic, "ICE: failed to glob ({})", pattern),
    }
}
