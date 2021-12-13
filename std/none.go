package std

import (
	"fmt"
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

func (t *None) ExecuteOperators(op Operator) (func(l, r *Value) (*Value, error), error) {
	switch op {
	default:
		return func(l, r *Value) (*Value, error) { return nil, nil }, fmt.Errorf("operator %s is not defined for type None", op)
	}
}
