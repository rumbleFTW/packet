/// init
/// Set up a project in an existing directory
use super::types::ExecResult;
use git2::Repository;
use packet::util::parsers::Toml;
use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

/// Execute the specified commands to set up a Rust project in the current directory with additional files and configurations.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if any of the operations fail.

pub fn exec() -> ExecResult<()> {
    // Get current directory path to initialize the project
    let current_directory_path = env::current_dir()?;

    //Extract the directory name from the path
    let dir = current_directory_path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let name = dir;
    let project_path: &Path = Path::new(".");

    // Creating virtual environment (venv)
    let mut env_process = Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg(project_path.join("env"))
        .spawn()?;

    // Set the path to the current directory
    let _ = fs::create_dir(&project_path.join("src"))?;
    let mut main = fs::File::create(project_path.join("src").join("main.py"))?;
    let _ = main.write_all(b"print('Hello, world!')")?;
    let _ = Repository::init(&project_path)?;
    let mut gitignore = fs::File::create(project_path.join(".gitignore"))?;
    let _ = gitignore.write_all(b"__pycache__/\nenv/");

    // Creating packet files
    let mut toml_data = Toml::init(name)?;
    let _ = toml_data.write(PathBuf::from("Packet.toml"));

    let _ = env_process.wait();
    Ok(())
}
