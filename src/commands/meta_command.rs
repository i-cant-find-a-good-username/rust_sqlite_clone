#[derive(Debug)]
pub enum MetaCommand {
    Tables,
    Exit,
    Open(String),
    Help,
    Unknown(String),
}

impl MetaCommand {
    pub fn new(command: String) -> MetaCommand {
        let mut args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned();
        args.remove(0).to_string();

        match cmd.as_ref() {
            ".tables" => MetaCommand::Tables,
            ".exit" => MetaCommand::Exit,
            ".help" => MetaCommand::Help,
            ".open" => MetaCommand::Open(args.get(0).unwrap().to_string()),
            _ => MetaCommand::Unknown(cmd),
        }
    }
}

pub fn run_meta_command(command: MetaCommand) -> Result<String, String> {
    match command {
        MetaCommand::Exit => {
            //repl.append_history("history").unwrap();
            std::process::exit(0);
        }
        MetaCommand::Tables => Ok(format!("here we get tables from dataabse")),
        MetaCommand::Open(file) => Ok(format!("open file: {} ", file)),
        MetaCommand::Help => Ok("the help message is here".to_string()),
        MetaCommand::Unknown(unknown_command) => {
            Err(format!("unknown command: {} ", unknown_command))
        }
    }
}
