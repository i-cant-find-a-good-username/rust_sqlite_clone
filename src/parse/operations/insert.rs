use std::mem::{size_of, size_of_val};

use crate::table::Table;
use crate::user::User;

fn is_string_numeric(str: &&str) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn prepare(query: &str) -> (bool, &str) {
    let parts = query.split(" ");
    let collection = parts.collect::<Vec<&str>>();
    println!("{:?}", collection);

    if collection.len() == 4 {
        if !is_string_numeric(collection.get(1).unwrap()) {
            return (false, "invalid arguments types");
        }
        // match to make sure provided in is u32
        match collection.get(1).unwrap().parse::<u64>() {
            Ok(..) => {
                if size_of_val(*collection.get(2).unwrap()) > 255
                    || size_of_val(*collection.get(3).unwrap()) > 255
                {
                    return (false, "overflow");
                }
            }
            Err(..) => return (false, "int or not int overflow"),
        }
    } else {
        return (false, "invalid syntax");
    }
    let gg = User {
        id: collection.get(1).unwrap().parse::<u64>().unwrap(),
        username: collection.get(2).unwrap().to_string(),
        email: collection.get(3).unwrap().to_string(),
    };
    println!("********************************: {}", size_of::<User>());
    println!("********************************: {}", size_of_val(&gg));
    println!("********************************: {}", size_of_val(&gg.id));
    println!(
        "********************************: {}",
        size_of_val(&*gg.username)
    );
    println!(
        "********************************: {}",
        size_of_val(&*gg.email)
    );
    (true, "valid")
}

fn execute(query: &str, table: &mut Table) {
    let parts = query.split(" ").collect::<Vec<&str>>();

    let new_user: User = User {
        id: parts.get(1).unwrap().parse::<u64>().unwrap(),
        username: parts.get(2).unwrap().to_string(),
        email: parts.get(3).unwrap().to_string(),
    };

    table.add_row();
    table.insert_row(new_user);
    println!("table rows  {} ", table.rows_number);
}

/*
insert 4000000 emanlemaabvealineiam zertyuiodd;
*/

pub fn insert(query: String, table: &mut Table) {
    let result = prepare(&query);
    if result.0 {
        println!("query is correct and to bee exuceted ");
        let mm = execute(&query, table);
    } else {
        println!("error occured: {}", result.1);
    }
}

//
//
//
//
