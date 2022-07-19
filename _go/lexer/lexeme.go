package lexer

import (
	"fmt"
	"oxyl/token"
)

type Lexeme interface {
	String() string     // returns the string representation of the lexeme
	Token() token.Token // returns the token it represents
}

type LiteralLexeme struct {
	value any
	token token.Token
}

func NewLiteralLexeme(value any, tokenT token.Token) LiteralLexeme {
	return LiteralLexeme{
		value: value,
		token: tokenT,
	}
}

func (l LiteralLexeme) String() string {
	return fmt.Sprintf("%s: %v", l.token.String(), l.value)
}

func (l LiteralLexeme) Token() token.Token {
	return l.token
}
