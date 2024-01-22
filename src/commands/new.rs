use git2::Repository;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    result,
};
use toml;

// Define a custom result type for command execution
type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;

/// Execute the specified commands to set up a Rust project with additional files and configurations.
///
/// # Arguments
///
/// * `name` - The name of the project.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if any of the operations fail.
pub fn exec(name: &str) -> CommandResult<()> {
    // Set the base path to the current directory
    let base_path: &Path = Path::new(".");
    // Create a PathBuf for the project path by joining the base path and project name
    let project_path: PathBuf = base_path.join(name);

    // Creating directories and initializing repository
    let _ = fs::create_dir(&project_path)?;
    let _ = fs::create_dir(&project_path.join("src"))?;
    let mut main = fs::File::create(project_path.join("src").join("main.py"))?;
    let _ = main.write_all(b"print('Hello, world!')")?;
    let _ = Repository::init(&project_path)?;
    let mut gitignore = fs::File::create(project_path.join(".gitignore"))?;
    let _ = gitignore.write_all(b"__pycache__/\nenv/");

    // Creating packet files
    let mut toml_file: fs::File = fs::File::create(project_path.join("Packet.toml"))?;

    let toml_data = toml::toml! {
        [package]
        package = name
        version = "0.1.0"

        [dependencies]
    };

    let _ = toml_file.write_all(toml::to_string_pretty(&toml_data)?.as_bytes());

    // Creating virtual environment (venv)
    let _ = Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg(project_path.join("env"))
        .status()?;
    Ok(())
}
