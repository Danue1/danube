#![warn(clippy::all)]

pub struct Diagnostic {
    reports: Vec<miette::Report>,
}

impl Diagnostic {
    pub const fn new() -> Self {
        Self { reports: vec![] }
    }

    pub fn report(&mut self, report: miette::Report) {
        self.reports.push(report);
    }

    pub const fn is_empty(&self) -> bool {
        self.reports.is_empty()
    }
}

impl std::fmt::Debug for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for report in &self.reports {
            writeln!(f, "{:?}", report)?;
        }

        Ok(())
    }
}
