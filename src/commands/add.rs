/// add
/// Install modules to the current project
use serde::{Deserialize, Serialize};
use std::{env, fs, io::Write, path::Path, process::Command, result};

#[derive(Deserialize, Serialize)]
struct Toml {
    package: toml::Table,
    dependencies: toml::Table,
}

// Define a custom result type for command execution
type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;

/// Execute the specified command to install a Python package using pip.
///
/// # Arguments
///
/// * `package_name` - The name of the package to install.
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if the installation fails.
pub fn exec(package_name: &str) -> CommandResult<()> {
    // Execute the pip command to install the specified package
    let executable = if env::consts::OS == "windows" {
        Path::new(".").join("env").join("Scripts").join("pip")
    } else {
        Path::new(".").join("env").join("bin").join("pip")
    };
    let _ = Command::new(executable.as_os_str())
        .arg("install")
        .arg(package_name)
        .status();
    
    // Parse the package name and version from the input argument
    let (pkg_name, pkg_version) = match package_name.find("=="){
        // If the package name contains a version, split it into name and version
        Some(index) => {
            let (pkg_name, pkg_version) = package_name.split_at(index);
            (pkg_name.trim(), pkg_version.trim_start_matches("=="))
        }
        // If the package name does not contain a version, use "*" as the version
        None => (package_name, "*"),
    };

    // Read and deserialize the existing TOML data from "Packet.toml"
    let mut toml_data: Toml = toml::from_str(&fs::read_to_string("Packet.toml")?)?;

    // Insert the new package and version into the dependencies section

    toml_data.dependencies.insert(
        pkg_name.to_string(),
        toml::Value::String(format!("{}", pkg_version)),
    );

    // Create or overwrite "Packet.toml" with the updated TOML data
    let mut toml_file: fs::File = fs::File::create("Packet.toml")?;
    toml_file.write_all(toml::to_string_pretty(&toml_data)?.as_bytes())?;

    Ok(())
}
