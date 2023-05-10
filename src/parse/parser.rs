use crate::commands::sql_command;
use super::tokenizer;






pub fn parse(query: String) -> String {
    println!("start tokenize {:?}", query);


    let mut new_tokenizer = tokenizer::Tokenizer::new(&query);
    let tokens = new_tokenizer.tokenize();

    
    String::from("heello")
    //let gg = tokenizer::tokenize();

}