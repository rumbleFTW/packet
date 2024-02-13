/// add
/// Install modules to the current project
use super::types::ExecResult;
use packet::resolver::package::Package;
use packet::util::parsers::Toml;
use std::{env, path::PathBuf};

/// Execute the specified command to install a Python package using pip.
///
/// # Arguments
///
/// * `package_name` - The name of the package to install.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the installation fails.
pub fn exec(package: &str) -> ExecResult<()> {
    // Execute the pip command to install the specified package
    let executable = if env::consts::OS == "windows" {
        PathBuf::from(".").join("env").join("Scripts").join("pip")
    } else {
        PathBuf::from(".").join("env").join("bin").join("pip")
    };

    // Read Packet.toml file
    let mut tomlfile = Toml::load(PathBuf::from("Packet.toml"))?;

    let mut package_obj = Package::from_str(package);

    if package_obj.add(executable)? {
        tomlfile.add_dependency(package_obj)?;
        let _ = tomlfile.write(PathBuf::from("Packet.toml"));
    }
    Ok(())
}
