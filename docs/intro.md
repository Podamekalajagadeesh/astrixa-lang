# Introduction to ASTRIXA

**ASTRIXA is a modern programming language designed for Web, Web3, and AI — with simplicity, safety, and performance.**

In 10 minutes, you'll understand what makes ASTRIXA different.  
In 30 minutes, you'll write useful code.  
By the end, you'll trust it enough to try in a real project.

## What is ASTRIXA?

ASTRIXA is a general-purpose programming language that gives you **one unified syntax** for:

- **Web applications** — Build servers, APIs, and full-stack apps
- **Web3 smart contracts** — Deploy deterministic, gas-efficient blockchain code
- **AI operations** — Sentiment analysis, text classification, embeddings

No framework sprawl. No polyglot chaos. One language, full stack.

## Your First ASTRIXA Program

Here's a complete web server in ASTRIXA:

```astrixa
use std::web

server {
    route GET "/" {
        return json({ hello: "ASTRIXA" })
    }
}

server.run(8080)
```

That's it. No boilerplate. No configuration. Just code.

Visit `http://localhost:8080` and you'll see:
```json
{ "hello": "ASTRIXA" }
```

## Why ASTRIXA?

### ✅ **Predictable**
Every operation has deterministic behavior. What you write is what executes.

### ✅ **Safe**
Type-safe expressions, memory safety, and built-in error handling prevent common bugs.

### ✅ **Modern**
Native async/await, first-class JSON, blockchain primitives, and AI operations built-in.

### ✅ **Full Stack**
Write your frontend, backend, smart contracts, and AI models in the same language.

## Core Concepts

### Variables & Types
```astrixa
let name = "Alice"           // String
let count: int = 42          // Integer
let price: float = 19.99     // Float
let active: bool = true      // Boolean
```

### Functions
```astrixa
fn greet(name: string) -> string {
    return "Hello, " + name
}

print(greet("World"))  // Hello, World
```

### Async Operations
```astrixa
async fn fetchData(url: string) -> string {
    let response = await http.get(url)
    return response.body
}
```

### Smart Contracts
```astrixa
contract Token {
    state balances: map<Address, U256>
    
    fn transfer(to: Address, amount: U256) {
        balances[tx.sender] -= amount
        balances[to] += amount
    }
}
```

### AI Operations
```astrixa
use std::ai

let text = "This product is amazing!"
let sentiment = ai.sentiment(text)  // "positive"
```

## What Makes ASTRIXA Different?

| Feature | ASTRIXA | Other Languages |
|---------|---------|-----------------|
| **Web + Web3 + AI** | ✅ Native support | ❌ Need multiple tools |
| **One Language** | ✅ Full stack | ❌ Frontend/backend split |
| **Gas Efficiency** | ✅ Deterministic costs | ⚠️ Runtime overhead |
| **Type Safety** | ✅ Static + inferred | ⚠️ Optional or weak |
| **Learning Curve** | ✅ Familiar syntax | ⚠️ Varies widely |

## Who Should Use ASTRIXA?

✅ **Web developers** building modern applications  
✅ **Blockchain developers** creating smart contracts  
✅ **Full-stack engineers** tired of juggling languages  
✅ **AI researchers** needing deterministic ML operations  
✅ **Startups** wanting fast iteration without technical debt  

## What ASTRIXA is NOT

❌ **Not a "better X"** — We're not trying to replace Python, Rust, or JavaScript  
❌ **Not overhyped** — We make no exaggerated claims about performance  
❌ **Not feature-bloated** — We say "no" to complexity  

## Getting Started

Ready to write your first ASTRIXA program?

→ [Installation Guide](installation.md)  
→ [Language Syntax](language/syntax.md)  
→ [Standard Library](stdlib/web.md)  
→ [Examples](../examples/)  

## Philosophy

ASTRIXA is built on three principles:

1. **Simplicity** — Obvious code is better than clever code
2. **Safety** — Catch errors at compile time, not production
3. **Pragmatism** — Real problems need practical solutions

We believe a programming language should **get out of your way** and let you build.

---

**Next:** [Install ASTRIXA →](installation.md)
