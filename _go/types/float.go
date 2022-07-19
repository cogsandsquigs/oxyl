package types

import "fmt"

type Float float64

func NewFloat(f float64) Float {
	return Float(f)
}

func (f Float) Name() string {
	return "Float"
}

func (f Float) Format() string {
	return fmt.Sprintf("%f", f)
}

func (f Float) Value() any {
	return float64(f)
}
