use clap::Args;
use crate::{mapper::find_environment, prelude::*};

/// Open the project
#[derive(Debug, Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct CommandOpen {
    /// Project to open
    project: String,
}

impl CommandOpen {
    pub fn execute(self, root: &Path) -> Result<()> {
        let env = find_environment(root, &self.project).context("environment not found")?;
        return env.open();
    }
}
