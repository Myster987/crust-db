pub enum SqlCommandError {
    UnrecognizedCommand(String),
}

#[derive(Debug)]
pub enum SqlCommand {
    Select,
    Insert,
    Update,
    Delete,
}

impl SqlCommand {
    pub fn new(command: String) -> Result<Self, SqlCommandError> {
        Ok(Self::Select)
    }
}
