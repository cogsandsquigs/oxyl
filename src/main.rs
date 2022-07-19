pub mod lexer;

use lexer::*;

fn main() {
    let lexer = Lexer::new("test");

    lexer.lex();
}
