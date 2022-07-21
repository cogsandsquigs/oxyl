pub mod error;
pub mod lexer;
pub mod types;

use lexer::*;

fn main() {
    let mut lexer = Lexer::new("let x = 0.0");
    println!("{:?}", lexer.lex());
}
