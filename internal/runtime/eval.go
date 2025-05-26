package runtime

import (
	"fmt"

	"github.com/nbursa/sentience/internal/types"
)

func Eval(node types.Node, indent string) {
	switch n := node.(type) {

	case *types.Program:
		for _, stmt := range n.Statements {
			Eval(stmt, indent)
		}

	case *types.AgentStatement:
		fmt.Printf("%sAgent: %s\n", indent, n.Name)
		for _, stmt := range n.Body {
			Eval(stmt, indent+"  ")
		}

	case *types.MemStatement:
		fmt.Printf("%sMem: %s\n", indent, n.Target)

	case *types.OnInputStatement:
		fmt.Printf("%sOn Input: (%s)\n", indent, n.Param)
		for _, stmt := range n.Body {
			Eval(stmt, indent+"  ")
		}

	case *types.ReflectStatement:
		fmt.Printf("%sReflect block:\n", indent)
		for _, stmt := range n.Body {
			Eval(stmt, indent+"  ")
		}

	case *types.TrainStatement:
		fmt.Printf("%sTrain block:\n", indent)
		for _, stmt := range n.Body {
			Eval(stmt, indent+"  ")
		}

	default:
		fmt.Printf("%sUnknown node: %T\n", indent, n)
	}
}
