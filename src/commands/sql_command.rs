use crate::parse::parser;


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
    let parsed_query = parser::parse(command);

    Ok("zad".to_string())
}
