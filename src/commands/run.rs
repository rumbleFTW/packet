use std::{process::Command, result};

type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;

pub fn exec() -> CommandResult<()> {
    let env_path = "/home/rumbleFTW/Documents/Codes/packet/packet/env";
    let new_path = format!(
        "{}/bin:{}",
        env_path,
        std::env::var("PATH").unwrap_or_default()
    );
    std::env::set_var("PATH", &new_path);
    let _ = Command::new("where").arg("python");
    println!("PATH: {:?}", std::env::var_os("PATH").unwrap_or_default());
    Ok(())
}
