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
	fmt.Println("Sentience REPL v0.1")
	scanner := bufio.NewScanner(os.Stdin)
	ctx := runtime.NewAgentContext()

	for {
		fmt.Print(">>> ")
		if !scanner.Scan() {
			break
		}

		line := strings.TrimSpace(scanner.Text())
		if line == "exit" {
			break
		}

		if strings.HasPrefix(line, ".save") {
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

		if strings.HasPrefix(line, ".load") {
			path := "ctx.json"
			parts := strings.Fields(line)
			if len(parts) > 1 {
				path = parts[1]
			}
			if err := ctx.Load(path); err != nil {
				fmt.Println("Error loading:", err)
			} else {
				fmt.Println("Loaded from", path)
				fmt.Println("MEM:", ctx.MemShort)
			}
			continue
		}

		if strings.HasPrefix(line, ".input ") {
			input := strings.TrimSpace(strings.TrimPrefix(line, ".input "))
			if ctx.CurrentAgent == nil {
				fmt.Println("No agent registered.")
				continue
			}

			// find on input
			found := false
			for _, stmt := range ctx.CurrentAgent.Body {
				if inputStmt, ok := stmt.(*types.OnInputStatement); ok {
					found = true
					param := inputStmt.Param
					ctx.SetMem("short", param, input)
					for _, s := range inputStmt.Body {
						runtime.Eval(s, "  ", ctx)
					}
					fmt.Println("MEM:", ctx.MemShort)
				}
			}

			if !found {
				fmt.Println("Agent has no on input handler.")
			}
			continue
		}

		lexer := parser.NewLexer(line)
		p := parser.NewParser(lexer)
		program := p.ParseProgram()
		runtime.Eval(program, "", ctx)
		fmt.Println("MEM:", ctx.MemShort)
	}
}
