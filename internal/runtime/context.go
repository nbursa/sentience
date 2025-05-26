package runtime

import (
	"encoding/json"
	"os"
)

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

func (ctx *AgentContext) Save(path string) error {
	data, err := json.MarshalIndent(ctx, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(path, data, 0644)
}

func (ctx *AgentContext) Load(path string) error {
	data, err := os.ReadFile(path)
	if err != nil {
		return err
	}
	return json.Unmarshal(data, ctx)
}
