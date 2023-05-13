use super::tokenizer::{self, Token, KeyWord, Word, Whitespace};
#[derive(Debug)]

pub struct ObjectName(String);
#[derive(Debug)]

pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub options: Vec<ColumnOptionDef>,
}
#[derive(Debug)]

pub struct Selection {
    column: ColumnDef,
    value: DataType, // changing type
}
#[derive(Debug)]

pub struct Assignment {}
#[derive(Debug)]

// like not_null or auto_increment
pub struct ColumnOptionDef {
    pub name: Option<ObjectName>,
    pub option: ColumnOption,
}
#[derive(Debug)]

pub enum DataType {
    Text(u32),    //lenght
    Integer(u32), //lenght
    Float(u32),   //lenght
    Null,
}
#[derive(Debug)]

pub enum ObjectType {
    Table,
}
#[derive(Debug)]

pub enum ColumnOption {
    Null,
    NotNull,
    Default(),
    Unique { is_primary: bool },
    Check(Selection),
}

#[derive(Debug)]
pub enum Statement {
    Select {
        table_name: ObjectName,
        all: bool,
        columns: Option<Vec<ObjectName>>,
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

pub struct ParserError {
    pub message: String,
    index: usize,
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

    pub fn parse(query: String) -> Result<Vec<Statement>, ParserError> {
        let mut new_tokenizer = tokenizer::Tokenizer::new(&query);
        // might need error handling
        let tokens = new_tokenizer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let mut statements: Vec<Statement> = Vec::new();


        loop{
            if parser.check_query_end() {
                println!("add statement here");
            }
            if parser.check_file_end() {
                break;
            }
            
            let statement = parser.parse_statement()??;
            statements.push(statement);
            
            //println!("hello there my dude{:?}", statements);

            parser.index += 1;
        }
        //println!("hello there my dude{:?}", statements);

        Ok(statements)
    }



    pub fn check_query_end(&mut self) -> bool {
        let result = match self.tokens[self.index]{
            Token::SemiColon => true,
            _ => false
        };
        result
    }


    pub fn check_file_end(&mut self) -> bool {
        let result = match self.tokens[self.index]{
            Token::EOF => true,
            _ => false
        };
        result
    }

    pub fn parse_statement(&mut self) -> Result<Result<Statement, ParserError>, ParserError>  {
        match &self.tokens[self.index]{
            Token::Word(key_word) => {
                match key_word.keyword{
                    KeyWord::Select => Ok(self.select_statement()),
                    KeyWord::Insert => Ok(self.insert_statement()),
                    KeyWord::Update => Ok(self.update_statement()),
                    KeyWord::Delete => Ok(self.delete_statement()),
                    KeyWord::Create => Ok(self.create_statement()),
                    _ => Err(ParserError{
                        message: "no keyword".to_owned(),
                        index: self.index
                    }),
                }
            }
            _ => Err(ParserError{
                message: "no keyword".to_owned(),
                index: self.index
            }),
        }
    }


    pub fn select_statement(&mut self) -> Result<Statement, ParserError> {
        // selection type * or cols
        // from 
        // table name
        // ;
        let mut cols: Vec<String> = Vec::new();
        let mut all = false;
        self.trim();

        // if first token == ; self.index += 1;

        if self.tokens[self.index] == Token::Mul {
            println!("********************");
            all = true;
            self.index += 1;
            self.trim();
        }else {
            loop{
                match &self.tokens[self.index]{
                    Token::Word(Word { keyword: KeyWord::NotAKeyword, value: col }) => {
                        cols.push(col.to_string());
                        self.index += 1;                        
                    },
                    Token::Comma => self.index += 1,
                    Token::Whitespace(Whitespace::Space) => self.index += 1,
                    Token::Whitespace(Whitespace::Newline) => self.index += 1,
                    Token::Whitespace(Whitespace::Tab) => self.index += 4,
                    Token::Word(Word { keyword: KeyWord::From, .. }) => break,
                    err => panic!("not allowed {} ", err)// errr ,
                }
            }
            self.index += 1;
            self.trim();
        }

        println!("==================================================>{:?}",cols);
        println!("333333{}",&self.tokens[self.index] );
        match &self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::From, .. }) => {
                println!("fromfromfromfromfromfromfromfromfromfromfromfromfromfromfromfromfromfromfromfromfromfromfrom");
                self.index += 1;
                self.trim();
            },
            _ => println!("err no from")
        }

        match &self.tokens[self.index]{
            Token::Word(Word {keyword: KeyWord::NotAKeyword, value: table_name}) => {
                println!("from table name here  '''''''''''''''{}''''''  ", table_name);
                self.index += 1;                        
            },
            _ => println!("error in table name idk fr")
        }



        // add 1 beause it stops on this query ;
        Ok(Statement::Select {
            table_name: ObjectName(String::from("hello")),
            all: true,
            columns: Some(Vec::new())
        })
    }













    pub fn insert_statement(&self) -> Result<Statement, ParserError> {
        Ok(Statement::Select { table_name: ObjectName(String::from("hello")), all: true, columns: Some(Vec::new())})
    }
    pub fn update_statement(&self) -> Result<Statement, ParserError> {
        Ok(Statement::Select { table_name: ObjectName(String::from("hello")), all: true, columns: Some(Vec::new())})
    }
    pub fn delete_statement(&self) -> Result<Statement, ParserError> {
        Ok(Statement::Select { table_name: ObjectName(String::from("hello")), all: true, columns: Some(Vec::new())})
    }
    pub fn create_statement(&self) -> Result<Statement, ParserError> {
        Ok(Statement::Select { table_name: ObjectName(String::from("hello")), all: true, columns: Some(Vec::new())})
    }





    pub fn trim (&mut self) {
        loop{
            match &self.tokens[self.index]{
                Token::Comma => self.index += 1,
                Token::Whitespace(Whitespace::Space) => self.index += 1,
                Token::Whitespace(Whitespace::Newline) => self.index += 1,
                Token::Whitespace(Whitespace::Tab) => self.index += 4,
                Token::Word(Word { keyword: KeyWord::Select, .. }) => self.index += 1,
                _ => break
            }
        }
    }


}









//select * from users;
//select * from users;
//select * from users;
//select * from users;
//select * from users;
//select * from users;
//select * from users;
