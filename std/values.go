package std

// Value defines the structure for the values in Oxyl
type Value struct {
	typ Type
	val interface{}
}

// NewValue creates a new value with a type and an interface
func NewValue(typ Type, val interface{}) *Value {
	return &Value{typ, val}
}

// gets the value's type
func (v *Value) Type() Type {
	return v.typ
}

// gets the value's value
func (v *Value) Value() interface{} {
	return v.val
}
