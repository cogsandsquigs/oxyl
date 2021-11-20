package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
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
