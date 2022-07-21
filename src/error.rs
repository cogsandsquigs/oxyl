use std::fmt;

/**
 * The error type used by the compiler. Contains the line number,
 * starting character number, and the error message.
 * The different variants of the error type are used to differentiate between
 * an error at a certain stage of compilation.
*/
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Lexer(usize, usize, String),
    Parser(usize, usize, String),
    TypeChecker(usize, usize, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Lexer(line, col, msg) => {
                write!(f, "Lexing error at line {}, column {}: {}", line, col, msg)
            }
            Error::Parser(line, col, msg) => {
                write!(f, "Parsing error at line {}, column {}: {}", line, col, msg)
            }
            Error::TypeChecker(line, col, msg) => {
                write!(f, "Type error at line {}, column {}: {}", line, col, msg)
            }
        }
    }
}
