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
    case ADD, SUB, MUL, DIV:
        return []Type{NewIntType(), NewFloatType()}, nil
    default:
        return nil, fmt.Errorf("operator %s is not defined for type Int", op)
    }
} 

func (t *Int) ExecuteOperators(op Operator) (func(l, r *Value) (*Value, error), error) {
    switch op {
    case ADD:
        return intaddv, nil
    default:
        return func(l,r *Value)(*Value, error){return nil, nil}, fmt.Errorf("operator %s is not defined for type Int", op)
    }
}

func intaddv(l, r *Value) (*Value, error) {
    switch r.Type().(type) {
    case *Int:
        return NewValue(NewIntType(), l.v.(int) + r.v.(int)), nil
    default:
        return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
    }
}