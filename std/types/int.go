package types

import (
	"fmt"

	. "oxyl/std/operators"
	"oxyl/std/traits"
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

func (t *Int) ExecuteOperators(op Operator) func(l, r *Value) (*Value, error) {
	switch op {
	case ADD:
		return intaddv
	case SUB:
		return intsubv
	case MUL:
		return intmulv
	case DIV:
		return intdivv
	default:
		return func(l, r *Value) (*Value, error) { return nil, nil }
	}
}

func (t *Int) IsTraitDefined(trait traits.Trait) bool {
	return false
}

func (t *Int) IsMethodDefined(method string) bool {
	return false
}

func (t *Int) GetMethod(method string) func(in ...*Value) (*Value, error) {
	return func(in ...*Value) (*Value, error) { return nil, fmt.Errorf("method %s is not defined", method) }
}

// funcs
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
