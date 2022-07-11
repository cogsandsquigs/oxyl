package lexer

import (
	"testing"
)

func TestLexer(t *testing.T) {
	lexer := New(".+-*/()[]{}")
	tokens, err := lexer.Lex()

	if err != nil {
		t.Error(err)
	}

	if tokens == nil {
		t.Error("tokens is nil")
	}

	for _, token := range tokens {
		t.Logf("%d", token)
	}
}
