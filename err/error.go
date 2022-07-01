package err

import "fmt"

// this is the internal error type for oxyl
type Error struct {
	message   string // the error message
	line      int    // line where it occured
	character int    // character where it occured
}

func New(message string, line, character int) Error {
	return Error{
		message:   message,
		line:      line,
		character: character,
	}
}

// implements error type
func (e Error) Error() string {
	return fmt.Sprintf("[ERROR] line %d, character %d: %s", e.line, e.character, e.message)
}
