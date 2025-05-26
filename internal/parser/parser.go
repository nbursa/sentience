package parser

import (
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
