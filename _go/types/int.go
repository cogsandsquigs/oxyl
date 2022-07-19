package types

import "fmt"

type Int struct {
	value int64
}

func NewInt(i int64) Int {
	return Int{i}
}

func (i Int) Name() string {
	return "Int"
}

func (i Int) Format() string {
	return fmt.Sprintf("%d", i.value)
}

func (i Int) Value() any {
	return int64(i.value)
}
