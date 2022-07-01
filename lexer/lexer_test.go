package lexer

import (
	"strings"
	"testing"
)

func TestLexer(t *testing.T) {
	in := strings.NewReader(".,+-*/")
	lexer := New(in)
	errs, tokens := lexer.Lex()

	if len(errs) > 0 {
		for _, err := range errs {
			t.Error(err)
		}
	}

	if tokens == nil {
		t.Error("tokens is nil")
	}

	for _, token := range tokens {
		t.Logf("%s", token)
	}
}
