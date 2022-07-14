package main

import (
	"fmt"
	"log"
	"oxyl/lexer"
	"oxyl/types"
)

func main() {
	lexer := lexer.NewLexer(`
	fun main() {
		println("hello world")

		let a = 1
		let b = 2
		let c = a + b
		println(c)
	}
	`)

	toks, err := lexer.Lex()

	if err != nil {
		log.Panicln(err)
	}

	for _, tok := range toks {
		fmt.Println(tok.String())
	}

	x := types.NewInt(1)

	fmt.Println(x.Format())
	x = types.NewInt(2)

	fmt.Println(x.Format())
}
