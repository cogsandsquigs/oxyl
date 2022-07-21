use crate::types::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    // Illegal/unexpected/unknown token
    Illegal(char),

    // Newline token
    Newline,

    // Comments
    Comment(String),

    // Single-character tokens
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Colon,

    // Keywords
    Let,
    Return,
    If,
    Else,
    Fun,
    For,
    While,
    Type,

    // Assignment
    Assign,

    // Identifiers and literals
    Identifier(String),
    Literal(Type),

    // Operators
    Operator(Operator),
}

impl Token {
    pub fn from_char(c: char) -> Option<Token> {
        match c {
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            ':' => Some(Token::Colon),
            '=' => Some(Token::Assign),
            _ => None,
        }
    }

    pub fn from_keyword(ident: &str) -> Option<Token> {
        match ident {
            "let" => Some(Token::Let),
            "return" => Some(Token::Return),
            "if" => Some(Token::If),
            "else" => Some(Token::Else),
            "fun" => Some(Token::Fun),
            "for" => Some(Token::For),
            "while" => Some(Token::While),
            "type" => Some(Token::Type),
            _ => None,
        }
    }
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
    BitNot,
    BitXor,
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

impl Operator {
    pub fn from_char(c: char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Sub),
            '*' => Some(Operator::Mul),
            '/' => Some(Operator::Div),
            '%' => Some(Operator::Mod),
            '&' => Some(Operator::BitAnd),
            '|' => Some(Operator::BitOr),
            '~' => Some(Operator::BitNot),
            '^' => Some(Operator::BitXor),
            '<' => Some(Operator::Lt),
            '>' => Some(Operator::Gt),
            '!' => Some(Operator::Not),
            _ => None,
        }
    }

    pub fn from_chars(cs: &str) -> Option<Operator> {
        match cs {
            "==" => Some(Operator::Eq),
            "!=" => Some(Operator::Neq),
            "<" => Some(Operator::Lt),
            ">" => Some(Operator::Gt),
            "<=" => Some(Operator::Leq),
            ">=" => Some(Operator::Geq),
            _ => None,
        }
    }
}
