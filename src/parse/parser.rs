use super::tokenizer::{self, KeyWord, Token, Whitespace, Word};

#[derive(Debug)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub options: Option<Vec<ColumnOptions>>,
}



//pub struct Clause {
//    left: BinOp,
//    operation: Token,
//    right: BinOp,
//}
//struct BinOp{
//    left: String,
//    operation: Token,
//    right: String,
//}
//struct nested(BinOp);


#[derive(Debug)]
pub struct Clause {
    column: String,
    operation: Token,
    value: String,
}

#[derive(Debug)]
pub struct Allocation {
    column: String,
    value: String,
}

#[derive(Debug)]
pub enum DataType {
    Text,    //lenght
    Integer, //lenght
    Float,   //lenght
    Boolean,   //lenght
    Null,
}


#[derive(Debug)]

pub enum ObjectType {
    Table,
}

#[derive(Debug)]
pub enum ColumnOptions {
    AutoIncrement,
    PrimaryKey,
    NotNull,
    Default(String),
    Unique,
}

#[derive(Debug)]
pub enum Statement {
    Select {
        table_name: String,
        all: bool,
        columns: Option<Vec<String>>,
        clauses: Option<Vec<Clause>>,
    },
    Insert {
        table_name: String,
        all: bool,
        columns: Option<Vec<String>>,
        values: Vec<String>,
    },
    Update {
        table_name: String,
        allocations: Vec<Allocation>,
        clauses: Vec<Clause>,
    },
    Delete {
        table_name: String,
        selection: Vec<Clause>,
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
        let tokens = match new_tokenizer.tokenize(){
            Ok(value) => value,
            Err(err) => return Err(ParserError {
                message: err.message,
                index: err.col as usize,
            }),
        };

        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let mut statements: Vec<Statement> = Vec::new();

        loop {
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

    pub fn parse_statement(&mut self) -> Result<Result<Statement, ParserError>, ParserError> {
        // this block is returned
        match &self.tokens[self.index] {
            Token::Word(key_word) => match key_word.keyword {
                KeyWord::Select => Ok(self.select_statement()),
                KeyWord::Insert => Ok(self.insert_statement()),
                KeyWord::Update => Ok(self.update_statement()),
                KeyWord::Delete => Ok(self.delete_statement()),
                KeyWord::Create => Ok(self.create_statement()),
                KeyWord::Drop => Ok(self.drop_statement()),
                _ => return Err(self.return_error("no keywords"))
            },
            _ => return Err(self.return_error("idk the error"))
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
            _ => match self.tokens[self.index] {
                Token::LParen => {
                    cols = self.get_words_in_paren()?;
                }
                _ => return Err(self.return_error("columns are req"))
            },
        }

        self.confirm_keyword(KeyWord::From)?;

        let table_name = self.get_object_name()??;

        let clauses = self.get_clauses()?;

        // makes sure notthing is after last token
        self.finalize_query()?;

        Ok(Statement::Select {
            table_name,
            all,
            columns: if cols.len() != 0 { Some(cols) } else { None },
            clauses: if clauses.len() != 0 {
                Some(clauses)
            } else {
                None
            },
        })
    }

    pub fn insert_statement(&mut self) -> Result<Statement, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
        let mut all = false;
        self.next_token();

        self.confirm_keyword(KeyWord::Into)?;

        let table_name = self.get_object_name()??;

        match &mut self.tokens[self.index] {
            Token::LParen => {
                // get table cols insert into table_name(col1, col2, col3)
                cols = self.get_words_in_paren()?;
            }
            Token::Word(Word {
                keyword: KeyWord::Values,
                ..
            }) => {
                // if next token is Token::Values means the columns are unspecified so all of them
                all = true;
                self.next_token();
                if self.tokens[self.index] != Token::LParen {
                    return Err(self.return_error("values not found"));
                }
                values = self.get_values_in_paren()?;
            }
            _ => return Err(self.return_error("invalid syntax after table name"))
        };

        if all == false {
            match &mut self.tokens[self.index] {
                Token::Word(Word {
                    keyword: KeyWord::Values,
                    ..
                }) => {
                    self.next_token();
                    values = self.get_values_in_paren()?;
                }
                _ => return Err(self.return_error("invalid syntax after table name"))
            };
        }

        // makes sure notthing is after last token
        self.finalize_query()?;

        Ok(Statement::Insert {
            table_name,
            all,
            columns: if cols.len() != 0 { Some(cols) } else { None },
            values,
        })
    }

    pub fn update_statement(&mut self) -> Result<Statement, ParserError> {
        let mut allocations: Vec<Allocation> = Vec::new();
        self.next_token();

        let table_name = self.get_object_name()??;

        match self.tokens[self.index] {
            Token::Word(Word {
                keyword: KeyWord::Set,
                ..
            }) => {
                self.next_token();
                println!("{}", &self.tokens[self.index]);

                loop {
                    let mut allocation = Allocation {
                        column: String::new(),
                        value: String::new(),
                    };
                    match &self.tokens[self.index] {
                        Token::Word(Word { value: col, .. }) => {
                            allocation.column = col.to_string();
                            self.next_token();
                        }
                        _ => return Err(self.return_error("column name wher error"))
                    }
                    match &self.tokens[self.index] {
                        Token::Eq => {
                            self.next_token();
                        }
                        _ => return Err(self.return_error("no = sign"))

                    }
                    match &self.tokens[self.index] {
                        Token::SingleQuotedString(value) => {
                            allocation.value = value.to_string();
                            self.next_token();
                        }
                        _ => return Err(self.return_error("string values must be qouted"))
                    }
                    allocations.push(allocation);
                    match &self.tokens[self.index] {
                        Token::Word(Word {
                            keyword: KeyWord::And,
                            ..
                        }) => {
                            self.next_token();
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }

        let clauses = self.get_clauses()?;
        // makes sure notthing is after last token
        self.finalize_query()?;

        Ok(Statement::Update {
            table_name: table_name,
            clauses,
            allocations,
        })
    }

    pub fn delete_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();

        self.confirm_keyword(KeyWord::From)?;
        let table_name = self.get_object_name()??;
        let clauses = self.get_clauses()?;
        self.finalize_query()?;

        if clauses.len() == 0 {
            return Err(self.return_error("conditions required"))
        }

        Ok(Statement::Delete {
            table_name,
            selection: clauses,
        })
    }

    pub fn drop_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();
        let mut drop: Vec<String> = Vec::new();

        loop {
            match &mut self.tokens[self.index] {
                Token::SemiColon => break,
                Token::Word(Word { value, .. }) => {
                    drop.push(value.to_string());
                    self.next_token();
                }
                _ => return Err(self.return_error("invalid syntax after table name"))
            };
        }
        if drop.len() == 0 {
            return Err(self.return_error("nothing selected"))
        }
        self.finalize_query()?;

        Ok(Statement::Drop {
            object_type: String::from("table"),
            names: drop,
        })
    }



























    pub fn create_statement(&mut self) -> Result<Statement, ParserError> {
        /* 
            create table users (
                id integer primary_key,
                username text not_null,
                email text not_null,
            );
        */

//
//pub struct ColumnDef {
//    pub name: String,
//    pub data_type: DataType,
//    pub options: Vec<ColumnOptions>,
//}
//
//pub struct Selection {
//    column: ColumnDef,
//    value: DataType, // changing type
//}
//
//pub enum DataType {
//    Text(u32),    //lenght
//    Integer(u32), //lenght
//    Float(u32),   //lenght
//    Null,
//}
//
//#[derive(Debug)]
////pub enum ColumnOptions {
//    NotNull,
//    Default(),
//    Unique { is_primary: bool },
//    Check(Selection),
//}



        self.next_token();
        self.confirm_keyword(KeyWord::Table)?;
        // table name
        let name = self.get_object_name()??;
        self.next_token();
        


        let mut columns: Vec<ColumnDef> = Vec::new();


        loop{
            let mut column = ColumnDef{ name: name.to_string(), data_type: DataType::Null, options: None };
            // get col name
            match &self.tokens[self.index] {
                Token::Word(Word {
                    value: col_name,
                    ..
                }) => column.name = col_name.to_string(),
                _ => return Err(self.return_error("col name is required"))
            }
            self.next_token();
            // get datatype
            match &self.tokens[self.index] {
                Token::Word(Word { keyword: KeyWord::Integer, .. }) => column.data_type = DataType::Integer,
                Token::Word(Word { keyword: KeyWord::Float,   .. }) => column.data_type = DataType::Float,
                Token::Word(Word { keyword: KeyWord::Boolean, .. }) => column.data_type = DataType::Boolean,
                Token::Word(Word { keyword: KeyWord::Text,    .. }) => column.data_type = DataType::Text,
                Token::Word(Word { keyword: KeyWord::Null,    .. }) => column.data_type = DataType::Null,
                _ => return Err(self.return_error("col type req"))
            }
            self.next_token();
            loop{
                let mut column_options: Vec<ColumnOptions> = Vec::new();
                println!("{:?}",&self.tokens[self.index]  );
                match &self.tokens[self.index] {
                    Token::Word(Word { keyword: KeyWord::PrimaryKey,    .. }) => column_options.push(ColumnOptions::PrimaryKey),
                    Token::Word(Word { keyword: KeyWord::NotNull,       .. }) => column_options.push(ColumnOptions::NotNull),
                    Token::Word(Word { keyword: KeyWord::AutoIncrement, .. }) => column_options.push(ColumnOptions::AutoIncrement),
                    Token::Word(Word { keyword: KeyWord::Default,       .. }) => {
                        match &self.tokens[self.index] {
                            Token::LParen => {},
                            _ => return Err(self.return_error("no default value"))
                        }
                        self.next_token();
                        match column.data_type {
                            DataType::Integer => {
                                match &self.tokens[self.index]{
                                    Token::Number(s, b) => column_options.push(ColumnOptions::Default(s.to_string())),
                                    _ => return Err(self.return_error("wrong datatype for default"))
                                }
                            },
                            DataType::Float => {
                                match &self.tokens[self.index]{
                                    Token::Number(s, b) => column_options.push(ColumnOptions::Default(s.to_string())),
                                    _ => return Err(self.return_error("wrong datatype for default"))
                                }
                            },
                            DataType::Boolean => {
                                match &self.tokens[self.index]{
                                    Token::Word(Word { keyword: KeyWord::True, .. }) => column_options.push(ColumnOptions::Default("true".to_string())),
                                    Token::Word(Word { keyword: KeyWord::False, .. }) => column_options.push(ColumnOptions::Default("false".to_string())),
                                    _ => return Err(self.return_error("wrong datatype for default"))
                                }
                            },
                            DataType::Text => {
                                match &self.tokens[self.index]{
                                    Token::Word(Word { value, .. }) => column_options.push(ColumnOptions::Default(value.to_string())),
                                    _ => return Err(self.return_error("wrong datatype for default"))
                                }
                            },
                            DataType::Null => {
                                match &self.tokens[self.index]{
                                    Token::Number(s, b) => column_options.push(ColumnOptions::Default("".to_string())),
                                    _ => return Err(self.return_error("wrong datatype for default"))
                                }
                            },
                        };
                        self.next_token();
                        match &self.tokens[self.index] {
                            Token::RParen => {},
                            _ => return Err(self.return_error("default value not closed"))
                        }
                        self.next_token();
                    },
                    _ => {
                        println!("{:?}",&self.tokens[self.index]  );
                        return Err(self.return_error("invalid column option"))
                    }
                }
                self.next_token();
                println!("inside inside loop");
                if &self.tokens[self.index] == &Token::Comma {
                    self.next_token();
                    break;
                } 
            }
            println!("inside loop");
            if &self.tokens[self.index] == &Token::Comma {
                break;
            }
        }





        println!("create_");
        self.next_token();
        Ok(Statement::CreateTable {
            name,
            columns
        })
    }





    pub fn check_value_type(&self, data_type: DataType) -> bool{

        true
    }




































    // object is table_name col
    pub fn get_object_name(&mut self) -> Result<Result<String, ParserError>, ParserError> {
        let table_name: Result<String, ParserError> = match &mut self.tokens[self.index] {
            Token::Word(Word {
                value: table_name,
                ..
            }) => Ok(table_name.to_string()),
            _ => return Err(self.return_error("idk table name error"))
        };
        self.next_token();
        Ok(table_name)
    }

    pub fn finalize_query(&mut self) -> Result<(), ParserError> {
        match &mut self.tokens[self.index] {
            Token::SemiColon => return Ok(()),
            _ => return Err(self.return_error("invalid syntax at the end"))
        };
    }

    pub fn get_words_in_paren(&mut self) -> Result<Vec<String>, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        self.next_token();
        loop {
            match &self.tokens[self.index] {
                Token::Word(Word { value: col, .. }) => {
                    cols.push(col.to_string());
                    self.next_token();
                }
                Token::Comma => self.next_token(),
                Token::RParen => {
                    self.next_token();
                    break;
                }
                _ => return Err(self.return_error("cols req"))
            }
        }
        Ok(cols)
    }

    pub fn get_values_in_paren(&mut self) -> Result<Vec<String>, ParserError> {
        let mut values: Vec<String> = Vec::new();
        self.next_token();
        loop {
            match &self.tokens[self.index] {
                Token::SingleQuotedString(value) => {
                    values.push(value.to_string());
                    self.next_token();
                }
                Token::Comma => self.next_token(),
                Token::RParen => {
                    self.next_token();
                    break;
                }
                _ => return Err(self.return_error("values req"))
            }
        }
        Ok(values)
    }

    pub fn next_token(&mut self) {
        self.index += 1;
        loop {
            match &self.tokens[self.index] {
                Token::Comma => self.index += 1,
                Token::Whitespace(Whitespace::Space) => self.index += 1,
                Token::Whitespace(Whitespace::Newline) => self.index += 1,
                Token::Whitespace(Whitespace::Tab) => self.index += 1,
                //Token::Word(Word { keyword: KeyWord::Select, .. }) => self.index += 1,
                _ => break,
            }
        }
    }

    pub fn confirm_keyword(&mut self, expected_keyword: KeyWord) -> Result<(), ParserError> {
        if let Token::Word(Word { keyword, .. }) = &self.tokens[self.index] {
            if keyword == &expected_keyword {
                self.next_token();
                return Ok(());
            } else {
                return Err(self.return_error("unexpected keyword"))
            }
        }
        return Err(self.return_error("no keyword"))
    }

    pub fn check_file_end(&mut self) -> bool {
        let result = match self.tokens[self.index] {
            Token::EOF => true,
            _ => false,
        };
        result
    }

    // get where condition=value
    //pub fn get_clauses(&mut self) -> Result<Vec<Clause>, ParserError> {
    //    let mut selection: Vec<Clause> = Vec::new();
    //    match self.tokens[self.index] {
    //        Token::Word(Word {
    //            keyword: KeyWord::Where,
    //            ..
    //        }) => {
    //            self.next_token();
    //            loop {
    //                let mut clause = Clause {
    //                    column: String::new(),
    //                    operation: Token::Eq, // doesnt matter, will change
    //                    value: String::new(),
    //                };
    //                match &self.tokens[self.index] {
    //                    Token::Word(Word { value: col, .. }) => {
    //                        clause.column = col.to_string();
    //                        self.next_token();
    //                    }
    //                    _ => {
    //                        return Err(ParserError {
    //                            message: "column name wher error".to_string(),
    //                            index: self.index,
    //                        })
    //                    }
    //                }
    //                match &self.tokens[self.index] {
    //                    Token::Eq
    //                    | Token::Gt
    //                    | Token::Lt
    //                    | Token::GtEq
    //                    | Token::LtEq
    //                    | Token::Neq => {
    //                        if self.tokens[self.index] == Token::Eq {
    //                            clause.operation = Token::Eq
    //                        } else if self.tokens[self.index] == Token::Gt {
    //                            clause.operation = Token::Gt
    //                        } else if self.tokens[self.index] == Token::Lt {
    //                            clause.operation = Token::Lt
    //                        } else if self.tokens[self.index] == Token::GtEq {
    //                            clause.operation = Token::GtEq
    //                        } else if self.tokens[self.index] == Token::LtEq {
    //                            clause.operation = Token::LtEq
    //                        } else if self.tokens[self.index] == Token::Neq {
    //                            clause.operation = Token::Neq
    //                        }
    //                        self.next_token();
    //                    }
    //                    _ => {
    //                        return Err(ParserError {
    //                            message: "no = sign".to_string(),
    //                            index: self.index,
    //                        })
    //                    }
    //                }
    //                match &self.tokens[self.index] {
    //                    Token::SingleQuotedString(value) => {
    //                        clause.value = value.to_string();
    //                        self.next_token();
    //                    }
    //                    _ => {
    //                        return Err(ParserError {
    //                            message: "values must be quoted".to_string(),
    //                            index: self.index,
    //                        })
    //                    }
    //                }
    //                selection.push(clause);
    //                match &self.tokens[self.index] {
    //                    Token::Word(Word {
    //                        keyword: KeyWord::And | KeyWord::Or,
    //                        ..
    //                    }) => {
    //                        self.next_token();
    //                    }
    //                    _ => {
    //                        break;
    //                    }
    //                }
    //            }
    //        }
    //        _ => {}
    //    }
    //    Ok(selection)
    //}





    pub fn get_clauses(&mut self) -> Result<Vec<Clause>, ParserError> {
        let mut selection: Vec<Clause> = Vec::new();
        match self.tokens[self.index] {
            Token::Word(Word {
                keyword: KeyWord::Where,
                ..
            }) => {
                self.next_token();
                loop {
                    let mut clause = Clause {
                        column: String::new(),
                        operation: Token::Eq, // doesnt matter, will change
                        value: String::new(),
                    };
                    match &self.tokens[self.index] {
                        Token::Word(Word { value: col, .. }) => {
                            clause.column = col.to_string();
                            self.next_token();
                        }
                        _ => return Err(self.return_error("col name where error"))
                    }
                    match &self.tokens[self.index] {
                        Token::Eq
                        | Token::Gt
                        | Token::Lt
                        | Token::GtEq
                        | Token::LtEq
                        | Token::Neq => {
                            if self.tokens[self.index] == Token::Eq {
                                clause.operation = Token::Eq
                            } else if self.tokens[self.index] == Token::Gt {
                                clause.operation = Token::Gt
                            } else if self.tokens[self.index] == Token::Lt {
                                clause.operation = Token::Lt
                            } else if self.tokens[self.index] == Token::GtEq {
                                clause.operation = Token::GtEq
                            } else if self.tokens[self.index] == Token::LtEq {
                                clause.operation = Token::LtEq
                            } else if self.tokens[self.index] == Token::Neq {
                                clause.operation = Token::Neq
                            }
                            self.next_token();
                        }
                        _ => return Err(self.return_error("no = sign"))
                    }
                    match &self.tokens[self.index] {
                        Token::SingleQuotedString(value) => {
                            clause.value = value.to_string();
                            self.next_token();
                        }
                        // numbers too
                        _ => return Err(self.return_error("values must be qouted"))
                    }
                    selection.push(clause);
                    match &self.tokens[self.index] {
                        Token::Word(Word {
                            keyword: KeyWord::And | KeyWord::Or,
                            ..
                        }) => {
                            self.next_token();
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            _ => {/*in case there is no where the execution continues*/}
        }
        Ok(selection)
    }






    pub fn return_error(&mut self, message: &str) -> ParserError {
        ParserError {
            message: message.to_string(),
            index: self.index,
        }
    }






}

// insert into users values('fez', 'zefzef' ,'fzefze');


