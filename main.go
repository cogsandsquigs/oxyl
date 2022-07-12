package main

import (
	"fmt"
	"log"
	"oxyl/lexer"
)

func main() {
	lexer := lexer.New(`
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
}
