/// run
/// Run the current project from the entrypoint
use std::{path::Path, process::Command, result};

// Define a custom result type for command execution
type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;

/// Execute the specified Python script using the env interpreter.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the execution fails.
pub fn exec() -> CommandResult<()> {
    // Execute the Python script using the Python interpreter
    let executable = Path::new(".").join("env").join("bin").join("python");
    let entrypoint = Path::new("src").join("main.py");
    let _ = Command::new(executable.as_os_str())
        .arg(entrypoint.as_os_str())
        .status();
    Ok(())
}
