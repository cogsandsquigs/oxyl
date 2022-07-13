package expression

import (
	"oxyl/types"
)

type Literal[T types.Type] struct {
	typ T
}

func NewLiteral[T types.Type](val T) *Literal[T] {
	return &Literal[T]{val}
}

func (l Literal[T]) Type() types.Type {
	return l.typ
}
