use std::{process::id, string};

pub mod token;

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_string(ch: char) -> bool {
    ch != '\"'
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input: input,
            position: 0,
            read_position: 0,
            ch: '0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_letter(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_string = |l: &mut Lexer| -> Vec<char> {
            l.read_char();
            let position = l.position;

            while l.position < l.input.len() && is_string(l.ch) {
                l.read_char();
            }

            let final_position = l.position;

            l.read_char();

            l.input[position..final_position].to_vec()
        };

        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_digit(l.ch) {
                l.read_char();
            }

            l.input[position..l.position].to_vec()
        };

        let tok: token::Token;
        self.skip_whitespace();
        match self.ch {
            '{' => {
                tok = token::Token::StartScope;
            }
            '}' => {
                tok = token::Token::EndScope;
            }
            '(' => {
                tok = token::Token::OpenParamn;
            }
            ')' => {
                tok = token::Token::CloseParamn;
            }
            '=' => {
                tok = token::Token::Assign;
            }
            ';' => {
                tok = token::Token::SemiColon;
            }
            '"' => {
                let string: String = read_string(self).into_iter().collect();

                return token::Token::String(string);
            }
            '0' => {
                tok = token::Token::EndOfFile;
            }
            _ => {
                if is_letter(self.ch) {
                    let ident: Vec<char> = read_identifier(self);
                    match token::get_keyword_token(&ident) {
                        Ok(keyword_token) => {
                            return keyword_token;
                        }
                        Err(_err) => {
                            return token::Token::Identifier(ident.iter().collect());
                        }
                    }
                } else if is_digit(self.ch) {
                    let ident: Vec<char> = read_number(self);
                    return token::Token::Int(ident.iter().collect());
                } else {
                    return token::Token::Illegal;
                }
            }
        }
        self.read_char();
        tok
    }
}
