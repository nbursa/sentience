package runtime

import (
	"fmt"

	"github.com/nbursa/sentience/internal/types"
)

func Eval(node types.Node, indent string, ctx *AgentContext) {
	switch n := node.(type) {

	case *types.Program:
		for _, stmt := range n.Statements {
			Eval(stmt, indent, ctx)
		}

	case *types.AgentStatement:
		fmt.Printf("%sAgent: %s\n", indent, n.Name)
		for _, stmt := range n.Body {
			Eval(stmt, indent+"  ", ctx)
		}
		ctx.CurrentAgent = n
		fmt.Printf("%sAgent: %s [registered]\n", indent, n.Name)

	case *types.MemStatement:
		fmt.Printf("%sInit mem: %s\n", indent, n.Target)
		ctx.SetMem(n.Target, "__init__", "1")

	case *types.OnInputStatement:
		fmt.Printf("%sOn Input: (%s)\n", indent, n.Param)
		for _, stmt := range n.Body {
			Eval(stmt, indent+"  ", ctx)
		}

	case *types.ReflectStatement:
		fmt.Printf("%sReflect block:\n", indent)
		for _, stmt := range n.Body {
			Eval(stmt, indent+"  ", ctx)
		}

	case *types.TrainStatement:
		fmt.Printf("%sTrain block:\n", indent)
		for _, stmt := range n.Body {
			Eval(stmt, indent+"  ", ctx)
		}

	case *types.GoalStatement:
		fmt.Printf("%sGoal: \"%s\"\n", indent, n.Value)

	case *types.EmbedStatement:
		fmt.Printf("%sEmbed: %s -> %s\n", indent, n.Source, n.Target)

		val := ctx.GetMem("short", n.Source)

		var target string
		if n.Target == "mem.short" {
			target = "short"
		} else if n.Target == "mem.long" {
			target = "long"
		} else {
			target = "short" // fallback
		}
		ctx.SetMem(target, n.Source, val)

	case *types.LinkStatement:
		fmt.Printf("%sLink: %s <-> %s\n", indent, n.From, n.To)

	case *types.IfStatement:
		fmt.Printf("%sIf: %s\n", indent, n.Condition)
		for _, stmt := range n.Body {
			Eval(stmt, indent+"  ", ctx)
		}

	case *types.EnterStatement:
		fmt.Printf("%sEnter: %s\n", indent, n.Target)

	case *types.ReflectAccessStatement:
		val := ctx.GetMem(n.MemTarget, n.Key)
		fmt.Printf("%smem.%s[\"%s\"] = \"%s\"\n", indent, n.MemTarget, n.Key, val)

	default:
		fmt.Printf("%sUnknown node: %T\n", indent, n)
	}
}
