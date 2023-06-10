use std::collections::{HashMap, HashSet};
use std::io::{Write, SeekFrom, Seek};

use crate::parse::parser::{ColumnDef, DataType, Statement};

use crate::btree;

use super::database::Database;

#[derive(Debug)]

pub struct Table {
    name: String,
    columns: Vec<Column>,
    last_id: u64,
    primary_key: Option<String>,
}

#[derive(Debug)]
pub struct Column {
    name: String,
    data_type: DataType,
    is_pk: bool,
    is_unique: bool,
    nullable: bool,
}

use std::fs::{File, OpenOptions};

impl Table {
    pub fn new(params: (String, Vec<ColumnDef>), database: &mut Database, mut file: &File) -> Result<Self, String> {
        let mut table_string = String::from("table");
        table_string.push_str(&" ");
        table_string.push_str(&params.0);
        table_string.push_str(&"(");
        let mut cols: Vec<Column> = Vec::new();
        let mut primary_key = String::from("");
        for col in params.1 {
            if col.primary_key {
                if primary_key == "" {
                    primary_key = col.name.to_string();
                } else {
                    return Err(String::from("only 1 primary key allowed per table"));
                }
            }

            table_string.push_str(&col.name);
            table_string.push_str(&" ");
            match col.data_type{
                DataType::Text => table_string.push_str(&"text"),
                DataType::Integer => table_string.push_str(&"integer"),
                DataType::Float => table_string.push_str(&"float"),
                DataType::Boolean => table_string.push_str(&"boolean"),
                DataType::Null => table_string.push_str(&"null"),
            }
            table_string.push_str(&" ");
            if col.primary_key{
                table_string.push_str(&"primary_key");
                table_string.push_str(&" ");
            }
            if col.unique{
                table_string.push_str(&"unique");
                table_string.push_str(&" ");
            }
            if col.not_null{
                table_string.push_str(&"not_null");
                table_string.push_str(&" ");
            }
            table_string = table_string[0..table_string.len() - 1].to_string();
            table_string.push_str(&", ");
            cols.push(Column {
                name: col.name,
                data_type: col.data_type,
                is_pk: col.primary_key,
                is_unique: col.unique,
                nullable: !col.not_null,
            })
        }
        println!("{:?}", table_string);
        table_string = table_string[0..table_string.len() - 2].to_string();
        table_string.push_str(&");");
        println!("{:?}", table_string);



   

        file.seek(SeekFrom::Start(4096)).unwrap();
        file.write_all(table_string.as_bytes()).unwrap();

        Ok(Table {
            name: params.0,
            columns: cols,
            last_id: 0,
            primary_key: Some(primary_key),
        })
    }

    pub fn show_table_structure(&self) {}

    pub fn get_table(&self) {}
}
