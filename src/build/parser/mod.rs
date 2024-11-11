pub mod ast;

use crate::build::lexer::token::Token;

pub fn parser(tokens_vec: Vec<Token>) {
    println!("parsing with tokens: {:?}", tokens_vec);
   
    for token in tokens_vec.iter() {
        println!("{:?}", token);
    }
    syntax_analyzer(tokens_vec);
}


pub fn syntax_analyzer(tokens_vec: Vec<Token>) {
    println!("analyzing syntax with tokens");
    let program: ast::Program;
    let mut statements: Vec<ast::Statement> = vec![];
    let var_name = "meow";
    statements.push(ast::Statement::ReturnStmt(ast::Expr::String(var_name.to_owned())));
    program = ast::Program::Statements(statements);
    println!("{:?} {:?}", program, tokens_vec);
}
