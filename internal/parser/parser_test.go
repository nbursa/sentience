package parser

import (
	"testing"

	"github.com/nbursa/sentience/internal/types"
)

func TestParseProgram(t *testing.T) {
	input := `agent Cortex {
		mem short
		on input(data) {
			mem long
		}
	}`

	lexer := NewLexer(input)
	parser := NewParser(lexer)
	program := parser.ParseProgram()

	if program == nil {
		t.Fatal("ParseProgram() returned nil")
	}

	if len(program.Statements) != 1 {
		t.Fatalf("Expected 1 top-level statement, got %d", len(program.Statements))
	}

	agentStmt, ok := program.Statements[0].(*types.AgentStatement)
	if !ok {
		t.Fatalf("Expected AgentStatement, got %T", program.Statements[0])
	}

	if agentStmt.Name != "Cortex" {
		t.Errorf("Expected agent name 'Cortex', got %q", agentStmt.Name)
	}

	if len(agentStmt.Body) != 2 {
		t.Fatalf("Expected 2 statements inside agent body, got %d", len(agentStmt.Body))
	}

	_, ok = agentStmt.Body[0].(*types.MemStatement)
	if !ok {
		t.Errorf("Expected first inner stmt to be MemStatement, got %T", agentStmt.Body[0])
	}

	onInput, ok := agentStmt.Body[1].(*types.OnInputStatement)
	if !ok {
		t.Fatalf("Expected second inner stmt to be OnInputStatement, got %T", agentStmt.Body[1])
	}

	if onInput.Param != "data" {
		t.Errorf("Expected input param 'data', got %q", onInput.Param)
	}

	if len(onInput.Body) != 1 {
		t.Errorf("Expected 1 statement inside on input, got %d", len(onInput.Body))
	}
}
