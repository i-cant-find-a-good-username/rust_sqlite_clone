pub mod meta_command;
pub mod sql_command;

use meta_command::MetaCommand;
use sql_command::SQLCommand;

#[derive(Debug)]
pub enum CommandType {
    TypeMeta(MetaCommand),
    TypeSQL(SQLCommand),
}

pub fn process_command(command: &String) -> CommandType {
    match command.starts_with(".") {
        true => CommandType::TypeMeta(MetaCommand::new(command.to_owned())),
        false => CommandType::TypeSQL(SQLCommand::new(command.to_owned())),
    }
}
