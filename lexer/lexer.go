package lexer

import (
	"fmt"
	"io"
	"regexp"
	"strconv"

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

		switch ch := ch; {
		case ch == ' ':
			l.next(1)
			continue
		case ch == '\t':
			l.next(1)
			continue
		case ch == '\n':
			l.line++
			lexemes = append(lexemes, token.NEWLINE)
		case regexp.MustCompile("[a-zA-Z]").MatchString(string(ch)):
			identifier := ""
			for regexp.MustCompile("[a-zA-Z]").MatchString(string(ch)) {
				identifier += string(ch)
				ch, err = l.next(1)

				if err != nil && err != io.EOF {
					return nil, err
				}
			}

			l.backup(1)

			lexemes = append(lexemes, NewLiteralLexeme(identifier, token.IDENTIFIER))

		// parses ints and floats
		case regexp.MustCompile("[0-9]").MatchString(string(ch)):
			number := ""
			for regexp.MustCompile("[0-9]").MatchString(string(ch)) {
				number += string(ch)
				ch, err = l.next(1)

				if err != nil && err != io.EOF {
					return nil, err
				}
			}

			// this means floats can be parsed like 1. instead of 1.0
			if ch == '.' {
				number += string(ch)
				ch, err = l.next(1)

				if err != nil && err != io.EOF {
					return nil, err
				}

				for regexp.MustCompile("[0-9]").MatchString(string(ch)) {
					number += string(ch)
					ch, err = l.next(1)

					if err != nil && err != io.EOF {
						return nil, err
					}
				}

				l.backup(1)

				i, err := strconv.ParseFloat(number, 64)

				if err != nil {
					fmt.Println(err)
					return nil, err
				}

				lexemes = append(lexemes, NewLiteralLexeme(i, token.FLOAT))
			} else {
				l.backup(1)

				i, err := strconv.ParseInt(number, 10, 64)

				if err != nil {
					fmt.Println(err)
					return nil, err
				}

				lexemes = append(lexemes, NewLiteralLexeme(i, token.INT))
			}

		default:
			lexemes = append(lexemes, token.UNKNOWN)
		}
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
