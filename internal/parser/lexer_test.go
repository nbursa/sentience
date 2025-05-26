package parser

import (
	"testing"
)

func TestNextToken(t *testing.T) {
	input := `agent Mind {
  mem short
  on input(data) {
    embed data -> mem.short
  }
}`

	expected := []Token{
		{AGENT, "agent"},
		{IDENT, "Mind"},
		{LBRACE, "{"},
		{MEM, "mem"},
		{IDENT, "short"},
		{ON, "on"},
		{INPUT, "input"},
		{LPAREN, "("},
		{IDENT, "data"},
		{RPAREN, ")"},
		{LBRACE, "{"},
		{EMBED, "embed"},
		{IDENT, "data"},
		{ARROW, "->"},
		{MEM, "mem"},
		{DOT, "."},
		{IDENT, "short"},
		{RBRACE, "}"},
		{RBRACE, "}"},
		{EOF, ""},
	}

	lexer := NewLexer(input)

	for i, expectedTok := range expected {
		tok := lexer.NextToken()
		if tok.Type != expectedTok.Type {
			t.Fatalf("tests[%d] - type wrong. expected=%q, got=%q", i, expectedTok.Type, tok.Type)
		}
		if tok.Literal != expectedTok.Literal {
			t.Fatalf("tests[%d] - literal wrong. expected=%q, got=%q", i, expectedTok.Literal, tok.Literal)
		}
	}
}
