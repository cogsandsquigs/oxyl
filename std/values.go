package std

type Value struct {
    t Type
    v interface{}
}

func (v *Value) Type() Type {
    return v.t
}

func (v *Value) Value() interface{} {
    return v.v
}