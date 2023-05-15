pub mod parser;
pub mod tokenizer;
pub mod operations;

use parser::{Statement, ParserError};



pub fn parse(command: String) -> Result<Vec<Statement>, ParserError> {
    // this bclock is returned
    //parser::Parser::new(command);
    parser::Parser::parse(command)
}