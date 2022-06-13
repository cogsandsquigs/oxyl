package main

type ASTNode interface{}

type EvalNode interface {
	ASTNode

	Eval() int
}

type Literal struct {
	v int
}

func NewLiteral(v int) Literal {
	return Literal{v}
}

func (n Literal) Eval() int {
	return n.v
}

type Add struct {
	l EvalNode
	r EvalNode
}

func NewAdd(l, r EvalNode) Add {
	return Add{l, r}
}

func (n Add) Eval() int {
	return n.l.Eval() + n.r.Eval()
}
