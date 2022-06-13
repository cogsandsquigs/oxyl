package main

import "fmt"

type ASTNode interface {}

type EvalNode interface {
	ASTNode

	Eval() int
}

type Add struct {
	l EvalNode
	r EvalNode
}

func NewAdd(l, r EvalNode) Add {
	return Add{ l, r }
}

func (n Add) Eval() int {
	return n.l.Eval() + n.r.Eval()
}

type Literal struct {
	v int
}

func NewLiteral(v int) Literal {
	return Literal{ v }
}

func (n Literal) Eval() int {
	return n.v
}

func main() {
	fmt.Printf(
		"%v\n",
		NewAdd(
			NewLiteral(1),
			NewAdd(
				NewLiteral(2),
				NewLiteral(3),
			),
		).Eval(),
	)
}