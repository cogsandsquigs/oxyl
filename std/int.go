package std

import (
	"fmt"
)

type Int struct{}

func (t *Int) Name() string {
	return "Int"
}

func (t *Int) DefinedOperators(op Operator) ([]Type, error) {
	switch op {
	case ADD, SUB, MUL, DIV:
		return []Type{&Int{}, &Float{}}, nil
	default:
		return nil, fmt.Errorf("operator %s is not defined for type Int", op)
	}
}

func (t *Int) ExecuteOperators(op Operator) (func(l, r *Value) (*Value, error), error) {
	switch op {
	case ADD:
		return intaddv, nil
	case SUB:
		return intsubv, nil
	case MUL:
		return intmulv, nil
	case DIV:
		return intdivv, nil
	default:
		return func(l, r *Value) (*Value, error) { return nil, nil }, fmt.Errorf("operator %s is not defined for type Int", op)
	}
}

func intaddv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(&Int{}, l.v.(int)+r.v.(int)), nil
	case *Float:
		return NewValue(&Float{}, float64(l.v.(int))+r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func intsubv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(&Int{}, l.v.(int)-r.v.(int)), nil
	case *Float:
		return NewValue(&Float{}, float64(l.v.(int))-r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func intmulv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(&Int{}, l.v.(int)*r.v.(int)), nil
	case *Float:
		return NewValue(&Float{}, float64(l.v.(int))*r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func intdivv(l, r *Value) (*Value, error) {
	switch r.Type().(type) {
	case *Int:
		return NewValue(&Int{}, l.v.(int)/r.v.(int)), nil
	case *Float:
		return NewValue(&Float{}, float64(l.v.(int))/r.v.(float64)), nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}
