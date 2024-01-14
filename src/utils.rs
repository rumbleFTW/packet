mod utils {
    use std::result;

    pub type CommandResult<T> = result::Result<T, Box<dyn std::error::Error>>;
}
