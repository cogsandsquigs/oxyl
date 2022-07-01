package lexer

import (
	"go/token"
	"io"
	"oxyl/err"
)

type Lexer struct {
	in     io.Reader // input for lexer
	line   int       // current line we are on
	char   int       // current character of line we are on
	pos    int       // current position of input we are on
	fn     string    // name of file we are reading from
	errors []error   // errors we have encountered
}

func New(in io.Reader) *Lexer {
	lexer := &Lexer{
		in:     in,
		line:   1,
		char:   1,
		pos:    0,
		fn:     "none",
		errors: []error{},
	}

	return lexer
}

func (l *Lexer) Lex() ([]error, []token.Token) {
	return []error{err.New("Lexer is unimplemented", 0, 0, "none")}, nil
}

func (l *Lexer) error(message string) {
	l.errors = append(l.errors, err.New(message, l.line, l.char, l.fn))
}
