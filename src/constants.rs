pub const PAGE_SIZE: usize = 4096;
pub const MAX_PAGES: usize = 100;


pub const DATABASE_PAGES_FIRST_BYTE: u8 = 0;
pub const TABLES_PAGES_FIRST_BYTE: u8 = 1;

pub const INTERNAL_NODE_PAGES_FIRST_BYTE: u8 = 2;
pub const LEAF_NODE_PAGES_FIRST_BYTE: u8 = 3;

