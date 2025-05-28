package runtime

import (
	"testing"

	"github.com/nbursa/sentience/internal/types"
)

func TestReflectAccess_ExistingKey(t *testing.T) {
	ctx := NewAgentContext()
	ctx.SetMem("short", "msg", "danas je ponedeljak")

	stmt := &types.ReflectAccessStatement{
		MemTarget: "short",
		Key:       "msg",
	}

	Eval(stmt, "", ctx)

	got := ctx.GetMem("short", "msg")
	want := "danas je ponedeljak"
	if got != want {
		t.Errorf("expected %q, got %q", want, got)
	}
}

func TestReflectAccess_MissingKey(t *testing.T) {
	ctx := NewAgentContext()

	stmt := &types.ReflectAccessStatement{
		MemTarget: "short",
		Key:       "nema",
	}

	Eval(stmt, "", ctx)

	got := ctx.GetMem("short", "nema")
	if got != "" {
		t.Errorf("expected empty string, got %q", got)
	}
}

func TestEmbedStatement_Eval(t *testing.T) {
	ctx := NewAgentContext()
	ctx.SetMem("short", "msg", "vrednost")

	embed := &types.EmbedStatement{
		Source: "msg",
		Target: "mem.short",
	}

	Eval(embed, "", ctx)

	got := ctx.GetMem("short", "msg")
	want := "vrednost"
	if got != want {
		t.Errorf("expected %q, got %q", want, got)
	}
}

func TestAgentStatement_Eval(t *testing.T) {
	ctx := NewAgentContext()

	agent := &types.AgentStatement{
		Name: "Echo",
		Body: []types.Statement{
			&types.MemStatement{Target: "short"},
		},
	}

	Eval(agent, "", ctx)

	if ctx.CurrentAgent == nil || ctx.CurrentAgent.Name != "Echo" {
		t.Fatalf("expected agent Echo to be registered")
	}

	got := ctx.GetMem("short", "__init__")
	if got != "1" {
		t.Errorf("expected mem.short[__init__] = %q, got %q", "1", got)
	}
}

func TestOnInputStatement_Eval(t *testing.T) {
	ctx := NewAgentContext()
	ctx.SetMem("short", "inputParam", "pozdrav")

	onInput := &types.OnInputStatement{
		Param: "inputParam",
		Body: []types.Statement{
			&types.EmbedStatement{
				Source: "inputParam",
				Target: "mem.short",
			},
		},
	}

	Eval(onInput, "", ctx)

	got := ctx.GetMem("short", "inputParam")
	want := "pozdrav"
	if got != want {
		t.Errorf("expected %q, got %q", want, got)
	}
}
