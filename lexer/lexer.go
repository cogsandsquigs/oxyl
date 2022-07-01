package lexer

import (
	"fmt"
	"io"
	"log"

	"oxyl/err"
	"oxyl/token"
)

type Lexer struct {
	in       io.Reader // input for lexer
	line     int       // current line we are on
	char     int       // current character of line we are on
	pos      int       // current position of input we are on
	fn       string    // name of file we are reading from
	hadError bool      // errors we have encountered
}

func New(in io.Reader) *Lexer {
	lexer := &Lexer{
		in:       in,
		line:     1,
		char:     1,
		pos:      0,
		fn:       "none",
		hadError: false,
	}

	return lexer
}

func (l *Lexer) Lex() ([]token.Token, error) {
	return nil, fmt.Errorf("unimplemented")
}

func (l *Lexer) error(message string) {
	log.Println(err.New(message, l.line, l.char, l.fn))
}
