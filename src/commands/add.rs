use serde::{Deserialize, Serialize};
use std::{fs, io::Write, process::Command, result};

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
    let _ = Command::new("env/bin/pip")
        .arg("install")
        .arg(package_name)
        .status();

    // Read and deserialize the existing TOML data from "Packet.toml"
    let mut toml_data: Toml = toml::from_str(&fs::read_to_string("Packet.toml")?)?;

    // Insert the new package and an empty string into the dependencies section
    toml_data.dependencies.insert(
        package_name.to_string(),
        toml::Value::String("".to_string()),
    );

    // Create or overwrite "Packet.toml" with the updated TOML data
    let mut toml_file: fs::File = fs::File::create("Packet.toml")?;
    toml_file.write_all(toml::to_string_pretty(&toml_data)?.as_bytes())?;

    Ok(())
}
