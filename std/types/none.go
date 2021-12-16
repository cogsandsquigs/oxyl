package types

import (
	"fmt"

	. "github.com/ipratt-code/oxyl/std/operators"
)

type None struct{}

func (t *None) Name() string {
	return "None"
}

func (t *None) DefinedOperators(op Operator) ([]Type, error) {
	switch op {
	default:
		return nil, fmt.Errorf("operator %s is not defined for type None", op)
	}
}

func (t *None) ExecuteOperators(op Operator) func(l, r *Value) (*Value, error) {
	switch op {
	default:
		return func(l, r *Value) (*Value, error) { return nil, nil }
	}
}
