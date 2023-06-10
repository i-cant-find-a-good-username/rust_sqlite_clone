use std::{path::Path, fs::File};

use crate::constants::{
    PAGE_SIZE,
    MAX_PAGES
};



#[derive(Debug)]
pub struct Pager {
    pub pages: [[u8; PAGE_SIZE]; MAX_PAGES],
    pub file_length: usize,
    pub file_desc: usize,
    pub database_page: [u8; PAGE_SIZE],//[u8; PAGE_SIZE],
    pub tables_pages: Vec<[u8; PAGE_SIZE]>,//[u8; PAGE_SIZE],
}



pub fn new(name: String) -> Pager{
    let path = Path::new(&name);

    
    let mut file = File::create(path).unwrap();

 

    let ff = Pager{
        pages: [[0; PAGE_SIZE]; MAX_PAGES],
        file_length: file.metadata().unwrap().len() as usize / PAGE_SIZE,
        file_desc: 0,
        database_page: [0; PAGE_SIZE],
        tables_pages: Vec::from([[0; PAGE_SIZE]]),
    };
    ff
}
impl Pager{

    pub fn get_page(&self){
        
    }
    pub fn add_page(&self){
        
    }
    pub fn add_table(&self){
        
    }
}