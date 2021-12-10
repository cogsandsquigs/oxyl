package std

type Type interface {
    Name() string
    DefinedOperators(op Operator) ([]Type, error)
    ExecuteOperators(op Operator) (func(l, r *Value) (*Value, error), error)
}

