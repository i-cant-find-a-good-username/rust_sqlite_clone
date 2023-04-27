use std::io::{self, BufRead, stdout, stdin, Write};



mod select;
mod insert;
mod update;
mod delete;
mod create;
mod drop;


mod table;
mod user;
use table::Table;








fn print_prompt() {
    print!("->");
    stdout().flush().unwrap();
}

fn proccess(query: String, table: &mut Table){
    if &query[..1] == "." {
        match query.as_str(){
            ".exit" => std::process::exit(0),
            ".databases" => println!("databases"),
            ".tables" => println!("tables"),
            _ => println!("unknown command {:?} ", query),
        } 
    }else{
        let statement_type: &str = query.split(" ").next().unwrap();
        match statement_type {
            "select" => select::select(query),
            "insert" => insert::insert(query, table),
            "update" => update::update(query),
            "delete" => delete::delete(query),
            "create" => create::create(query),
            "drop"   => drop::drop(query),
            _ => println!("unknown command {:?} ", query),
        }
    }
}





//
//fn main() -> io::Result<()> {
//    let mut table: Table = Table::new();
//
//
//    loop{
//        print_prompt();
//        
//        let stdin = stdin();
//        let mut lines = stdin.lock().lines();
//        
//
//        let mut query: String = String::new();
//
//        while let Some(line) = lines.next() {
//            match line {
//                Ok(line) => {
//                    match line.chars().last() {
//                        Some(last_char) => {
//                            query.push_str(&line);
//                            query.push_str(" ");
//
//                            if last_char == ';' || &line[..1] == "." {
//                                proccess(query.trim().to_string(), &mut table);
//                                query = String::new();
//                                print_prompt();
//                            }
//                        }
//                        None => {
//                            println!("line not have any chars");
//                        }
//                    }
//                }
//                Err(err) => {
//                    println!("line not read");
//                }
//            }
//        }
//
//    }
//    
//}
//
//







use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

fn main() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}
