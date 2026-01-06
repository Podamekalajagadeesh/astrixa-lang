# Standard Library: Web

Build web applications and APIs with ASTRIXA's web module.

## Quick Start

```astrixa
use std::web

server {
    route GET "/" {
        return json({ message: "Hello, ASTRIXA!" })
    }
}

server.run(8080)
```

Visit `http://localhost:8080` to see your API running!

## Server Creation

### Basic Server

```astrixa
use std::web

server {
    route GET "/" {
        return html("<h1>Welcome to ASTRIXA</h1>")
    }
}

server.run(8080)
```

### Server with Configuration

```astrixa
let config = {
    port: 8080,
    host: "0.0.0.0",
    timeout: 30,
    maxBodySize: 1048576  // 1MB
}

server.start(config)
```

## Routing

### HTTP Methods

```astrixa
server {
    route GET "/users" {
        return json(getAllUsers())
    }
    
    route POST "/users" {
        let user = request.body
        saveUser(user)
        return json({ success: true })
    }
    
    route PUT "/users/:id" {
        let id = request.params.id
        updateUser(id, request.body)
        return json({ updated: true })
    }
    
    route DELETE "/users/:id" {
        let id = request.params.id
        deleteUser(id)
        return json({ deleted: true })
    }
}
```

### Route Parameters

```astrixa
server {
    // Simple parameter
    route GET "/users/:id" {
        let userId = request.params.id
        let user = getUser(userId)
        return json(user)
    }
    
    // Multiple parameters
    route GET "/posts/:postId/comments/:commentId" {
        let postId = request.params.postId
        let commentId = request.params.commentId
        return json(getComment(postId, commentId))
    }
    
    // Optional parameters (coming soon)
    route GET "/search/:query?" {
        let query = request.params.query ?? "default"
        return json(search(query))
    }
}
```

### Query Parameters

```astrixa
server {
    route GET "/search" {
        // /search?q=astrixa&limit=10
        let query = request.query.q
        let limit = int(request.query.limit ?? "20")
        
        let results = search(query, limit)
        return json(results)
    }
}
```

### Wildcards

```astrixa
server {
    // Match any path starting with /static/
    route GET "/static/*" {
        let filepath = request.params.wild
        return file("./public/" + filepath)
    }
}
```

## Request Object

### Accessing Request Data

```astrixa
server {
    route POST "/data" {
        // Request properties
        let method = request.method        // "POST"
        let path = request.path            // "/data"
        let headers = request.headers      // Map of headers
        let body = request.body            // Parsed body
        let rawBody = request.rawBody      // Raw string
        
        // Headers
        let contentType = request.headers["Content-Type"]
        let auth = request.headers["Authorization"]
        
        // Cookies
        let sessionId = request.cookies.sessionId
        
        return json({ received: true })
    }
}
```

### JSON Body Parsing

```astrixa
server {
    route POST "/api/users" {
        let userData = request.body  // Automatically parsed JSON
        
        let name = userData.name
        let email = userData.email
        
        // Validation
        if name == null || email == null {
            return json({ error: "Missing fields" }, 400)
        }
        
        createUser(name, email)
        return json({ success: true }, 201)
    }
}
```

### Form Data

```astrixa
server {
    route POST "/submit" {
        // Form data (application/x-www-form-urlencoded)
        let name = request.form.name
        let email = request.form.email
        
        return html("<h1>Thanks, " + name + "!</h1>")
    }
}
```

### File Uploads

```astrixa
server {
    route POST "/upload" {
        let file = request.files.avatar
        
        let filename = file.name
        let size = file.size
        let content = file.content
        
        // Save file
        fs.writeFile("uploads/" + filename, content)
        
        return json({ uploaded: filename })
    }
}
```

## Response Types

### JSON Response

```astrixa
server {
    route GET "/api/data" {
        return json({ 
            message: "Hello",
            timestamp: Date.now(),
            status: "ok"
        })
    }
}
```

### HTML Response

```astrixa
server {
    route GET "/" {
        return html("""
            <!DOCTYPE html>
            <html>
                <head><title>ASTRIXA</title></head>
                <body>
                    <h1>Welcome to ASTRIXA!</h1>
                </body>
            </html>
        """)
    }
}
```

### Plain Text

```astrixa
server {
    route GET "/health" {
        return text("OK")
    }
}
```

### File Response

```astrixa
server {
    route GET "/download/:filename" {
        let filename = request.params.filename
        return file("./files/" + filename)
    }
}
```

### Custom Response

```astrixa
server {
    route GET "/custom" {
        return response({
            status: 200,
            headers: {
                "Content-Type": "application/json",
                "X-Custom-Header": "value"
            },
            body: '{"custom": true}'
        })
    }
}
```

### Status Codes

```astrixa
server {
    route GET "/resource" {
        // Success responses
        return json(data, 200)  // OK
        return json(data, 201)  // Created
        return json(data, 204)  // No Content
        
        // Client errors
        return json({error: "Not found"}, 404)
        return json({error: "Bad request"}, 400)
        return json({error: "Unauthorized"}, 401)
        return json({error: "Forbidden"}, 403)
        
        // Server errors
        return json({error: "Internal error"}, 500)
    }
}
```

## Middleware

### Basic Middleware

```astrixa
// Logger middleware
fn logger(request, next) {
    print(request.method + " " + request.path)
    return next(request)
}

// CORS middleware
fn cors(request, next) {
    let response = next(request)
    response.headers["Access-Control-Allow-Origin"] = "*"
    return response
}

server {
    middleware logger
    middleware cors
    
    route GET "/" {
        return json({ message: "Hello" })
    }
}
```

### Authentication Middleware

```astrixa
fn requireAuth(request, next) {
    let token = request.headers["Authorization"]
    
    if token == null {
        return json({ error: "Unauthorized" }, 401)
    }
    
    let user = verifyToken(token)
    
    if user == null {
        return json({ error: "Invalid token" }, 401)
    }
    
    request.user = user  // Attach user to request
    return next(request)
}

server {
    // Apply to all routes
    middleware requireAuth
    
    route GET "/profile" {
        return json(request.user)
    }
}
```

### Route-Specific Middleware

```astrixa
server {
    // Public route
    route GET "/" {
        return html("<h1>Public Page</h1>")
    }
    
    // Protected routes
    middleware requireAuth
    
    route GET "/dashboard" {
        return html("<h1>Dashboard</h1>")
    }
    
    route GET "/settings" {
        return html("<h1>Settings</h1>")
    }
}
```

## Static Files

```astrixa
server {
    // Serve entire directory
    static "/public" => "./public"
    
    // Multiple static routes
    static "/css" => "./assets/css"
    static "/js" => "./assets/js"
    static "/images" => "./assets/images"
    
    // Other routes
    route GET "/" {
        return html("<h1>Home</h1>")
    }
}
```

## WebSockets (coming soon)

```astrixa
server {
    websocket "/ws" {
        on "connection" {
            print("Client connected")
        }
        
        on "message" (data) {
            print("Received: " + data)
            broadcast(data)  // Send to all clients
        }
        
        on "disconnect" {
            print("Client disconnected")
        }
    }
}
```

## Complete Example: REST API

```astrixa
use std::web
use std::json

// In-memory database
let users = []
let nextId = 1

// Helper functions
fn findUser(id: int) -> map? {
    for user in users {
        if user.id == id {
            return user
        }
    }
    return null
}

// Middleware
fn logger(request, next) {
    print(request.method + " " + request.path)
    return next(request)
}

// Server
server {
    middleware logger
    
    // Get all users
    route GET "/api/users" {
        return json(users)
    }
    
    // Get specific user
    route GET "/api/users/:id" {
        let id = int(request.params.id)
        let user = findUser(id)
        
        if user == null {
            return json({ error: "User not found" }, 404)
        }
        
        return json(user)
    }
    
    // Create user
    route POST "/api/users" {
        let userData = request.body
        
        // Validation
        if userData.name == null || userData.email == null {
            return json({ error: "Name and email required" }, 400)
        }
        
        // Create user
        let newUser = {
            id: nextId,
            name: userData.name,
            email: userData.email,
            created: Date.now()
        }
        
        users.push(newUser)
        nextId = nextId + 1
        
        return json(newUser, 201)
    }
    
    // Update user
    route PUT "/api/users/:id" {
        let id = int(request.params.id)
        let user = findUser(id)
        
        if user == null {
            return json({ error: "User not found" }, 404)
        }
        
        let updates = request.body
        
        if updates.name != null {
            user.name = updates.name
        }
        if updates.email != null {
            user.email = updates.email
        }
        
        return json(user)
    }
    
    // Delete user
    route DELETE "/api/users/:id" {
        let id = int(request.params.id)
        let index = -1
        
        for i in 0..users.length {
            if users[i].id == id {
                index = i
                break
            }
        }
        
        if index == -1 {
            return json({ error: "User not found" }, 404)
        }
        
        users.remove(index)
        return json({ deleted: true })
    }
}

server.run(8080)
print("Server running on http://localhost:8080")
```

## HTTP Client

Make HTTP requests from your application:

```astrixa
use std::net

async fn fetchData() {
    // GET request
    let response = await http.get("https://api.example.com/data")
    print(response.body)
    
    // POST request
    let postResponse = await http.post("https://api.example.com/users", {
        body: json({ name: "Alice", email: "alice@example.com" }),
        headers: {
            "Content-Type": "application/json"
        }
    })
    
    // With error handling
    try {
        let data = await http.get("https://api.example.com/risky")
        print(data.body)
    } catch error {
        print("Request failed: " + error.message)
    }
}
```

## Best Practices

### ✅ Do: Validate Input

```astrixa
route POST "/api/users" {
    let data = request.body
    
    if !isValidEmail(data.email) {
        return json({ error: "Invalid email" }, 400)
    }
    
    if data.name.length < 2 {
        return json({ error: "Name too short" }, 400)
    }
    
    // Process valid data...
}
```

### ✅ Do: Use Proper Status Codes

```astrixa
// 200 OK - successful GET
return json(data, 200)

// 201 Created - successful POST
return json(newResource, 201)

// 204 No Content - successful DELETE
return json(null, 204)

// 400 Bad Request - invalid input
return json({ error: "Invalid data" }, 400)

// 404 Not Found - resource doesn't exist
return json({ error: "Not found" }, 404)

// 500 Internal Server Error - unexpected error
return json({ error: "Server error" }, 500)
```

### ✅ Do: Handle Errors Gracefully

```astrixa
route GET "/api/data" {
    try {
        let data = fetchData()
        return json(data)
    } catch error {
        print("Error: " + error.message)
        return json({ error: "Internal server error" }, 500)
    }
}
```

## Next Steps

- [Web3 Module →](web3.md)
- [AI Module →](ai.md)
- [Standard Library: net →](net.md)

---

**Examples:** See [examples/](../../examples/) for complete web applications.
