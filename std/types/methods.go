package types

// Methods are built on types, and as such, are in
// the types directory. However, they can be attached
// to types, or they can be stand alone (i.e. functions)
type Method struct {
	Name string
	In   []*Type
	Out  []*Type
	Func func(in []*Value) ([]*Value, error)
}
