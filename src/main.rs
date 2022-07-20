pub mod lexer;
pub mod types;

use std::{collections::HashMap, vec};

use lexer::*;
use types::*;

fn main() {
    let lexer = Lexer::new("test");
    println!("{:?}", lexer.lex());

    let a = Type::Struct("test", HashMap::from_iter(vec![("t", Type::Int(0))]));
    let b = Type::Struct("test", HashMap::from_iter(vec![("test", Type::Int(1))]));

    println!("{:?}", Type::same(a, b));
}
