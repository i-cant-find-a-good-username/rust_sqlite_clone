use super::tokenizer::{self, Token};

pub struct ObjectName(String);

pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub options: Vec<ColumnOptionDef>,
}

pub struct Selection {
    column: ColumnDef,
    value: DataType, // changing type
}

pub struct Assignment {}

// like not_null or auto_increment
pub struct ColumnOptionDef {
    pub name: Option<ObjectName>,
    pub option: ColumnOption,
}

pub enum DataType {
    Text(u32),    //lenght
    Integer(u32), //lenght
    Float(u32),   //lenght
    Null,
}

pub enum ObjectType {
    Table,
}

pub enum ColumnOption {
    Null,
    NotNull,
    Default(),
    Unique { is_primary: bool },
    Check(Selection),
}

pub enum Statement {
    Select {
        table_name: ObjectName,
        all: bool,
        columns: Vec<ObjectName>,
        table: bool,
    },
    Insert {
        into: bool,
        table_name: ObjectName,
        columns: Vec<ObjectName>,
        values: Vec<DataType>,
        table: bool,
    },
    Update {
        table: ObjectName,
        assignments: Vec<Assignment>,
        from: Option<ObjectName>,
        selection: Option<Selection>,
    },
    Delete {
        /// FROM
        table_name: ObjectName,
        /// WHERE
        selection: Option<Selection>,
    },
    CreateTable {
        name: ObjectName,
        columns: Vec<ColumnDef>,
    },
    Drop {
        object_type: ObjectName,
        names: Vec<ObjectName>,
    },
}

#[derive(Debug)]
pub struct Parser /*<'a>*/ {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser /*<'a>*/ {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }

    pub fn parse(query: String) -> Result<Vec<Statement>, tokenizer::TokenizerError> {
        println!("start tokenize {:?}", query);

        let mut new_tokenizer = tokenizer::Tokenizer::new(&query);
        let tokens = new_tokenizer.tokenize()?;
        println!("\x1b[93mtokens{:?}\x1b[0m", tokens);
        let mut parser = Parser::new(tokens);
        let mut statements: Vec<Statement> = Vec::new();

        parser.parse_stmts();





        //let statements = statements.parse_statement();

        //println!("\x1b[93mtokens{:?}\x1b[0m", tokens);
        //println!("\x1b[34mtokens{:?}\x1b[0m", new_tokenizer);

        // parsing

        Ok(statements)
    }
    pub fn parse_stmts(&mut self) {
        loop{



            match self.tokens[self.index]{
                Token::EOF => {
                    println!("end of file goodbye");
                    break;
                }
                Token::SemiColon => {
                    println!("end of statement push to array");
                }
                _ => {
                    println!("other");
                }
            }


            self.index += 1;
        }
    }


    //pub fn parse_statement() -> Result<Statement, tokenizer::TokenizerError> {
//
    //}

}

//select * from users;
//select * from users;
//select * from users;
//select * from users;
//select * from users;
//select * from users;
//select * from users;
