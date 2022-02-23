package types

import (
	"fmt"

	. "oxyl/std/operators"
	"oxyl/std/traits"
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

func (t *Float) ExecuteOperators(op Operator) *Method {
	switch op {
	case ADD:
		return &Method{Name:string(ADD), Func: floataddv}
	case SUB:
		return &Method{Name:string(SUB), Func: floatsubv}
	case MUL:
		return &Method{Name:string(MUL), Func: floatmulv}
	case DIV:
		return &Method{Name:string(DIV), Func: floatdivv}
	default:
		return &Method{Name:"", Func:func([]*Value) ([]*Value, error) { return nil, fmt.Errorf("operator not defined") }}
	}
}

func (t *Float) IsTraitDefined(trait traits.Trait) bool {
	return false
}

func (t *Float) IsMethodDefined(method string) bool {
	return false
}

func (t *Float) GetMethod(method string) *Method {
	return &Method{
    Name: "", 
    Func: func(in []*Value) ([]*Value, error) { return nil, fmt.Errorf("method %s is not defined", method) },
  }
}

// funcs
func floataddv(v []*Value) ([]*Value, error) {
  if len(v) < 2 {
    return nil, fmt.Errorf("not enough arguments")
  }

  r := v[0]
  l := v[1]
	switch r.Type().(type) {
	case *Int:
		return []*Value{NewValue(&Float{}, l.v.(float64)+float64(r.v.(int)))}, nil
	case *Float:
    return []*Value{NewValue(&Float{}, l.v.(float64)+r.v.(float64))}, nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func floatsubv(v []*Value) ([]*Value, error) {
  if len(v) < 2 {
    return nil, fmt.Errorf("not enough arguments")
  }

  r := v[0]
  l := v[1]
	switch r.Type().(type) {
	case *Int:
		return []*Value{NewValue(&Float{}, l.v.(float64)-float64(r.v.(int)))}, nil
	case *Float:
		return []*Value{NewValue(&Float{}, l.v.(float64)-r.v.(float64))}, nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func floatmulv(v []*Value) ([]*Value, error) {
  if len(v) < 2 {
    return nil, fmt.Errorf("not enough arguments")
  }

  r := v[0]
  l := v[1]
	switch r.Type().(type) {
	case *Int:
		return []*Value{NewValue(&Float{}, l.v.(float64)*float64(r.v.(int)))}, nil
	case *Float:
		return []*Value{NewValue(&Float{}, l.v.(float64)*r.v.(float64))}, nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}

func floatdivv(v []*Value) ([]*Value, error) {
  if len(v) < 2 {
    return nil, fmt.Errorf("not enough arguments")
  }

  r := v[0]
  l := v[1]
	switch r.Type().(type) {
	case *Int:
		return []*Value{NewValue(&Float{}, l.v.(float64)/float64(r.v.(int)))}, nil
	case *Float:
		return []*Value{NewValue(&Float{}, l.v.(float64)/r.v.(float64))}, nil
	default:
		return nil, fmt.Errorf("types %s and %s are not defined for operator +", l.t.Name(), r.t.Name())
	}
}
