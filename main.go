package main

import (
	"fmt"
	"log"
	"oxyl/lexer"
	"strings"
)

func main() {
	in := strings.NewReader("test 1 2 3")
	lexer := lexer.New(in)

	toks, err := lexer.Lex()

	if err != nil {
		log.Panicln(err)
	}

	for _, tok := range toks {
		fmt.Println(tok.String())
	}
}
