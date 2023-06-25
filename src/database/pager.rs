use std::{path::Path, fs::{File, OpenOptions}, io::{Seek, SeekFrom, Read}};

use crate::constants::{
    PAGE_SIZE,
    MAX_PAGES
};



#[derive(Debug)]
pub struct Pager {
    // maybe a hashmap better
    // hashmap to decide which page is loaded
    pub pages: [[u8; PAGE_SIZE]; MAX_PAGES],
    pub file_length: usize,
    pub file_desc: usize,
    pub current_page: usize,
    pub page_cursor: usize,
}



pub fn new(name: String, mut file: File) -> Pager{
    let path = Path::new(&name);
    let mut pages: [[u8; 4096]; MAX_PAGES] = [[0; PAGE_SIZE]; MAX_PAGES];

    let mut buffer: [u8; 4096] = [0; PAGE_SIZE];
    
    for i in 0..MAX_PAGES {
        file.seek(SeekFrom::Start((PAGE_SIZE*i).try_into().unwrap())).unwrap();
        match file.read_exact(&mut buffer){
            Ok(_) => pages[i] = buffer,
            Err(_) => break
        };
    }


    let ff = Pager{
        // uninitializes pages inited later
        // act as cache
        pages,
        file_length: file.metadata().unwrap().len() as usize / PAGE_SIZE,
        file_desc: 0,
        current_page: 0,
        page_cursor: 0,
    };
    println!("{:?}", ff);
    ff
}
impl Pager{
    pub fn get_page(&mut self, page_number: usize, file: &mut File) -> bool{
        if page_number > MAX_PAGES {
            // erooooooooooooooor
            return false
        }
        if self.pages[page_number] == [0; PAGE_SIZE]{
            file.seek(SeekFrom::Start((page_number * PAGE_SIZE) as u64)).unwrap();
            let mut buf = [0; PAGE_SIZE];
            file.read_exact(&mut buf).unwrap();

            self.pages[page_number] = buf;
            self.current_page = page_number;
            self.page_cursor = 0;
        }else{
            return true
        };
        true
    }



    pub fn add_page(&self){
        
    }
    


    pub fn add_table(&mut self, table_string: String, file: &mut File) {
        // do match for error here
        // goes to tables page
        let table_string_bytes = table_string.as_bytes();
        let table_string_len = table_string_bytes.len();

        self.current_page = 1;
        self.page_cursor = 0;


        let page = self.get_page(1, file);
        println!("{:?}", table_string);
        println!("{:?}", table_string.len());
        for i in  0..PAGE_SIZE-1 {
            if self.pages[self.current_page][i] != 0 {
                self.page_cursor += 1;
                println!("{:?}", self.page_cursor);

            }else{
                if self.pages[self.current_page][i+1] != 0 {
                    self.page_cursor += 1;
                    println!("{:?}", self.page_cursor);
                }
            }
        }
    }
}