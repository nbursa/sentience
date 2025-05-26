package types

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

type Statement interface {
	Node
	statementNode()
}

type Expression interface {
	Node
	expressionNode()
}

// === agent Foo { ... } ===

type AgentStatement struct {
	Name string
	Body []Statement
}

func (as *AgentStatement) statementNode()       {}
func (as *AgentStatement) TokenLiteral() string { return "agent" }
func (as *AgentStatement) String() string       { return "agent " + as.Name }

// === mem short ===

type MemStatement struct {
	Target string
}

func (ms *MemStatement) statementNode()       {}
func (ms *MemStatement) TokenLiteral() string { return "mem" }
func (ms *MemStatement) String() string       { return "mem " + ms.Target }

// === on input(data) { ... } ===

type OnInputStatement struct {
	Param string
	Body  []Statement
}

func (o *OnInputStatement) statementNode()       {}
func (o *OnInputStatement) TokenLiteral() string { return "on" }
func (o *OnInputStatement) String() string       { return "on input(" + o.Param + ")" }
