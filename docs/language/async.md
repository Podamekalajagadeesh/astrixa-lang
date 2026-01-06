# Async Programming

Build concurrent and responsive applications with ASTRIXA's async model.

## Async Basics

ASTRIXA uses **async/await** for asynchronous programming - the same pattern used in JavaScript, Python, and Rust.

### Why Async?

**Synchronous (blocking):**
```astrixa
let data1 = fetchData("https://api.example.com/1")  // Wait...
let data2 = fetchData("https://api.example.com/2")  // Wait again...
let data3 = fetchData("https://api.example.com/3")  // Still waiting...
// Total time: 3 seconds (1s each)
```

**Asynchronous (non-blocking):**
```astrixa
let data1 = await fetchData("https://api.example.com/1")
let data2 = await fetchData("https://api.example.com/2")
let data3 = await fetchData("https://api.example.com/3")
// Can run concurrently: ~1 second total
```

## Async Functions

### Defining Async Functions

Use the `async` keyword:

```astrixa
async fn fetchUser(id: int) -> string {
    let response = await http.get("https://api.example.com/users/" + id)
    return response.body
}
```

### Calling Async Functions

Use `await` to get the result:

```astrixa
async fn main() {
    let user = await fetchUser(123)
    print(user)
}
```

**Important:** You can only use `await` inside `async` functions.

## Common Async Patterns

### HTTP Requests

```astrixa
use std::net

async fn fetchData(url: string) -> string {
    let response = await http.get(url)
    
    if response.status == 200 {
        return response.body
    } else {
        panic("Request failed: " + response.status)
    }
}
```

### Multiple Concurrent Requests

```astrixa
async fn fetchMultiple() {
    // Sequential (slow)
    let data1 = await fetchData("url1")
    let data2 = await fetchData("url2")
    
    // Concurrent (fast) - coming soon
    let [data1, data2] = await Promise.all([
        fetchData("url1"),
        fetchData("url2")
    ])
}
```

### Database Operations

```astrixa
async fn getUser(id: int) -> map {
    let query = "SELECT * FROM users WHERE id = " + id
    let result = await db.query(query)
    return result[0]
}

async fn createUser(name: string, email: string) -> int {
    let query = "INSERT INTO users (name, email) VALUES (?, ?)"
    let result = await db.execute(query, [name, email])
    return result.insertId
}
```

### File Operations

```astrixa
use std::fs

async fn readConfig() -> map {
    let content = await fs.readFile("config.json")
    return json.parse(content)
}

async fn writeLog(message: string) {
    await fs.appendFile("app.log", message + "\n")
}
```

## Web Server with Async

Complete async web application:

```astrixa
use std::web

async fn handleRequest(req: Request) -> Response {
    // Fetch data from database
    let user = await db.getUser(req.params.id)
    
    // Call external API
    let weather = await fetchWeather(user.location)
    
    // Return combined response
    return json({
        user: user,
        weather: weather
    })
}

server {
    route GET "/user/:id" {
        return await handleRequest(request)
    }
}

server.run(8080)
```

## Async Control Flow

### Sequential vs Concurrent

**Sequential (one after another):**
```astrixa
async fn sequential() {
    let a = await operation1()  // Wait
    let b = await operation2()  // Wait
    let c = await operation3()  // Wait
    return [a, b, c]
}
```

**Concurrent (all at once):**
```astrixa
async fn concurrent() {
    // Start all operations
    let promiseA = operation1()
    let promiseB = operation2()
    let promiseC = operation3()
    
    // Wait for all to complete
    return [await promiseA, await promiseB, await promiseC]
}
```

### Async Loops

```astrixa
async fn processUsers(ids: [int]) {
    for id in ids {
        let user = await fetchUser(id)
        await processUser(user)
    }
}
```

**Concurrent processing:**
```astrixa
async fn processUsersConcurrent(ids: [int]) {
    let promises = []
    
    for id in ids {
        promises.push(fetchUser(id))
    }
    
    let users = await Promise.all(promises)
    
    for user in users {
        await processUser(user)
    }
}
```

## Error Handling in Async

### Try-Catch with Async

```astrixa
async fn safeRequest(url: string) -> string {
    try {
        let response = await http.get(url)
        return response.body
    } catch error {
        print("Request failed: " + error.message)
        return "fallback data"
    }
}
```

### Timeout Handling

```astrixa
async fn requestWithTimeout(url: string, timeoutMs: int) -> string {
    try {
        let response = await http.get(url, { timeout: timeoutMs })
        return response.body
    } catch error {
        if error.type == "timeout" {
            return "Request timed out"
        }
        throw error
    }
}
```

## Async Best Practices

### ✅ Do: Await Only When Needed

```astrixa
// ✅ Good - concurrent
async fn good() {
    let p1 = fetchData1()
    let p2 = fetchData2()
    return [await p1, await p2]
}

// ❌ Slow - sequential
async fn slow() {
    let d1 = await fetchData1()
    let d2 = await fetchData2()
    return [d1, d2]
}
```

### ✅ Do: Handle Errors Properly

```astrixa
async fn fetchWithRetry(url: string, maxRetries: int) -> string {
    let retries = 0
    
    while retries < maxRetries {
        try {
            return await http.get(url)
        } catch error {
            retries = retries + 1
            if retries >= maxRetries {
                throw error
            }
            await sleep(1000)  // Wait before retry
        }
    }
}
```

### ✅ Do: Use Async for I/O Operations

```astrixa
// ✅ Good - async I/O
async fn loadData() {
    let data = await fs.readFile("data.json")
    return json.parse(data)
}

// ❌ Bad - blocking I/O
fn loadDataSync() {
    let data = fs.readFileSync("data.json")  // Blocks entire program!
    return json.parse(data)
}
```

### ❌ Don't: Forget await

```astrixa
// ❌ Bad - returns Promise, not data
async fn wrong() {
    let data = fetchData()  // Missing await!
    print(data)  // Prints "[Promise]"
}

// ✅ Good
async fn correct() {
    let data = await fetchData()
    print(data)  // Prints actual data
}
```

## Advanced Async Patterns

### Promise Utilities (coming soon)

```astrixa
// Wait for first to complete
let fastest = await Promise.race([
    fetchFromServer1(),
    fetchFromServer2()
])

// Wait for all, even if some fail
let results = await Promise.allSettled([
    operation1(),
    operation2(),
    operation3()
])
```

### Async Generators (future)

```astrixa
async fn* streamData() {
    for i in 0..100 {
        yield await fetchPage(i)
    }
}

async fn processStream() {
    for await page in streamData() {
        processPage(page)
    }
}
```

### Debouncing and Throttling (future)

```astrixa
async fn searchWithDebounce(query: string) {
    await debounce(300)  // Wait 300ms
    return await search(query)
}
```

## Async in Smart Contracts

**Note:** Smart contracts are synchronous by design for determinism. Async is used for:
- Testing contracts
- Deploying contracts
- Interacting with contracts from applications

```astrixa
// Application code (async)
async fn deployContract() {
    let contract = await web3.deploy(TokenContract)
    print("Deployed at: " + contract.address)
}

// Contract code (sync)
contract Token {
    fn transfer(to: Address, amount: U256) {
        // Synchronous, deterministic
        balances[tx.sender] -= amount
        balances[to] += amount
    }
}
```

## Performance Considerations

### Concurrency Limits

```astrixa
async fn processWithLimit(items: [string], limit: int) {
    let active = 0
    let results = []
    
    for item in items {
        while active >= limit {
            await sleep(10)
        }
        
        active = active + 1
        let promise = processItem(item)
        promise.then(fn(result) {
            results.push(result)
            active = active - 1
        })
    }
    
    return results
}
```

### Caching Async Results

```astrixa
let cache: map<string, string> = {}

async fn fetchWithCache(url: string) -> string {
    if cache.has(url) {
        return cache[url]
    }
    
    let data = await http.get(url)
    cache[url] = data
    return data
}
```

## Testing Async Code

```astrixa
async fn testFetchUser() {
    let user = await fetchUser(123)
    assert(user.name == "Alice")
}

async fn testTimeout() {
    try {
        await requestWithTimeout("slow-api", 100)
        assert(false, "Should have timed out")
    } catch error {
        assert(error.type == "timeout")
    }
}
```

## Next Steps

- [Error Handling →](errors.md)
- [Modules →](modules.md)
- [Standard Library: net →](../stdlib/net.md)
- [Standard Library: web →](../stdlib/web.md)

---

**Examples:** See [examples/](../../examples/) for complete async applications.
