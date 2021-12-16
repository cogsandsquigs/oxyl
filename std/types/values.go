package types

type Value struct {
	t Type
	v interface{}
}

func NewValue(t Type, v interface{}) *Value {
	return &Value{
		t: t,
		v: v,
	}
}

func (v *Value) Type() Type {
	return v.t
}

func (v *Value) Value() interface{} {
	return v.v
}
