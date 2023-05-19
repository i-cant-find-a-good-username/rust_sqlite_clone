pub mod parser;
pub mod tokenizer;
pub mod operations;
//pub crate::database::database::has_table;

use parser::{Statement, ParserError, DataType};



pub fn parse(command: String) -> Result<Vec<Statement>, ParserError> {
    // this bclock is returned
    //parser::Parser::new(command);
    let result = match parser::Parser::parse(command){
        Ok(result) => {
            // validate
            //for statment in result {
            //    match statment{
            //        Statement::Select { table_name, all, columns, clauses } => todo!(),
            //        Statement::Insert { table_name, all, columns, values } => todo!(),
            //        Statement::Update { table_name, allocations, clauses } => todo!(),
            //        Statement::Delete { table_name, selection } => todo!(),
            //        Statement::CreateTable { name, columns } => todo!(),
            //        Statement::Drop { object_type, names } => todo!(),
            //    }
            //}
            Ok(result)
        },
        Err(err) => Err(err)
    };

    result
}





// check table exist
// check if selected cols exist
// check conditions cols correct and correct types
fn validate_select() {}
// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_insert() {}
// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_update() {}
// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_delete() {}
// check table exist
fn validate_create() {}
fn validate_drop()   {}







fn check_table_exist(table_name: String) -> bool {
    false
}