package main

import (
	"fmt"
	"log"
	"oxyl/lexer"
)

func main() {
	lexer := lexer.New("test\n1 2.0 3.1")

	toks, err := lexer.Lex()

	if err != nil {
		log.Panicln(err)
	}

	for _, tok := range toks {
		fmt.Println(tok.String())
	}
}
