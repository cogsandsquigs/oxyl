package lexer

import (
	"strings"
	"testing"
)

func TestLexer(t *testing.T) {
	in := strings.NewReader(".,+-*/")
	lexer := New(in)
	errs := lexer.Lex()
	if errs != nil {
		for _, err := range errs {
			t.Error(err)
		}
	}

}
