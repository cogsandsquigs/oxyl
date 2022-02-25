package types

var Printable = &Trait{
	Name: "Printable",
	Methods: []Method{
		Method{
			Name: "fmt",
			In:   []*Type{},
			Out:  []*Type{},
			Func: nil,
		},
	},
}
