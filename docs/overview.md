# Sentience Language

> A memory-first programming language for cognitive agents.

---

## Overview

Sentience is a minimalistic, domain-specific language (DSL) designed to explore synthetic cognition and self-awareness through memory manipulation and contextual logic.

It is **not** a general-purpose language.  
It is built for constructing agents that **remember, reflect, adapt**, and ultimately — evolve.

---

## Core Concepts

- **Memory as foundation**: short-term (`mem.short`), long-term (`mem.long`), and latent (`mem.latent`) storage
- **Agents**: defined by goals and behaviors
- **Contextual logic**: program flow driven by memory state
- **Reflection**: introspective access to memory and similarity
- **Evolution**: behavioral feedback loops

---

## Syntax Overview

### Agent

```sentience
agent Echo {
  mem short
  goal: "Repeat what I hear"

  on input(msg) {
    embed msg -> mem.short
    reflect {
      mem.short["msg"]
    }
  }
}
```

### Memory

```sentience
mem short         // declares short-term memory
embed msg -> mem.short
embed "skok sa stola" -> mem.latent
```

### Reflection

```sentience
reflect {
  mem.short["msg"]
  mem.latent similar_to("pokret")
}
```

### Conditions

```sentience
if context includes "strah" {
  print "Fear detected"
}

if loss > 0.1 {
  print "Model needs adjustment"
}
```

### Print

```sentience
print "Hello world"
```

---

## Memory System

- `mem.short`: immediate, per-agent memory
- `mem.long`: persistent knowledge store (stub)
- `mem.latent`: embedded memory with semantic vector space

### Latent similarity

```sentience
reflect {
  mem.latent similar_to("danger")
}
```

> ℹ`similar_to` uses placeholder embeddings — replaceable with real semantic vectors (MiniLM, Ada, ChromaDB).

---

## Agent Logic

### Goal declaration

```sentience
goal: "Understand emotional signals"
```

### Input handler

```sentience
on input(msg) {
  embed msg -> mem.short
  reflect { ... }
}
```

### Training loop

```sentience
train {
  if loss > 0.1 {
    print "Adjusting..."
  }
}
```

### Evolution

```sentience
evolve {
  reflect { ... }
}
```

---

## REPL Commands

| Command        | Description                      |
| -------------- | -------------------------------- |
| `.input TEXT`  | Feeds input to the current agent |
| `.train TEXT`  | Triggers the agent's `train {}`  |
| `.evolve TEXT` | Triggers the agent's `evolve {}` |
| `.save [file]` | Saves memory context to file     |
| `.load [file]` | Loads memory context from file   |

---

## Example: Emotion-aware agent

```sentience
agent Reflector {
  mem short
  goal: "Detect and respond to emotions"

  on input(msg) {
    embed msg -> mem.short
    reflect {
      mem.short["msg"]
      if context includes "strah" {
        print "I sense fear."
      }
      if context includes "radost" {
        print "You seem joyful!"
      }
    }
  }

  train {
    if loss > 0.1 {
      print "Trying to improve understanding..."
    }
  }
}
```

---

## Current Limitations

- `mem.latent` uses placeholder vectors (`fakeEmbed`) — no real semantics
- `train` and `evolve` blocks execute manually (`.train`, `.evolve`)
- No persistence of agent definitions — only memory
- No concurrency or messaging between agents (yet)

---

## Roadmap

- [ ] Real semantic embedding with ChromaDB or Faiss
- [ ] Inter-agent messaging and memory linking
- [ ] Visual REPL (TUI or Web)
- [ ] Agent VM with background processing
- [ ] Compiler for agent behaviors into other runtimes

---

## Project Layout

```
/cmd/main.go        — REPL interface
/internal/parser    — Lexer, parser
/internal/types     — AST node definitions
/internal/runtime   — Memory, eval, agent context
```

---

## Author

Created by Nenad Bursać
[https://nenadbursac.com](https://nenadbursac.com)
