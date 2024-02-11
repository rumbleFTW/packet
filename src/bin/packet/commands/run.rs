/// run
/// Run the current project from the entrypoint
use super::types::ExecResult;
use std::{env, path::Path, process};

/// Execute the specified Python script using the env interpreter.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the execution fails.
pub fn exec() -> ExecResult<()> {
    // Execute the Python script using the Python interpreter
    let executable = if env::consts::OS == "windows" {
        Path::new(".").join("env").join("Scripts").join("python")
    } else {
        Path::new(".").join("env").join("bin").join("python")
    };
    let entrypoint = Path::new("src").join("main.py");
    let _ = process::Command::new(executable.as_os_str())
        .arg(entrypoint.as_os_str())
        .stdin(process::Stdio::inherit())
        .stdout(process::Stdio::inherit())
        .status();
    Ok(())
}
