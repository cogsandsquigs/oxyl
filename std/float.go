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
		return []Type{NewFloatType(), NewFloatType()}, nil
	default:
		return nil, fmt.Errorf("operator %s is not defined for type Float", op)
	}
}

func (t *Float) ExecuteOperators(op Operator) (func(l, r *Value) (*Value, error), error) {
	switch op {
	case ADD:
		return floataddv, nil
	case SUB:
		return floatsubv, nil
	case MUL:
		return floatmulv, nil
	case DIV:
		return floatdivv, nil
	default:
		return func(l, r *Value) (*Value, error) { return nil, nil }, fmt.Errorf("operator %s is not defined for type Int", op)
	}
}

func floataddv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(NewFloatType(), l.v.(float64)+float64(r.v.(int))), nil
	case *Float:
		return NewValue(NewFloatType(), l.v.(float64)+r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func floatsubv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(NewFloatType(), l.v.(float64)-float64(r.v.(int))), nil
	case *Float:
		return NewValue(NewFloatType(), l.v.(float64)-r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func floatmulv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(NewFloatType(), l.v.(float64)*float64(r.v.(int))), nil
	case *Float:
		return NewValue(NewFloatType(), l.v.(float64)*r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func floatdivv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(NewFloatType(), l.v.(float64)/float64(r.v.(int))), nil
	case *Float:
		return NewValue(NewFloatType(), l.v.(float64)/r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}