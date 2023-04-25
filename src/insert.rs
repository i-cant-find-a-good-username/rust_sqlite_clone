use std::mem::{size_of, size_of_val};

struct Table {
    rows_number: u32,
    pages: u32,
}

const PAGE_SIZE: u16 = 4096;
const TABLE_MAX_PAGE: u8 = 100;
const ROWS_PER_PAGE: u16 = 4096;
const MAX_ROWS_PER_PAGE: u16 = 4096;

// insert into users(id, username, email) values('fze', 'efz', 'fezf');
pub fn insert (query: String) {

    let id_size = size_of::<Table>();



    println!("the insert query is: {}", query);
    println!("the insert query is: {}", size_of_val(&*query));
    let query_parts: Vec<&str> = query.split(" ").collect();
}