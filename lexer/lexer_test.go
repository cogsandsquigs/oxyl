package lexer

import (
	"testing"
)

func TestLexer(t *testing.T) {
	lexer := New(`
	fun main() {
		println("hello world")

		let a = 1
		let b = 2
		let c = a + b
		println(c)
	}
	`)
	tokens, err := lexer.Lex()

	if err != nil {
		t.Error(err)
	}

	if lexer.hadError {
		t.Error("had error")
	}

	if tokens == nil {
		t.Error("tokens is nil")
	}

	if len(tokens) != 41 {
		t.Errorf("expected 41 tokens, got %d", len(tokens))
	}

	for _, token := range tokens {
		t.Logf("%s", token.String())
	}
}
