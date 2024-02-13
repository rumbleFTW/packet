use regex::Regex;
use std::{error::Error, path::PathBuf, process};

pub struct Package {
    pub name: String,
    pub version: String,
    pub registry: Option<String>,
}

impl Package {
    pub fn from_str(package_string: &str) -> Package {
        let regex_pattern = Regex::new(r"([=~><])").unwrap();
        let (package_name, package_version) =
            if let Some(index) = regex_pattern.find(&package_string) {
                package_string.split_at(index.start())
            } else {
                (package_string, "")
            };
        Package {
            name: String::from(package_name),
            version: String::from(package_version),
            registry: None,
        }
    }
    pub fn add(&mut self, executable: PathBuf) -> Result<bool, Box<dyn Error>> {
        let exit_status = process::Command::new(executable.as_os_str())
            .arg("install")
            .arg(format!("{}{}", self.name, self.version))
            .status()?;
        Ok(exit_status.success())
    }
    // pub fn to_table(&mut self)
}
