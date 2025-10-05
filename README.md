# Sentience

A structured language of thought for cognitive AI systems, with deterministic token processing and reflection capabilities.

## Overview

Sentience provides a complete implementation of the Sentience DSL specification, enabling structured cognitive processing through deterministic token generation, symbolic embeddings, and reflection capabilities.

### Key Features

- **Deterministic Token Processing**: Canonical AST → Hash → Embed → Execute pipeline
- **Symbolic Embeddings**: 256-dimensional deterministic vector generation for semantic operations
- **Reflection Engine**: Built-in cognitive reflection and meta-cognitive operations
- **Python Integration**: PyO3 bindings for seamless Python usage
- **Cross-Platform**: Works on macOS, Linux, and Windows
- **Production Ready**: Comprehensive testing and documentation

## Architecture

```text
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│   Sentience     │     │   Sentience      │     │   Token         │
│   DSL Code      │───▶│   Engine         │───▶│   Generation    │
└─────────────────┘     └──────────────────┘     └─────────────────┘
                                │
                                ▼
                       ┌──────────────────┐
                       │   Deterministic  │
                       │   Processing     │
                       │   Pipeline       │
                       └──────────────────┘
```

## Quick Start

### Installation

```bash
# Install from source
pip install .

# Or build locally
make install
```

### Basic Usage

```python
import sentience_core

# Create Sentience instance
core = sentience_core.create_sentience_core()

# Process Sentience DSL
result = core.process_step('embed "hello world" -> percept.text')

print(f"Token ID: {result.token_id()}")
print(f"Embedding dimension: {len(result.embedding())}")
```

### Rust Usage

```rust
use sentience_core::{SentienceCore, SimpleRuntime};

let runtime = Box::new(SimpleRuntime::new());
let mut core = SentienceCore::new(runtime);

let result = core.process_step("embed test -> percept.text");
println!("Generated token: {}", result.token_id.unwrap());
```

## Sentience DSL

The Sentience DSL is a structured language for expressing cognitive operations:

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
embed visual_data -> percept.vision
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

## Token Types

Sentience supports several token types:

- **Percept**: Sensory input (text, audio, visual)
- **Reflection**: Meta-cognitive operations
- **Action**: Behavioral outputs
- **Concept**: Abstract knowledge representations
- **SelfModel**: Self-awareness and identity

## API Reference

### Python API

```python
# Core functionality
core = sentience_core.create_sentience_core()
result = core.process_step(dsl_code)

# Token operations
ast = core.parse(dsl_code)
canonical = core.canonicalize(ast)
token_id = core.hash(canonical)
embedding = core.embed(canonical)
```

### Rust API

```rust
use sentience_core::{
    SentienceCore, SimpleRuntime,
    ast::{SentienceTokenAst, ThoughtType, Value},
    runtime::ExecutionResult
};

let runtime = Box::new(SimpleRuntime::new());
let mut core = SentienceCore::new(runtime);

// Process DSL
let result = core.process_step("embed test -> percept.text");

// Individual operations
let ast = core.parse("embed test -> percept.text")?;
let canonical = core.canonicalize(&ast);
let token_id = core.hash(&canonical);
let embedding = core.embed(&canonical);
```

## Deterministic Processing

Sentience ensures deterministic behavior:

- **Canonical AST**: Normalized token structure with sorted fields
- **SHA-256 Hashing**: Deterministic token IDs (`mem_` prefix)
- **Symbolic Embeddings**: Fixed-seed vector generation
- **Reproducible Results**: Same input → identical output across runs

## Integration Options

### Standalone Usage

Use Sentience independently for cognitive processing:

```python
core = sentience_core.create_sentience_core()
result = core.process_step("""
    embed "I need to understand this" -> percept.text
    reflect {
        recall ltm[similar: "understanding", k=3]
        reframe "analyze_and_synthesize"
    }
""")
```

### SRAI Integration

Sentience is designed to integrate with SRAI (Structured Reflective AI):

```python
from srai import SRAISentienceCore, Cortex, RefNet

cortex = Cortex()
refnet = RefNet()
integration = SRAISentienceCore(cortex, refnet)

result = integration.process_dsl("embed test -> percept.text")
```

## Development

### Project Structure

```text
src/
├── lib.rs                    # Main Rust library
├── sentience_core/          # Core implementation
│   ├── mod.rs              # Core API
│   ├── ast.rs              # Token AST definitions
│   ├── canonicalizer.rs    # Deterministic normalization
│   ├── hasher.rs           # SHA-256 token IDs
│   ├── executor.rs         # Token execution engine
│   ├── runtime.rs          # Runtime traits
│   └── parser.rs           # DSL parser
├── python_bridge.rs        # PyO3 Python bindings
└── main.rs                 # REPL interface

python/
└── sentience_core/
    ├── __init__.py
    └── srai_integration.py  # Optional SRAI integration

examples/
├── sentience_core_demo.rs  # Rust demo
└── srai_sentience_integration.py  # SRAI integration demo
```

### Building from Source

```bash
# Install Rust dependencies
cargo build

# Install Python dependencies
pip install maturin numpy

# Build Python extension
maturin build --release

# Or use make
make build
```

### Testing

```bash
# Run Rust tests
cargo test

# Run Python tests
python test_srai_integration.py

# Run all tests
make test
```

## Examples

### Basic Token Processing

```python
import sentience_core

core = sentience_core.create_sentience_core()

# Process different token types
percept_result = core.process_step('embed "hello" -> percept.text')
reflection_result = core.process_step('reflect { recall; reframe; consolidate }')
agent_result = core.process_step('agent TestAgent { mem short }')

print(f"Percept ID: {percept_result.token_id()}")
print(f"Reflection ID: {reflection_result.token_id()}")
print(f"Agent ID: {agent_result.token_id()}")
```

### Deterministic Behavior

```python
# Same input produces identical results
result1 = core.process_step('embed test -> percept.text')
result2 = core.process_step('embed test -> percept.text')

assert result1.token_id() == result2.token_id()
assert result1.embedding() == result2.embedding()
```

### Custom Runtime

```rust
use sentience_core::runtime::{Runtime, Cortex, RefNet, Superego};

struct MyRuntime {
    // Custom implementation
}

impl Runtime for MyRuntime {
    fn cortex(&mut self) -> &mut dyn Cortex { /* ... */ }
    fn refnet(&mut self) -> &mut dyn RefNet { /* ... */ }
    fn superego(&mut self) -> &mut dyn Superego { /* ... */ }
}
```

## Performance

- **Fast**: Rust implementation with optimized processing
- **Memory Efficient**: Minimal memory footprint
- **Deterministic**: Reproducible results across platforms
- **Scalable**: Handles large token graphs efficiently

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `make check` (format, lint, test)
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## References

- [Sentience DSL Specification](docs/Sentience__Reflective_Executable_Language_for_Structured_Thought_and_Memory_in_Cognitive_AI_Systems.pdf)
- [SRAI Architecture](https://github.com/nbursa/srai) (optional integration)
