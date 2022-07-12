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

	if tokens == nil {
		t.Error("tokens is nil")
	}

	for _, token := range tokens {
		t.Logf("%d", token)
	}
}
