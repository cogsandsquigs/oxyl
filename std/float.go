package std

import (
    "fmt"
)

type Float struct{}

func NewFloatType() Type {
    return &Float{}
}

func (t *Float) Name() string {
    return "Float"
}

func (t *Float) DefinedOperators(op Operator) ([]Type, error) {
    switch op {
    case ADD, SUB, MUL, DIV:
        return []Type{NewIntType(), NewFloatType()}, nil
    default:
        return nil, fmt.Errorf("operator %s is not defined for type Float", op)
    }
} 

func (t *Float) ExecuteOperators(op Operator) (func(l, r Value) Value, error) {
    return nil, nil
}