mod lexer;

use std::{fs, path::PathBuf};

pub fn build() {
    println!("building...");

    let content = fs::read_to_string(PathBuf::from("./src/main.kty"))
        .expect("Should have been able to read the file");

    println!("{}", content);

    let mut l = lexer::Lexer::new(content.chars().collect());
    l.read_char();
    loop {
        let token = l.next_token();
        if token == lexer::token::Token::EndOfFile {
            break;
        } else {
            println!("{:?}", token);
        }
    }
    println!("{} {} {}", char::from(l.ch), l.position, l.read_position);
}
