use super::tokenizer::{self, Token, KeyWord, Word, Whitespace};

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


pub struct Clause {
    column: String,
    operation: Token,
    value: String,
}
#[derive(Debug)]

// like not_null or auto_increment
pub struct ColumnOptionDef {
    pub name: Option<String>,
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
        table_name: String,
        all: bool,
        columns: Option<Vec<String>>,
        selection: Option<Vec<Clause>>,
    },
    Insert {
        table_name: String,
        all: bool,
        columns: Option<Vec<String>>,
        values: Vec<String>,
    },
    Update {
        table: String,
        Clauses: Vec<Clause>,
        selection: Option<Clause>,
    },
    Delete {
        table_name: String,
        selection: Option<Clause>,
    },
    CreateTable {
        name: String,
        columns: Vec<ColumnDef>,
    },
    Drop {
        object_type: String,
        names: Vec<String>,
    },
}
#[derive(Debug)]
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
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let mut statements: Vec<Statement> = Vec::new();

        loop{
            // not needeed parsers take care of query end
            //if parser.check_query_end() {
            //    println!("add statement here");
            //}
            if parser.check_file_end() {
                break;
            }
            
            let statement = parser.parse_statement()??;
            statements.push(statement);
            
            parser.index += 1;
        }

        Ok(statements)
    }

    pub fn parse_statement(&mut self) -> Result<Result<Statement, ParserError>, ParserError>  {
        println!("parse_statement start");
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
                message: "idk what the error".to_owned(),
                index: self.index
            }),
        }
    }











    pub fn select_statement(&mut self) -> Result<Statement, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        let mut all = false;
        self.next_token();

        match self.tokens[self.index] {
            Token::Mul => {
                all = true;
                self.next_token();
            }
            _ => {
                match self.tokens[self.index] {
                    Token::LParen => {
                        cols = self.get_words_in_paren()?;
                    }
                    _ => return Err(
                        ParserError {
                            message: String::from("columns are required"),
                            index: self.index,
                        }
                    )
                }
            }
        }

        // keyword
        match &self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::From, .. }) => {
                self.next_token();
            },
            _ => {
                return Err(
                    ParserError {
                        message: "no from".to_owned(),
                        index: self.index,
                    }
                )
            }
        }
    
        let table_name: Result<String, ParserError> = match &mut self.tokens[self.index]{
            Token::Word(Word {keyword: KeyWord::NotAKeyword, value: table_name}) => {
                Ok(table_name.to_string())
            },
            _ => {
                return Err(
                    ParserError {
                        message: "idk table name error".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
        self.next_token();
    
        

        let selection = self.get_clauses()?;


        // makes sure notthing is after last token
        match &mut self.tokens[self.index]{
            Token::SemiColon => {},
            _ => {
                return Err(
                    ParserError {
                        message: "invalid syntax after table name".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
    


        

        
        Ok(Statement::Select {
            table_name: table_name?,
            all: all,
            columns: if cols.len() != 0 { Some(cols) }else { None },
            selection: if selection.len() != 0 { Some(selection) }else { None },
        })
    }

























    pub fn insert_statement(&mut self) -> Result<Statement, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
        let mut all = false;
        self.next_token();
    
    
        match &self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::Into, .. }) => {
                self.next_token();
            },
            _ => {
                return Err(
                    ParserError {
                        message: "no into".to_owned(),
                        index: self.index,
                    }
                )
            }
        }
    
        let table_name: Result<String, ParserError> = match &mut self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::NotAKeyword, value: table_name}) => {
                Ok(table_name.to_string())
            },
            _ => {
                return Err(
                    ParserError {
                        message: "idk table name error".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
        self.next_token();
    
        match &mut self.tokens[self.index]{
            Token::LParen => {
                // get table cols insert into table_name(col1, col2, col3)
                cols = self.get_words_in_paren()?;
            },
            Token::Word(Word { keyword: KeyWord::Values, .. }) => {
                // if next token is Token::Values means the columns are unspecified so all of them
                all = true;
                if self.tokens[self.index] != Token::LParen {
                    return Err(
                        ParserError {
                            message: "values not found".to_owned(),
                            index: self.index,
                        }
                    )
                }
                values = self.get_words_in_paren()?;
            },
            _ => {
                return Err(
                    ParserError {
                        message: "invalid syntax after table name".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
    


        if all == false{
            match &mut self.tokens[self.index]{
                Token::Word(Word { keyword: KeyWord::Values, .. }) => {
                    self.next_token();
                    values = self.get_words_in_paren()?;
                },
                _ => {
                    return Err(
                        ParserError {
                            message: "invalid syntax after table name".to_owned(),
                            index: self.index,
                        }
                    )
                }
            };
        }


        // makes sure notthing is after last token
        self.finalize_query()?;    
    
        Ok(Statement::Insert {
            table_name: table_name?,
            all: all,
            columns: Some(cols),
            values: values,
        })
    
    }













































    pub fn update_statement(&mut self) -> Result<Statement, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
        let mut all = false;
        self.next_token();

        let table_name: Result<String, ParserError> = match &mut self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::NotAKeyword, value: table_name}) => {
                Ok(table_name.to_string())
            },
            _ => {
                return Err(
                    ParserError {
                        message: "idk table name error".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
        self.next_token();






        match &mut self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::And, .. }) => {
                self.next_token();
                loop{
                    println!("{:?}", &self.tokens[self.index]);
                    match &self.tokens[self.index]{
                        Token::Word(Word { value: col, .. }) => {
                            cols.push(col.to_string());
                            self.next_token();
                        },
                        Token::Comma => self.next_token(),
                        Token::RParen => {
                            self.next_token();
                            break;
                        },
                        _ => return Err(
                            ParserError {
                                message: String::from("error in columns"),
                                index: self.index,
                            }
                        )
                    }
                }
        
            },
            Token::Word(Word { keyword: KeyWord::Set, .. }) => {
                // get col name
                // get =
                // get value 
            },
            _ => {
                return Err(
                    ParserError {
                        message: "invalid syntax after table name".to_owned(),
                        index: self.index,
                    }
                )
            }
        };









        self.next_token();
        Ok(Statement::Update {
            table: table_name?,
            Clauses: todo!(),
            selection: todo!()
        })
    }












































    pub fn delete_statement(&mut self) -> Result<Statement, ParserError> {
        println!("delete_");
        self.next_token();
        Ok(Statement::Delete {
            table_name: todo!(),
            selection: todo!()
        })
    }
    pub fn create_statement(&mut self) -> Result<Statement, ParserError> {
        println!("create_");
        self.next_token();
        Ok(Statement::CreateTable {
            name: todo!(),
            columns: todo!()
        })
    }






















    pub fn finalize_query(&mut self) -> Result<(), ParserError> {
        match &mut self.tokens[self.index]{
            Token::SemiColon => return Ok(()),
            _ => {
                return Err(
                    ParserError {
                        message: "invalid syntax after table name".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
    }




    pub fn get_words_in_paren (&mut self) -> Result<Vec<String>, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        self.next_token();
        loop{
            match &self.tokens[self.index]{
                Token::Word(Word { value: col, .. }) => {
                    cols.push(col.to_string());
                    self.next_token();
                },
                Token::Comma => self.next_token(),
                Token::RParen => {
                    self.next_token();
                    break;
                },
                _ => return Err(
                    ParserError {
                        message: String::from("columns are required"),
                        index: self.index,
                    }
                )
            }
        }
        Ok(cols)
    }




    pub fn get_values_in_paren (&mut self) -> Result<Vec<String>, ParserError> {
        let mut values: Vec<String> = Vec::new();
        self.next_token();
        loop{
            match &self.tokens[self.index]{
                Token::SingleQuotedString(value) => {
                    values.push(value.to_string());
                    self.next_token();
                },
                Token::Comma => self.next_token(),
                Token::RParen => {
                    self.next_token();
                    break;
                },
                _ => return Err(
                    ParserError {
                        message: String::from("values are required"),
                        index: self.index,
                    }
                )
            }
        }
        Ok(values)
    }


    pub fn next_token (&mut self) {
        self.index += 1;
        loop{
            match &self.tokens[self.index]{
                Token::Comma => self.index += 1,
                Token::Whitespace(Whitespace::Space) => self.index += 1,
                Token::Whitespace(Whitespace::Newline) => self.index += 1,
                Token::Whitespace(Whitespace::Tab) => self.index += 1,
                //Token::Word(Word { keyword: KeyWord::Select, .. }) => self.index += 1,
                _ => break
            }
        }
    }


    pub fn confirm_keyword(keyword: KeyWord){

    }


    pub fn check_file_end(&mut self) -> bool {
        let result = match self.tokens[self.index]{
            Token::EOF => true,
            _ => false
        };
        result
    }








    pub fn get_clauses(&mut self) -> Result<Vec<Clause>, ParserError> {
        let mut selection: Vec<Clause> = Vec::new();
        match self.tokens[self.index]{
            Token::Word(Word {keyword: KeyWord::Where, ..}) => {
                self.next_token();
                loop{
                    let mut clause = Clause{
                        column: String::new(),
                        operation: Token::Eq, // doesnt matter, will change
                        value: String::new(),
                    };
                    match &self.tokens[self.index]{
                        Token::Word(Word { value: col, .. }) => {
                            clause.column = col.to_string();
                            self.next_token();
                        },
                        _ => {
                            return Err(         
                                ParserError {
                                    message: "column name wher error".to_owned(),
                                    index: self.index,         
                                }         
                            )           
                        }
                    }
                    match &self.tokens[self.index]{
                        Token::Eq | Token::Gt | Token::Lt | Token::GtEq | Token::LtEq | Token::Neq => {
                            if self.tokens[self.index] == Token::Eq {clause.operation = Token::Eq}else
                            if self.tokens[self.index] == Token::Gt {clause.operation = Token::Gt}else
                            if self.tokens[self.index] == Token::Lt {clause.operation = Token::Lt}else
                            if self.tokens[self.index] == Token::GtEq {clause.operation = Token::GtEq}else
                            if self.tokens[self.index] == Token::LtEq {clause.operation = Token::LtEq}else
                            if self.tokens[self.index] == Token::Neq {clause.operation = Token::Neq}
                            self.next_token();
                        },
                        _ => {
                            return Err(
                                ParserError {
                                    message: "no = sign".to_owned(),
                                    index: self.index,
                                }
                            )
                        }
                    }
                    match &self.tokens[self.index]{
                        Token::SingleQuotedString(value) => {
                            clause.value = value.to_string();
                            self.next_token();
                        },
                        _ => {
                            return Err(
                                ParserError {
                                    message: "values must be quoted".to_owned(),
                                    index: self.index,
                                }
                            )
                        }
                    }
                    selection.push(clause);
                    match &self.tokens[self.index]{
                        Token::Word(Word {keyword: KeyWord::And, .. }) => {
                            self.next_token();
                        },
                        _ => {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(selection)
    }




}






// insert into users values('fez', 'zefzef' ,'fzefze');
