package lexer

import "testing"

func TestLexer(t *testing.T) {
	defer func() {
		if r := recover(); r != nil {
			t.Errorf("panic: %v", r)
		}
	}()

	lexer := NewLexer("")
	go lexer.Lex()
}
