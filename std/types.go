package std

import (
	"fmt"
)

type Type interface {
	String() string
	IsOperable(Operator, Type) (Type, error)
}

type Int struct {
}

func (t Int) String() string {
	return "Int"
}

func (t Int) IsOperable(op Operator, r Type) (Type, error) {
	if r.String() != "Float" && r.String() != "Int" {
		return nil, fmt.Errorf("types " + t.String() + " and " + r.String() + " are not compatible")
	}

	switch op {
	case ADD:
	case SUB:
	case MUL:
	case DIV:
		if r.String() == "Float" {
			return Float{}, nil
		} else {
			return Int{}, nil
		}

	default:
		return nil, fmt.Errorf("operator " + string(op) + " is not compatible with types " + t.String() + " and " + r.String())
	}
	return nil, fmt.Errorf("this error should not occur")
}

type Float struct {
}

func (t Float) String() string {
	return "Float"
}

func (t Float) IsOperable(op Operator, r Type) (Type, error) {
	if r.String() != "Float" && r.String() != "Int" {
		return nil, fmt.Errorf("types " + t.String() + " and " + r.String() + " are not compatible")
	}

	switch op {
	case ADD:
	case SUB:
	case MUL:
	case DIV:
		if r.String() == "Float" {
			return Float{}, nil
		} else {
			return Int{}, nil
		}

	default:
		return nil, fmt.Errorf("operator " + string(op) + " is not compatible with types " + t.String() + " and " + r.String())
	}

	return nil, fmt.Errorf("this error should not occur")
}

type Bool struct {
}

func (t Bool) String() string {
	return "Bool"
}
