mod lexer;
mod parser;

use std::{fs, path::PathBuf};
use lexer::token::Token;

pub fn build() {

    println!("building...");

    let content = fs::read_to_string(PathBuf::from("./src/main.kty"))
        .expect("Should have been able to read the file");

    println!("{}", content);
    let mut start = std::time::Instant::now();
    let mut l = lexer::Lexer::new(content.chars().collect());
    let mut tokens: Vec<Token> = vec![];
    l.read_char();
    loop {
        let token = l.next_token();
        if token == Token::EndOfFile {
            tokens.push(token);
            break;
        } else {
            tokens.push(token);
        }
    }
    let lex_duration = start.elapsed();
    println!("Finished lexing in {:?}!", lex_duration);
    start = std::time::Instant::now();

    let mut p = parser::Parser::new(tokens);
    
    p.read_token();
    loop {
        let token = p.read_token();
        match token {
            Token::EndOfFile => {
                println!("hallo wereld!");
                break;
            }
            Token::Function =>{
                let function_name = p.read_token();
                let function_decl: parser::ast::Decleration;

                if function_name == Token::String() {
                    function_decl = parser::ast::Decleration::FunctionDecl(function_name, , ())
                }
            }
            _ => {
                println!("meow");
            }
        }
        println!("{:?}", token);
    }

    let par_duration = start.elapsed();
    println!("finished parsing in: {:?}!", par_duration)
}
