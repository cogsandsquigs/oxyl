package types

type Type interface {
	Name() string   // Name of the type
	Format() string // Returns string representation of type
	Value() any     // Returns value of type
}
