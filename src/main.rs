pub mod lexer;
pub mod types;

use lexer::*;

fn main() {
    let lexer = Lexer::new("test");

    println!("{:?}", lexer.lex());
}
