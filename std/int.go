package std

import (
    "fmt"
)

type Int struct{}

func NewIntType() Type {
    return &Int{}
}

func (t *Int) Name() string {
    return "Int"
}

func (t *Int) DefinedOperators(op Operator) ([]Type, error) {
    switch op {
    case ADD:
        return []Type{NewIntType()}, nil
    default:
        return nil, fmt.Errorf("operator %s is not defined for type Int", op)
    }
} 

func (t *Int) ExecuteOperators(op Operator) (func(l, r Value) Value, error) {
    return nil, nil
}