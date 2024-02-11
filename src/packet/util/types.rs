use std::result::Result;

pub type TomlResult<T> = Result<T, Box<dyn std::error::Error>>;
