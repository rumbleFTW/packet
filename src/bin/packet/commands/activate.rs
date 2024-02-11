/// activate
/// Activate a project environment in the current shell session
use super::types::ExecResult;
use std::io::{self, Write};
use std::process::{Command, Stdio};

pub fn exec() -> ExecResult<()> {
    let output = Command::new("emulate")
        .arg("bash -c '. env/bin/activate'")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to execute command");

    io::stdout().write_all(&output.stdout).unwrap();
    Ok(())
}
