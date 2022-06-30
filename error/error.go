package error

// this is the internal error type for oxyl
type Error struct {
	message   string // the error message
	line      int    // line where it occured
	character int    // character where it occured
	file      string // name of file where it occured (path to file from where oxyl was executed)
}
