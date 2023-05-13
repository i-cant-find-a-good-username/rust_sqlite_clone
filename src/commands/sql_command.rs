use crate::parse::parser::{self, Parser, Statement, ParserError};

#[derive(Debug)]
pub enum SQLCommand {
    Insert(String),
    Select(String),
    Create(String),
    Update(String),
    Delete(String),
    Invalid(String),
}

impl SQLCommand {
    pub fn new(command: String) -> SQLCommand {
        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned().to_lowercase();

        match cmd.as_ref() {
            "insert" => SQLCommand::Insert(command),
            "select" => SQLCommand::Select(command),
            "create" => SQLCommand::Create(command),
            "update" => SQLCommand::Update(command),
            "delete" => SQLCommand::Delete(command),
            _ => SQLCommand::Invalid("invalid query type".to_string()),
        }
    }
}

pub fn run_sql_command(command: String) -> Result<String, String> {

    // match this result
    let result: Result<Vec<Statement>, ParserError> = match Parser::parse(command) {
        Ok(msg) => {
            return Ok(format!("{:?}", msg))
        },
        Err(msg) => {
            return Err(format!("{:?}", msg))
        },
    };


}
