package std

type Type interface {
	Name() string

	// DefinedOperators is used to list operators defined for type, used for type checking
	DefinedOperators(op Operator) ([]Type, error)

	// ExecuteOperators is ONLY be called AFTER type checking occurs
	ExecuteOperators(op Operator) func(l, r *Value) (*Value, error)
}
