# Sentience Core - SRAI Integration

A SRAI-compliant implementation of the Sentience DSL with full integration to RefNet, Cortex, and Superego components.

## Overview

Sentience Core provides a production-ready implementation of the Sentience DSL as specified in the SRAI research papers. It includes:

- **Deterministic token processing** with canonical AST and SHA-256 hashing
- **Symbolic embeddings** (ℝ^256) for semantic search and recall
- **RefNet integration** for cognitive evaluation (valence, SMD, quality)
- **Superego gating** for alignment and safety
- **Cortex memory** with append-only token storage and graph relations
- **Complete pipeline**: Parse → Canonicalize → Hash → Embed → Execute → RefNet → Superego → Cortex

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Sentience     │    │      SRAI        │    │    RefNet       │
│   DSL Code      │───▶│   Integration    │───▶│   Evaluation    │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌──────────────────┐
                       │    Cortex        │
                       │   Memory +       │
                       │   Graph Store    │
                       └──────────────────┘
```

## Quick Start

### 1. Build the Extension

```bash
# Build Rust library and Python extension
make build

# Or install in development mode
make dev
```

### 2. Basic Usage

```python
from sentience_core.srai_integration import create_sentience_agent

# Create agent with SRAI components
agent = create_sentience_agent()

# Execute Sentience DSL
result = agent.think("""
    embed "I need to solve this problem" -> percept.text
    reflect {
        recall ltm[similar: "problem", k=5]
        reframe "analyze_and_solve"
        consolidate
    }
""")

print(f"Quality: {result.metrics.quality:.3f}")
print(f"Generated {len(result.srai_tokens)} tokens")
```

### 3. Run Demos

```bash
# Rust demo
make demo-rust

# Python integration demo
make demo-python
```

## Sentience DSL Syntax

The Sentience DSL supports the following constructs:

### Agent Declaration
```sentience
agent ProblemSolver {
    mem short
    goal: "Solve complex problems systematically"
}
```

### Percept Creation
```sentience
embed "user input" -> percept.text
embed audio_data -> percept.audio
```

### Reflection Operations
```sentience
reflect {
    recall ltm[similar: "topic", k=5]
    reframe "analyze_and_synthesize"
    consolidate
}
```

### Memory Operations
```sentience
set mem.short["key"] = "value"
recall ltm[similar: query, k=10, since="2024-01-01"]
```

## API Reference

### SentienceAgent

Main interface for executing Sentience DSL with SRAI integration.

```python
class SentienceAgent:
    def think(self, dsl_code: str) -> SentienceExecutionResult
    def reflect_on_input(self, input_text: str) -> SentienceExecutionResult
    def get_memory_stats(self) -> Dict[str, Any]
```

### SentienceExecutionResult

Result of Sentience execution containing both Sentience and SRAI tokens.

```python
@dataclass
class SentienceExecutionResult:
    token_id: Optional[str]
    embedding: Optional[np.ndarray]
    metrics: Optional[SentienceRefNetMetrics]
    tokens: List[Dict[str, Any]]  # Sentience tokens
    edges: List[Dict[str, Any]]  # Sentience edges
    srai_tokens: List[SentienceToken]  # SRAI tokens
    srai_edges: List[Edge]  # SRAI edges
```

### RefNet Metrics

Cognitive evaluation metrics from RefNet.

```python
@dataclass
class SentienceRefNetMetrics:
    valence: float      # Emotional valence (-1 to 1)
    smd: float          # Self-Model Drift (0 to 1)
    quality: float      # Reflection quality (0 to 1)
    next_action: str    # Recommended next action
    action_logits: Dict[str, float]  # Action probabilities
```

## Integration with SRAI

### Cortex Memory

Sentience tokens are automatically committed to Cortex with:
- Vector embeddings for semantic search
- Graph edges for structural relations
- Provenance tracking (stm_ids, refnet_id, rules_applied)

### RefNet Evaluation

Every reflection is evaluated by RefNet to provide:
- Quality assessment for Superego gating
- Valence and SMD metrics for alignment
- Next action recommendations

### Superego Gating

Tokens are filtered through Superego rules:
- Quality thresholds (default: ≥0.6)
- Alignment checks
- Safety constraints

## Development

### Project Structure

```
src/
├── lib.rs                    # Main Rust library
├── sentience_core/          # SRAI-compliant implementation
│   ├── mod.rs              # Core API
│   ├── ast.rs              # Token AST definitions
│   ├── canonicalizer.rs    # Deterministic normalization
│   ├── hasher.rs           # SHA-256 token IDs
│   ├── executor.rs         # Token execution engine
│   ├── runtime.rs          # Runtime traits
│   └── parser.rs           # DSL parser
├── python_bridge.rs        # PyO3 Python bindings
└── context.rs              # Original REPL (kept for compatibility)

python/
└── sentience_core/
    ├── __init__.py
    └── srai_integration.py  # SRAI integration module

examples/
├── sentience_core_demo.rs  # Rust demo
└── srai_sentience_integration.py  # Python integration demo
```

### Building from Source

```bash
# Install dependencies
pip install pyo3-setuptools-rust numpy

# Build extension
python setup.py build_ext --inplace

# Or use make
make build
```

### Testing

```bash
# Run all tests
make test

# Run specific tests
cargo test
python -m pytest tests/
```

## Examples

### Basic Reflection

```python
agent = create_sentience_agent()
result = agent.reflect_on_input("I need to understand this concept")

print(f"Quality: {result.metrics.quality:.3f}")
print(f"Next action: {result.metrics.next_action}")
```

### Complex Cognitive Processing

```python
dsl_code = """
agent Analyst {
    mem short
    goal: "Analyze complex topics systematically"
    
    on input(topic) {
        embed topic -> percept.text
        
        reflect {
            recall ltm[similar: topic, k=10]
            reframe "analyze_and_synthesize"
            consolidate
        }
        
        reflect {
            recall ltm[similar: "related_concepts", k=5]
            reframe "connect_and_integrate"
            consolidate
        }
    }
}
"""

result = agent.think(dsl_code)
```

### Memory Operations

```python
# Add memories
agent.reflect_on_input("I learned that systematic thinking helps")
agent.reflect_on_input("Pattern recognition is crucial")

# Test recall
result = agent.reflect_on_input("What have I learned about thinking?")
```

## Performance

- **Deterministic**: Same input → identical token IDs across runs
- **Fast**: Rust implementation with Python bindings
- **Memory efficient**: Append-only storage with compaction
- **Scalable**: Supports large token graphs and long-term memory

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `make check` (format, lint, test)
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## References

- [SRAI Architecture Documentation](../../SRAI/docs/)
- [Sentience DSL Specification](../../SRAI/docs/papers/Sentience__Reflective_Executable_Language_for_Structured_Thought_and_Memory_in_Cognitive_AI_Systems.pdf)
- [RefNet Model Documentation](../../SRAI/docs/05_REFNET_SPEC.md)