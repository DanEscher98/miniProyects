package lexer

import "monkeylang_go/token"

type Lexer struct {
	input        string
	position     int  // current position in input (points to current char)
	readPosition int  // current reading position in input (after current char)
	character    byte // current char under examination
}

func (lex *Lexer) readChar() {
	if lex.readPosition >= len(lex.input) {
		lex.character = 0 // reached EOF
	} else {
		lex.character = lex.input[lex.readPosition]
	}
	lex.position = lex.readPosition
	lex.readPosition += 1
}

func (lex *Lexer) NextToken() token.Token {
	var tok token.Token

	switch lex.character {
	case '=':
		tok = newToken(token.ASSIGN, lex.character)
	}
}

func newToken(tokenType token.TokenType, character byte) token.Token {
	return token.Token{Type: tokenType, Literal: string(character)}
}

func New(input string) *Lexer {
	l := &Lexer{input: input}
	l.readChar()
	return l
}
