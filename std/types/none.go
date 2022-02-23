package types

import (
	"fmt"

	. "oxyl/std/operators"
	"oxyl/std/traits"
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

func (t *None) IsTraitDefined(trait traits.Trait) bool {
	return false
}

func (t *None) IsMethodDefined(method string) bool {
	return false
}

func (t *None) GetMethod(method string) *Method {
	return &Method{
    Name: "", 
    Func: func(in []*Value) ([]*Value, error) { return nil, fmt.Errorf("method %s is not defined", method) },
  }
}