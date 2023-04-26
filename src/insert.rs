use std::mem::{ size_of_val };


const USERNAME_SIZE: u8 = 255;
const EMAIL_SIZE: u8 = 255;

const PAGE_SIZE: u16 = 4096;
const TABLE_MAX_PAGE: u16 = 100;
const ROW_SIZE: u16 = 10;
const ROWS_PER_PAGE: u16 = PAGE_SIZE / ROW_SIZE;
const MAX_ROWS_PER_PAGE: u16 = 4096;

struct Table {
    rows_number: u32,
    pages: u32,
}







struct User<'a> {
    id: u32,
    username: &'a str,
    email: &'a str,
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
    let collection = &parts.collect::<Vec<&str>>();
    println!("{:?}", collection);

    if collection.len() == 4{
        if !is_string_numeric(collection.get(1).unwrap()) {
            return (false, "invalid arguments types")
        }
        println!("********************************: size of {:?} is: {}", &*collection, size_of_val(&*collection));
        println!("********************************: size of {:?} is: {}", &collection, size_of_val(&collection));
        println!("********************************: size of {:?} is: {}", collection, size_of_val(collection));
        println!("********************************: size of {} is: {}", collection.get(0).unwrap(), size_of_val(collection.get(0).unwrap()));
        println!("********************************: size of {} is: {}", collection.get(1).unwrap(), size_of_val(collection.get(1).unwrap()));
        println!("********************************: size of {} is: {}", collection.get(2).unwrap(), size_of_val(collection.get(2).unwrap()));
        println!("********************************: size of {} is: {}", collection.get(3).unwrap(), size_of_val(collection.get(3).unwrap()));

        //if size_of_val(&*collection.get(2).unwrap()) > 255 || size_of_val(&*collection.get(3).unwrap()) > 255 {
        //    return (false, "overflow")
        //}
    }else{
        return (false, "invalid syntax")
    }





    let gg = User{
        id: collection.get(1).unwrap().parse::<u32>().unwrap(),
        username: collection.get(2).unwrap(),
        email: collection.get(3).unwrap(),
    };
    println!("********************************: {}", size_of_val(&gg.id));
    println!("********************************: {}", size_of_val(&*gg.username));
    println!("********************************: {}", size_of_val(&*gg.email));
    (true, "valid")
}




fn execute(query: &str) {
    
}



/*
insert 4000000000 hello iamjustarandomdudemanleabvealineanleabvealineiamjustarandomdudemanleabvealineiamjustarandomdudemanleabvealineiamjustarandomdudemanleabvealineiamjustarandomdudemanleabvealineiamjustarandomdudemanleabvealineiamjustarandomdudemanleabvealineiamjustarandomdudemanleabvealineiamjustarandomdudemanleabvealine;
*/


pub fn insert (query: String) {

    let result = prepare(&query);
    if result.0 {
        println!("query is correct and to bee exuceted ");
        execute(&query)
    }else{
        println!("error occured: {}", result.1);
    }


    println!("the insert query is: {}", query);
}




//
//
//
//
