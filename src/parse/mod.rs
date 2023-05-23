pub mod operations;
pub mod parser;
pub mod tokenizer;
//pub crate::database::database::has_table;

use parser::{ParserError, Statement};

use crate::database::{
    database::Database,
    table::Table
};

pub fn parse(command: String, database: &mut Database) -> Result<Vec<Statement>, ParserError> {
    // this bclock is returned
    //parser::Parser::new(command);
    let result = match parser::Parser::parse(command) {
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
        }
        Err(err) => Err(err),
    };

    result
}

// check table exist
// check if selected cols exist
// check conditions cols correct and correct types
fn validate_select(table_name: String) -> Result<String, String> {
    match check_table_exist("table_name".to_string()){
        true => Ok(String::from("dazdazd")),
        false => return Err(String::from("table doesnt exist"))
    }
}


































// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_insert(table_name: String) -> Result<String, String> {
    match check_table_exist("table_name".to_string()){
        true => {
            Ok(String::from("dazdazd"))
        },
        false => return Err(String::from("table doesnt exist"))
    }
}
































// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_update(table_name: String) -> Result<String, String> {
    match check_table_exist("table_name".to_string()){
        true => Ok(String::from("dazdazd")),
        false => return Err(String::from("table doesnt exist"))
    }
}
// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_delete(table_name: String) -> Result<String, String> {
    match check_table_exist("table_name".to_string()){
        true => Ok(String::from("dazdazd")),
        false => return Err(String::from("table doesnt exist"))
    }
}



































// check table exist
fn validate_create(stmt: Statement, database: &mut Database) -> Result<String, String> {
    match check_table_exist("table_name".to_string()){
        true => return Err(String::from("table already exists")),
        false => {
            Table::new(stmt, database);
            Ok(String::from("dazdazd"))
        }
    }
}

















































fn validate_drop(table_name: String) -> Result<String, String>{
    match check_table_exist("table_name".to_string()){
        true => {
            //let table = Table::new();
            //table.show_table_structure();
            // create table
            // show table structure
            // add table to database
            Ok(String::from("dazdazd"))
        },
        false => return Err(String::from("table doesnt exist"))
    }
}

fn check_table_exist(table_name: String) -> bool {
    false
}
