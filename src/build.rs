mod lexer;
mod parser;

use core::panic;
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

    loop {
        let token = p.read_token();
        match token {
            Token::EndOfFile => {
                break;
            }
            Token::Function => {
                let skibidi = decl_function(&mut p);
                println!("{:#?}", skibidi)
            }
            _ => {
                //    println!("meow");
            }
        }
        // println!("{:?}", token);
    }

    let par_duration = start.elapsed();
    println!("finished parsing in: {:?}!", par_duration);



}

fn decl_function(parser: &mut parser::Parser) -> parser::ast::Statement {
    println!("skibidi");

    let mut cur_token = parser.read_token();

    let fn_name = match cur_token {
        Token::Identifier(string) => string,
        _ => {
            panic!("Error function name is not valid!");
        }
    };

    cur_token = parser.read_token();

    match cur_token {
        Token::OpenParamn => {},
        _ => {
            panic!("function not valid")
        }
    }

    cur_token = parser.read_token();

    let mut params: Vec<parser::ast::Statement> = vec![];

    loop {
        if cur_token != Token::CloseParamn {
            match cur_token {
                Token::Identifier(string) => {
                    params.push(parser::ast::Statement::VariableDecl(string, None));
                    cur_token = parser.read_token();
                }
                _ => {
                    panic!("a param isnt valid");
                }
            }
        } else {
            cur_token = parser.read_token();
            break;
        }
    }


    let mut statements: Vec<Box<parser::ast::Statement>> = vec![];

    loop {
        if cur_token != Token::EndScope {
            match cur_token {
                Token::Let => {
                    let statement = decl_var(parser);
                    statements.push(Box::new(statement));
                    cur_token = parser.read_token();
                }
                _ => {
                    cur_token = parser.read_token();
                    println!("syntax error somewer?!");
                }
            }
        } else {
            break;
        }
    }

    return parser::ast::Statement::FunctionDecl(fn_name, params, statements);
}

// fn decl_statement(parser: parser::Parser) -> parser::ast::Statement {

// }
//
//

fn decl_var(parser: &mut parser::Parser) -> parser::ast::Statement {
    let mut token = parser.read_token();

    let var_name;
    let var_value: parser::ast::Expr;

    match token{
        Token::Identifier(string) => {
            var_name = string;
            token = parser.read_token();
        }
        _ => {
            panic!("not a valid variable name!");
        }
    };
   
    if token == Token::Equals {
        token = parser.read_token();    
        match token {
            Token::String(string) => {
                var_value = parser::ast::Expr::String(string);
            }
            Token::Int(int) => {
                var_value = parser::ast::Expr::Integer(int);
            }
            _ => {
                panic!("not a valid variable!");
            }
        }
        return parser::ast::Statement::VariableDecl(var_name, Some(var_value));
    } else {
        return parser::ast::Statement::VariableDecl(var_name, None);
    }
    

}
