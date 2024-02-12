use super::types::TomlResult;
use crate::resolver::package::Package;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};
use toml::{map::Map, Table, Value};

#[derive(Deserialize, Serialize)]
pub struct Toml {
    pub package: Table,
    pub dependencies: Table,
}

impl Toml {
    pub fn init(name: &str) -> TomlResult<Toml> {
        let doc = Toml {
            package: format!("name = '{}'\nversion = '0.1.0'", name).parse::<Table>()?,
            dependencies: "".parse::<Table>()?,
        };
        Ok(doc)
    }
    pub fn load(path: PathBuf) -> TomlResult<Toml> {
        let doc = toml::from_str(&fs::read_to_string(path)?)?;
        Ok(doc)
    }
    pub fn write(&mut self, path: PathBuf) -> TomlResult<()> {
        let mut toml_file: fs::File = fs::File::create(path)?;
        toml_file.write_all(toml::to_string_pretty(self)?.as_bytes())?;
        Ok(())
    }
    pub fn add_dependency(&mut self, package: Package) -> TomlResult<()> {
        let mut dependency_table = Map::new();

        dependency_table.insert("version".to_string(), Value::String(package.version));
        if package.registry.is_some() {
            dependency_table.insert(
                "registry".to_string(),
                Value::String(package.registry.unwrap()),
            );
        }

        self.dependencies
            .insert(package.name, Value::Table(dependency_table));
        Ok(())
    }
}
