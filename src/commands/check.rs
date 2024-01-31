/// check
/// Check if there is any error

use std::{
    process::{Command, exit},
    result,
};

type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;

/// Check if the Python script in the current directory can be executed without errors.
pub fn exec() -> CommandResult<()> {
    // Get the current directory
    let current_directory = std::env::current_dir().expect("Failed to get current directory.");

    // Set the path to the Python script
    let path = current_directory.join("src").join("main.py");

    // Check if the Python script exists
    if path.exists() {
        // Try running the Python script
        let output = Command::new("python")
            .arg(&path)
            .output()
            .expect("Failed to run Python script.");

        if output.status.success() {
            println!("No error in the Python Script.");
        } else {
            eprintln!(
                "Error running Python script:\n{}",
                String::from_utf8_lossy(&output.stderr)
            );
            exit(1);
        }
    }
    Ok(())
}
