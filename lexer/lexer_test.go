package lexer

import (
	"strings"
	"testing"
)

func TestLexer(t *testing.T) {
	in := strings.NewReader(".,+-*/")
	lexer := New(in)
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
