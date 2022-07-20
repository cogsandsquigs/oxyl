use crate::types::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    // Illegal/unexpected/unknown token
    Illegal(char),

    // End-of-file token
    EOF,

    // Comments
    Comment(String),

    // Keywords
    Let,
    Return,
    If,
    Else,
    Func,
    For,
    While,
    Type,

    // Identifiers and literals
    Ident(String),
    Literal(Type),

    // Operators
    Operator(Operator),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Operator {
    // Arithmetic/Numerical algebraic operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Logical/Boolean algebraic operators
    And,
    Or,
    Not,

    // Bitwise algebraic operators
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    BitShl,
    BitShr,

    // Comparison operators
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
}
