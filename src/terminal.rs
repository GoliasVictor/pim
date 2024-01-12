//! Structs and functions for executing commands and scripts

use std::{env, process::Command};
use crate::{prelude::*, metadata::MetadataScript};

/// Information for running a script 
#[derive(Debug, Clone)]
pub struct Script {
    /// Script interpreter, the program and the first nth arguments to the command
    pub interpreter: String,
    /// The script to be executed (it will be the last argument when executing the command)
    pub value: String,
    /// The directory where the script/command will be executed
    pub directory: PathBuf,
}

impl Script {
    /// Convert the script into a command to build a process
    pub fn to_command(&self) -> Result<Command> {
        let mut command = get_command(&self.interpreter, &self.directory)?;
        command.arg(&self.value);
        Ok(command)
    }
    /// Create a new script, if interpreter is None, call `default_shell_interpreter()` to use the default shell
    pub fn new(interpreter: Option<String>, value: String, directory: PathBuf) -> Self {
        Self {
            interpreter: interpreter.unwrap_or_else(default_shell_interpreter),
            value,
            directory,
        }
    }
}

impl From<Script> for MetadataScript {
    fn from(script : Script) -> MetadataScript {
        MetadataScript::Struct { interpreter: 
            Some(script.interpreter), 
            script: script.value, 
            directory: Some(script.directory) 
        }
    }
}
/// get the default shell interpreter, from the environment var, `SHELL` added argument `-c`
pub fn default_shell_interpreter() -> String {
    env::var("SHELL").unwrap_or("bash".to_owned()) + " -c"
}

/// convert the string in a command, splitting each part, and using the first as a program and the rest as an argument 
pub fn get_command(command_string: &str, dir: &Path) -> Result<Command> {
    let argv = shlex::split(command_string).context("invalid command")?;
    let mut command = Command::new(&argv[0]);
    command.args(&argv[1..]).current_dir(dir);
    Ok(command)
}
