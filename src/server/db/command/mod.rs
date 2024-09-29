use meta::MetaCommand;
use sql::SqlCommand;

pub mod meta;
pub mod sql;

#[derive(Debug)]
pub enum CommandError {
    UnrecognizedCommand(String),
    SomethingWentWrong,
}

pub type CommandResult<T> = Result<T, CommandError>;

#[derive(Debug)]
pub enum Command {
    MetaCommand(meta::MetaCommand),
    SqlCommand(sql::SqlCommand),
}

// -- Command -- Constructor
impl Command {
    pub fn new(command: String) -> CommandResult<Self> {
        let meta_regex = regex::Regex::new(r"\.(.+)").unwrap();
        let sql_regex = regex::Regex::new(r"(?:select|insert|update|delete)(.+)").unwrap();

        let lower_case_command = command.to_lowercase();

        if meta_regex.is_match(&lower_case_command) {
            return match MetaCommand::new(lower_case_command) {
                Ok(meta_command) => Ok(Command::MetaCommand(meta_command)),
                Err(meta::MetaCommandError::UnrecognizedCommand(c)) => {
                    Err(CommandError::UnrecognizedCommand(c))
                }
                #[allow(unreachable_patterns)]
                _ => Err(CommandError::SomethingWentWrong),
            };
        }

        if sql_regex.is_match(&lower_case_command) {
            return match SqlCommand::new(command) {
                Ok(sql_command) => Ok(Command::SqlCommand(sql_command)),
                Err(sql::SqlCommandError::UnrecognizedCommand(c)) => {
                    Err(CommandError::UnrecognizedCommand(c))
                }
                #[allow(unreachable_patterns)]
                _ => Err(CommandError::SomethingWentWrong),
            };
        } else {
            Err(CommandError::UnrecognizedCommand(command))
        }
    }
}
