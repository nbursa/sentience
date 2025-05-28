package types

import "fmt"

type Node interface {
	TokenLiteral() string
	String() string
}

// === Statements ===

type Program struct {
	Statements []Statement
}

func (p *Program) TokenLiteral() string {
	if len(p.Statements) > 0 {
		return p.Statements[0].TokenLiteral()
	}
	return ""
}

func (p *Program) String() string {
	return "[program]"
}

type Statement interface {
	Node
	statementNode()
}

type Expression interface {
	Node
	expressionNode()
}

type AgentStatement struct {
	Name string
	Body []Statement
}

func (as *AgentStatement) statementNode()       {}
func (as *AgentStatement) TokenLiteral() string { return "agent" }
func (as *AgentStatement) String() string       { return "agent " + as.Name }

type MemStatement struct {
	Target string
}

func (ms *MemStatement) statementNode()       {}
func (ms *MemStatement) TokenLiteral() string { return "mem" }
func (ms *MemStatement) String() string       { return "mem " + ms.Target }

type OnInputStatement struct {
	Param string
	Body  []Statement
}

func (o *OnInputStatement) statementNode()       {}
func (o *OnInputStatement) TokenLiteral() string { return "on" }
func (o *OnInputStatement) String() string       { return "on input(" + o.Param + ")" }

type ReflectStatement struct {
	Body []Statement
}

func (r *ReflectStatement) statementNode()       {}
func (r *ReflectStatement) TokenLiteral() string { return "reflect" }
func (r *ReflectStatement) String() string       { return "reflect { ... }" }

type TrainStatement struct {
	Body []Statement
}

func (t *TrainStatement) statementNode()       {}
func (t *TrainStatement) TokenLiteral() string { return "train" }
func (t *TrainStatement) String() string       { return "train { ... }" }

type GoalStatement struct {
	Value string
}

func (g *GoalStatement) statementNode()       {}
func (g *GoalStatement) TokenLiteral() string { return "goal" }
func (g *GoalStatement) String() string       { return "goal: " + g.Value }

type EmbedStatement struct {
	Source string
	Target string
}

func (e *EmbedStatement) statementNode()       {}
func (e *EmbedStatement) TokenLiteral() string { return "embed" }
func (e *EmbedStatement) String() string {
	return "embed " + e.Source + " -> " + e.Target
}

type LinkStatement struct {
	From string
	To   string
}

func (l *LinkStatement) statementNode()       {}
func (l *LinkStatement) TokenLiteral() string { return "link" }
func (l *LinkStatement) String() string {
	return "link " + l.From + " <-> " + l.To
}

type IfStatement struct {
	Condition string // raw for now
	Body      []Statement
}

func (i *IfStatement) statementNode()       {}
func (i *IfStatement) TokenLiteral() string { return "if" }
func (i *IfStatement) String() string       { return "if " + i.Condition + " { ... }" }

type EnterStatement struct {
	Target string
}

func (e *EnterStatement) statementNode()       {}
func (e *EnterStatement) TokenLiteral() string { return "enter" }
func (e *EnterStatement) String() string       { return "enter " + e.Target }

type ReflectAccessStatement struct {
	MemTarget string
	Key       string
}

func (r *ReflectAccessStatement) statementNode()       {}
func (r *ReflectAccessStatement) TokenLiteral() string { return "reflect-access" }
func (r *ReflectAccessStatement) String() string {
	return fmt.Sprintf(`mem.%s["%s"]`, r.MemTarget, r.Key)
}

type PrintStatement struct {
	Value string
}

func (p *PrintStatement) statementNode()       {}
func (p *PrintStatement) TokenLiteral() string { return "print" }
func (p *PrintStatement) String() string       { return "print \"" + p.Value + "\"" }
