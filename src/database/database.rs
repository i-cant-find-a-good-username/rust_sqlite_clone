use std::collections::HashMap;

use crate::table::Table;


struct DatabaseMetaData{
    page_size: u16,
    pages_number: u32,
    changes_counter: u32,
    locked: bool,
}


struct Database{
    name: String,

    tables: HashMap<String, Table>,
}







impl Database{
    
    fn new(){

    }

    
    fn has_table(){

    }

    
    fn get_table(){

    }
    
}