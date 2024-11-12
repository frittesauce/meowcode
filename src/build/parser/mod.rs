pub mod ast;

use std::{iter::Peekable, vec::IntoIter};

use crate::{build::lexer::token::{self, Token}, new::new};

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(token_vec: Vec<Token>) -> Self {
        println!("Parsing code with tokens: \n {:#? \n}", token_vec);
        Self {
            tokens: token_vec.into_iter().peekable(),
        }
    }
   
    pub fn peek(&mut self) -> &Token{
        self.tokens.peek().unwrap()
    }

    pub fn read_token(&mut self) -> Token{
        self.tokens.next().unwrap()
    }
}
