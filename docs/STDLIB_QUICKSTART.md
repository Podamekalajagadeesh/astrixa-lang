# 🚀 ASTRIXA Standard Library - Quick Start

**Get productive in 15 minutes**

---

## The 8 Core Modules

```
std::io      Print, input, format
std::fs      Files and directories
std::net     HTTP and WebSockets
std::json    Parse and stringify
std::async   Async/await and channels
std::crypto  Hashing and signing
std::ai      Generation and classification
std::web     Web server framework
```

---

## 5-Minute Tutorial

### Print and Input

```javascript
print("Hello, ASTRIXA!")

let name = input("Enter your name: ")
println(format("Hello, {}", name))
```

### Read and Write Files

```javascript
// Write to file
write("greeting.txt", "Hello World!")

// Read from file
let content = read("greeting.txt")
print(content)

// Check if exists
if exists("data.json") {
    let data = parse(read("data.json"))
}
```

### HTTP Requests

```javascript
// GET request
let response = http_get("https://api.example.com/users")
if response.is_ok() {
    let users = response.json()
}

// POST request
let body = stringify({name: "John"})
let result = http_post("https://api.example.com/users", body)
```

### Parse JSON

```javascript
// Parse JSON safely
let result = try_parse(text)
if result.is_ok() {
    let data = result.unwrap()
} else {
    print("Error:", result.error)
}

// Build JSON safely
let builder = JSONBuilder::new()
let json = builder
    .add("name", "John")
    .add("age", 30)
    .build()
```

### Async/Await

```javascript
// Sleep without blocking
await sleep(1000)

// Run in background
spawn(fn() {
    do_work()
})

// Wait for completion
let result = await http_get("https://api.example.com/data")
```

### Cryptography

```javascript
// Hash password
let hash = sha256("password123")

// Generate random token
let token = random_hex(32)

// Sign data (Web3)
let (pub, priv) = generate_keypair()
let signature = sign(priv, "message")
```

### AI

```javascript
// Generate text
let response = generate("Write a poem about code")
print(response)

// Classify text
let sentiment = analyze_sentiment("I love ASTRIXA!")
print(sentiment.label)  // "positive"

// Get embeddings
let embedding = embed("hello world")
```

### Web Server

```javascript
let server = Server::new()

server.get("/", fn(req, res) {
    res.json({message: "Hello World"}).send()
})

server.post("/api/users", fn(req, res) {
    let user = req.json()
    // Process user...
    res.status(201).json({id: 1, ...user}).send()
})

server.listen(3000)
print("Server running on http://localhost:3000")
```

---

## Common Patterns

### API Client

```javascript
fn async get_user(id: int) {
    let response = await http_get("https://api.example.com/users/" + id)
    if response.is_ok() {
        return response.json()
    }
    return null
}

let user = await get_user(123)
if user {
    print(user.name)
}
```

### File Processing

```javascript
// Read all files in directory
let files = list_dir("./data")

for file in files {
    let content = read("./data/" + file)
    let data = parse(content)
    process(data)
}
```

### Error Handling

```javascript
// Safe JSON parsing
let result = try_parse(text)
if result.is_ok() {
    let data = result.unwrap()
} else {
    print("JSON Error at line", result.error_line)
    print("Details:", result.error)
}
```

### Concurrent Tasks

```javascript
// Run 3 tasks in parallel
let task1 = spawn(fn() { return work1() })
let task2 = spawn(fn() { return work2() })
let task3 = spawn(fn() { return work3() })

let results = await join_all([task1, task2, task3])
```

### Channel Communication

```javascript
let (tx, rx) = channel()

spawn(fn() {
    for i in range(0, 10) {
        tx.send("Message " + i)
        await sleep(100)
    }
})

for i in range(0, 10) {
    let msg = await rx.recv()
    print(msg)
}
```

### Data Validation

```javascript
// Validate JSON structure
let schema = Schema::new({
    name: "string",
    age: "int",
    email: "string"
})

let result = schema.validate(user_data)
if !result.valid {
    for error in result.errors {
        print("Validation error:", error)
    }
}
```

---

## Cheatsheet

```javascript
// === I/O ===
print(message)                    // Output
println(value)                    // Output with newline
let name = input(prompt)          // Read input
let msg = format(template, args)  // Format string

// === Files ===
let content = read(path)          // Read file
write(path, content)              // Write file
if exists(path) { }               // Check file exists
delete(path)                      // Delete file
let files = list_dir(path)        // List directory

// === Network ===
let res = http_get(url)           // GET request
let res = http_post(url, body)    // POST request
let html = fetch(url)             // Fetch as text
let server = Server::new()        // Create server

// === JSON ===
let obj = parse(text)             // Parse JSON
let text = stringify(obj)         // JSON string
let result = try_parse(text)      // Safe parse
if is_valid_json(text) { }        // Validate

// === Async ===
await sleep(ms)                   // Sleep
spawn(task)                        // Run in background
let (tx, rx) = channel()          // Create channel
tx.send(value)                    // Send value
let val = await rx.recv()         // Receive value

// === Crypto ===
let hash = sha256(data)           // Hash
let (pub, priv) = generate_keypair()
let sig = sign(priv, msg)         // Sign
if verify(pub, msg, sig) { }      // Verify
let encrypted = encrypt_aes256(text, key)

// === AI ===
let text = generate(prompt)       // Generate text
let category = classify(text, cats)
let sentiment = analyze_sentiment(text)
let embedding = embed(text)       // Vector

// === Web ===
server.get(path, handler)         // GET route
server.post(path, handler)        // POST route
res.json(data).send()             // Send JSON
res.status(code).send()           // Set status
```

---

## Next Steps

1. **Run examples** - Try each module
2. **Read the docs** - [STDLIB_COMPLETE_REFERENCE.md](STDLIB_COMPLETE_REFERENCE.md)
3. **Build something** - Use what you learned
4. **Check source** - [stdlib/](stdlib/) modules

---

**Everything you need to build Web, Web3, and AI applications. No Python. No JavaScript. Just ASTRIXA.**
