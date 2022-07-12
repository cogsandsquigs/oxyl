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
	INT
	FLOAT
	BOOLEAN

	// keywords
	// variable declarations
	LET
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

	// newline
	NEWLINE

	// end of file
	EOF
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
	case IDENTIFIER:
		return "Identifier"
	case STRING:
		return "String"
	case INT:
		return "Int"
	case FLOAT:
		return "Float"
	case FUN:
		return "Function"
	case NEWLINE:
		return "Newline"
	default:
		return "Unknown"
	}
}

func (t Token) Token() Token {
	return t
}

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
	CARET
	PERCENT
	AMPERSAND
	PIPE
	GREATER
	LESS
	BANG
	EQUAL

	// operators (cont.), one or more characters
	BANG_EQUAL
	EQUAL_EQUAL
	GREATER_EQUAL
	LESS_EQUAL
	AND
	OR
)

// Returns the operator's precedence in binary expression parsing.
// Ex: PLUS.Precedence() // returns 1
func (o Operator) Precedence() int {
	switch o {
	case STAR, SLASH, PERCENT:
		return 5
	case PLUS, DASH:
		return 4
	case EQUAL_EQUAL, BANG_EQUAL, GREATER, GREATER_EQUAL, LESS, LESS_EQUAL:
		return 3
	case AND:
		return 2
	case OR:
		return 1
	default:
		return 0
	}
}
