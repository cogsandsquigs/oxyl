package operators

// Token defines the type for Oxyl tokens like (, ), {, and }
type Token string

// Oxyl tokens
const (
	LPN Token = "("
	RPN Token = ")"
)

// Token defines the type for Oxyl operators like +, -, *, and /
type Operator Token

// these are the operators in Oxyl
const (
	ADD Operator = "+"
	SUB Operator = "-"
	MUL Operator = "*"
	DIV Operator = "/"

	AND Operator = "&&"
	OR  Operator = "||"
)
