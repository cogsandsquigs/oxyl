package lexer

import (
	"fmt"
	"io"

	"oxyl/err"
	"oxyl/token"
)

type Lexer struct {
	in       string // input for lexer
	line     int    // current line we are on
	col      int    // current character of line we are on
	pos      int    // current position in input
	hadError bool   // errors we have encountered
}

func New(in string) *Lexer {
	lexer := &Lexer{
		in:       in,
		line:     1,
		col:      1,
		pos:      0,
		hadError: false,
	}

	return lexer
}

func (l *Lexer) Lex() ([]Lexeme, error) {
	lexemes := []Lexeme{}
	for {
		ch, err := l.current()

		if err == io.EOF {
			lexemes = append(lexemes, token.EOF)
			return lexemes, nil
		} else if err != nil {
			return nil, err
		}

		switch ch {
		case ' ':
		case '\t':
			break
		case '\n':
			l.line++
			lexemes = append(lexemes, token.NEWLINE)
		default:
			lexemes = append(lexemes, token.UNKNOWN)
		}

		fmt.Printf("%s", string(ch))
		l.next(1)
	}
}

// returns the current character we are at
func (l *Lexer) current() (byte, error) {
	if l.pos >= len(l.in) {
		return 0, io.EOF
	}

	return l.in[l.pos], nil
}

// Advances the lexer by one character, and returns the character we are at.
// For example, `l.next(1)` advances one place and returns the character 1 character ahead of the current character.
// If the end of the input is reached, `EOF` is returned.
func (l *Lexer) next(chars int) (byte, error) {
	l.pos += chars
	return l.current()
}

// backs up the lexer by one character
// and returns the character we are at
func (l *Lexer) backup(chars int) (byte, error) {
	l.pos -= chars
	return l.current()
}

// Returns a character peeked at, `chars` characters away from the current character.
// For example, `l.peek(1)` returns the character 1 character ahead of the current character.
// If the end of the input is reached, `EOF` is returned.
func (l *Lexer) peek(chars int) (byte, error) {
	ch, err := l.next(chars)

	if err != nil {
		return 0, err
	}

	l.backup(chars)
	return ch, nil
}

func (l *Lexer) error(message string) error {
	return err.New(message, l.line, l.col)
}
