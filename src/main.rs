use std::io::{self, BufRead, stdout, stdin, Write};

mod select;
mod insert;
mod update;
mod delete;
mod create;
mod drop;




fn main() -> io::Result<()> {

    loop{
        print_prompt();
        
        let stdin = stdin();
        let mut lines = stdin.lock().lines();
        

        let mut query: String = String::new();

        while let Some(line) = lines.next() {
            match line {
                Ok(line) => {
                    match line.chars().last() {
                        Some(last_char) => {
                            query.push_str(&line);
                            query.push_str(" ");

                            if last_char == ';' || &line[..1] == "." {
                                proccess(query.trim().to_string());
                                query = String::new();
                                print_prompt();
                            }
                        }
                        None => {
                            println!("line not have any chars");
                        }
                    }
                }
                Err(err) => {
                    println!("line not read");
                }
            }
        }

    }
    
}






fn print_prompt() {
    print!("->");
    stdout().flush().unwrap();
}

fn proccess(query: String){
    if &query[..1] == "." {
        match query.as_str(){
            ".exit" => std::process::exit(0),
            ".databases" => println!("databases"),
            ".tables" => println!("tables"),
            _ => println!("unknown command {:?} ", query),
        } 
        if &query == ".exit" {
            std::process::exit(0);
        }else{
            println!("unknown command {:?} ", query);
        }
    }else{
        let statement_type: &str = query.split(" ").next().unwrap();
        match statement_type {
            "select" => select::select(query),
            "insert" => insert::insert(query),
            "update" => update::update(query),
            "delete" => delete::delete(query),
            "create" => create::create(query),
            "drop"   => drop::drop(query),
            _ => println!("unknown command {:?} ", query),
        }
    }
}







