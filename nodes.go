package main

type ASTNode interface{}

type EvalNode interface {
	ASTNode

	Eval() int
}

type Literal struct {
	V int
}

func NewLiteral(v int) Literal {
	return Literal{v}
}

func (n Literal) Eval() int {
	return n.V
}

type Add struct {
	L EvalNode
	R EvalNode
}

func NewAdd(l, r EvalNode) Add {
	return Add{l, r}
}

func (n Add) Eval() int {
	return n.L.Eval() + n.R.Eval()
}
