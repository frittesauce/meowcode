mod lexer;
mod parser;

use core::panic;
use lexer::token::Token;
use parser::{ast::Statement, Parser};
use std::{fs, path::PathBuf};

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
    let mut cur_token = parser.read_token();

    let fn_name = match cur_token {
        Token::Identifier(string) => string,
        _ => {
            panic!("Error function name is not valid!");
        }
    };

    cur_token = parser.read_token();

    match cur_token {
        Token::OpenParamn => {}
        _ => {
            panic!("function missing params")
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
            parser.read_token();
            break;
        }
    }

    let mut statements: Vec<Box<parser::ast::Statement>> = vec![];
    cur_token = parser.read_token();

    loop {
        if cur_token != Token::EndScope {
            match &cur_token {
                Token::Let => {
                    let statement = decl_var(parser);
                    statements.push(Box::new(statement));
                    cur_token = parser.read_token();
                }
                Token::Identifier(string) => {
                    let peek = parser.peek();
                    let statement;

                    match peek {
                        Token::OpenParamn => {
                            statement = parser::ast::Statement::ExprStm(decl_call(
                                parser,
                                string.to_string(),
                            ));
                        }
                        Token::Equals => {
                            parser.read_token();
                            let expr = decl_expr(parser);
                            let name: String = string.to_string();
                            statement = parser::ast::Statement::AsigmentStmt(name, expr);
                        }
                        _ => {
                            panic!("variable or call invalid {:?}", string)
                        }
                    }
                    statements.push(Box::new(statement));
                    cur_token = parser.read_token();
                }
                Token::SemiColon => {
                    cur_token = parser.read_token();
                }
                Token::If => {
                    decl_if(parser);
                }
                _ => {
                    println!("syntax error somewer?! {:?}", cur_token);
                    cur_token = parser.read_token();
                }
            }
            if cur_token != Token::SemiColon {
                println!(
                    "stupid stupid supid forgot semicolon {:?} statements {:#?}",
                    cur_token, statements
                );
                cur_token = parser.read_token();
            } else {
                cur_token = parser.read_token();
            }
        } else {
            parser.read_token();
            break;
        }
    }

    return parser::ast::Statement::FunctionDecl(fn_name, params, statements);
}

fn decl_if(parser: &mut parser::Parser) -> parser::ast::Statement {
    let condition: Box<parser::ast::Expr>;
    let statements: Vec<parser::ast::Statement>;
    let else_statements: Option<Vec<parser::ast::Statement>>;

    return parser::ast::Statement::IfStmt(condition, statements, else_statements);
}

fn decl_call(parser: &mut parser::Parser, string: String) -> parser::ast::Expr {
    let name = string;

    let mut token = parser.read_token();
    let statement: parser::ast::Expr;

    let mut params: Vec<parser::ast::Expr> = vec![];
    loop {
        if token == Token::CloseParamn {
            break;
        } else {
            let expr = decl_expr(parser);
            params.push(expr);
            token = parser.read_token();
        }
    }
    statement = parser::ast::Expr::Call(name, params);

    return statement;
}

fn decl_expr(parser: &mut parser::Parser) -> parser::ast::Expr {
    let mut token = parser.read_token();
    let mut peek = parser.peek();
    let first_expr: parser::ast::Expr;
    let second_expr: parser::ast::Expr;
    let expr: parser::ast::Expr;

    println!("ignore {:?}", token);

    match token {
        Token::String(string) => {
            first_expr = parser::ast::Expr::String(string);
        }
        Token::Int(int) => {
            first_expr = parser::ast::Expr::Integer(int);
        }
        Token::Identifier(string) => match peek {
            Token::OpenParamn => {
                parser.read_token();
                first_expr = decl_call(parser, string);
            }
            _ => {
                first_expr = parser::ast::Expr::Identify(string);
            }
        },
        Token::SemiColon => {
            first_expr = parser::ast::Expr::String("woof".to_string());
            println!("test")
        }
        _ => {
            panic!("expressions are wrong somewhere {:?}, {:?}", token, peek)
        }
    }

    peek = parser.peek();
    println!("{:?}", peek);

    match peek {
        Token::Plus => {
            parser.read_token();
            second_expr = decl_expr(parser);
            expr = parser::ast::Expr::BinaryOp(
                Box::new(first_expr),
                parser::ast::BinaryOperator::Add,
                Box::new(second_expr),
            );
        }
        Token::Minus => {
            parser.read_token();
            second_expr = decl_expr(parser);
            expr = parser::ast::Expr::BinaryOp(
                Box::new(first_expr),
                parser::ast::BinaryOperator::Subtract,
                Box::new(second_expr),
            );
        }
        Token::Star => {
            parser.read_token();
            second_expr = decl_expr(parser);
            expr = parser::ast::Expr::BinaryOp(
                Box::new(first_expr),
                parser::ast::BinaryOperator::Multiply,
                Box::new(second_expr),
            );
        }
        Token::Slash => {
            parser.read_token();
            second_expr = decl_expr(parser);
            expr = parser::ast::Expr::BinaryOp(
                Box::new(first_expr),
                parser::ast::BinaryOperator::Divide,
                Box::new(second_expr),
            );
        }
        Token::Equals => {
            parser.read_token();
            parser.read_token();
            second_expr = decl_expr(parser);
            expr = parser::ast::Expr::BinaryOp(
                Box::new(first_expr),
                parser::ast::BinaryOperator::Equals,
                Box::new(second_expr),
            );
        }
        _ => {
            expr = first_expr;
        }
    }
    return expr;
}

fn decl_var(parser: &mut parser::Parser) -> parser::ast::Statement {
    let mut token = parser.read_token();

    let var_name;
    let var_value: parser::ast::Expr;

    match token {
        Token::Identifier(string) => {
            var_name = string;
            token = parser.read_token();
        }
        _ => {
            panic!("not a valid variable name!");
        }
    };

    if token == Token::Equals {
        var_value = decl_expr(parser);

        return parser::ast::Statement::VariableDecl(var_name, Some(var_value));
    } else {
        return parser::ast::Statement::VariableDecl(var_name, None);
    }
}
