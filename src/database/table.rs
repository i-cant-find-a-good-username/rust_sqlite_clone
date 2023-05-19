use std::collections::{HashMap, HashSet};

use crate::parse::parser::Statement;

enum DataType {
    Integer,
    String,
    Float,
    Boolean,
    Invalid,
}

struct Table {
    name: String,
    columns: Vec<Column>,
    last_id: u64,
    primary_key: String,
}

struct Column {
    name: String,
    data_type: DataType,
    is_pk: bool,
    is_unique: bool,
    nullable: bool,
}

impl Table {
    pub fn new(stmt: Statement) -> self {
        let data = match stmt{
            Statement::Insert { table_name, all, columns, values } => {
                (table_name, all, columns, values)
            },
            _ => {panic!("wrong query type")}
        };

        if data.1 == true && data.2 == None {
            
        }


    }
    pub fn show_table_structure() {}
}
