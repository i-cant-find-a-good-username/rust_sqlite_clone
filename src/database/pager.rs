use crate::constants::{
    PAGE_SIZE,
    MAX_PAGES
};



#[derive(Debug)]
pub struct Pager {
    pub pages: [[u8; PAGE_SIZE]; MAX_PAGES],
    pub file_length: usize,
    pub file_desc: usize,
    pub database_page: [u8; PAGE_SIZE],
    pub tables_page: [u8; PAGE_SIZE],
}



pub fn new(name: String) -> Pager{
    Pager{
        pages: [[0; PAGE_SIZE]; MAX_PAGES],
        file_length: 58,// pages number i think
        file_desc: 0,// idk yet
        database_page: [0; PAGE_SIZE],
        tables_page: [0; PAGE_SIZE],
    }     
}
impl Pager{

    pub fn get_page(&self){
        
    }
    pub fn add_page(&self){
        
    }
}