package parser

import (
	"strings"

	"github.com/nbursa/sentience/internal/types"
)

type Parser struct {
	lexer     *Lexer
	curToken  Token
	peekToken Token
	errors    []string
}

func NewParser(l *Lexer) *Parser {
	p := &Parser{
		lexer:  l,
		errors: []string{},
	}
	p.nextToken()
	p.nextToken()
	return p
}

func (p *Parser) nextToken() {
	p.curToken = p.peekToken
	p.peekToken = p.lexer.NextToken()
}

func (p *Parser) ParseProgram() *types.Program {
	program := &types.Program{}
	program.Statements = []types.Statement{}

	for p.curToken.Type != EOF {
		stmt := p.parseStatement()
		if stmt != nil {
			program.Statements = append(program.Statements, stmt)
		}
		p.nextToken()
	}

	return program
}

func (p *Parser) parseStatement() types.Statement {
	switch p.curToken.Type {
	case AGENT:
		return p.parseAgentStatement()
	case MEM:
		return p.parseMemStatement()
	case ON:
		return p.parseOnInputStatement()
	case REFLECT:
		return p.parseReflectStatement()
	case TRAIN:
		return p.parseTrainStatement()
	case GOAL:
		return p.parseGoalStatement()
	case EMBED:
		return p.parseEmbedStatement()
	case LINK:
		return p.parseLinkStatement()
	case IF:
		return p.parseIfStatement()
	case ENTER:
		return p.parseEnterStatement()
	default:
		return nil
	}
}

func (p *Parser) parseAgentStatement() types.Statement {
	stmt := &types.AgentStatement{}

	p.nextToken() // expect agent name
	if p.curToken.Type != IDENT {
		return nil
	}
	stmt.Name = p.curToken.Literal

	p.nextToken() // expect {
	if p.curToken.Type != LBRACE {
		return nil
	}

	stmt.Body = []types.Statement{}
	p.nextToken()

	for p.curToken.Type != RBRACE && p.curToken.Type != EOF {
		bodyStmt := p.parseStatement()
		if bodyStmt != nil {
			stmt.Body = append(stmt.Body, bodyStmt)
		}
		p.nextToken()
	}

	return stmt
}

func (p *Parser) parseMemStatement() types.Statement {
	stmt := &types.MemStatement{}
	p.nextToken()

	if p.curToken.Type != IDENT {
		return nil
	}
	stmt.Target = p.curToken.Literal
	return stmt
}

func (p *Parser) parseOnInputStatement() types.Statement {
	stmt := &types.OnInputStatement{}

	p.nextToken() // expect input
	if p.curToken.Type != INPUT {
		return nil
	}

	p.nextToken() // expect (
	if p.curToken.Type != LPAREN {
		return nil
	}

	p.nextToken() // expect param
	if p.curToken.Type != IDENT {
		return nil
	}
	stmt.Param = p.curToken.Literal

	p.nextToken() // expect )
	if p.curToken.Type != RPAREN {
		return nil
	}

	p.nextToken() // expect {
	if p.curToken.Type != LBRACE {
		return nil
	}

	stmt.Body = []types.Statement{}
	p.nextToken()

	for p.curToken.Type != RBRACE && p.curToken.Type != EOF {
		bodyStmt := p.parseStatement()
		if bodyStmt != nil {
			stmt.Body = append(stmt.Body, bodyStmt)
		}
		p.nextToken()
	}

	return stmt
}

func (p *Parser) parseReflectStatement() types.Statement {
	stmt := &types.ReflectStatement{}

	p.nextToken() // expect {
	if p.curToken.Type != LBRACE {
		return nil
	}

	stmt.Body = []types.Statement{}
	p.nextToken()

	for p.curToken.Type != RBRACE && p.curToken.Type != EOF {
		bodyStmt := p.parseStatement()
		if bodyStmt != nil {
			stmt.Body = append(stmt.Body, bodyStmt)
		}
		p.nextToken()
	}

	return stmt
}

func (p *Parser) parseTrainStatement() types.Statement {
	stmt := &types.TrainStatement{}

	p.nextToken() // expect {
	if p.curToken.Type != LBRACE {
		return nil
	}

	stmt.Body = []types.Statement{}
	p.nextToken()

	for p.curToken.Type != RBRACE && p.curToken.Type != EOF {
		bodyStmt := p.parseStatement()
		if bodyStmt != nil {
			stmt.Body = append(stmt.Body, bodyStmt)
		}
		p.nextToken()
	}

	return stmt
}

func (p *Parser) parseGoalStatement() types.Statement {
	stmt := &types.GoalStatement{}

	p.nextToken() // expect :
	if p.curToken.Type != COLON {
		return nil
	}

	p.nextToken() // expect string
	if p.curToken.Type != STRING {
		return nil
	}

	stmt.Value = p.curToken.Literal
	return stmt
}

func (p *Parser) parseEmbedStatement() types.Statement {
	stmt := &types.EmbedStatement{}

	p.nextToken() // expect source
	if p.curToken.Type != IDENT {
		return nil
	}
	stmt.Source = p.curToken.Literal

	p.nextToken() // expect ->
	if p.curToken.Type != ARROW {
		return nil
	}

	// p.nextToken() // expect target
	// if p.curToken.Type != IDENT {
	// 	return nil
	// }
	// stmt.Target = p.curToken.Literal
	p.nextToken() // expect target (could be mem.short)
	targetParts := []string{}

	if p.curToken.Type == MEM {
		targetParts = append(targetParts, p.curToken.Literal)
		p.nextToken()
		if p.curToken.Type == DOT {
			targetParts = append(targetParts, ".")
			p.nextToken()
		}
	}

	if p.curToken.Type == IDENT {
		targetParts = append(targetParts, p.curToken.Literal)
	}

	stmt.Target = strings.Join(targetParts, "")

	return stmt
}

func (p *Parser) parseLinkStatement() types.Statement {
	stmt := &types.LinkStatement{}

	p.nextToken() // from
	if p.curToken.Type != IDENT {
		return nil
	}
	stmt.From = p.curToken.Literal

	p.nextToken() // <-> arrow
	if p.curToken.Type != LINKARROW {
		return nil
	}

	p.nextToken() // to
	if p.curToken.Type != IDENT {
		return nil
	}
	stmt.To = p.curToken.Literal

	return stmt
}

func (p *Parser) parseIfStatement() types.Statement {
	stmt := &types.IfStatement{}

	p.nextToken() // grab condition start
	condParts := []string{}
	for p.curToken.Type != LBRACE && p.curToken.Type != EOF {
		lit := p.curToken.Literal

		// restore quotes for STRING token
		if p.curToken.Type == STRING {
			lit = `"` + lit + `"`
		}

		condParts = append(condParts, lit)
		p.nextToken()
	}

	stmt.Condition = smartJoin(condParts)

	if p.curToken.Type != LBRACE {
		return nil
	}

	stmt.Body = []types.Statement{}
	p.nextToken()

	for p.curToken.Type != RBRACE && p.curToken.Type != EOF {
		bodyStmt := p.parseStatement()
		if bodyStmt != nil {
			stmt.Body = append(stmt.Body, bodyStmt)
		}
		p.nextToken()
	}

	return stmt
}

func (p *Parser) parseEnterStatement() types.Statement {
	stmt := &types.EnterStatement{}

	p.nextToken()
	if p.curToken.Type != IDENT {
		return nil
	}

	stmt.Target = p.curToken.Literal
	return stmt
}

// === HELPERS ===

func smartJoin(parts []string) string {
	out := ""
	for i, part := range parts {
		if i > 0 && isAlphaNum(parts[i-1]) && isAlphaNum(part) {
			out += " "
		}
		out += part
	}
	return out
}

func isAlphaNum(s string) bool {
	for _, ch := range s {
		if ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ('0' <= ch && ch <= '9') {
			return true
		}
	}
	return false
}
