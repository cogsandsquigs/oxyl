package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/ipratt-code/oxyl/parser"
	"github.com/ipratt-code/oxyl/std/types"
)

func main() {
	/*
		i := &std.Int{}
		f := &std.Float{}
		v1 := std.NewValue(f, 1.2)
		fmt.Printf("value 1: %s, %v\n", v1.Type().Name(), v1.Value())
		v2 := std.NewValue(i, 2)
		fmt.Printf("value 2: %s, %v\n", v2.Type().Name(), v2.Value())
		v3, _ := f.ExecuteOperators(std.ADD)(v1, v2)
		fmt.Printf("adding value 1 and value 2: %s, %v\n", v3.Type().Name(), v3.Value())
	*/

	n := parser.NewLiteralNode(types.NewValue(&types.Int{}, 1))
	n.Print("")

	reader := bufio.NewReader(os.Stdin)
	fmt.Println("Welcome to the Oxyl REPL!")
	fmt.Println("Please keep in mind this programming language is still under development!")

	for {
		fmt.Print("~> ")
		text, _ := reader.ReadString('\n')
		// convert CRLF to LF
		text = strings.Replace(text, "\n", "", -1)

		// do stuff with the text
		fmt.Println(text)
	}

}
