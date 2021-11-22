package std

import "fmt"

// Type defines the structure for types in Oxyl
type Type struct {
	name        string
	cmpoptypes  map[Operator][]string
	execopfuncs map[Operator]func(l, r *Value) (interface{}, string)
}

// Name returns the name of the type
func (t *Type) Name() string {
	return t.name
}

// CmpOpType stands for Compatible Operator and Type. Checks to see if type and operator can be used together.
// The assumption here is that `t` is on the left side, which sets the precedent for the operation.
func (t *Type) CmpOpType(op Operator, typ *Type) bool {
	typeNameList, ok := t.cmpoptypes[op]
	if !ok {
		return ok
	}
	for _, typeName := range typeNameList {
		if typeName == t.name {
			return true
		}
	}

	return false
}

// ExecuteOp executes an operation on a left and right value, with the assumption that the left value has the same type as the type being used here.
func (t *Type) ExecuteOp(op Operator, l, r *Value) (interface{}, string, error) {
	if _, ok := t.cmpoptypes[op]; !ok {
		return nil, "None", fmt.Errorf("execution error: operator " + string(op) + " does not exist")
	} else if l.typ.name != t.name {
		return nil, "None", fmt.Errorf("execution error: left operand of type " + l.typ.name + " is not equal to expected type " + t.name)
	} else if !t.CmpOpType(op, r.typ) {
		return nil, "None", fmt.Errorf("execution error: the operator " + string(op) + " is not defined for types " + t.name + " and " + r.typ.name)
	}
	i, s := t.execopfuncs[op](l, r)
	return i, s, nil
}

// Int is the integer type
var Int = Type{
	name: "Int",
	cmpoptypes: map[Operator][]string{
		ADD: {"Int", "Float"},
		SUB: {"Int", "Float"},
		MUL: {"Int", "Float"},
		DIV: {"Int", "Float"},
	},
	execopfuncs: map[Operator]func(l, r *Value) (interface{}, string){
		ADD: func(l, r *Value) (interface{}, string) {
			switch r.typ.name {
			case "Int":
				return l.val.(int64) + r.val.(int64), "Int"
			case "Float":
				return float64(l.val.(int64)) + l.val.(float64), "Float"
			default:
				return nil, "None"
			}
		},
		SUB: func(l, r *Value) (interface{}, string) {
			switch r.typ.name {
			case "Int":
				return l.val.(int64) - r.val.(int64), "Int"
			case "Float":
				return float64(l.val.(int64)) - l.val.(float64), "Float"
			default:
				return nil, "None"
			}
		},
		MUL: func(l, r *Value) (interface{}, string) {
			switch r.typ.name {
			case "Int":
				return l.val.(int64) * r.val.(int64), "Int"
			case "Float":
				return float64(l.val.(int64)) * l.val.(float64), "Float"
			default:
				return nil, "None"
			}
		},
		DIV: func(l, r *Value) (interface{}, string) {
			switch r.typ.name {
			case "Int":
				return l.val.(int64) / r.val.(int64), "Int"
			case "Float":
				return float64(l.val.(int64)) / l.val.(float64), "Float"
			default:
				return nil, "None"
			}
		},
	},
}

// Float is the floating point number type
var Float = Type{
	name: "Int",
	cmpoptypes: map[Operator][]string{
		ADD: {"Int", "Float"},
		SUB: {"Int", "Float"},
		MUL: {"Int", "Float"},
		DIV: {"Int", "Float"},
	},
	execopfuncs: map[Operator]func(l, r *Value) (interface{}, string){
		ADD: func(l, r *Value) (interface{}, string) {
			switch r.typ.name {
			case "Int":
				return l.val.(float64) + float64(r.val.(int64)), "Int"
			case "Float":
				return l.val.(float64) + l.val.(float64), "Float"
			default:
				return nil, "None"
			}
		},
		SUB: func(l, r *Value) (interface{}, string) {
			switch r.typ.name {
			case "Int":
				return l.val.(float64) - float64(r.val.(int64)), "Int"
			case "Float":
				return l.val.(float64) - l.val.(float64), "Float"
			default:
				return nil, "None"
			}
		},
		MUL: func(l, r *Value) (interface{}, string) {
			switch r.typ.name {
			case "Int":
				return l.val.(float64) * float64(r.val.(int64)), "Int"
			case "Float":
				return l.val.(float64) * l.val.(float64), "Float"
			default:
				return nil, "None"
			}
		},
		DIV: func(l, r *Value) (interface{}, string) {
			switch r.typ.name {
			case "Int":
				return l.val.(float64) / float64(r.val.(int64)), "Int"
			case "Float":
				return l.val.(float64) / l.val.(float64), "Float"
			default:
				return nil, "None"
			}
		},
	},
}
