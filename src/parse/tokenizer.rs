
pub enum Token {
    EOF,
    //Word(Word),
    Number(String, bool),
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
    Colon,
    DoubleColon,
    SemiColon,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Placeholder(String),
}

pub struct TokenizerError {
    pub message: String,
    pub line: u64,
    pub col: u64,
}

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
        println!("{:?}", peekable);
        Ok(Vec::new())
    }

}

