use std::{process::Command, result};
type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;

pub fn exec() -> CommandResult<()> {
    let _ = Command::new("env/bin/python").arg("src/main.py").status();
    Ok(())
}
