mod rustyline_config;
mod database;
mod table;
mod user;







use rustyline_config::{get_config, REPLHelper};

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() -> rustyline::Result<()> {

    //let mut rl = DefaultEditor::new()?;
    let config = get_config();
    let helper = REPLHelper::default();
    let mut repl = Editor::with_config(config).unwrap();
    repl.set_helper(Some(helper));

    if repl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }


    let p = format!("RSQL> ");
    repl.helper_mut().expect("No helper found").colored_prompt = format!("\x1b[1;32m{}\x1b[0m", p);
    
    loop {
        let readline = repl.readline(&p);
        match readline {
            Ok(line) => {
                repl.add_history_entry(line.as_str());
                let gg = line.as_bytes();
                let kk = format!("{:x?}", gg);
                println!("{}", kk);
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


