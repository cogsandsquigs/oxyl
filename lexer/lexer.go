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
	in       []rune // input for lexer
	line     int    // current line we are on
	col      int    // current character of line we are on
	pos      int    // current position in input
	hadError bool   // errors we have encountered
}

func New(in string) *Lexer {
	lexer := &Lexer{
		in:       []rune(in),
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

		switch {
		case ch == ' ':
			l.next(1)
			continue
		case ch == '\t':
			l.next(1)
			continue
		case ch == '\n':
			l.line++
			lexemes = append(lexemes, token.NEWLINE)
		case ch == '\r':
			lexemes = append(lexemes, token.NEWLINE)
		case ch == '(':
			lexemes = append(lexemes, token.LEFT_PAREN)
		case ch == ')':
			lexemes = append(lexemes, token.RIGHT_PAREN)
		case ch == '{':
			lexemes = append(lexemes, token.LEFT_BRACE)
		case ch == '}':
			lexemes = append(lexemes, token.RIGHT_BRACE)
		case ch == '[':
			lexemes = append(lexemes, token.LEFT_BRACKET)
		case ch == ']':
			lexemes = append(lexemes, token.RIGHT_BRACKET)
		case ch == ',':
			lexemes = append(lexemes, token.COMMA)
		case ch == '.':
			lexemes = append(lexemes, token.DOT)
		case ch == '+':
			lexemes = append(lexemes, token.PLUS)
		case ch == '-':
			lexemes = append(lexemes, token.DASH)
		case ch == '*':
			lexemes = append(lexemes, token.STAR)
		case ch == '/':
			lexemes = append(lexemes, token.SLASH)
		case ch == '%':
			lexemes = append(lexemes, token.PERCENT)
		case ch == '^':
			lexemes = append(lexemes, token.CARET)
		case ch == '=':
			lexemes = append(lexemes, token.EQUAL)
		case ch == '<':
			lexemes = append(lexemes, token.LESS)
		case ch == '>':
			lexemes = append(lexemes, token.GREATER)
		case ch == '"':
			lexemes = append(lexemes, token.DOUBLE_QUOTE)
		case ch == '\'':
			lexemes = append(lexemes, token.SINGLE_QUOTE)
		case ch == '!':
			lexemes = append(lexemes, token.BANG)
		case l.match("&&"): // TODO: more efficient way to do this?
			lexemes = append(lexemes, token.AND)
		case ch == '&':
			lexemes = append(lexemes, token.AMP)
		case l.match("||"): // TODO: more efficient way to do this?
			lexemes = append(lexemes, token.OR)
		case ch == '|':
			lexemes = append(lexemes, token.PIPE)
		case l.match("=="): // TODO: more efficient way to do this?
			lexemes = append(lexemes, token.EQUAL_EQUAL)
		case l.match("!="): // TODO: more efficient way to do this?
			lexemes = append(lexemes, token.BANG_EQUAL)
		case l.match("<="): // TODO: more efficient way to do this?
			lexemes = append(lexemes, token.LESS_EQUAL)
		case l.match(">="): // TODO: more efficient way to do this?
			lexemes = append(lexemes, token.GREATER_EQUAL)
		// matches "let " so that it does not return a LET token for something
		// like "lettest", when it should be "let test"
		case l.match("let "):
			lexemes = append(lexemes, token.LET)
		case l.match("fun "):
			lexemes = append(lexemes, token.FUN)
		// parses identifiers, must come after keywords
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

				n, err := strconv.ParseFloat(number, 64)

				if err != nil {
					return nil, err
				}

				lexemes = append(lexemes, NewLiteralLexeme(n, token.FLOAT))
			} else {
				l.backup(1)

				n, err := strconv.ParseInt(number, 10, 64)

				if err != nil {
					return nil, err
				}

				lexemes = append(lexemes, NewLiteralLexeme(n, token.INT))
			}

		default:
			l.report("Unrecognized character %s", string(ch))
			lexemes = append(lexemes, token.UNKNOWN)
		}
		l.next(1)
	}
}

// Returns the current character we are at.
// If the end of the input is reached, `EOF` is returned.
func (l *Lexer) current() (rune, error) {
	if l.pos >= len(l.in) {
		return 0, io.EOF
	}

	return l.in[l.pos], nil
}

// Advances the lexer by one character, and returns the character we are at.
// For example, `l.next(1)` advances one place and returns the character 1 character ahead of the current character.
// If the end of the input is reached, `EOF` is returned.
func (l *Lexer) next(chars int) (rune, error) {
	l.pos += chars
	return l.current()
}

// Matches a list of characters to the current list of characters in the buffer.
// If the characters match, the lexer is advanced by the number of characters in the list and returns true.
// If the characters do not match, the lexer remains in place and returns false.
func (l *Lexer) match(chars string) bool {
	for i, ch := range chars {
		peek, err := l.peek(i)
		if ch != peek || err != nil {
			return false
		}
	}
	l.next(len(chars) - 1)
	return true
}

// backs up the lexer by one character
// and returns the character we are at
func (l *Lexer) backup(chars int) (rune, error) {
	l.pos -= chars
	return l.current()
}

// Returns a character peeked at, `chars` characters away from the current character.
// For example, `l.peek(1)` returns the character 1 character ahead of the current character.
// If the end of the input is reached, `EOF` is returned.
func (l *Lexer) peek(chars int) (rune, error) {
	ch, err := l.next(chars)

	if err != nil {
		return 0, err
	}

	l.backup(chars)
	return ch, nil
}

func (l *Lexer) report(message string, args ...any) {
	l.hadError = true
	fmt.Println(l.error(message, args...))
}

func (l *Lexer) error(message string, args ...any) error {
	return err.New(fmt.Sprintf(message, args...), l.line, l.col)
}
