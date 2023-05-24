pub mod operations;
pub mod parser;
pub mod tokenizer;
//pub crate::database::database::has_table;

use parser::{Allocation, Clause, ParserError, Statement};

use crate::database::{database::Database, table::Table};

use self::parser::ColumnDef;

pub fn parse(command: String, database: &mut Database) -> Result<String, String> {
    // this bclock is returned
    //parser::Parser::new(command);
    match parser::Parser::parse(command) {
        Ok(result) => {
            // validate
            for statement in result {
                let gg = match statement {
                    Statement::Select {
                        table_name,
                        all,
                        columns,
                        clauses,
                    } => validate_select((table_name, all, columns, clauses), database),
                    Statement::Insert {
                        table_name,
                        all,
                        columns,
                        values,
                    } => validate_insert((table_name, all, columns, values), database),
                    Statement::Update {
                        table_name,
                        allocations,
                        clauses,
                    } => validate_update((table_name, allocations, clauses), database),
                    Statement::Delete {
                        table_name,
                        selection,
                    } => validate_delete((table_name, selection), database),
                    Statement::CreateTable { name, columns } => {
                        validate_create((name, columns), database)
                    }
                    Statement::Drop { object_type, names } => {
                        validate_drop((object_type, names), database)
                    }
                };
                return match gg {
                    Ok(ff) => Ok(ff),
                    Err(err) => Err(err),
                };
            }
        }
        Err(err) => {
            return Err(format!(
                "message {},\ntoken: {},\ntoken index: {}",
                err.message, err.token, err.index
            ))
        }
    };

    Ok("result".to_string())
}

// check table exist
// check if selected cols exist
// check conditions cols correct and correct types
fn validate_select(
    params: (String, bool, Option<Vec<String>>, Option<Clause>),
    database: &mut Database,
) -> Result<String, String> {
    match check_table_exist("table_name".to_string()) {
        true => {
            // if all
            if params.1 {
            } else {
            }

            Ok(String::from("dazdazd"))
        }
        false => return Err(String::from("table doesnt exist")),
    }
}

// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_insert(
    params: (String, bool, Option<Vec<String>>, Vec<String>),
    database: &mut Database,
) -> Result<String, String> {
    match check_table_exist("table_name".to_string()) {
        true => Ok(String::from("dazdazd")),
        false => return Err(String::from("table doesnt exist")),
    }
}

// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_update(
    params: (String, Vec<Allocation>, Clause),
    database: &mut Database,
) -> Result<String, String> {
    match check_table_exist("table_name".to_string()) {
        true => Ok(String::from("dazdazd")),
        false => return Err(String::from("table doesnt exist")),
    }
}
// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_delete(params: (String, Clause), database: &mut Database) -> Result<String, String> {
    match check_table_exist("table_name".to_string()) {
        true => Ok(String::from("dazdazd")),
        false => return Err(String::from("table doesnt exist")),
    }
}

// check table exist
fn validate_create(
    params: (String, Vec<ColumnDef>),
    database: &mut Database,
) -> Result<String, String> {
    match database.has_table(&params.0) {
        true => Err(String::from("table already exists")),
        false => {
            let table_name = params.0.to_string();
            let table = Table::new(params, database)?;
            table.show_table_structure();
            database.tables.insert(table_name, table);
            println!("{:?}", database);
            Ok(String::from("table created"))
        }
    }
}

fn validate_drop(params: (String, Vec<String>), database: &mut Database) -> Result<String, String> {
    match check_table_exist("table_name".to_string()) {
        true => {
            //let table = Table::new();
            //table.show_table_structure();
            // create table
            // show table structure
            // add table to database
            Ok(String::from("dazdazd"))
        }
        false => return Err(String::from("table doesnt exist")),
    }
}

fn check_table_exist(table_name: String) -> bool {
    false
}
