use std::collections::HashMap;
use std::fs::File;
use std::io::{SeekFrom, Seek};
use crate::constants::{
    PAGE_SIZE
};
use super::table::Table;
use super::pager;

//pub struct DatabaseMetaData {
//    page_size: u16,
//    pages_number: u32,
//    changes_counter: u32,
//    locked: bool,
//}
#[derive(Debug)]
pub struct Database {
    pub name: String,
    pub file: File,
    pub pager: pager::Pager,
    pub tables: HashMap<String, Table>,
}



impl Database {
    pub fn new(name: String, file: File) -> Self {
        //  here we read it from the file
        //  temporary values
        //  get database data and tables from the first pages in file and fill tables hashmap
        Database {
            name: name.clone(),
            file: file.try_clone().unwrap(),
            pager: pager::new(file),
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

    pub fn fetch_page(&self) {
        
    }

    pub fn close_database(&mut self) {
        for (i, page) in self.pager.pages.iter().enumerate() {
            //if (page != &[0; PAGE_SIZE]) {
            //    self.file.seek(SeekFrom::Start((i * PAGE_SIZE) as u64)).unwrap();
            //    self.file.write_all(&self.pager.pages[i]).unwrap();
            //}
        }
    }


    pub fn insert_row(&mut self, table: String, cols: Vec<String>, values: Vec<String>) {
        println!("hello there here we add the row::::n, {:?}, {:?}, {:?}", self.tables.get(&table).unwrap(), cols, values);
    }


    pub fn add_table(&mut self, table_name: String, table: Table) -> Result<(), String> {
        self.tables.insert(table_name, table);
        // create page for table
        //
        
        self.file.seek(SeekFrom::Start(PAGE_SIZE.try_into().unwrap())).unwrap();
        //let tables_page = self.file.read_exact(&mut [0; 4096]);

        
        // creates a new page
        //self.file.seek(SeekFrom::End(0)).unwrap();
        //self.file.write_all(&[1; 4096]).unwrap();

        Ok(())
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
