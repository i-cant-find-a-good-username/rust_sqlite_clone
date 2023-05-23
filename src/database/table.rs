use std::{collections::{HashMap, HashSet}};

use crate::parse::parser::{Statement, ColumnDef, DataType};

use super::database::Database;

#[derive(Debug)]

pub struct Table {
    name: String,
    columns: Vec<Column>,
    last_id: u64,
    primary_key: String,
}


#[derive(Debug)]
pub struct Column {
    name: String,
    data_type: DataType,
    is_pk: bool,
    is_unique: bool,
    nullable: bool,
}

impl Table {
    pub fn new(params: (String, Vec<ColumnDef>), database: &mut Database) -> Result<Self, String> {
    

        let mut cols: Vec<Column> = Vec::new();
        let mut primary_key = String::from("");
        for col in params.1 {
            if col.primary_key {
                if primary_key == "" {
                    primary_key = col.name.to_string();
                }else{
                    return Err(String::from("only 1 primary key allowed per table"))
                }
            }
            
            cols.push(Column {
                name: col.name,
                data_type: col.data_type,
                is_pk: col.primary_key,
                is_unique: col.unique,
                nullable: !col.not_null,
            })
        }




        Ok(
            Table{
                name: params.0,
                columns: cols,
                last_id: 0,
                primary_key: "ss".to_string(),
            }
        )


    }
    
    
    pub fn show_table_structure(&self) {}
    
    pub fn get_table(&self) {}
    pub fn get_mut_table(&self) {}
}
