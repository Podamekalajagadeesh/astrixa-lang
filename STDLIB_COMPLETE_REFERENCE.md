# 📚 ASTRIXA Standard Library - Complete Reference

**Status:** ✅ Production Ready  
**Version:** 1.0.0  
**Date:** January 4, 2026

---

## Table of Contents

1. [Philosophy](#philosophy)
2. [Modules Overview](#modules-overview)
3. [io Module](#io-module)
4. [fs Module](#fs-module)
5. [net Module](#net-module)
6. [json Module](#json-module)
7. [async Module](#async-module)
8. [crypto Module](#crypto-module)
9. [ai Module](#ai-module)
10. [web Module](#web-module)
11. [Best Practices](#best-practices)
12. [Migration Guide](#migration-guide)

---

## Philosophy

### The ASTRIXA Stdlib Manifesto

ASTRIXA's standard library must be:

✅ **Minimal but Powerful**
- Only essential functions
- Composable building blocks
- 80% of what developers need

✅ **Safe by Default**
- No undefined behavior
- Bounds checking automatic
- Memory safe, no buffer overflows
- Type safe

✅ **Async-First**
- Native async/await (not bolted-on)
- Concurrent by design
- Zero unnecessary allocations
- Perfect for servers and daemons

✅ **Web + Web3 + AI Ready**
- HTTP first (not TCP sockets)
- Crypto built-in
- AI-native (not library import)
- Blockchain-aware

✅ **Zero Magic, Clear Intent**
- Explicit over implicit
- Clear error messages
- Composable, not magical
- Auditable code

### Comparison

| Feature | Python | JavaScript | Rust | **ASTRIXA** |
|---------|--------|-----------|------|-----------|
| Minimal | ✗ | ✓ | ✗ | ✓ |
| Safe | ✗ | ✗ | ✓ | ✓ |
| Async-first | ✗ | ✓ | ✓ | ✓ |
| Chaos | ✓ | ✓ | ✗ | ✗ |
| AI built-in | ✗ | ✗ | ✗ | ✓ |
| Web3 built-in | ✗ | ✗ | ✗ | ✓ |
| Learn curve | Low | Low | High | **Low** |
| Power | High | Medium | High | **High** |

---

## Modules Overview

```
std/
├── io         - Input/Output (print, input, format)
├── fs         - File System (read, write, list)
├── net        - Networking (HTTP, WebSocket, DNS)
├── json       - JSON parsing & manipulation
├── async      - Native async/await & concurrency
├── crypto     - Cryptography & Web3 signing
├── ai         - AI operations (generation, classification)
└── web        - Web server framework
```

### Quick Reference Table

| Module | Purpose | Key Functions | Status |
|--------|---------|---------------|--------|
| **io** | Input/Output | print, input, format | ✅ Ready |
| **fs** | File System | read, write, exists, list_dir | ✅ Ready |
| **net** | Networking | http_get, http_post, fetch | ✅ Ready |
| **json** | JSON | parse, stringify, validate | ✅ Ready |
| **async** | Concurrency | await, spawn, channel | ✅ Ready |
| **crypto** | Security | sha256, sign, verify, encrypt | ✅ Ready |
| **ai** | AI/ML | generate, classify, embed | ✅ Ready |
| **web** | Web Framework | Server, Router, Middleware | ✅ Ready |

---

## io Module

**Purpose:** Safe input/output operations

### Basic I/O

```javascript
// Print to output
print("Hello, ASTRIXA!")
println(42)

// Read from input
let name = input("Enter name: ")

// Format strings safely
let greeting = format("Hello {}, you are {} years old", name, 25)
```

### Advanced I/O

```javascript
// String buffer for efficient concatenation
let buffer = StringBuffer::new()
let result = buffer
    .append("Hello ")
    .append(name)
    .append("!")
    .to_string()

// Raw bytes
let bytes = read_bytes(1024)
write_bytes(bytes)
```

### Key Functions

```javascript
fn print(message: string)
fn println(value: any)
fn input(prompt: string) -> string
fn format(template: string, args: array) -> string
fn write_bytes(data: array)
fn read_bytes(count: int) -> array
fn flush()
fn line_number() -> int
fn has_input() -> bool
```

---

## fs Module

**Purpose:** Safe file system operations

### Basic File Operations

```javascript
// Read entire file
let content = read("config.txt")

// Write to file
write("output.txt", "Hello World")

// Check if exists
if exists("data.json") {
    let data = parse(read("data.json"))
}

// Delete file
delete("temp.txt")
```

### Directory Operations

```javascript
// List files
let files = list_dir("./src")

// Create directories
mkdir("output")
mkdir_all("output/data/logs")

// Get current directory
let cwd = current_dir()
change_dir("/path/to/dir")
```

### File Metadata

```javascript
// Get file size
let size = file_size("data.bin")

// Get metadata
let meta = stat("file.txt")
if meta.is_file {
    print("File size:", meta.size)
    print("Modified:", meta.modified)
}
```

### File Handles

```javascript
// Open file for reading
let file = File::open("data.txt")
let line = file.read_line()
file.close()

// Open for writing
let out = File::open_write("output.txt")
out.write("Hello")
out.close()
```

### Key Functions

```javascript
fn read(path: string) -> string
fn read_bytes(path: string) -> array
fn write(path: string, content: string)
fn write_bytes(path: string, data: array)
fn append(path: string, content: string)
fn exists(path: string) -> bool
fn delete(path: string)
fn file_size(path: string) -> int
fn list_dir(path: string) -> array
fn stat(path: string) -> FileMetadata
fn mkdir(path: string)
fn mkdir_all(path: string)
fn rmdir(path: string)
fn absolute_path(path: string) -> string
fn current_dir() -> string
fn change_dir(path: string)
```

---

## net Module

**Purpose:** Web-first networking (HTTP, WebSocket)

### HTTP Requests

```javascript
// Simple GET
let response = http_get("https://api.example.com/users")
if response.is_ok() {
    let users = response.json()
}

// POST with data
let data = stringify({name: "John", email: "john@example.com"})
let result = http_post("https://api.example.com/users", data)

// PUT and DELETE
http_put("https://api.example.com/users/1", updated_data)
http_delete("https://api.example.com/users/1")
```

### Advanced Requests

```javascript
// Custom request
let req = http_request("GET", "https://api.example.com/data")
let response = req
    .header("Authorization", "Bearer token123")
    .header("X-Custom", "value")
    .timeout(30)
    .follow_redirects(true)
    .send()
```

### Web Server

```javascript
// Create server
let server = Server::new()

// Add routes
server.get("/", fn(req, res) {
    res.json({message: "Hello World"}).send()
})

server.post("/api/users", fn(req, res) {
    let data = req.json()
    let user = create_user(data)
    res.status(201).json(user).send()
})

// Add middleware
server.use(logger_middleware())
server.use(json_parser_middleware({}))

// Listen
server.listen(3000)
```

### WebSocket

```javascript
// Server-side
server.get("/ws", fn(req, res) {
    let ws = upgrade_to_websocket(req, res)
    ws.on_message(fn(msg) {
        print("Received:", msg)
        ws.send("Echo: " + msg)
    })
})

// Client-side
let ws = WebSocket::new("wss://example.com/chat")
ws.on_message(fn(msg) {
    print("Message:", msg)
})
ws.connect()
ws.send("Hello Server!")
```

### Key Functions

```javascript
fn http_get(url: string) -> Response
fn http_post(url: string, body: string) -> Response
fn http_put(url: string, body: string) -> Response
fn http_delete(url: string) -> Response
fn http_request(method: string, url: string) -> Request
fn fetch(url: string) -> string
fn fetch_bytes(url: string) -> array
fn download(url: string, path: string)
fn resolve(hostname: string) -> string
fn is_reachable(hostname: string) -> bool
fn local_ip() -> string
fn Server::new() -> Server
fn upgrade_to_websocket(req: Request, res: Response) -> WebSocketConnection
```

---

## json Module

**Purpose:** Safe JSON parsing and manipulation

### Parsing and Stringifying

```javascript
// Parse JSON
let obj = parse('{"name":"John","age":30}')
print(obj.name)     // "John"
print(obj.age)      // 30

// Convert to JSON
let json = stringify({name: "Alice", age: 28})
print(json)         // {"name":"Alice","age":28}

// Pretty print
let pretty = stringify_pretty({name: "Bob"}, 2)
```

### Safe Parsing

```javascript
// Try parse (doesn't panic)
let result = try_parse(possibly_invalid)
if result.is_ok() {
    let data = result.unwrap()
} else {
    print("Error:", result.error)
    print("At line:", result.error_line)
}

// Validate
if is_valid_json(text) {
    let data = parse(text)
}
```

### Path Operations

```javascript
// Get nested values safely
let name = get_path(obj, "user.profile.name")

// Set nested values
set_path(obj, "user.age", 30)

// Check keys
if has_key(obj, "email") {
    print(obj.email)
}
```

### Merging and Cloning

```javascript
// Merge objects
let merged = merge({a: 1}, {b: 2})  // {a: 1, b: 2}

// Deep clone
let copy = clone(original)

// Get keys/values
let keys = keys(obj)
let values = values(obj)
```

### Builder Pattern

```javascript
// Safe JSON construction
let builder = JSONBuilder::new()
let data = builder
    .add("name", "John")
    .add("age", 30)
    .add_array("skills", ["Rust", "ASTRIXA"])
    .build()

// Array builder
let arr = JSONArray::new()
arr.push(1).push(2).push(3)
let result = arr.build()  // [1, 2, 3]
```

### Key Functions

```javascript
fn parse(text: string) -> object
fn stringify(obj: object) -> string
fn stringify_pretty(obj: object, indent: int) -> string
fn try_parse(text: string) -> Result
fn is_valid_json(text: string) -> bool
fn get_path(obj: object, path: string) -> any
fn set_path(obj: object, path: string, value: any)
fn merge(obj1: object, obj2: object) -> object
fn clone(obj: object) -> object
fn has_key(obj: object, key: string) -> bool
fn keys(obj: object) -> array
fn values(obj: object) -> array
fn to_csv(data: array) -> string
fn from_csv(text: string) -> array
fn JSONBuilder::new() -> JSONBuilder
fn JSONArray::new() -> JSONArray
fn Schema::new(definition: object) -> Schema
```

---

## async Module

**Purpose:** Native async/await and concurrency primitives

### Async/Await

```javascript
// Define async function
fn async fetch_user(id: int) {
    let response = await http_get("https://api.example.com/users/" + id)
    return response.json()
}

// Use async function
let user = await fetch_user(123)
print(user.name)

// Sleep (non-blocking)
await sleep(1000)     // 1000 milliseconds
await sleep_sec(5)    // 5 seconds
```

### Spawning Tasks

```javascript
// Spawn background task
spawn(fn() {
    print("Running in background")
    process_data()
})

// Spawn with timeout
spawn_with_timeout(fn() {
    do_work()
}, 5000)  // 5 second timeout

// Join multiple tasks
let task1 = spawn(fn() { work1() })
let task2 = spawn(fn() { work2() })
let results = await join_all([task1, task2])
```

### Channels

```javascript
// Create channel
let (tx, rx) = channel()

// Spawn producer
spawn(fn() {
    tx.send("Message 1")
    tx.send("Message 2")
    tx.close()
})

// Consume messages
let msg1 = await rx.recv()
let msg2 = await rx.recv()
```

### Synchronization

```javascript
// Mutex for shared state
let counter = Mutex::new(0)

spawn(fn() {
    let guard = counter.lock()
    guard.set(guard.get() + 1)
})

// RwLock for read-heavy
let config = RwLock::new(configuration)
let reader = config.read()
let current = reader.get()

// Semaphore for limiting
let permits = Semaphore::new(5)
permit.acquire()
do_work()
permit.release()
```

### Advanced Patterns

```javascript
// Race (first to complete)
let result = await race([
    http_get("server1.example.com"),
    http_get("server2.example.com")
])

// Timeout a task
let result = await timeout(fn() {
    return fetch_data()
}, 3000)  // 3 second timeout

// Retry with backoff
let data = await retry(fn() {
    return http_get("flaky-api.example.com")
}, 3)  // Retry up to 3 times

// Batch process
let results = await batch([
    process(item1),
    process(item2),
    process(item3)
], batch_size: 2)
```

### Key Functions

```javascript
fn await(promise: Promise) -> any
fn sleep(milliseconds: int)
fn sleep_sec(seconds: int)
fn spawn(task: fn) -> Task
fn spawn_with_timeout(task: fn, timeout_ms: int) -> Task
fn channel() -> (Sender, Receiver)
fn broadcast_channel(capacity: int) -> (BroadcastSender, BroadcastReceiver)
fn join_all(tasks: array) -> array
fn race(promises: array) -> any
fn timeout_promise(milliseconds: int) -> Promise
fn timeout(task: fn, milliseconds: int) -> any
fn retry(task: fn, max_attempts: int) -> any
fn batch(tasks: array, batch_size: int) -> array
fn Promise::new(executor: fn) -> Promise
fn Promise::resolve(value: any) -> Promise
fn Promise::reject(error: string) -> Promise
fn Mutex::new(value: any) -> Mutex
fn RwLock::new(value: any) -> RwLock
fn Semaphore::new(permits: int) -> Semaphore
```

---

## crypto Module

**Purpose:** Cryptography and Web3 operations (safe by default)

### Hashing

```javascript
// SHA-256 (Web3 standard)
let hash = sha256("password")
let hash_bytes = sha256_bytes(data)

// Keccak-256 (Ethereum)
let eth_hash = keccak256("data")

// HMAC for authentication
let auth = hmac_sha256("message", "secret_key")
```

### Digital Signatures

```javascript
// Generate keypair
let (pub, priv) = generate_keypair()

// Sign data
let sig = sign(priv, "transaction_hash")

// Verify signature
if verify(pub, "transaction_hash", sig) {
    print("Signature valid!")
}

// Recover public key from signature
let recovered_pub = recover_key(message, signature)
```

### Web3 Keys

```javascript
// Ethereum keypair
let pair = generate_keypair_secp256k1()

// Get public key from private
let pub = public_key("private_key_hex")

// Get Ethereum address
let addr = address_from_pubkey(pub)
```

### Encryption

```javascript
// AES-256-GCM (modern, safe)
let encrypted = encrypt_aes256("secret message", "encryption_key")
let plaintext = decrypt_aes256(encrypted, "encryption_key")

// ChaCha20-Poly1305 (alternative)
let enc = encrypt_chacha20("data", "key")
let dec = decrypt_chacha20(enc, "key")
```

### Key Derivation

```javascript
// Password-based key derivation
let key = pbkdf2("user_password", "random_salt")

// Memory-hard (Argon2)
let strong_key = argon2("password", "salt")
```

### Random Numbers

```javascript
// Cryptographically secure random
let nonce = random_bytes(32)
let token = random_hex(16)

// Random integer
let value = random_int(1, 100)
```

### Encoding

```javascript
// Hex encoding
let hex = bytes_to_hex([72, 101, 108, 108, 111])
let bytes = hex_to_bytes("48656c6c6f")

// Base64
let encoded = base64_encode("Hello World")
let decoded = base64_decode(encoded)
```

### Key Functions

```javascript
fn sha256(data: string) -> string
fn sha256_bytes(data: array) -> array
fn keccak256(data: string) -> string
fn blake2b(data: string) -> string
fn hmac_sha256(message: string, key: string) -> string
fn sign(private_key: string, message: string) -> Signature
fn verify(public_key: string, message: string, signature: Signature) -> bool
fn recover_key(message: string, signature: Signature) -> string
fn generate_keypair() -> (string, string)
fn generate_keypair_secp256k1() -> KeyPair
fn public_key(private_key: string) -> string
fn address_from_pubkey(public_key: string) -> string
fn encrypt_aes256(plaintext: string, key: string) -> Encrypted
fn decrypt_aes256(encrypted: Encrypted, key: string) -> string
fn encrypt_chacha20(plaintext: string, key: string) -> Encrypted
fn decrypt_chacha20(encrypted: Encrypted, key: string) -> string
fn pbkdf2(password: string, salt: string) -> string
fn argon2(password: string, salt: string) -> string
fn random_bytes(count: int) -> array
fn random_hex(count: int) -> string
fn random_int(min: int, max: int) -> int
fn hex_to_bytes(hex: string) -> array
fn bytes_to_hex(bytes: array) -> string
fn base64_encode(data: string) -> string
fn base64_decode(encoded: string) -> string
```

---

## ai Module

**Purpose:** First-class AI integration (no Python needed)

### Text Generation

```javascript
// Generate text
let poem = generate("Write a haiku about programming")

// With options
let options = GenerateOptions::new()
options.max_tokens = 100
options.temperature = 0.7
let response = generate_with_options(prompt, options)
```

### Classification

```javascript
// Classify text
let category = classify("This is amazing!", ["positive", "negative"])

// Sentiment analysis
let sentiment = analyze_sentiment("I love ASTRIXA!")
print(sentiment.label)     // "positive"
print(sentiment.score)     // 0.95
```

### Embeddings

```javascript
// Get embedding (vector)
let embedding = embed("hello world")

// Similarity between texts
let sim = similarity("cat", "kitten")  // 0.85

// Semantic search
let results = semantic_search("best practices", documents)
```

### Entity Extraction

```javascript
// Extract named entities
let entities = extract_entities("John works at Google in California")
// Returns: [
//   {text: "John", label: "PERSON"},
//   {text: "Google", label: "ORG"},
//   {text: "California", label: "LOCATION"}
// ]
```

### Content Processing

```javascript
// Summarize text
let summary = summarize(long_article, max_length: 200)

// Translate
let spanish = translate("Hello world", "en", "es")

// Detect language
let lang = detect_language("Bonjour")  // "fr"

// Paraphrase
let variations = paraphrase("I like ASTRIXA", count: 3)
```

### Question Answering

```javascript
// Answer from context
let answer = answer_question("What is ASTRIXA?", documentation)

// Search for answers in documents
let answers = qa_search("How to install?", docs)
```

### Safety & Moderation

```javascript
// Detect toxicity
if is_toxic(user_input) {
    block_content()
}

// Detect spam
if is_spam(email) {
    filter_email()
}

// Moderate content
let result = moderate(text)
if !result.safe {
    print("Flagged for:", result.scores)
}
```

### Batch Operations

```javascript
// Classify multiple texts
let results = batch_classify([
    "Great product!",
    "Terrible experience",
    "It's okay"
], ["positive", "negative"])

// Get embeddings for all
let embeddings = batch_embed(documents)
```

### Key Functions

```javascript
fn generate(prompt: string) -> string
fn generate_with_options(prompt: string, options: GenerateOptions) -> string
fn classify(text: string, categories: array) -> string
fn analyze_sentiment(text: string) -> SentimentResult
fn extract_entities(text: string) -> array
fn summarize(text: string, max_length: int) -> string
fn embed(text: string) -> Embedding
fn similarity(text1: string, text2: string) -> float
fn semantic_search(query: string, documents: array) -> array
fn detect_language(text: string) -> string
fn translate(text: string, from_lang: string, to_lang: string) -> string
fn answer_question(question: string, context: string) -> string
fn qa_search(question: string, documents: array) -> array
fn paraphrase(text: string, count: int) -> array
fn generate_code(description: string) -> string
fn complete(text: string) -> string
fn is_toxic(text: string) -> bool
fn is_spam(text: string) -> bool
fn moderate(text: string) -> ModerationResult
fn extract_json(text: string) -> object
fn extract_table(text: string) -> array
fn batch_classify(texts: array, categories: array) -> array
fn batch_embed(texts: array) -> array
fn set_model(model_name: string)
fn set_api_key(key: string)
fn configure(options: AIConfig)
```

---

## web Module

**Purpose:** Backend web framework

### Creating a Server

```javascript
// Basic server
let server = Server::new()
server.listen(3000)

// With options
let options = ServerOptions::new()
options.port = 8080
options.timeout = 60
let server = Server::with_options(options)
server.listen(options.port)
```

### Routing

```javascript
// Add routes
server.get("/", fn(req, res) {
    res.text("Home page").send()
})

server.post("/api/users", fn(req, res) {
    let data = req.json()
    res.status(201).json({id: 1, ...data}).send()
})

server.put("/api/users/:id", fn(req, res) {
    let id = req.param("id")
    let data = req.json()
    update_user(id, data)
    res.json({success: true}).send()
})

server.delete("/api/users/:id", fn(req, res) {
    let id = req.param("id")
    delete_user(id)
    res.status(204).send()
})
```

### Middleware

```javascript
// Built-in middleware
server.use(logger_middleware())
server.use(json_parser_middleware({}))
server.use(cors_middleware({
    origins: ["https://example.com"],
    methods: ["GET", "POST", "PUT", "DELETE"],
    allow_credentials: true
}))

// Custom middleware
server.use(fn(req, res, next) {
    print("Request:", req.method, req.path)
    next()
})

// Authentication
server.use(auth_middleware(fn(token) {
    return verify_token(token)
}))

// Rate limiting
server.use(rate_limit_middleware(100))  // 100 requests/second
```

### Request Processing

```javascript
// Get JSON body
let data = req.json()

// Get form data
let form = req.form()

// Get query parameters
let query = req.query["search"]

// Get path parameters
let id = req.param("id")

// Get headers
let auth = req.header("Authorization")

// Get cookies
let session = req.cookie("session_id")

// Get client info
let ip = req.ip()
let agent = req.user_agent()
```

### Responses

```javascript
// Send JSON
res.json({status: "ok", data: items}).send()

// Send text
res.text("Hello World").send()

// Send HTML
res.html("<h1>Title</h1>").send()

// Send file
res.file("./downloads/report.pdf").send()

// Set status code
res.status(404).json({error: "Not found"}).send()

// Redirect
res.redirect("/new-path").send()

// Set headers
res
    .header("X-Custom", "value")
    .header("Cache-Control", "no-cache")
    .send()

// Set cookies
res.set_cookie(Cookie::new("session", "abc123")).send()
```

### WebSocket

```javascript
// Upgrade to WebSocket
server.get("/chat", fn(req, res) {
    let ws = upgrade_to_websocket(req, res)
    
    ws.on_message(fn(msg) {
        print("Received:", msg)
        broadcast(msg)
    })
    
    ws.on_close(fn() {
        print("Client disconnected")
    })
})
```

### Routers

```javascript
// Organize routes
let api = Router::new()

// Add routes to router
api.get("/users", list_users)
api.get("/users/:id", get_user)
api.post("/users", create_user)
api.put("/users/:id", update_user)
api.delete("/users/:id", delete_user)

// Mount router
server.use_router(api)

// Start server
server.listen(8080)
```

### Key Functions

```javascript
fn Server::new() -> Server
fn Server::with_options(options: ServerOptions) -> Server
fn Router::new() -> Router
fn upgrade_to_websocket(req: Request, res: Response) -> WebSocketConnection
fn logger_middleware() -> fn
fn cors_middleware(options: CORSOptions) -> fn
fn auth_middleware(verify: fn) -> fn
fn rate_limit_middleware(requests_per_second: int) -> fn
fn json_parser_middleware(options: ParserOptions) -> fn
fn form_parser_middleware(options: ParserOptions) -> fn
fn session_middleware(options: SessionOptions) -> fn
fn static_files(directory: string) -> fn
fn render_template(path: string, context: object) -> string
fn compile_template(template: string) -> Template
fn parse_url(url: string) -> URLInfo
fn redirect_response(url: string) -> Response
fn json_error(status: int, message: string) -> Response
```

---

## Best Practices

### 1. Use Type Safety

```javascript
// ✅ Good: Explicit types
fn process_user(id: int, data: object) -> object {
    let user = get_user(id)
    return {...user, ...data}
}

// ❌ Avoid: Any types
fn process_user(id: any, data: any) -> any {
    // Loses type safety
}
```

### 2. Handle Errors Properly

```javascript
// ✅ Good: Check before use
let result = try_parse(text)
if result.is_ok() {
    let data = result.unwrap()
} else {
    print("Error:", result.error)
}

// ❌ Avoid: Panic on invalid
let data = parse(text)  // Panics if invalid
```

### 3. Use Async for I/O

```javascript
// ✅ Good: Non-blocking
fn async fetch_users() {
    let response = await http_get(API_URL + "/users")
    return response.json()
}

// ❌ Avoid: Blocking
fn fetch_users() {
    // Blocks entire thread
    let response = http_get(API_URL + "/users")
    return response.json()
}
```

### 4. Secure Crypto by Default

```javascript
// ✅ Good: Modern algorithms
let encrypted = encrypt_aes256(data, key)    // AES-256-GCM
let sig = sign(priv_key, message)            // ECDSA

// ❌ Avoid: Weak algorithms
let hash = md5(data)        // Deprecated
let base64 = base64_encode(data)  // Not encryption!
```

### 5. Use Timeouts

```javascript
// ✅ Good: Timeout protection
let result = await timeout(fn() {
    return http_get(url)
}, 5000)

// ❌ Avoid: No timeout
let response = http_get(url)  // Can hang forever
```

### 6. Validate Input

```javascript
// ✅ Good: Validate before use
if is_valid_json(text) {
    let data = parse(text)
}

// ❌ Avoid: Parse without validation
let data = parse(possibly_invalid)  // Panic!
```

### 7. Use Resource Cleanup

```javascript
// ✅ Good: Automatic cleanup
let file = File::open("data.txt")
let content = file.read_line()
file.close()

// Or use with statements (future)
```

---

## Migration Guide

### From Python

```python
# Python
import json
data = json.loads(text)
with open("file.txt") as f:
    content = f.read()
print(content)
```

```javascript
// ASTRIXA
let data = parse(text)
let content = read("file.txt")
print(content)
```

### From JavaScript

```javascript
// JavaScript
const data = await fetch(url).then(r => r.json())
const result = data.map(item => item.id)
```

```javascript
// ASTRIXA
let response = await http_get(url)
let data = response.json()
let result = data.map(fn(item) { return item.id })
```

### From Python Requests

```python
# Python
import requests
response = requests.get("https://api.example.com/users")
users = response.json()
```

```javascript
// ASTRIXA
let response = http_get("https://api.example.com/users")
let users = response.json()
```

### From Express.js

```javascript
// Express
const express = require("express")
const app = express()
app.get("/api/users", (req, res) => {
    res.json(users)
})
app.listen(3000)
```

```javascript
// ASTRIXA
let server = Server::new()
server.get("/api/users", fn(req, res) {
    res.json(users).send()
})
server.listen(3000)
```

---

## Summary

ASTRIXA's standard library provides:

✅ Everything developers need (80%)  
✅ Clean, safe APIs by default  
✅ Async-first architecture  
✅ Web + Web3 + AI built-in  
✅ Better than Python/JS/Rust alternatives  

**It's not just a library. It's a philosophy.**

---

**Status:** ✅ Complete  
**Quality:** ⭐⭐⭐⭐⭐  
**Ready for:** Production use
