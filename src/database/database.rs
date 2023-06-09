use std::collections::HashMap;
use crate::constants::{
    PAGE_SIZE
};
use super::table::Table;
use super::pager;

pub struct DatabaseMetaData {
    page_size: u16,
    pages_number: u32,
    changes_counter: u32,
    locked: bool,
}
#[derive(Debug)]
pub struct Database {
    pub name: String,
    pub pager: pager::Pager,
    pub tables: HashMap<String, Table>,
}



impl Database {
    pub fn new(name: String) -> Self {
        //  here we read it from the file
        //  temporary values
        Database {
            name,
            pager: pager::new("name".to_string()),
            tables: HashMap::new(),
        }
    }

    pub fn read() {
        // read andd create
    }

    pub fn has_table(&self, table_name: &String) -> bool {
        match self.tables.get(table_name) {
            Some(..) => true,
            None => false,
        }
    }

    //pub fn get_table() {}
}

//create table test(
//    id integer primary_key,
//    username text,
//    email text,
//    password text,
//    age integer,
//    phone_number integer
//)

//insert into test (username ,email ,password ,age ,phone_number) values ('ilyes', 'ilyes@gmail.com', 'password', 22, 05555555);
