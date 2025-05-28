package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/nbursa/sentience/internal/parser"
	"github.com/nbursa/sentience/internal/runtime"
	"github.com/nbursa/sentience/internal/types"
)

func main() {
	fmt.Println("Sentience REPL v0.1 (reader mode)")
	reader := bufio.NewReader(os.Stdin)
	ctx := runtime.NewAgentContext()

	var buffer []string
	blockDepth := 0

	for {
		prompt := ">>> "
		if blockDepth > 0 {
			prompt = "... "
		}

		fmt.Print(prompt)
		line, err := reader.ReadString('\n')
		if err != nil {
			break
		}
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		if blockDepth == 0 && strings.HasPrefix(line, ".input ") {
			input := strings.TrimSpace(strings.TrimPrefix(line, ".input "))
			if ctx.CurrentAgent == nil {
				fmt.Println("No agent registered.")
				continue
			}
			found := false
			for _, stmt := range ctx.CurrentAgent.Body {
				if inputStmt, ok := stmt.(*types.OnInputStatement); ok {
					found = true
					param := inputStmt.Param
					ctx.SetMem("short", param, input)
					for _, s := range inputStmt.Body {
						runtime.Eval(s, "  ", ctx)
					}
				}
			}
			if !found {
				fmt.Println("Agent has no on input handler.")
			}
			continue
		}

		if blockDepth == 0 && strings.HasPrefix(line, ".train ") {
			input := strings.TrimSpace(strings.TrimPrefix(line, ".train "))
			if ctx.CurrentAgent == nil {
				fmt.Println("No agent registered.")
				continue
			}
			found := false
			for _, stmt := range ctx.CurrentAgent.Body {
				if trainStmt, ok := stmt.(*types.TrainStatement); ok {
					found = true
					ctx.SetMem("short", "msg", input)
					for _, s := range trainStmt.Body {
						runtime.Eval(s, "  ", ctx)
					}
				}
			}
			if !found {
				fmt.Println("Agent has no train block.")
			}
			continue
		}

		if blockDepth == 0 && strings.HasPrefix(line, ".save") {
			path := "ctx.json"
			parts := strings.Fields(line)
			if len(parts) > 1 {
				path = parts[1]
			}
			if err := ctx.Save(path); err != nil {
				fmt.Println("Error saving:", err)
			} else {
				fmt.Println("Saved to", path)
			}
			continue
		}

		if blockDepth == 0 && strings.HasPrefix(line, ".load") {
			path := "ctx.json"
			parts := strings.Fields(line)
			if len(parts) > 1 {
				path = parts[1]
			}
			if err := ctx.Load(path); err != nil {
				fmt.Println("Error loading:", err)
			} else {
				fmt.Println("Loaded from", path)
			}
			continue
		}

		blockDepth += strings.Count(line, "{")
		blockDepth -= strings.Count(line, "}")

		buffer = append(buffer, line)

		if blockDepth == 0 {
			fullInput := strings.Join(buffer, " ")
			lexer := parser.NewLexer(fullInput)
			p := parser.NewParser(lexer)
			program := p.ParseProgram()
			runtime.Eval(program, "", ctx)
			buffer = nil
		}
	}
}
