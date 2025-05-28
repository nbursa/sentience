package runtime

import (
	"bytes"
	"strings"
	"testing"

	"github.com/nbursa/sentience/internal/parser"
	"github.com/nbursa/sentience/internal/types"
)

func TestTrainReflectEval(t *testing.T) {
	input := `
	agent Echo {
	  mem short
	  train {
	    if loss > 0.1 {
	      reflect {
	        mem.short["msg"]
	      }
	    }
	  }
	}`

	lexer := parser.NewLexer(input)
	p := parser.NewParser(lexer)
	program := p.ParseProgram()

	// === LOG STRUCTURE ===
	t.Logf("Parsed Program:\n")
	for _, stmt := range program.Statements {
		t.Logf("Top-level: %T", stmt)
		if agent, ok := stmt.(*types.AgentStatement); ok {
			for _, sub := range agent.Body {
				t.Logf("  AgentBody: %T", sub)
				if train, ok := sub.(*types.TrainStatement); ok {
					for _, sub2 := range train.Body {
						t.Logf("    TrainBody: %T", sub2)
						if ifstmt, ok := sub2.(*types.IfStatement); ok {
							for _, sub3 := range ifstmt.Body {
								t.Logf("      IfBody: %T", sub3)
								if refl, ok := sub3.(*types.ReflectStatement); ok {
									for _, sub4 := range refl.Body {
										t.Logf("        ReflectBody: %T", sub4)
									}
								}
							}
						}
					}
				}
			}
		}
	}

	ctx := NewAgentContext()
	var out bytes.Buffer

	withCapturedOutput(&out, func() {
		Eval(program, "", ctx)
		ctx.SetMem("short", "msg", "hello")
		for _, stmt := range ctx.CurrentAgent.Body {
			if trainStmt, ok := stmt.(*types.TrainStatement); ok {
				for _, s := range trainStmt.Body {
					Eval(s, "  ", ctx)
				}
			}
		}
	})

	output := out.String()
	if !strings.Contains(output, `mem.short["msg"] = "hello"`) {
		t.Errorf("expected reflect output not found, got:\n%s", output)
	}
}

func withCapturedOutput(buf *bytes.Buffer, fn func()) {
	prev := outWriter
	outWriter = buf
	defer func() { outWriter = prev }()
	fn()
}
