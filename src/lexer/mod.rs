pub struct Lexer {
    input: String,
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        return Lexer {
            input: input.into(),
        };
    }

    pub fn lex(self) -> Vec<u8> {
        unimplemented!();
    }
}
