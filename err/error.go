package err

import "fmt"

// this is the internal error type for oxyl
type Error struct {
	message string // the error message
	line    int    // line where it occured
	column  int    // character where it occured
}

func New(message string, line, column int) Error {
	return Error{
		message: message,
		line:    line,
		column:  column,
	}
}

// implements error type
func (e Error) Error() string {
	return fmt.Sprintf("[ERROR] line %d, column %d: %s", e.line, e.column, e.message)
}
