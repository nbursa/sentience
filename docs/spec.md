# 📘 Sentience Language Specification

## 1. Lexical Structure

### 1.1 Identifiers

- Alphanumeric + `_`
- Example: `agent`, `msg`, `mem`

### 1.2 Literals

- String: `"text"`
- Number: `123`, `42.0` (not yet supported semantically)

### 1.3 Keywords

```

agent, goal, mem, on, input, reflect, train, evolve,
if, context, includes, loss, embed, link, enter, print

```

### 1.4 Symbols

```

{ } ( ) \[ ] -> <-> : .

```

---

## 2. Program Structure

```ebnf
program     ::= { statement }
statement   ::= agent_stmt | mem_stmt | reflect_stmt | on_input_stmt | train_stmt |
                evolve_stmt | goal_stmt | embed_stmt | link_stmt | if_stmt |
                enter_stmt | print_stmt
```

---

## 3. Agent Declaration

```sentience
agent <name> {
  [mem <target>]
  [goal: "<string>"]
  [on input(<param>) { ... }]
  [train { ... }]
  [evolve { ... }]
}
```

- Only one agent active at a time
- Agent is registered via REPL evaluation

---

## 4. Memory Model

### 4.1 Declaration

```sentience
mem short
mem long
```

### 4.2 Embed

```sentience
embed <identifier> -> mem.short
embed <identifier> -> mem.latent
```

- `mem.latent` supports semantic clustering (placeholder logic)
- Embedding source must be string in memory

### 4.3 Reflect Access

```sentience
reflect {
  mem.short["key"]
  mem.latent similar_to("query")
}
```

---

## 5. Reflection Block

```sentience
reflect {
  <statement>*
}
```

- Supports direct memory access, printing, conditional logic
- Reflect blocks are printed by default in output

---

## 6. Input Handling

```sentience
on input(msg) {
  <statement>*
}
```

- Parameter bound to `.input <value>` in REPL
- Stored as `msg` in memory if embedded

---

## 7. Train Block

```sentience
train {
  if loss > 0.1 {
    ...
  }
}
```

- Only executed with `.train <value>`
- `loss > 0.1` is placeholder condition

---

## 8. Evolve Block

```sentience
evolve {
  reflect { ... }
}
```

- Used for introspection, feedback, self-modification
- Requires `.evolve <value>` in REPL

---

## 9. Conditions

```sentience
if context includes "<keyword>" {
  ...
}
```

- Searches all keys and values in short-term memory
- Executes body if match is found

```sentience
if loss > 0.1 {
  ...
}
```

- Placeholder for training simulation

---

## 10. Linking Concepts

```sentience
link topic1 <-> topic2
```

- Stores bidirectional mapping in context
- Used for symbolic associations

---

## 11. REPL Commands

| Command          | Description                           |
| ---------------- | ------------------------------------- |
| `.input <text>`  | Feeds string to on input(msg) handler |
| `.train <text>`  | Executes train block with `msg` input |
| `.evolve <text>` | Executes evolve block                 |
| `.save [file]`   | Saves AgentContext to file            |
| `.load [file]`   | Loads AgentContext from file          |

---

## 12. Built-in Print

```sentience
print "Text to print"
```

- Writes to REPL output

---

## 13. Runtime Semantics

- All statements are evaluated via `Eval(Node, indent, ctx)`
- Each node implements the `Node` interface and is type-asserted in a central evaluator
- Memory is contextual (`AgentContext`)
- Memory context (`AgentContext`) is persistent between statements
- `reflect`, `train`, and `evolve` blocks are evaluated explicitly
- Reflection accesses memory at runtime
- Conditions are symbolic; no arithmetic
- There is no call stack — execution is linear and event-based

---

## 14. Latent Memory (Experimental)

```sentience
embed "some text" -> mem.latent
reflect {
  mem.latent similar_to("query")
}
```

- `similar_to` performs cosine similarity (mocked)
- Real implementation planned via MiniLM/Chroma

> TODO: Replace `fakeEmbed()` with real embedding system (e.g. MiniLM or Ada) using ChromaDB or Faiss backend.

---

## 15. Current Limitations

- No function declarations
- No arithmetic
- No loops
- Memory space is per-agent
- No persistence of agents, only memory
- No concurrency

---

## 16. Version

- Language version: `Sentience v0.1.0`
- Last updated: `2025-05-29`
