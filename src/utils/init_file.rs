use std::{io::{SeekFrom, Write, Seek}, process, path::Path, fs::{OpenOptions, File}};

use crate::{constants::PAGE_SIZE, utils::int_byte_convert::transform_u16_to_array_of_u8};

pub fn file_init(path: &String) -> Result<File, String> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(path)
            .unwrap();
        println!("{:?}", file.metadata().unwrap().len());


        // init file
        if file.metadata().unwrap().len() == 0 {
            // databse header 100 bytes
            file.write_all(&[0; PAGE_SIZE]).unwrap();
            // write tables page
            // The header string: "SQLite format 3\000" 
            file.seek(SeekFrom::Start(0)).unwrap();
            file.write_all(b"SQLite format 3\000").unwrap();
            // The database page size in bytes. Must be a power of two between 512 and 32768 inclusive, or the value 1 representing a page size of 65536. 
            file.seek(SeekFrom::Start(16)).unwrap();
            file.write_all(&transform_u16_to_array_of_u8(PAGE_SIZE.try_into().unwrap())).unwrap();
            // File format write version. 1 for legacy; 2 for WAL. 
            file.seek(SeekFrom::Start(18)).unwrap();
            file.write_all(&[1]).unwrap();
            // File format read version. 1 for legacy; 2 for WAL. 
            file.seek(SeekFrom::Start(19)).unwrap();
            file.write_all(&[1]).unwrap();
            // Bytes of unused "reserved" space at the end of each page. Usually 0. 
            file.seek(SeekFrom::Start(20)).unwrap();
            file.write_all(&[0]).unwrap();
            // Maximum embedded payload fraction. Must be 64. 
            file.seek(SeekFrom::Start(21)).unwrap();
            file.write_all(&[64]).unwrap();
            // Maximum embedded payload fraction. Must be 32. 
            file.seek(SeekFrom::Start(22)).unwrap();
            file.write_all(&[32]).unwrap();
            // Leaf payload fraction. Must be 32. 
            file.seek(SeekFrom::Start(23)).unwrap();
            file.write_all(&[32]).unwrap();
            // File change counter. 
            file.seek(SeekFrom::Start(24)).unwrap();
            file.write_all(&[0, 0, 0, 0]).unwrap();
            // Size of the database file in pages. The "in-header database size". 
            file.seek(SeekFrom::Start(28)).unwrap();
            file.write_all(&[0, 0, 0, 2]).unwrap();
            // Page number of the first freelist trunk page.
            file.seek(SeekFrom::Start(32)).unwrap();
            file.write_all(&[0, 0, 0, 0]).unwrap();
            // Total number of freelist pages. 
            file.seek(SeekFrom::Start(36)).unwrap();
            file.write_all(&[0, 0, 0, 0]).unwrap();
            // The schema cookie. 
            file.seek(SeekFrom::Start(40)).unwrap();
            file.write_all(&[0, 0, 0, 2]).unwrap();
            // The schema format number. Supported schema formats are 1, 2, 3, and 4. 
            file.seek(SeekFrom::Start(44)).unwrap();
            file.write_all(&[0, 0, 0, 4]).unwrap();
            // Default page cache size. 
            file.seek(SeekFrom::Start(48)).unwrap();
            file.write_all(&[0, 0, 0, 0]).unwrap();
            // The page number of the largest root b-tree page when in auto-vacuum or incremental-vacuum modes, or zero otherwise. 
            file.seek(SeekFrom::Start(52)).unwrap();
            file.write_all(&[0, 0, 0, 0]).unwrap();
            // The database text encoding. A value of 1 means UTF-8. A value of 2 means UTF-16le. A value of 3 means UTF-16be. 
            file.seek(SeekFrom::Start(56)).unwrap();
            file.write_all(&[0, 0, 0, 1]).unwrap();
            // The "user version" as read and set by the user_version pragma. 
            file.seek(SeekFrom::Start(60)).unwrap();
            file.write_all(&[0, 0, 0, 0]).unwrap();
            // True (non-zero) for incremental-vacuum mode. False (zero) otherwise. 
            file.seek(SeekFrom::Start(64)).unwrap();
            file.write_all(&[0, 0, 0, 0]).unwrap();
            // The "Application ID" set by PRAGMA application_id. 
            file.seek(SeekFrom::Start(68)).unwrap();
            file.write_all(&[0, 0, 0, 0]).unwrap();
            // Reserved for expansion. Must be zero. 
            file.seek(SeekFrom::Start(72)).unwrap();
            file.write_all(&[0, 20]).unwrap();
            // The version-valid-for number. 
            file.seek(SeekFrom::Start(92)).unwrap();
            file.write_all(&[0, 0, 0, 5]).unwrap();
            // SQLITE_VERSION_NUMBER
            file.seek(SeekFrom::Start(96)).unwrap();
            file.write_all(&[00, 64, 75, 144]).unwrap();

            file.seek(SeekFrom::Start(PAGE_SIZE.try_into().unwrap())).unwrap();
            file.write_all(&[0; PAGE_SIZE]).unwrap();
        }
        if file.metadata().unwrap().len() as usize % PAGE_SIZE != 0 {
            println!("database file corrupted");
            process::exit(1)
        }
    Ok(file)
}