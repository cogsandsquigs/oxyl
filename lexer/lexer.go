package lexer

import (
	"io"
	"oxyl/error"
)

type Lexer struct {
	in io.Reader
}

func New(in io.Reader) *Lexer {
	lexer := &Lexer{
		in: in,
	}

	return lexer
}

func (l *Lexer) Lex() []error.Error {
	return []error.Error{error.New("Lexer is unimplemented", 0, 0, "none")}
}
