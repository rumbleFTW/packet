/// pull
/// Install modules to the current environment from Packet.toml
use super::types::ExecResult;
use packet::resolver::{environment, package::Package};
use packet::util::parsers::Toml;
use std::path::PathBuf;
pub fn exec() -> ExecResult<()> {
    // Check if env exists
    if !PathBuf::from("env").exists() {
        environment::create(PathBuf::from("env"))?;
    }

    let executable = if std::env::consts::OS == "windows" {
        PathBuf::from(".").join("env").join("Scripts").join("pip")
    } else {
        PathBuf::from(".").join("env").join("bin").join("pip")
    };

    let tomlfile = Toml::load(PathBuf::from("Packet.toml"))?;
    for (package, table) in tomlfile.dependencies {
        let package = format!("{}{}", package, table["version"]).replace("\"", "");
        let mut package_obj = Package::from_str(&package);
        let _ = package_obj.add(executable.clone())?;
    }
    Ok(())
}
