
#[derive(Debug)]
pub enum SQLCommand{
    Insert(String),
    Select(String),
    Create(String),
    Update(String),
    Delete(String),
    Invalid(String)
}




impl SQLCommand {
    pub fn new(command: String) -> SQLCommand {

        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned();

        match cmd.as_ref() {
            "insert" => SQLCommand::Insert(command),
            "Select" => SQLCommand::Select(command),
            "Create" => SQLCommand::Create(command),
            "Update" => SQLCommand::Update(command),
            "Delete" => SQLCommand::Delete(command),
            _ => SQLCommand::Invalid("invalid query type".to_string()),
        }
        
    }




}


pub fn run_sql_command(command: SQLCommand) -> Result<String, String> {

    //tokenize parse then execute the query

    Ok("zad".to_string())
}