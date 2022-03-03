pub mod builder;

pub use builder::*;

use std::path::PathBuf;

pub struct Config {
    pub(crate) source_path: Option<PathBuf>,
    pub(crate) std_path: Option<PathBuf>,
    pub(crate) optimization_level: OptimizationLevel,
    pub(crate) report_level: ReportLevel,
}

#[derive(Debug)]
pub enum OptimizationLevel {
    Full,
    Light,
    Nope,
}

#[derive(Debug)]
pub enum ReportLevel {
    Error,
    Warning,
    Debug,
    Info,
}

impl Default for OptimizationLevel {
    fn default() -> Self {
        OptimizationLevel::Nope
    }
}

impl Default for ReportLevel {
    fn default() -> Self {
        ReportLevel::Info
    }
}
