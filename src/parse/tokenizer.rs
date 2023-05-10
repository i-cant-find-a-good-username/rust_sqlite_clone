use std::{iter::Peekable, str::Chars, fmt};
#[derive(Debug)]

pub enum KeyWord {
    Select,
    Insert,
    Update,
    Delete,
    Create,
    
    Table,
    Database,
    
    Where,
    Set,
    DISTINCT,
    All,
    
    Avg,
    Sum,

    NotAKeyword
}
#[derive(Debug)]

pub enum Token {
    EOF,
    Word(KeyWord),
    //Number(String, bool),
    Char(char),
    SingleQuotedString(String),
    Comma,
    Whitespace(Whitespace),
    DoubleEq,
    Eq,
    Neq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    LParen,
    RParen,
    Period,
    SemiColon,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::EOF => f.write_str("EOF"),
            Token::Word(ref w) => write!(f, "{:?}", w),
            //Token::Number(ref n, l) => write!(f, "{}{long}", n, long = if *l { "L" } else { "" }),
            Token::Char(ref c) => write!(f, "{}", c),
            Token::SingleQuotedString(ref s) => write!(f, "'{}'", s),
            Token::Comma => f.write_str(","),
            Token::Whitespace(ws) => write!(f, "{}", ws),
            Token::DoubleEq => f.write_str("=="),
            Token::Eq => f.write_str("="),
            Token::Neq => f.write_str("<>"),
            Token::Lt => f.write_str("<"),
            Token::Gt => f.write_str(">"),
            Token::LtEq => f.write_str("<="),
            Token::GtEq => f.write_str(">="),
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Mul => f.write_str("*"),
            Token::Div => f.write_str("/"),
            Token::Mod => f.write_str("%"),
            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
            Token::Period => f.write_str("."),
            Token::SemiColon => f.write_str(";"),
            Token::LBracket => f.write_str("["),
            Token::RBracket => f.write_str("]"),
            Token::LBrace => f.write_str("{"),
            Token::RBrace => f.write_str("}"),
        }
    }
}



impl fmt::Display for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Whitespace::Space => f.write_str(" "),
            Whitespace::Newline => f.write_str("\n"),
            Whitespace::Tab => f.write_str("\t"),
            Whitespace::SingleLineComment { prefix, comment } => write!(f, "{}{}", prefix, comment),
            Whitespace::MultiLineComment(s) => write!(f, "/*{}*/", s),
        }
    }
}


impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.value)
    }
}
















#[derive(Debug)]
pub struct Word{
    pub value: String,
    pub keyword: KeyWord
}

pub struct TokenizerError {
    pub message: String,
    pub line: u64,
    pub col: u64,
}
#[derive(Debug)]

pub enum Whitespace {
    Space,
    Newline,
    Tab,
    SingleLineComment { comment: String, prefix: String },
    MultiLineComment(String),
}



pub struct Tokenizer<'a> {
    query: &'a str,
    line: u64,
    col: u64,
}


impl<'a> Tokenizer<'a> {
    
    pub fn new(query: &'a str) -> Self {
        Self {
            query,
            line: 1,
            col: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut peekable = self.query.chars().peekable();

        while let Some(token) = self.next_token(&mut peekable)? {
            peekable.next();
            println!("{:?}", token);
        }

        


        Ok(Vec::new())
    }

    fn consume_and_return(&self, chars: &mut Peekable<Chars<'_>>, t: Token) -> Result<Option<Token>, TokenizerError> {
        chars.next();
        Ok(Some(t))
    }
    fn tokenize_single_quote_string(&self, chars: &mut Peekable<Chars<'_>>) -> Result<Option<Token>, TokenizerError> {
        // keep iterating until you find '
        chars.next();
        let text = String::from("hello there");
        Ok(Some(Token::SingleQuotedString(String::from("hello there"))))

    }
    
    fn next_token (&self, chars: &mut Peekable<Chars<'_>>) -> Result<Option<Token>, TokenizerError>  {
        match chars.peek(){
            Some(char) => {
                println!("{}", char);
                match char{
                    ' ' => Ok(Some(Token::Whitespace(Whitespace::Space))),
                    '\t' => Ok(Some(Token::Whitespace(Whitespace::Tab))),
                    '\n' => Ok(Some(Token::Whitespace(Whitespace::Newline))),
                    '\r' => Ok(Some(Token::Whitespace(Whitespace::Newline))),
                    //'end of file' => Ok(Some(Token::Whitespace(Whitespace::Space))),
                    //'word' => Ok(Some(Token::Whitespace(Whitespace::Space))),
                    //'char' => Ok(Some(Token::Whitespace(Whitespace::Space))),
                    '\'' => self.tokenize_single_quote_string(chars),
                    ',' => self.consume_and_return(chars, Token::Comma),
                    //'==' => self.consume_and_return(chars, Token::Comma),
                    '=' => self.consume_and_return(chars, Token::Comma),
                    //'!=' => self.consume_and_return(chars, Token::Comma),
                    '<' => self.consume_and_return(chars, Token::Comma),
                    '>' => self.consume_and_return(chars, Token::Comma),
                    //'<=' => self.consume_and_return(chars, Token::Comma),
                    //'>=' => self.consume_and_return(chars, Token::Comma),
                    '+' => self.consume_and_return(chars, Token::Comma),
                    '-' => self.consume_and_return(chars, Token::Comma),
                    '*' => self.consume_and_return(chars, Token::Comma),
                    '/' => self.consume_and_return(chars, Token::Comma),
                    '%' => self.consume_and_return(chars, Token::Comma),
                    '(' => self.consume_and_return(chars, Token::Comma),
                    ')' => self.consume_and_return(chars, Token::Comma),
                    '.' => self.consume_and_return(chars, Token::Comma),
                    ';' => self.consume_and_return(chars, Token::Comma),
                    '[' => self.consume_and_return(chars, Token::Comma),
                    ']' => self.consume_and_return(chars, Token::Comma),
                    '{' => self.consume_and_return(chars, Token::Comma),
                    '}' => self.consume_and_return(chars, Token::Comma),
                    


                    _ => Ok(None),
                }
            }
            None => Ok(None)
        }
        
    }


}

