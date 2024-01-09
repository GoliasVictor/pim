use clap::Args;
use crate::{mapper::find_environment, prelude::*};

/// Open a environmnet (run the `open` script or open vscode in the environment directory)
#[derive(Debug, Args, Clone)]
#[command(arg_required_else_help = true)]
pub struct CommandOpen {
    /// Environmnet to open
    environment: String,
}

impl CommandOpen {
    /// Execute the command
    pub fn execute(self, root: &Path) -> Result<()> {
        let env = find_environment(root, &self.environment).context("environment not found")?;
        env.open()
    }
}
