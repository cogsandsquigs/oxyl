package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/ipratt-code/oxyl/std"
)

func main() {
	v1 := std.NewValue(std.Int.Name(), int64(1))
  v2 := std.NewValue(std.Float.Name(), float64(2))
  v3, _ := std.Int.ExecuteOp("+", v1, v2)
  fmt.Printf("Got value of %v with type of %s\n", v3.Value(), v3.Type())

	reader := bufio.NewReader(os.Stdin)
	fmt.Println("Welcome to the Oxyl REPL!")
	fmt.Println("Please keep in mind this programming language is still under development!")

	for {
		fmt.Print("~> ")
		text, _ := reader.ReadString('\n')
		// convert CRLF to LF
		text = strings.Replace(text, "\n", "", -1)

		fmt.Println("=> " + text)
	}

}
