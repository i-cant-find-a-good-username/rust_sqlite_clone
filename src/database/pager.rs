use std::{fs::{File}, io::{Seek, SeekFrom, Read, Write}, collections::{HashMap}};

use crate::constants::{
    PAGE_SIZE,
    MAX_PAGES
};



#[derive(Debug)]
pub struct Pager {
    // maybe a hashmap better
    // hashmap to decide which page is loaded
    pub pages: HashMap<usize, [u8; PAGE_SIZE]>,
    pub file_length: usize,
    pub file_desc: usize,
    pub current_page: usize,
    pub page_cursor: usize,
}



pub fn new(mut file: File) -> Pager{
    let mut pages = HashMap::new();

    let mut buffer: [u8; 4096] = [0; PAGE_SIZE];
    
    for i in 0..MAX_PAGES {
        file.seek(SeekFrom::Start((PAGE_SIZE*i).try_into().unwrap())).unwrap();
        match file.read_exact(&mut buffer){
            Ok(_) => pages.insert(i, buffer),
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
    pub fn get_page(&mut self, page_number: usize, file: &mut File) -> Result<[u8; PAGE_SIZE], String>{
        let cache_miss  = self.pages.contains_key(&page_number);
        if cache_miss {
            return Ok(self.pages.get(&page_number).unwrap().clone())
        }else{
            // reads new page
            if self.pages.len() == MAX_PAGES {
                // removes first element
                let bind = self.pages.clone();
                if let Some((key, _)) = bind.iter().next() {
                    self.pages.remove(key);
                }
            }

            let mut buffer: [u8; 4096] = [0; PAGE_SIZE];
            file.seek(SeekFrom::Start((PAGE_SIZE * page_number).try_into().unwrap())).unwrap();
            match file.read_exact(&mut buffer) {
                Ok(_) => {
                    self.pages.insert(page_number, buffer);
                    Ok(buffer)
                },
                Err(_) => Err(String::from("hello there")),
            }
        }
    }
    
  


    //pub fn add_page(&self){
    //    
    //}
    


    pub fn add_table(&mut self, table_string: String, file: &mut File) {
        // do match for error here
        // goes to tables page
        self.current_page = 1;
        self.page_cursor = 0;

        let mut affected_in_this_page = false;
        while !affected_in_this_page{
            for i in  0..PAGE_SIZE-1 {
                if self.pages.get(&self.current_page).unwrap()[i] == 0 {
                    let arr: &[u8] = &self.pages.get(&self.current_page).unwrap()[i..];
                    let len = arr.len();
                    if arr == vec![0; len].as_slice() {
                        if len > table_string.len() {
                            // to leaver null char between every 2 tables
                            //if len == PAGE_SIZE {
                            //    file.seek(SeekFrom::Start(((self.current_page * PAGE_SIZE) +i).try_into().unwrap())).unwrap();
                            //}else{
                            //    file.seek(SeekFrom::Start(((self.current_page * PAGE_SIZE) + i + 1).try_into().unwrap())).unwrap();
                            //}
                            file.seek(SeekFrom::Start(((self.current_page * PAGE_SIZE) +i).try_into().unwrap())).unwrap();
                            file.write_all(table_string.as_bytes()).unwrap();
                            affected_in_this_page = true;
                            break
                        }
                    }
                }
                if affected_in_this_page {
                    // add page here
                    self.current_page += 1;
                }
            }
        }

    }
}