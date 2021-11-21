package std

type Value struct {
	typ Type
	val interface{}
}

func NewValue(typ Type, val interface{}) *Value {
	return &Value{typ, val}
}

func (v *Value) Type() Type {
	return v.typ
}

func (v *Value) Value() interface{} {
	return v.val
}
