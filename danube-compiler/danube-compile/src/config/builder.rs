use crate::{Config, OptimizationLevel, ReportLevel};
use std::path::PathBuf;

#[derive(Default)]
pub struct ConfigBuilder {
    source_path: Option<PathBuf>,
    std_path: Option<PathBuf>,
    optimization_level: OptimizationLevel,
    report_level: ReportLevel,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    pub fn source_path(mut self, source_path: PathBuf) -> Self {
        self.source_path = Some(source_path);
        self
    }

    pub fn std_path(mut self, std_path: PathBuf) -> Self {
        self.std_path = Some(std_path);
        self
    }

    pub fn optimization_level(mut self, optimization_level: OptimizationLevel) -> Self {
        self.optimization_level = optimization_level;
        self
    }

    pub fn report_level(mut self, report_level: ReportLevel) -> Self {
        self.report_level = report_level;
        self
    }

    pub fn build(self) -> Config {
        Config {
            source_path: self.source_path,
            std_path: self.std_path,
            optimization_level: self.optimization_level,
            report_level: self.report_level,
        }
    }
}
