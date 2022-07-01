package lexer

import (
	"io"

	"oxyl/err"
	"oxyl/token"
)

type Lexer struct {
	in       io.Reader // input for lexer
	line     int       // current line we are on
	char     int       // current character of line we are on
	pos      int       // current position of input we are on
	hadError bool      // errors we have encountered
}

func New(in io.Reader) *Lexer {
	lexer := &Lexer{
		in:       in,
		line:     1,
		char:     1,
		pos:      0,
		hadError: false,
	}

	return lexer
}

func (l *Lexer) Lex() ([]token.Token, error) {
	return nil, l.error("not implemented")
}

func (l *Lexer) error(message string) error {
	return err.New(message, l.line, l.char)
}
