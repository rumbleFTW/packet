use super::types::TomlResult;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};
use toml::{value::Array, Table};

#[derive(Deserialize, Serialize)]
pub struct Toml {
    pub package: Table,
    pub dependencies: Array,
}

impl Toml {
    pub fn init(name: &str) -> TomlResult<Toml> {
        let doc = Toml {
            package: format!("name = '{}'\nversion = '0.1.0'", name).parse::<Table>()?,
            dependencies: vec![],
        };
        Ok(doc)
    }
    pub fn load(path: PathBuf) -> TomlResult<Toml> {
        let doc = toml::from_str(&fs::read_to_string(path)?)?;
        Ok(doc)
    }
    pub fn write(document: &Toml, path: PathBuf) -> TomlResult<()> {
        let mut toml_file: fs::File = fs::File::create(path)?;
        toml_file.write_all(toml::to_string_pretty(document)?.as_bytes())?;
        Ok(())
    }
}
