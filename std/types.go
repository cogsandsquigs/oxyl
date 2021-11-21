package std

type Type struct {
	name       string
	cmpoptypes map[Operator][]string
}

// Name returns the name of the type
func (t *Type) Name() string {
	return t.name
}

// CmpOpType stands for COmpatible OPerator and TYPE. Checks to see if type and operator can be used together.
// The assumption here is that `t` is on the left side, which sets the precedent for the operation.
func (t *Type) CmpOpType(op Operator, typ Type) bool {
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

// Int is the integer type
var Int = Type{
	name: "Int",
	cmpoptypes: map[Operator][]string{
		ADD: {"Int", "Float"},
		SUB: {"Int", "Float"},
	},
}

// Float is the floating point number type
var Float = Type{
	name: "Int",
	cmpoptypes: map[Operator][]string{
		ADD: {"Int", "Float"},
		SUB: {"Int", "Float"},
	},
}
