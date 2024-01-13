use git2::Repository;
use std::{fs, io::Write, path};

pub fn exec(name: &str) {
    let base_path: &path::Path = path::Path::new(".");
    let project_path: path::PathBuf = base_path.join(name);

    // Creating directories and initialising repository
    let _ = fs::create_dir(&project_path).unwrap();
    let _ = fs::create_dir(&project_path.join(name)).unwrap();
    let mut main = fs::File::create(project_path.join(name).join("main.py")).unwrap();
    let _ = main.write_all(b"print('Hello, world!)").unwrap();
    let _ = Repository::init(&project_path).unwrap();
    let mut gitignore = fs::File::create(project_path.join(".gitignore")).unwrap();
    let _ = gitignore.write_all(b"__pycache__/\nenv/");

    // Creating packet files
    let mut toml: fs::File = fs::File::create(project_path.join("packet.toml")).unwrap();
    let _ = toml.write_all(b"packet.toml");
}
