package token

type Token int

const (
	// placeholder for any unknown tokens
	UNKNOWN Token = iota

	// single-character tokens
	LEFT_PAREN
	RIGHT_PAREN
	LEFT_BRACKET
	RIGHT_BRACKET
	LEFT_BRACE
	RIGHT_BRACE
	COMMA
	DOT

	// literals
	IDENTIFIER
	STRING
	NUMBER
	BOOLEAN

	// keywords
	// variable declarations
	VAR
	// control flow
	IF
	ELSE
	// function declaration
	FUN
	RETURN
	// loops
	WHILE
	FOR
	// type declarations
	TYPE
	SELF // for accessing the type's methods

	// end of file
	EOF
)

// operators are separate because this allows for
// custom operator functions to be added to types
// inbuilt and otherwise
type Operator Token

const (
	// operators, single caracters
	DASH Operator = iota + Operator(EOF)
	PLUS
	SEMICOLON
	SLASH
	STAR

	// operators (cont.), one or more characters
	BANG
	BANG_EQUAL
	EQUAL
	EQUAL_EQUAL
	GREATER
	GREATER_EQUAL
	LESS
	LESS_EQUAL
	AMPERSAND
	AND
	PIPE
	OR
)

// Returns the string representation of the token types.
// Ex:
//	FUN.String() // returns "fun"
//	IDENTIFIER.String() // returns "identifier"
func (t Token) String() string {
	switch t {
	case EOF:
		return "EOF"
	case LEFT_PAREN:
		return "("
	case RIGHT_PAREN:
		return ")"
	case LEFT_BRACKET:
		return "["
	case RIGHT_BRACKET:
		return "]"
	default:
		return "unknown"
	}
}
