use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::{env, process, path::Path, fs::{OpenOptions}, io::{Write, SeekFrom, Seek}};

mod btree;
mod commands;
mod database;
mod parse;
mod rustyline_config;
mod table;
mod user;
mod constants;
mod utils;
use utils::{
    init_file::{file_init},
    int_byte_convert::{transform_u16_to_array_of_u8}
};

use commands::{
    meta_command::run_meta_command, process_command, sql_command::run_sql_command, CommandType,
};
use rustyline_config::{get_config, REPLHelper};

use crate::{commands::sql_command::SQLCommand, constants::PAGE_SIZE, utils::init_file};

fn main() -> rustyline::Result<()> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() == 1 {
        println!("expected database file name");
        process::exit(1)
    } else if args.len() > 2 {
        println!("unexpected arguments");
        process::exit(1)
    }


    if args[0] == "--help" || args[0] == "-h" {
        println!("help message");
        process::exit(1)
    }else if args[0] == "--version" || args[0] == "-v"{
        println!("0.1.0");
        process::exit(1)
    } 

    let mut database = if Path::new(&args[1]).exists() {
        // maybe match here
        let file = file_init(&args[1]).unwrap();
        database::database::Database::new(args[1].to_string(), file)
    }else{
        println!("invalid database file");
        process::exit(1)
    };






    let config = get_config();
    let helper = REPLHelper::default();
    let mut repl = Editor::with_config(config).unwrap();
    repl.set_helper(Some(helper));

    if repl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let prompt = format!("RSQL> ");
    repl.helper_mut().expect("No helper found").colored_prompt =
        format!("\x1b[1;32m{}\x1b[0m", prompt);



    loop {
        let readline = repl.readline(&prompt);
        match readline {
            Ok(command) => {
                let _ = repl.add_history_entry(command.as_str());
                match process_command(&command) {
                    CommandType::TypeSQL(cmd) => match cmd {
                        SQLCommand::Invalid(err) => println!("an error occured: {}", err),
                        _ => match run_sql_command(command, &mut database) {
                            Ok(result) => println!("{}", result),
                            Err(err) => println!("an error occured: \n{}", err),
                        },
                    },
                    CommandType::TypeMeta(cmd) => match run_meta_command(cmd) {
                        Ok(result) => println!("{}", result),
                        Err(err) => println!("an error occured: {}", err),
                    },
                };
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    let _ = repl.save_history("history.txt");

    Ok(())
}
