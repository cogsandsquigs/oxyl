package err

import "fmt"

// this is the internal error type for oxyl
type Error struct {
	message   string // the error message
	line      int    // line where it occured
	character int    // character where it occured
	file      string // name of file where it occured (path to file from where oxyl was executed)
}

func New(message string, line, character int, file string) Error {
	return Error{
		message:   message,
		line:      line,
		character: character,
		file:      file,
	}
}

// implements error type
func (e Error) Error() string {
	return fmt.Sprintf("[%d:%d] Error in %s: %s", e.line, e.character, e.file, e.message)
}
