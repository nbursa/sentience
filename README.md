# Sentience Language (v0.1.0)

**Sentience** is an experimental AI-native programming language designed around memory-driven cognition, contextual agents, and introspective behavior.

> ⚠️ This project is in early research prototyping phase.  
> It is not production-ready, may change significantly and should be considered exploratory research.

## Highlights

- Memory-first architecture (`mem.short`, `mem.long`, `mem.latent`)
- Agent-based programming (`goal`, `on input`, `reflect`, `train`, `evolve`)
- Contextual conditions (`if context includes`)
- Embeddable REPL and file interpreter
- Designed for studying emergent agency, adaptation, and synthetic awareness

## Example

```sentience
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

## Installation

Clone the repository:

```bash
git clone https://github.com/nbursa/sentience.git
cd sentience
```

## Run

```bash
go run cmd/main.go
```

Or run an agent file:

```bash
go run cmd/main.go run examples/reflector.sent --input "I feel joy"
```

➡ See more in `/examples/*.sent`

## Status

> Prototype — work in progress.
> First functional version: `v0.1.0`, released `2025-05-29`.

## GitAds Sponsored

[![Sponsored by GitAds](https://gitads.dev/v1/ad-serve?source=nbursa/sentience@github)](https://gitads.dev/v1/ad-track?source=nbursa/sentience@github)
