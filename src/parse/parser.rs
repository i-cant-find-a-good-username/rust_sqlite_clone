use crate::commands::sql_command;
use super::tokenizer;






pub fn parse(query: String) -> Result<Vec<tokenizer::Token>, tokenizer::TokenizerError> {
    println!("start tokenize {:?}", query);


    let mut new_tokenizer = tokenizer::Tokenizer::new(&query);
    let tokens = new_tokenizer.tokenize();

    
    tokens
    //let gg = tokenizer::tokenize();

}