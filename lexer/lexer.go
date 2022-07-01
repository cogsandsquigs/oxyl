package lexer

import (
	"fmt"
	"io"

	"oxyl/err"
	"oxyl/token"
)

type Lexer struct {
	in       io.RuneReader // input for lexer
	line     int           // current line we are on
	col      int           // current character of line we are on
	pos      int           // current position in input
	ch       byte          // current character of input we are on
	hadError bool          // errors we have encountered
}

func New(in io.RuneReader) *Lexer {
	lexer := &Lexer{
		in:       in,
		line:     1,
		col:      1,
		pos:      0,
		ch:       0,
		hadError: false,
	}

	return lexer
}

func (l *Lexer) Lex() ([]token.Token, error) {
	tokens := []token.Token{}
	for {
		ch, err := l.current()

		if err == io.EOF {
			tokens = append(tokens, token.EOF)
			return tokens, nil
		} else if err != nil {
			return nil, err
		}

		tokens = append(tokens, token.UNKNOWN)

		fmt.Printf("%s", string(ch))
	}
}

// returns the current character we are at
func (l *Lexer) current() (rune, error) {
	ch, _, err := l.in.ReadRune()

	if err != nil {
		return 0, err
	}

	return ch, nil
}

func (l *Lexer) error(message string) error {
	return err.New(message, l.line, l.col)
}
