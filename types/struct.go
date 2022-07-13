package types

type Struct struct {
	name string
}

func (s Struct) Name() string {
	return s.name
}
