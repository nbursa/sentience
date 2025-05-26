package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"

	"github.com/nbursa/sentience/internal/parser"
	"github.com/nbursa/sentience/internal/runtime"
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

		lexer := parser.NewLexer(line)
		p := parser.NewParser(lexer)
		program := p.ParseProgram()
		runtime.Eval(program, "", ctx)
		fmt.Println("MEM:", ctx.MemShort)
	}
}
