package lexer

import "oxyl/utils/queue"

type Lexer struct {
	input      string
	position   int
	tokenQueue *queue.Queue[Token]
	line       int
	character  int
	errors     []error
}

func NewLexer(input string) *Lexer {
	return &Lexer{
		input:      input,
		position:   0,
		tokenQueue: queue.NewQueue[Token](),
		errors:     []error{},
	}
}

func (l *Lexer) Lex() {
	for _, char := range l.input {
		switch char {
		case '\n':
		case '\r':
			l.line++
			l.tokenQueue.Push(l.newToken(EOL))
		default:
			l.character++
			switch char {
			case '\t':
			case ' ':
				continue
			case '.':
				l.tokenQueue.Push(l.newToken(DOT))
			case ',':
				l.tokenQueue.Push(l.newToken(COMMA))
			case ';':
				l.tokenQueue.Push(l.newToken(SEMICOLON))
			case '(':
				l.tokenQueue.Push(l.newToken(LPAREN))
			case ')':
				l.tokenQueue.Push(l.newToken(RPAREN))
			case '{':
				l.tokenQueue.Push(l.newToken(LBRACE))
			case '}':
				l.tokenQueue.Push(l.newToken(RBRACE))
			case '[':
				l.tokenQueue.Push(l.newToken(LBRACKET))
			case ']':
				l.tokenQueue.Push(l.newToken(RBRACKET))
			case '+':
				l.tokenQueue.Push(l.newToken(PLUS))
			case '-':
				l.tokenQueue.Push(l.newToken(DASH))
			case '*':
				l.tokenQueue.Push(l.newToken(STAR))
			case '/':
				l.tokenQueue.Push(l.newToken(SLASH))
			case '=':
				l.tokenQueue.Push(l.newToken(EQUAL))
			case '!':
				l.tokenQueue.Push(l.newToken(NOT))
			case '<':
				l.tokenQueue.Push(l.newToken(LESS))
			case '>':
				l.tokenQueue.Push(l.newToken(GREATER))
			default:
				l.tokenQueue.Push(l.newToken(UNDEFINED))

			}
		}
	}
}

func (l *Lexer) Next() Token {
	if l.tokenQueue.Size() == 0 {
		return l.newToken(EOF)
	}
	return l.tokenQueue.Pop()
}

func (l *Lexer) Peek() Token {
	if l.tokenQueue.Size() == 0 {
		return l.newToken(EOF)
	}
	return l.tokenQueue.Peek()
}

func (l *Lexer) newToken(tokentype TokenType) Token {
	return NewToken(tokentype, l.line, l.character, "")
}
