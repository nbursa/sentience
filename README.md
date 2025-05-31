# sentience-rs (v0.1.0)

Sentience is an experimental AI-native programming language designed around memory-driven cognition, contextual agents, and introspective behavior.

    ⚠️ This project is in early research prototyping phase.
    It is not production-ready, may change significantly and should be considered exploratory research.

Highlights

    Memory-first architecture (mem.short, mem.long, mem.latent)
    Agent-based programming (goal, on input, reflect, train, evolve)
    Contextual conditions (if context includes)
    Embeddable REPL and file interpreter
    Designed for studying emergent agency, adaptation, and synthetic awareness

Entry point:

```bash
    src/main.rs
```

Example:

```bash
agent Reflector {
    mem short
    goal: "Detect emotion in input"

    on input(msg) {
        embed msg -> mem.short
        reflect {
            if context includes "joy" {
                print "You're radiating joy!"
            }
        }
    }
}
```
