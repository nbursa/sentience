package main

import (
	"bytes"
	"os"
	"os/exec"
	"strings"
	"testing"
)

func TestSentienceMainAgentFlow(t *testing.T) {
	cmd := exec.Command("go", "run", "main.go")
	stdin := strings.NewReader(
		`agent Echo {
			mem short
			train {
				if loss > 0.1 {
					reflect {
						mem.short["msg"]
					}
				}
			}
		}
		.train hello`)

	var stdout bytes.Buffer
	cmd.Stdin = stdin
	cmd.Stdout = &stdout
	cmd.Stderr = os.Stderr

	err := cmd.Run()
	if err != nil {
		t.Fatalf("REPL execution failed: %v", err)
	}

	output := stdout.String()
	if !strings.Contains(output, `mem.short["msg"] = "hello"`) {
		t.Errorf("expected reflect output not found, got:\n%s", output)
	}
}
