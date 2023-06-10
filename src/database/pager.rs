use std::{path::Path, fs::{File, OpenOptions}, io::{Seek, SeekFrom, Read}};

use crate::constants::{
    PAGE_SIZE,
    MAX_PAGES
};



#[derive(Debug)]
pub struct Pager {
    // maybe a hashmap better
    pub pages: [[u8; PAGE_SIZE]; MAX_PAGES],
    pub file_length: usize,
    pub file_desc: usize,
}



pub fn new(name: String) -> Pager{
    let path = Path::new(&name);

    
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();
    
    // maybe get database abd tables pages and put them in pager

    let ff = Pager{
        // uninitializes pages inited later
        // act as cache
        pages: [[0; PAGE_SIZE]; MAX_PAGES],
        file_length: file.metadata().unwrap().len() as usize / PAGE_SIZE,
        file_desc: 0,
    };
    ff
}
impl Pager{
    pub fn get_page(&mut self, page_number: usize, file: &mut File){
        if page_number > MAX_PAGES {
            // erooooooooooooooor
        }
        if self.pages[page_number] == [0; PAGE_SIZE]{
            file.seek(SeekFrom::Start((page_number * PAGE_SIZE) as u64)).unwrap();
            let mut buf = [0; PAGE_SIZE];
            file.read_exact(&mut buf).unwrap();
            self.pages[page_number] = buf;
        }else{

        }
    }
    pub fn add_page(&self){
        
    }
    pub fn add_table(&self){
        
    }
}