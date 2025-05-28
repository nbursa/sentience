package parser

type TokenType string

const (
	ILLEGAL TokenType = "ILLEGAL"
	EOF               = "EOF"
	IDENT             = "IDENT"
	NUMBER            = "NUMBER"
	STRING            = "STRING"

	// Keywords
	AGENT   = "AGENT"
	MEM     = "MEM"
	ON      = "ON"
	GOAL    = "GOAL"
	REFLECT = "REFLECT"
	TRAIN   = "TRAIN"
	IF      = "IF"
	ENTER   = "ENTER"
	EMBED   = "EMBED"
	LINK    = "LINK"
	INPUT   = "INPUT"
	PRINT   = "PRINT"

	// Symbols
	ASSIGN    = "="
	ARROW     = "->"
	LPAREN    = "("
	RPAREN    = ")"
	LBRACE    = "{"
	RBRACE    = "}"
	DOT       = "."
	COLON     = ":"
	LINKARROW = "<->"
	LBRACKET  = "["
	RBRACKET  = "]"
)

var keywords = map[string]TokenType{
	"agent":   AGENT,
	"mem":     MEM,
	"on":      ON,
	"goal":    GOAL,
	"reflect": REFLECT,
	"train":   TRAIN,
	"if":      IF,
	"enter":   ENTER,
	"embed":   EMBED,
	"link":    LINK,
	"input":   INPUT,
	"print":   PRINT,
}

func LookupIdent(ident string) TokenType {
	if tok, ok := keywords[ident]; ok {
		return tok
	}
	return IDENT
}

type Token struct {
	Type    TokenType
	Literal string
}
