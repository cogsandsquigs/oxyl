package std

import (
	"fmt"
)

type Float struct{}

func (t *Float) Name() string {
	return "Float"
}

func (t *Float) DefinedOperators(op Operator) ([]Type, error) {
	switch op {
	case ADD, SUB, MUL, DIV:
		return []Type{&Float{}, &Int{}}, nil
	default:
		return nil, fmt.Errorf("operator %s is not defined for type Float", op)
	}
}

func (t *Float) ExecuteOperators(op Operator) func(l, r *Value) (*Value, error) {
	switch op {
	case ADD:
		return floataddv
	case SUB:
		return floatsubv
	case MUL:
		return floatmulv
	case DIV:
		return floatdivv
	default:
		return func(l, r *Value) (*Value, error) { return nil, nil }
	}
}

func floataddv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(&Float{}, l.v.(float64)+float64(r.v.(int))), nil
	case *Float:
		return NewValue(&Float{}, l.v.(float64)+r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func floatsubv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(&Float{}, l.v.(float64)-float64(r.v.(int))), nil
	case *Float:
		return NewValue(&Float{}, l.v.(float64)-r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func floatmulv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(&Float{}, l.v.(float64)*float64(r.v.(int))), nil
	case *Float:
		return NewValue(&Float{}, l.v.(float64)*r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func floatdivv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(&Float{}, l.v.(float64)/float64(r.v.(int))), nil
	case *Float:
		return NewValue(&Float{}, l.v.(float64)/r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}
