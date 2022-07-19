package expression

import (
	"oxyl/token"
)

type Binary struct {
	lhs Expression
	rhs Expression
	op  token.Operator
}

func NewBinary(lhs, rhs Expression, op token.Operator) *Binary {
	return &Binary{lhs, rhs, op}
}

func (b *Binary) Lhs() Expression {
	return b.lhs
}

func (b *Binary) Rhs() Expression {
	return b.rhs
}

func (b *Binary) Op() token.Operator {
	return b.op
}
