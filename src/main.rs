pub mod error;
pub mod lexer;
pub mod types;

use lexer::*;

fn main() {
    let mut lexer = Lexer::new(
        r#"
    fun main() {
        let a = 12.3
        let b = 34.5
        let c = a + b
        println(c)
    }
    "#,
    );

    println!("{:?}", lexer.lex());
}
