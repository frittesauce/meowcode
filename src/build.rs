mod lexer;
use lexer::lexer;

use std::{fs, path::PathBuf};

pub fn build() {
    println!("building...");

    let content = fs::read_to_string(PathBuf::from("./src/main.kty"))
        .expect("Should have been able to read the file");

    println!("{}", content);
    lexer(content);
}
