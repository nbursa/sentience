package runtime

type AgentContext struct {
	MemShort map[string]string
	MemLong  map[string]string
}

func NewAgentContext() *AgentContext {
	return &AgentContext{
		MemShort: make(map[string]string),
		MemLong:  make(map[string]string),
	}
}

func (ctx *AgentContext) SetMem(target string, key, value string) {
	switch target {
	case "short":
		ctx.MemShort[key] = value
	case "long":
		ctx.MemLong[key] = value
	}
}

func (ctx *AgentContext) GetMem(target string, key string) string {
	switch target {
	case "short":
		return ctx.MemShort[key]
	case "long":
		return ctx.MemLong[key]
	}
	return ""
}
