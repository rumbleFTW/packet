/// init
/// Set up a project in an existing directory
use git2::Repository;
use std::{
    fs,
    io::Write,
    path::Path,
    process::Command,
    result,
};
use toml;

// Define a custom result type for command execution
type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;

/// Execute the specified commands to set up a Rust project in the current directory with additional files and configurations.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if any of the operations fail.

pub fn exec() -> CommandResult<()> {
    // Get current directory path to initialize the project
    let current_directory_path = match std::env::current_dir(){
        Ok(path) => path,
        Err(_) => panic!("Error getting current directory"),
    };
    //Extract the directory name from the path
    let dir = current_directory_path.file_name().unwrap().to_str().unwrap();
    let name = dir;
    // Set the path to the current directory
    let project_path: &Path = Path::new(".");
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
