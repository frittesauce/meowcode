mod lexer;
mod parser;

use std::{fs, path::PathBuf};

pub fn build() {
    println!("building...");

    let content = fs::read_to_string(PathBuf::from("./src/main.kty"))
        .expect("Should have been able to read the file");

    println!("{}", content);
    let start = std::time::Instant::now();
    let mut l = lexer::Lexer::new(content.chars().collect());
    let duration = start.elapsed();
    let mut tokens: Vec<lexer::token::Token> = vec![];
    l.read_char();
    loop {
        let token = l.next_token();
        if token == lexer::token::Token::EndOfFile {
            break;
        } else {
            tokens.push(token);
        }
    }
    
    parser::parser(tokens);

    println!("Finished building in {:?}!", duration);
}
