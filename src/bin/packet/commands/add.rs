/// add
/// Install modules to the current project
use super::types::ExecResult;
use packet::util::parsers::Toml;
use std::{env, path::PathBuf, process};

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
    let exit_status = process::Command::new(executable.as_os_str())
        .arg("install")
        .arg(package)
        .status()?;

    // Parse the package name and version from the input argument
    let (package_name, _) = match package.find("==") {
        // If the package name contains a version, split it into name and version
        Some(index) => {
            let (package_name, package_version) = package.split_at(index);
            (
                package_name.trim(),
                package_version.trim_start_matches("=="),
            )
        }
        // If the package name does not contain a version, use "*" as the version
        None => (package, "*"),
    };

    // If installed successfully
    if exit_status.success() {
        // Read and deserialize the existing TOML data from "Packet.toml"
        let mut toml_data = Toml::load(PathBuf::from("Packet.toml"))?;

        // Insert the new package name into the dependencies if not already
        if !toml_data
            .dependencies
            .contains(&toml::Value::String(package_name.to_string()))
        {
            toml_data
                .dependencies
                .push(toml::Value::String(package_name.to_string()));
            let _ = Toml::write(&toml_data, PathBuf::from("Packet.toml"))?;
        }
    }
    Ok(())
}
