use git2::Repository;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    result,
};

type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;

pub fn exec(name: &str) -> CommandResult<()> {
    let base_path: &Path = Path::new(".");
    let project_path: PathBuf = base_path.join(name);

    // Creating directories and initialising repository
    let _ = fs::create_dir(&project_path)?;
    let _ = fs::create_dir(&project_path.join(name))?;
    let mut main = fs::File::create(project_path.join(name).join("main.py"))?;
    let _ = main.write_all(b"print('Hello, world!')")?;
    let _ = Repository::init(&project_path)?;
    let mut gitignore = fs::File::create(project_path.join(".gitignore"))?;
    let _ = gitignore.write_all(b"__pycache__/\nenv/");

    // Creating packet files
    let mut toml: fs::File = fs::File::create(project_path.join("packet.toml"))?;
    let _ = toml.write_all(b"packet.toml");

    // Creating venv
    let _ = Command::new("python")
        .arg("-m")
        .arg("venv")
        .arg(project_path.join("env"))
        .status()?;
    Ok(())
}
