#[derive(PartialEq, Debug)]
pub enum Token {
    EndOfFile,
    Illegal,
    Function,
    StartScope,
    EndScope,
    OpenParamn,
    CloseParamn,
    String(String),
    Int(String),
    Identifier(String),
    SemiColon,
    Equals,
    Plus,
    Minus,
    Star,
    Slash,
    Let,
    Bang,
    True,
    False,
    If,
    Else,
    Return,
}

pub fn get_keyword_token(ident: &Vec<char>) -> Result<Token, String> {
    let identifier: String = ident.into_iter().collect();
    match &identifier[..] {
        "fn" => Ok(Token::Function),
        "let" => Ok(Token::Let),
        "true" => Ok(Token::True),
        "false" => Ok(Token::False),
        "if" => Ok(Token::If),
        "else" => Ok(Token::Else),
        "return" => Ok(Token::Return),
        _ => Err(String::from("Not a keyword")),
    }
}
