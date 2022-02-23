package types

import (
	. "oxyl/std/operators"
	"oxyl/std/traits"
)

type Type interface {
	Name() string

	// DefinedOperators is used to list operators defined for type, used for type checking
	DefinedOperators(op Operator) ([]Type, error)

	// ExecuteOperators is ONLY be called AFTER type checking occurs
	ExecuteOperators(op Operator) func(l, r *Value) (*Value, error)

	// IsTraitDefined checks if a trait is defined, returns true if it is, otherwise false.
	IsTraitDefined(trait traits.Trait) bool

	// IsMethodDefined returns true if the method is defined,
	// and false otherwise.
	IsMethodDefined(method string) bool

	// GetMethod takes in a method name and returns a function
	// that corresponds to the method name which acts on the value
	GetMethod(method string) *Method
}
