use std::{mem::{ size_of, size_of_val }, collections::HashSet};


use crate::user::User;




const EMPTY_STRING: String = String::new();

const USERNAME_SIZE: u64 = 50;
const EMAIL_SIZE: u64 = 20;

const PAGE_SIZE: u64 = 4096;
const TABLE_MAX_PAGE: u64 = 100;
const ROW_SIZE: u64 = USERNAME_SIZE + EMAIL_SIZE + size_of::<u64>() as u64;
const ROWS_PER_PAGE: u64 = PAGE_SIZE / ROW_SIZE;
const MAX_ROWS_PER_PAGE: u64 = 4096;


#[derive(Eq, PartialEq)]
pub struct Table{
    pub rows_number: u64,
    pub pages: HashSet<User>,
}


impl Table{
    pub fn new() -> Table  {
        Table{
            rows_number: 0,
            pages: HashSet::new(),
        }
    }


    pub fn add_row(&mut self) {
        self.rows_number += 1;
    }

    pub fn insert_row(&mut self, row: User){



        
        //let hh = self.pages;
        self.pages.insert(row);
        //println!("{:?}", self.pages);
        for row in &self.pages{
            println!("{:?}", row);
            println!("{:?}", size_of_val(row));
            println!("{:?}", size_of_val(&*row.username));
            println!("{:?}", size_of_val(&*row.email));
        }


        //println!("{:?}", self.pages);
        //self.pages[(self.rows_number - 1) as usize] = (5, "dfzadaz", "dazda");
        //println!("{:?}", self.pages[(self.rows_number - 1) as usize]);
        
    }
}


//from([
//  (5 as u64, String::from("hello"), String::from("hello")),
//]) 



//insert 0 ilyes ilyes@gmail;
//insert 1 ilyes ilyeszefzefzef@gmail;
//insert 2 ilyes ilazdzefazdazdyes@gmail;
//insert 3 ilyes ilyes@gmail;
//insert 4 ilyes ilyzefzefes@gmail;
//insert 5 ilyes ilyes@gmail;
//insert 6 ilyes ilyefzefeazdazdas@gmail;
//insert 7 ilyes ilyes@gmail;
//insert 8 ilyezfzefes ilyes@gmail;
//insert 9 ilyes ilyes@gmail;
//insert 10 ilyes ilyes@gmail;
//insert 11 ilyes ilyes@gmail;
//insert 12 izefzefzefzefzefzeflyes ilyes@gmazefzefzefzfezfefzefzefzefzil;
//insert 13 ilyes ilyes@gmail;











