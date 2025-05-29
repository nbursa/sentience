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
	fmt.Printf("%s %s (%s)\n", types.VersionName, types.Version, types.ReleaseDate)
	fmt.Println("REPL mode: reader")

	if len(os.Args) > 1 && os.Args[1] == "run" {
		if len(os.Args) < 3 {
			fmt.Println("Usage: run <file>")
			os.Exit(1)
		}
		content, err := os.ReadFile(os.Args[2])
		if err != nil {
			fmt.Println("Error:", err)
			os.Exit(1)
		}
		lexer := parser.NewLexer(string(content))
		p := parser.NewParser(lexer)
		program := p.ParseProgram()
		ctx := runtime.NewAgentContext()
		runtime.Eval(program, "", ctx)

		if len(os.Args) > 3 && os.Args[3] == "--input" {
			if len(os.Args) < 5 {
				fmt.Println("Usage: run <file> --input <text>")
				os.Exit(1)
			}
			input := os.Args[4]
			if program != nil && len(program.Statements) > 0 {
				ctx.SetMem("short", "msg", input)
				for _, stmt := range ctx.CurrentAgent.Body {
					if inputStmt, ok := stmt.(*types.OnInputStatement); ok {
						for _, s := range inputStmt.Body {
							runtime.Eval(s, "  ", ctx)
						}
					}
				}
			}
		}

		return
	}

	reader := bufio.NewReader(os.Stdin)
	ctx := runtime.NewAgentContext()

	var buffer []string
	blockDepth := 0

	for {
		prompt := ">>> "
		if blockDepth > 0 {
			prompt = "... "
		}

		if blockDepth == 0 {
			fmt.Print(prompt)
		}
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

		if blockDepth == 0 && strings.HasPrefix(line, ".evolve ") {
			input := strings.TrimSpace(strings.TrimPrefix(line, ".evolve "))
			if ctx.CurrentAgent == nil {
				fmt.Println("No agent registered.")
				continue
			}
			found := false
			for _, stmt := range ctx.CurrentAgent.Body {
				if evolveStmt, ok := stmt.(*types.EvolveStatement); ok {
					found = true
					ctx.SetMem("short", "msg", input)
					for _, s := range evolveStmt.Body {
						runtime.Eval(s, "  ", ctx)
					}
				}
			}
			if !found {
				fmt.Println("Agent has no evolve block.")
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
