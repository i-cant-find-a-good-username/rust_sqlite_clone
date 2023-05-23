use std::collections::{HashMap,
    HashSet};

use crate::parse::parser::Statement;

use super::database::Database;

enum DataType {
    Integer,
    String,
    Float,
    Boolean,
    Invalid,
}

pub struct Table {
    name: String,
    columns: Vec<Column>,
    last_id: u64,
    primary_key: String,
}

pub struct Column {
    name: String,
    data_type: DataType,
    is_pk: bool,
    is_unique: bool,
    nullable: bool,
}

impl Table {
    pub fn new(stmt: Statement, database: &mut Database) -> Self {
        let data = match stmt{
            Statement::CreateTable { name, columns } => {
                (name, columns)
            },
            // impossible case
            _ => {panic!("wrong query type")}
        };


        let mut cols: Vec<Column> = Vec::new();
        for col in data.1 {
            
        }




        Table{
            name: data.0,
            columns: todo!(),
            last_id: 0,
            primary_key: todo!(),
        }


    }
    
    
    //pub fn show_table_structure() {}
}
