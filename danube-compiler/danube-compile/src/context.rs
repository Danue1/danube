use crate::{Config, OptimizationLevel, ReportLevel};
use danube_diagnostics::{Diagnostics, MessageBuilder};
use std::path::PathBuf;

#[derive(Debug)]
pub(crate) struct Context {
    pub(crate) diagnostics: Diagnostics,

    pub(crate) source_path: Option<PathBuf>,
    pub(crate) std_path: Option<PathBuf>,
    pub(crate) optimization_level: OptimizationLevel,
    pub(crate) report_level: ReportLevel,
}

impl Context {
    pub(crate) fn new(config: Config) -> Self {
        let mut diagnostics = Diagnostics::new();

        if let Some(ref source_path) = config.source_path {
            let main_path = source_path.join("src/main.dnb");
            if !main_path.is_file() {
                diagnostics.report(
                    MessageBuilder::error(format!(
                        "Source path '{}' does not found",
                        main_path.to_string_lossy()
                    ))
                    .build(),
                );
            }
        }

        if let Some(ref std_path) = config.std_path {
            let std_path = std_path.join("src/lib.dnb");
            if !std_path.is_file() {
                diagnostics.report(
                    MessageBuilder::error(format!(
                        "Source path '{}' does not found",
                        std_path.to_string_lossy()
                    ))
                    .build(),
                );
            }
        }

        Context {
            diagnostics,
            source_path: config.source_path,
            std_path: config.std_path,
            optimization_level: config.optimization_level,
            report_level: config.report_level,
        }
    }
}
