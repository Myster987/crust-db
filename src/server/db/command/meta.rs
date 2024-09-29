use regex::Regex;

pub enum MetaCommandError {
    UnrecognizedCommand(String),
}

#[derive(Debug)]
pub enum MetaCommand {
    Exit,
    ShutDown,
}

impl MetaCommand {
    pub fn new(command: String) -> Result<Self, MetaCommandError> {
        let exit_regex = Regex::new(r"\.exit").unwrap();
        let shut_down_regex = Regex::new(r"\.shutdown").unwrap();
        
        if exit_regex.is_match(&command) {
            Ok(Self::Exit)
        } else if shut_down_regex.is_match(&command) {
            Ok(Self::ShutDown)
        } else {
            Err(MetaCommandError::UnrecognizedCommand(command))
        }
    }
}
