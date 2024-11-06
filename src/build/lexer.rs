pub enum Token {
    EndOfFile,
    Function,
    Print,
    StartScope,
    EndScope,
    OpenParamn,
    CloseParamn,
    String(String),
    Identifier(String),
    Literar,
}

pub fn lexer(input: String) -> Vec<Token> {
    println!("lexxing all over the place");

    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars();
    let mut iter = input.split(" ");

    // while let Some(ch) = chars.next() {
    //     match ch {
    //         '{' => tokens.push(Token::StartScope),
    //         '}' => tokens.push(Token::EndScope),
    //         '(' => tokens.push(Token::OpenParamn),
    //         ')' => tokens.push(Token::CloseParamn),
    //     }
    // }

    while let iter

    return tokens;
}
