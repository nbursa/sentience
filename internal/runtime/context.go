package runtime

import (
	"encoding/json"
	"os"
	"strings"

	"github.com/nbursa/sentience/internal/types"
)

type AgentContext struct {
	MemShort     map[string]string
	MemLong      map[string]string
	MemLatent    map[string][]float64
	CurrentAgent *types.AgentStatement `json:"-"`
	Links        map[string]string
}

func NewAgentContext() *AgentContext {
	return &AgentContext{
		MemShort:  make(map[string]string),
		MemLong:   make(map[string]string),
		MemLatent: make(map[string][]float64),
		Links:     make(map[string]string),
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

// TODO: Semantic value here is mocked — real embedding pending.
func (ctx *AgentContext) EmbedLatent(key, content string) {
	ctx.MemLatent[key] = fakeEmbed(content)
}

func (ctx *AgentContext) SimilarTo(query string) []string {
	qVec := fakeEmbed(query)
	var results []string
	for key, vec := range ctx.MemLatent {
		if cosineSimilarity(vec, qVec) > 0.75 {
			results = append(results, key)
		}
	}
	return results
}

// TODO: Replace fakeEmbed with real sentence embeddings (e.g. MiniLM, Ada, ChromaDB).
func fakeEmbed(text string) []float64 {
	words := strings.Fields(text)
	vec := make([]float64, 3)
	for i, word := range words {
		h := simpleHash(word)
		vec[i%3] += float64(h % 100)
	}
	return vec
}

// NOTE: This is a placeholder embedding function.
func simpleHash(s string) int {
	hash := 0
	for i := 0; i < len(s); i++ {
		hash = int(s[i]) + ((hash << 5) - hash)
	}
	return hash
}

func cosineSimilarity(a, b []float64) float64 {
	var dot, magA, magB float64
	for i := range a {
		dot += a[i] * b[i]
		magA += a[i] * a[i]
		magB += b[i] * b[i]
	}
	if magA == 0 || magB == 0 {
		return 0
	}
	return dot / (sqrt(magA) * sqrt(magB))
}

func sqrt(x float64) float64 {
	z := x
	for i := 0; i < 10; i++ {
		z -= (z*z - x) / (2 * z)
	}
	return z
}
