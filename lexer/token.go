package lexer

type Token struct {
	Type      TokenType
	Line      int
	Character int
	Value     string
}

func NewToken(t TokenType, l, c int, v string) Token {
	return Token{
		Type:      t,
		Line:      l,
		Character: l,
		Value:     v,
	}
}
