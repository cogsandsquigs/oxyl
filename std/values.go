package std

// Value defines the structure for the values in Oxyl
type Value struct {
	typ string
	val interface{}
}

// NewValue creates a new value with a type and an interface
func NewValue(typ string, val interface{}) *Value {
	return &Value{typ, val}
}

func NoneValue() *Value {
  return &Value{"None", nil}
}

// gets the value's type
func (v *Value) Type() string {
	return v.typ
}

// gets the value's value
func (v *Value) Value() interface{} {
	return v.val
}
