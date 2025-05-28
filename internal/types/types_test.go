package types

import (
	"testing"
)

func TestAgentStatement_String(t *testing.T) {
	stmt := &AgentStatement{Name: "Echo"}
	expected := "agent Echo"
	if stmt.String() != expected {
		t.Errorf("expected %q, got %q", expected, stmt.String())
	}
}

func TestMemStatement_String(t *testing.T) {
	stmt := &MemStatement{Target: "short"}
	expected := "mem short"
	if stmt.String() != expected {
		t.Errorf("expected %q, got %q", expected, stmt.String())
	}
}

func TestReflectAccessStatement_String(t *testing.T) {
	stmt := &ReflectAccessStatement{
		MemTarget: "short",
		Key:       "msg",
	}
	expected := `mem.short["msg"]`
	if stmt.String() != expected {
		t.Errorf("expected %q, got %q", expected, stmt.String())
	}
}

func TestEmbedStatement_String(t *testing.T) {
	stmt := &EmbedStatement{
		Source: "msg",
		Target: "mem.short",
	}
	expected := "embed msg -> mem.short"
	if stmt.String() != expected {
		t.Errorf("expected %q, got %q", expected, stmt.String())
	}
}

func TestLinkStatement_String(t *testing.T) {
	stmt := &LinkStatement{
		From: "a",
		To:   "b",
	}
	expected := "link a <-> b"
	if stmt.String() != expected {
		t.Errorf("expected %q, got %q", expected, stmt.String())
	}
}

func TestGoalStatement_String(t *testing.T) {
	stmt := &GoalStatement{Value: "remember"}
	expected := `goal: remember`
	if stmt.String() != expected {
		t.Errorf("expected %q, got %q", expected, stmt.String())
	}
}