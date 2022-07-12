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
	DOUBLE_QUOTE
	SINGLE_QUOTE

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
	case LEFT_BRACE:
		return "{"
	case RIGHT_BRACE:
		return "}"
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
	case BOOLEAN:
		return "Boolean"
	case LET:
		return "let"
	case IF:
		return "if"
	case FUN:
		return "Function"
	case RETURN:
		return "return"
	case WHILE:
		return "while"
	case FOR:
		return "for"
	case TYPE:
		return "type"
	case SELF:
		return "self"
	case NEWLINE:
		return "Newline"
	default:
		// maybe this could be an operator?
		return Operator(t).String()
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
	SLASH
	STAR
	CARET
	PERCENT
	AMP
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

// Returns the string representation of the token types.
// Ex:
//	FUN.String() // returns "fun"
//	IDENTIFIER.String() // returns "identifier"
func (o Operator) String() string {
	switch o {
	case DASH:
		return "-"
	case PLUS:
		return "+"
	case SLASH:
		return "/"
	case STAR:
		return "*"
	case CARET:
		return "^"
	case PERCENT:
		return "%"
	case AMP:
		return "&"
	case PIPE:
		return "|"
	case GREATER:
		return ">"
	case LESS:
		return "<"
	case BANG:
		return "!"
	case EQUAL:
		return "="
	case BANG_EQUAL:
		return "!="
	case EQUAL_EQUAL:
		return "=="
	case GREATER_EQUAL:
		return ">="
	case LESS_EQUAL:
		return "<="
	case AND:
		return "&&"
	case OR:
		return "||"
	default:
		return "Unknown"
	}
}

func (o Operator) Token() Token {
	return Token(o)
}

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
