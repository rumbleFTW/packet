use std::path::PathBuf;
use std::process::Command;

pub fn create(path: PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    // Creating virtual environment (venv)
    let env_process = Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg(path)
        .status()?;
    Ok(env_process.success())
}
