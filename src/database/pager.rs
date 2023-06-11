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
    pub current_page: usize,
    pub page_cursor: usize,
}



pub fn new(name: String) -> Pager{
    let path = Path::new(&name);

    
    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();
    

    let ff = Pager{
        // uninitializes pages inited later
        // act as cache
        pages: [[0; PAGE_SIZE]; MAX_PAGES],
        file_length: file.metadata().unwrap().len() as usize / PAGE_SIZE,
        file_desc: 0,
        current_page: 0,
        page_cursor: 0,
    };
    ff
}
impl Pager{
    pub fn get_page(&mut self, page_number: usize, file: &mut File) -> Result<[u8; PAGE_SIZE], String>{
        if page_number > MAX_PAGES {
            // erooooooooooooooor
            return Err(String::from("page does not exist"))
        }
        let page = if self.pages[page_number] == [0; PAGE_SIZE]{
            file.seek(SeekFrom::Start((page_number * PAGE_SIZE) as u64)).unwrap();
            let mut buf = [0; PAGE_SIZE];
            file.read_exact(&mut buf).unwrap();
            self.pages[page_number] = buf;
            return Ok(buf)
        }else{
            return Err(String::from("idk the error in pager"))
        };


    }



    pub fn add_page(&self){
        
    }
    


    pub fn add_table(&mut self, table_string: String, file: &mut File) {
        // do match for error here
        let page = self.get_page(1, file).unwrap();
        println!("{:?}", table_string);
        println!("{:?}", table_string.len());
        for i in  0..PAGE_SIZE {
            if page[i] == 0 && page[i+1] == 0 {

            }
        }
    }
}