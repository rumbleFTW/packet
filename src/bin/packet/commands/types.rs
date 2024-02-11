use std::result::Result;

pub type ExecResult<T> = Result<T, Box<dyn std::error::Error>>;
