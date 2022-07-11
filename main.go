package main

import (
	"fmt"
	"log"
	"oxyl/lexer"
)

func main() {
	lexer := lexer.New("test 1 2 3")

	toks, err := lexer.Lex()

	if err != nil {
		log.Panicln(err)
	}

	for _, tok := range toks {
		fmt.Println(tok.String())
	}
}
