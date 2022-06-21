package lexer

import "oxyl/utils/stream"

type Lexer struct {
}

func NewLexer() *Lexer {
	return &Lexer{}
}

func Lex(input string) *stream.Stream[Token] {
	return stream.NewStream[Token]()
}
