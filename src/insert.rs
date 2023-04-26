use std::mem::{ size_of_val, size_of };


const USERNAME_SIZE: u64 = 255;
const EMAIL_SIZE: u64 = 255;
const EMPTY_STRING: String = String::new();


const PAGE_SIZE: u64 = 4096;
const TABLE_MAX_PAGE: u64 = 100;
const ROW_SIZE: u64 = size_of::<u64>() as u64;
const ROWS_PER_PAGE: u64 = PAGE_SIZE / ROW_SIZE;
const MAX_ROWS_PER_PAGE: u64 = 4096;

struct Table {
    rows_number: u64,
    pages: [String; TABLE_MAX_PAGE as usize],
}







struct User {
    id: u64,
    username: String,
    email: String,
}





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

    if collection.len() == 4{
        if !is_string_numeric(collection.get(1).unwrap()) {
            return (false, "invalid arguments types")
        }
        // match to make sure provided in is u32
        match collection.get(1).unwrap().parse::<u64>(){
            Ok(..) => {
                if size_of_val(*collection.get(2).unwrap()) > 255 || size_of_val(*collection.get(3).unwrap()) > 255 {
                    return (false, "overflow")
                }
            },
            Err(..) => {
                return (false, "int or not int overflow")
            }
        }

    }else{
        return (false, "invalid syntax")
    }
    let gg = User{
        id: collection.get(1).unwrap().parse::<u64>().unwrap(),
        username: collection.get(2).unwrap().to_string(),
        email: collection.get(3).unwrap().to_string(),
    };
    println!("********************************: {}", size_of::<User>());
    println!("********************************: {}", size_of_val(&gg));
    println!("********************************: {}", size_of_val(&gg.id));
    println!("********************************: {}", size_of_val(&*gg.username));
    println!("********************************: {}", size_of_val(&*gg.email));
    (true, "valid")
}




fn execute<'a>(query: &'a str, table: &'a mut Table) -> &'a mut Table {
    println!("table rows  {} ", table.rows_number);
    table.rows_number+=1;
    table
}



/*
insert 40000000000000000 emanlemaabvealineiam justarandomdudemanleabvealinejustarandomdudemanleabvealinejustarandomdudemanleabvealinejustarandomdudemanleabvealinejustarandomdudemanleabvealinejustarandomdudemanleabvealinejustarandomdudemanleabvealinejustarandomdudemanleabvealineydzadazdazdazertyuiodd;
*/


pub fn insert (query: String) {

    
    let mut table: Table = Table{
        rows_number: 0,
        pages: [EMPTY_STRING; 100],
    };

    let result = prepare(&query);
    if result.0 {
        println!("query is correct and to bee exuceted ");
        let mm = execute(&query, &mut table);
    }else{
        println!("error occured: {}", result.1);
    }


}




//
//
//
//
