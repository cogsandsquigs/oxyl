package main

import (
	"fmt"
)

func main() {
	expr := NewAdd(
		NewLiteral(1),
		NewAdd(
			NewLiteral(2),
			NewLiteral(3),
		),
	)

	fmt.Printf("%v\n", expr.Eval())
}
