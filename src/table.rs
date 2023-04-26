use std::mem::{ size_of };


use crate::user::User;




const EMPTY_STRING: String = String::new();

const USERNAME_SIZE: u64 = 255;
const EMAIL_SIZE: u64 = 255;

const PAGE_SIZE: u64 = 4096;
const TABLE_MAX_PAGE: u64 = 100;
const ROW_SIZE: u64 = USERNAME_SIZE + EMAIL_SIZE + size_of::<u64>() as u64;
const ROWS_PER_PAGE: u64 = PAGE_SIZE / ROW_SIZE;
const MAX_ROWS_PER_PAGE: u64 = 4096;

pub struct Table {
    pub rows_number: u64,
    pub pages: [String; TABLE_MAX_PAGE as usize],
}


impl Table{
    pub fn new() -> Table {
        Table{
            rows_number: 0,
            pages: [EMPTY_STRING; 100],
        }
    }


    pub fn add_row(&mut self) {
        self.rows_number += 1;
    }

    pub fn insert_row(&mut self, row: User){

    }
}