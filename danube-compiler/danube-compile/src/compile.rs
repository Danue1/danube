use crate::{Config, Context};
use danube_diagnostics::Diagnostics;

pub fn compile(config: Config) -> Result<(), Diagnostics> {
    let context = Context::new(config);
    if !context.diagnostics.is_empty() {
        return Err(context.diagnostics);
    }

    Ok(())
}
