use std::{env, process::Command};
use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Script {
    pub interpreter: String,
    pub value: String,
    pub directory: PathBuf,
}

impl Script {
    pub fn to_process(&self) -> Result<Command> {
        let mut command = get_process(&self.interpreter, &self.directory)?;
        command.arg(&self.value);
        Ok(command)
    }
    pub fn new(interpreter: Option<String>, value: String, directory: PathBuf) -> Self {
        Self {
            interpreter: interpreter.unwrap_or_else(default_shell_interpreter),
            value,
            directory,
        }
    }
}
pub fn default_shell_interpreter() -> String {
    return env::var("SHELL").unwrap_or("bash".to_owned()) + " -c";
}
pub fn get_process(command_string: &str, dir: &Path) -> Result<Command> {
    let argv = shlex::split(&command_string).context("invalid command")?;
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]).current_dir(dir);
    return Ok(command);
}
