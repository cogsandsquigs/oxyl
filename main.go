package main

import (
	"fmt"
	"encoding/json"
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

	val, err := json.MarshalIndent(expr, "", "\t")
	if err != nil {
		fmt.Println(err)
	}
	fmt.Println(string(val))
}
