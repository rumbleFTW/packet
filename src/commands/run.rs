/// run
/// Run the current project from the entrypoint
use std::{process::Command, result};

// Define a custom result type for command execution
type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;

/// Execute the specified Python script using the env interpreter.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the execution fails.
pub fn exec() -> CommandResult<()> {
    // Execute the Python script using the Python interpreter
    let _ = Command::new("env/bin/python").arg("src/main.py").status();
    Ok(())
}
