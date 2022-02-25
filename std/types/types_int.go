package types

import (
	"fmt"

	. "oxyl/std/operators"
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

func (t *Int) ExecuteOperators(op Operator) *Method {
	switch op {
	case ADD:
		return &Method{Name: string(ADD), Func: intaddv}
	case SUB:
		return &Method{Name: string(SUB), Func: intsubv}
	case MUL:
		return &Method{Name: string(MUL), Func: intmulv}
	case DIV:
		return &Method{Name: string(DIV), Func: intdivv}
	default:
		return &Method{Name: "", Func: func([]*Value) ([]*Value, error) { return nil, fmt.Errorf("operator not defined") }}
	}
}

func (t *Int) IsTraitDefined(trait Trait) bool {
	return false
}

func (t *Int) IsMethodDefined(method string) bool {
	return false
}

func (t *Int) GetMethod(method string) *Method {
	return &Method{
		Name: "",
		Func: func(in []*Value) ([]*Value, error) { return nil, fmt.Errorf("method %s is not defined", method) },
	}
}

// funcs
func intaddv(v []*Value) ([]*Value, error) {
	if len(v) < 2 {
		return nil, fmt.Errorf("not enough arguments")
	}

	r := v[0]
	l := v[1]
	switch r.Type().(type) {
	case *Int:
		return []*Value{NewValue(&Int{}, l.v.(int)+r.v.(int))}, nil
	case *Float:
		return []*Value{NewValue(&Float{}, float64(l.v.(int))+r.v.(float64))}, nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func intsubv(v []*Value) ([]*Value, error) {
	if len(v) < 2 {
		return nil, fmt.Errorf("not enough arguments")
	}

	r := v[0]
	l := v[1]
	switch r.Type().(type) {
	case *Int:
		return []*Value{NewValue(&Int{}, l.v.(int)-r.v.(int))}, nil
	case *Float:
		return []*Value{NewValue(&Float{}, float64(l.v.(int))-r.v.(float64))}, nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func intmulv(v []*Value) ([]*Value, error) {
	if len(v) < 2 {
		return nil, fmt.Errorf("not enough arguments")
	}

	r := v[0]
	l := v[1]
	switch r.Type().(type) {
	case *Int:
		return []*Value{NewValue(&Int{}, l.v.(int)*r.v.(int))}, nil
	case *Float:
		return []*Value{NewValue(&Float{}, float64(l.v.(int))*r.v.(float64))}, nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func intdivv(v []*Value) ([]*Value, error) {
	if len(v) < 2 {
		return nil, fmt.Errorf("not enough arguments")
	}

	r := v[0]
	l := v[1]
	switch r.Type().(type) {
	case *Int:
		return []*Value{NewValue(&Int{}, l.v.(int)/r.v.(int))}, nil
	case *Float:
		return []*Value{NewValue(&Float{}, float64(l.v.(int))/r.v.(float64))}, nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}
