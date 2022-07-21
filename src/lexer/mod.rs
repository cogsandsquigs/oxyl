pub mod token;

use crate::{error::Error, types::Type};
use token::*;

pub struct Lexer {
    input: String,
    position: usize,
    line: usize,        // the current line we are at
    character: usize,   // the current character we are at
    errors: Vec<Error>, // whether or not we have had an error
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        return Lexer {
            input: input.into(),
            position: 0,
            line: 1,
            character: 1,
            errors: vec![],
        };
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens = vec![];

        // lex over all the characters in the input
        loop {
            // get the next character
            let c = self.current();

            // update the character count
            self.character += 1;

            // match the current character
            match c {
                // If the character is a NULL, return EOF and break out of the loop.
                '\0' => break,

                // If the character is a newline, return it, and increment
                // the line count while resetting the character count.
                '\r' | '\n' => {
                    tokens.push(Token::Newline);
                    self.line += 1;
                    self.character = 1;
                }

                // If the character is a whitespace, ignore it.
                ' ' | '\t' => (),

                // Match against single-character tokens.
                '(' | ')' | '{' | '}' | '[' | ']' | ',' | '.' | ':' => {
                    tokens.push(Token::from_char(c).unwrap());
                }

                // Match against keywords.
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = String::new();
                    ident.push(c);

                    // Lex the rest of the identifier.
                    loop {
                        let c = self.peek(1);

                        if c.is_alphanumeric() || c == '_' {
                            ident.push(self.next(1));
                        } else {
                            break;
                        }
                    }

                    // check if the identifier is a keyword
                    if let Some(token) = Token::from_keyword(&ident) {
                        tokens.push(token);
                    } else {
                        tokens.push(Token::Identifier(ident));
                    }
                }

                // If the character is a comment (either single-line //
                // or multi-line /* */), return a comment token.
                '/' => {
                    if self.peek(1) == '/' || self.peek(1) == '*' {
                        tokens.push(self.lex_comment());
                    } else {
                        tokens.push(Token::Operator(Operator::Div));
                    }
                }

                // match against multi-character operators
                '=' | '!' | '<' | '>' | '&' | '|' => {
                    let op = Operator::from_chars(format!("{}{}", c, self.next(1)).as_str());

                    if op.is_some() {
                        tokens.push(Token::Operator(op.unwrap()));
                    } else if c == '=' {
                        tokens.push(Token::Assign);
                    } else {
                        tokens.push(Token::Operator(Operator::from_char(c).unwrap()));
                    }
                }

                // match against single-character operators
                '+' | '-' | '*' | '%' | '^' | '~' => {
                    tokens.push(Token::Operator(Operator::from_char(c).unwrap()));
                }

                // If the character is a number, return a literal token.
                '0'..='9' => {
                    tokens.push(self.lex_number());
                }

                // If the character is a '"', return a literal token.
                '"' => {
                    let mut literal = String::new();

                    // lex the rest of the literal.
                    loop {
                        let c = self.next(1);

                        if c == '"' {
                            break;
                        }

                        literal.push(c);
                    }

                    // push the literal token.
                    tokens.push(Token::Literal(Type::String(literal)));
                }

                // If the character is unknown, return illegal token.
                c => {
                    self.report(format!("Illegal character: {}", c));
                    tokens.push(Token::Illegal(c)); // MAYBE: get rid of this, reduce memory usage?
                }
            }

            self.next(1);
        }

        if self.errors.len() > 0 {
            return Err(self.errors.clone());
        }

        return Ok(tokens);
    }

    /**
     * Lexes a comment, both single- and multi-line. Returns
     * a comment token.
     */
    fn lex_comment(&mut self) -> Token {
        let mut comment = String::new();

        // If the comment is a single-line comment, we can just
        // lex until the end of the line.
        if self.peek(1) == '/' {
            self.next(2);
            loop {
                let c = self.current();

                if c == '\r' || c == '\n' || c == '\0' {
                    break;
                }

                comment.push(c);
                self.next(1);
            }
        } else {
            self.next(2);

            // If the comment is a multi-line comment, we need to
            // lex until we find the closing */.
            loop {
                let c = self.current();

                // If there is no closing */, we have an error.
                if c == '\0' {
                    self.report("Multi-line comment is unclosed.");
                    break;
                }

                if c == '*' && self.peek(1) == '/' {
                    // skip the closing */
                    self.next(1);
                    break;
                }

                comment.push(c);
                self.next(1);
            }
        }

        return Token::Comment(comment);
    }

    /**
     * Lexes a number, which is an Int or Float. Returns a literal token.
     */
    fn lex_number(&mut self) -> Token {
        let mut literal = String::new();

        // Lex the rest of the literal.
        loop {
            let c = self.current();

            if c.is_numeric() {
                literal.push(c);
                self.next(1);
            } else {
                break;
            }
        }

        let c = self.current();

        if c == '.' {
            literal.push(c);
            self.next(1);
        } else {
            self.prev(1);
        }

        loop {
            let c = self.current();

            if c.is_numeric() {
                literal.push(c);
                self.next(1);
            } else {
                self.prev(1);
                break;
            }
        }

        // If the literal is an integer, return an Int literal.
        if literal.contains('.') {
            return Token::Literal(Type::Float(literal.parse::<f64>().unwrap()));
        } else {
            return Token::Literal(Type::Int(literal.parse::<i64>().unwrap()));
        }
    }

    /**
     * Gets the current character in the input. If there are
     * no more characters, returns '\0', the NULL character.
     */
    fn current(&self) -> char {
        return self.input.chars().nth(self.position).unwrap_or('\0');
    }

    /**
     * Moves the position forward by n characters. Returns
     * the character that was moved to.
     */
    fn next(&mut self, n: usize) -> char {
        self.position += n;
        return self.current();
    }

    /**
     * Moves the position backward by n characters. Returns
     * the character that was moved to.
     */
    fn prev(&mut self, n: usize) -> char {
        self.position -= n;
        return self.current();
    }

    /**
     * Peeks the the nth next character in the input. If there are
     * no more characters, returns '\0', the NULL character.
     * For example, if n = 1, it will peek the next character.
     * If n = 0, it will peek the current character.s
     */
    fn peek(&self, n: usize) -> char {
        return self.input.chars().nth(self.position + n).unwrap_or('\0');
    }

    /**
     * Reports an error that happened during lexing.
     */
    fn report<S: Into<String>>(&mut self, msg: S) {
        let err = Error::Lexer(self.line, self.character, msg.into());
        println!("{}", err);
        self.errors.push(err.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_eof() {
        let input = "";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_lexer_comment() {
        let input = "// This is a comment.\n";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Comment(" This is a comment.".to_string()));
    }

    #[test]
    fn test_lexer_whitespace() {
        let input = " \t\n\r";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Newline);
        assert_eq!(tokens[1], Token::Newline);
    }

    #[test]
    fn test_lexer_identifier() {
        let input = "abc123";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Identifier("abc123".to_string()));
    }

    #[test]
    fn test_lexer_int() {
        let input = "123";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Literal(Type::Int(123)));
    }

    #[test]
    fn test_lexer_float() {
        let input = "123.456";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        println!("{:?}", tokens);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Literal(Type::Float(123.456)));
    }

    #[test]
    fn test_lexer_string() {
        let input = "\"Hello, world!\"";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens[0],
            Token::Literal(Type::String("Hello, world!".to_string()))
        );
    }

    #[test]
    fn test_lexer_keyword() {
        let input = "if";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::If);
    }
}
