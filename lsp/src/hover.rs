use tower_lsp::lsp_types::*;

/// HoverProvider provides rich documentation on hover
/// Shows function signatures, types, and usage examples
pub struct HoverProvider;

impl HoverProvider {
    pub fn new() -> Self {
        HoverProvider
    }

    pub async fn get_hover(&self, text: &str, position: Position) -> Option<Hover> {
        let lines: Vec<&str> = text.lines().collect();
        
        if position.line as usize >= lines.len() {
            return None;
        }

        let line = lines[position.line as usize];
        let character = position.character as usize;

        // Find the word at cursor position
        if let Some(word) = self.get_word_at_position(line, character) {
            self.get_hover_for_word(&word)
        } else {
            None
        }
    }

    fn get_word_at_position(&self, line: &str, pos: usize) -> Option<String> {
        if pos > line.len() {
            return None;
        }

        let mut start = pos;
        let mut end = pos;

        // Expand left to find word start
        while start > 0 {
            let ch = line.chars().nth(start - 1)?;
            if ch.is_alphanumeric() || ch == '_' || ch == ':' {
                start -= 1;
            } else {
                break;
            }
        }

        // Expand right to find word end
        while end < line.len() {
            let ch = line.chars().nth(end)?;
            if ch.is_alphanumeric() || ch == '_' || ch == ':' {
                end += 1;
            } else {
                break;
            }
        }

        if start < end {
            Some(line[start..end].to_string())
        } else {
            None
        }
    }

    fn get_hover_for_word(&self, word: &str) -> Option<Hover> {
        let hover_info = match word {
            // AI Operations
            "model" if word.contains("ai") => {
                "```astrixa\nai.model(name: string) -> string\n```\n\n\
                Load an AI model by name.\n\n\
                **Available Models:**\n\
                - `sentiment` - Sentiment analysis\n\
                - `classifier` - Text classification\n\
                - `embedding` - Vector embeddings\n\n\
                **Example:**\n\
                ```astrixa\n\
                let m = ai.model(\"sentiment\")\n\
                let result = ai.infer(m, \"I love ASTRIXA!\")\n\
                ```"
            }
            "infer" if word.contains("ai") => {
                "```astrixa\nai.infer(model: string, input: string) -> AIResult\n```\n\n\
                Run AI inference on text input.\n\n\
                **Returns:** `AIResult` with `label` and `score` properties\n\n\
                **Example:**\n\
                ```astrixa\n\
                let result = ai.infer(ai.model(\"sentiment\"), \"Great!\");\n\
                print(result.label);  // positive\n\
                print(result.score);  // 0.92\n\
                ```"
            }
            "embed" if word.contains("ai") => {
                "```astrixa\nai.embed(text: string) -> array\n```\n\n\
                Generate vector embedding for text.\n\n\
                **Returns:** Array of numbers (8-dimensional vector)\n\n\
                **Example:**\n\
                ```astrixa\n\
                let embedding = ai.embed(\"ASTRIXA language\");\n\
                print(len(embedding));  // 8\n\
                ```"
            }
            "tokenize" if word.contains("ai") => {
                "```astrixa\nai.tokenize(text: string) -> array\n```\n\n\
                Split text into tokens.\n\n\
                **Example:**\n\
                ```astrixa\n\
                let tokens = ai.tokenize(\"Hello world\");\n\
                print(tokens);  // [\"hello\", \"world\"]\n\
                ```"
            }

            // Built-in functions
            "print" => {
                "```astrixa\nfn print(msg: any) -> ()\n```\n\n\
                Print value to stdout.\n\n\
                **Supports:** strings, numbers, booleans, arrays, AIResult\n\n\
                **Example:**\n\
                ```astrixa\n\
                print(\"Hello, ASTRIXA!\");\n\
                print(42);\n\
                print([1, 2, 3]);\n\
                ```"
            }
            "len" => {
                "```astrixa\nfn len(collection: array | string) -> number\n```\n\n\
                Get length of array or string.\n\n\
                **Example:**\n\
                ```astrixa\n\
                let arr = [1, 2, 3];\n\
                print(len(arr));  // 3\n\n\
                let text = \"hello\";\n\
                print(len(text));  // 5\n\
                ```"
            }
            "type" => {
                "```astrixa\nfn type(value: any) -> string\n```\n\n\
                Get type of value as string.\n\n\
                **Returns:** \"string\", \"number\", \"bool\", \"array\", \"ai_result\", etc.\n\n\
                **Example:**\n\
                ```astrixa\n\
                print(type(42));        // number\n\
                print(type(\"hello\"));  // string\n\
                print(type([1,2,3]));   // array\n\
                ```"
            }
            "panic" => {
                "```astrixa\nfn panic(message: string) -> !\n```\n\n\
                Panic with error message (never returns).\n\n\
                **Use in contracts to revert state on invalid conditions.**\n\n\
                **Example:**\n\
                ```astrixa\n\
                if balance < amount {\n\
                    panic(\"Insufficient balance\");\n\
                }\n\
                ```"
            }
            "emit" => {
                "```astrixa\nfn emit(event: string, data: any) -> ()\n```\n\n\
                Emit blockchain event (for contracts).\n\n\
                **Example:**\n\
                ```astrixa\n\
                emit(\"Transfer\", {from: sender, to: recipient, amount: 100});\n\
                ```"
            }

            // File operations
            "read" => {
                "```astrixa\nfn read(path: string) -> string\n```\n\n\
                Read entire file contents as string.\n\n\
                **Example:**\n\
                ```astrixa\n\
                let content = read(\"config.json\");\n\
                let data = parse(content);\n\
                ```"
            }
            "write" => {
                "```astrixa\nfn write(path: string, content: string) -> ()\n```\n\n\
                Write content to file.\n\n\
                **Example:**\n\
                ```astrixa\n\
                write(\"output.txt\", \"Hello, ASTRIXA!\");\n\
                ```"
            }

            // Network operations
            "http_get" => {
                "```astrixa\nfn http_get(url: string) -> Response\n```\n\n\
                Make HTTP GET request.\n\n\
                **Example:**\n\
                ```astrixa\n\
                let res = http_get(\"https://api.example.com/data\");\n\
                if res.is_ok() {\n\
                    let data = res.json();\n\
                }\n\
                ```"
            }
            "http_post" => {
                "```astrixa\nfn http_post(url: string, body: string) -> Response\n```\n\n\
                Make HTTP POST request.\n\n\
                **Example:**\n\
                ```astrixa\n\
                let body = stringify({name: \"Alice\"});\n\
                let res = http_post(\"https://api.example.com/users\", body);\n\
                ```"
            }

            // JSON operations
            "parse" => {
                "```astrixa\nfn parse(json: string) -> object\n```\n\n\
                Parse JSON string to object. Panics on invalid JSON.\n\n\
                **Example:**\n\
                ```astrixa\n\
                let data = parse('{\"name\": \"Alice\", \"age\": 30}');\n\
                print(data.name);\n\
                ```"
            }
            "stringify" => {
                "```astrixa\nfn stringify(obj: object) -> string\n```\n\n\
                Convert object to JSON string.\n\n\
                **Example:**\n\
                ```astrixa\n\
                let json = stringify({name: \"Bob\", active: true});\n\
                write(\"data.json\", json);\n\
                ```"
            }

            // Crypto operations
            "sha256" => {
                "```astrixa\nfn sha256(data: string | bytes) -> string\n```\n\n\
                Hash data with SHA-256 (Web3 standard).\n\n\
                **Returns:** Hex string\n\n\
                **Example:**\n\
                ```astrixa\n\
                let hash = sha256(\"password\");\n\
                print(hash);  // hex string\n\
                ```"
            }
            "sign" => {
                "```astrixa\nfn sign(private_key: string, message: string) -> Signature\n```\n\n\
                Sign message with private key (ECDSA).\n\n\
                **Example:**\n\
                ```astrixa\n\
                let (pub, priv) = generate_keypair();\n\
                let sig = sign(priv, \"message\");\n\
                ```"
            }

            // Async operations  
            "sleep" => {
                "```astrixa\nasync fn sleep(ms: number) -> ()\n```\n\n\
                Asynchronously sleep for milliseconds.\n\n\
                **Example:**\n\
                ```astrixa\n\
                async fn delayed_task() {\n\
                    await sleep(1000);  // Wait 1 second\n\
                    print(\"Done\");\n\
                }\n\
                ```"
            }
            "spawn" => {
                "```astrixa\nfn spawn(task: async fn) -> Task\n```\n\n\
                Spawn async task. Returns immediately.\n\n\
                **Example:**\n\
                ```astrixa\n\
                let task = spawn(fn() {\n\
                    // Work happens in background\n\
                });\n\
                ```"
            }

            // Keywords
            "fn" => {
                "**fn** - Function definition\n\n\
                Define a named function.\n\n\
                **Syntax:**\n\
                ```astrixa\n\
                fn name(param1: type, param2: type) -> return_type {\n\
                    // body\n\
                }\n\
                ```\n\n\
                **Example:**\n\
                ```astrixa\n\
                fn add(a: number, b: number) -> number {\n\
                    return a + b;\n\
                }\n\
                ```"
            }
            "let" => {
                "**let** - Variable binding\n\n\
                Bind a value to a name.\n\n\
                **Example:**\n\
                ```astrixa\n\
                let x = 42;\n\
                let name: string = \"Alice\";\n\
                let result = add(10, 20);\n\
                ```"
            }
            "if" => {
                "**if** - Conditional branch\n\n\
                Execute code based on condition.\n\n\
                **Example:**\n\
                ```astrixa\n\
                if x > 0 {\n\
                    print(\"positive\");\n\
                } else if x < 0 {\n\
                    print(\"negative\");\n\
                } else {\n\
                    print(\"zero\");\n\
                }\n\
                ```"
            }
            "while" => {
                "**while** - Loop while condition is true\n\n\
                **Example:**\n\
                ```astrixa\n\
                let i = 0;\n\
                while i < 10 {\n\
                    print(i);\n\
                    i = i + 1;\n\
                }\n\
                ```"
            }
            "return" => {
                "**return** - Return value from function\n\n\
                **Example:**\n\
                ```astrixa\n\
                fn square(x: number) -> number {\n\
                    return x * x;\n\
                }\n\
                ```"
            }
            "async" => {
                "**async** - Mark function as asynchronous\n\n\
                **Example:**\n\
                ```astrixa\n\
                async fn fetch_data() {\n\
                    let res = await http_get(url);\n\
                    return res.json();\n\
                }\n\
                ```"
            }
            "await" => {
                "**await** - Wait for async result\n\n\
                **Example:**\n\
                ```astrixa\n\
                let data = await fetch_data();\n\
                print(data);\n\
                ```"
            }
            "contract" => {
                "**contract** - Define smart contract\n\n\
                **Example:**\n\
                ```astrixa\n\
                contract Token {\n\
                    state: [\"balances\", \"total_supply\"]\n\n\
                    fn transfer(to: address, amount: number) {\n\
                        // Transfer logic\n\
                    }\n\
                }\n\
                ```"
            }
            "import" => {
                "**import** - Import module or package\n\n\
                **Example:**\n\
                ```astrixa\n\
                import \"math\";\n\
                import \"ai-tools\";\n\n\
                let result = add(5, 3);\n\
                ```"
            }

            // Web3 context
            "chain" => {
                "**chain** - Blockchain context\n\n\
                Access current blockchain information.\n\n\
                **Properties:**\n\
                - `chain.id` - Chain ID (number)\n\
                - `chain.name` - Chain name (string)\n\n\
                **Example:**\n\
                ```astrixa\n\
                if chain.id == 1 {\n\
                    print(\"Ethereum mainnet\");\n\
                }\n\
                ```"
            }
            "msg" => {
                "**msg** - Message context\n\n\
                Access transaction sender and value.\n\n\
                **Properties:**\n\
                - `msg.sender` - Caller address\n\
                - `msg.value` - ETH sent (U256)\n\
                - `msg.data` - Transaction data\n\n\
                **Example:**\n\
                ```astrixa\n\
                if msg.sender == owner {\n\
                    // Owner-only action\n\
                }\n\
                ```"
            }
            "tx" => {
                "**tx** - Transaction context\n\n\
                Access current transaction information.\n\n\
                **Properties:**\n\
                - `tx.hash` - Transaction hash\n\
                - `tx.timestamp` - Block timestamp\n\n\
                **Example:**\n\
                ```astrixa\n\
                emit(\"Action\", {hash: tx.hash, time: tx.timestamp});\n\
                ```"
            }

            _ => return None,
        };

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: hover_info.to_string(),
            }),
            range: None,
        })
    }
}

            // STDLIB functions
            "print" => {
                "**print**(msg: String) -> ()\n\nPrints text to stdout.\n\n```ax\nprint(\"Hello, World!\")\n```"
            }
            "input" => {
                "**input**(prompt: String) -> String\n\nReads a line from stdin.\n\n```ax\nlet name = input(\"Enter name: \")\n```"
            }
            "read" => {
                "**read**(path: String) -> String\n\nReads entire file contents.\n\n```ax\nlet content = read(\"file.txt\")\n```"
            }
            "write" => {
                "**write**(path: String, content: String) -> ()\n\nWrites content to file.\n\n```ax\nwrite(\"out.txt\", \"Hello\")\n```"
            }
            "http_get" => {
                "**http_get**(url: String) -> Response\n\nMakes an HTTP GET request.\n\n```ax\nlet res = http_get(\"https://api.example.com/data\")\nif res.is_ok() { let data = res.json() }\n```"
            }
            "http_post" => {
                "**http_post**(url: String, body: String) -> Response\n\nMakes an HTTP POST request.\n\n```ax\nlet res = http_post(\"https://api.example.com/users\", body_json)\n```"
            }
            "parse" => {
                "**parse**(text: String) -> Object\n\nParse JSON string to object. Panics on invalid JSON.\n\n```ax\nlet data = parse(json_str)\n```"
            }
            "stringify" => {
                "**stringify**(obj: Object) -> String\n\nConvert object to JSON string.\n\n```ax\nlet json = stringify({name: \"John\", age: 30})\n```"
            }
            "generate" => {
                "**generate**(prompt: String) -> String\n\nGenerate text using AI. First-class ASTRIXA primitive.\n\n```ax\nlet poem = generate(\"Write a haiku about programming\")\n```"
            }
            "classify" => {
                "**classify**(text: String, categories: [String]) -> String\n\nClassify text into categories using AI.\n\n```ax\nlet sentiment = classify(text, [\"positive\", \"negative\", \"neutral\"])\n```"
            }
            "analyze_sentiment" => {
                "**analyze_sentiment**(text: String) -> SentimentResult\n\nAnalyze sentiment of text.\n\n```ax\nlet result = analyze_sentiment(\"I love ASTRIXA!\")\nif result.label == \"positive\" { }\n```"
            }
            "sha256" => {
                "**sha256**(data: [Byte]) -> String\n\nHash data with SHA256.\n\n```ax\nlet hash = sha256(\"password\")\n```"
            }
            "sign" => {
                "**sign**(private_key: String, message: String) -> Signature\n\nSign message with private key. Web3 ready.\n\n```ax\nlet (pub, priv) = generate_keypair()\nlet sig = sign(priv, \"message\")\n```"
            }
            "sleep" => {
                "**async fn sleep**(ms: Int) -> ()\n\nAsynchronously sleep for milliseconds.\n\n```ax\nawait sleep(1000)\n```"
            }
            "spawn" => {
                "**spawn**(task: async Fn) -> Task\n\nSpawn async task. Returns immediately.\n\n```ax\nlet task = spawn(fn() { work() })\n```"
            }
            // Keywords
            "fn" => "**fn** - Define function\n\n```ax\nfn add(a: Int, b: Int) -> Int {\n    return a + b\n}\n```",
            "let" => "**let** - Bind variable\n\n```ax\nlet x = 42\nlet name: String = \"John\"\n```",
            "async" => "**async** - Mark function as asynchronous\n\n```ax\nasync fn fetch_data() {\n    let res = await http_get(url)\n}\n```",
            "await" => "**await** - Wait for async result\n\n```ax\nlet data = await fetch_data()\n```",
            "if" => "**if** - Conditional branch\n\n```ax\nif x > 0 {\n    print(\"positive\")\n}\n```",
            "else" => "**else** - Alternative branch\n\n```ax\nif x > 0 {\n    print(\"positive\")\n} else {\n    print(\"not positive\")\n}\n```",
            // Modules
            "io" => "**std::io** - Input/Output\n\nCore I/O operations: print, input, format\n\n- `print(msg)` - Print to stdout\n- `input(prompt)` - Read from stdin\n- `format(template, args)` - Format string",
            "fs" => "**std::fs** - File System\n\nFile and directory operations: read, write, list_dir, exists, mkdir\n\n- `read(path)` - Read file\n- `write(path, content)` - Write file\n- `exists(path)` - Check if file exists",
            "net" => "**std::net** - Network\n\nHTTP and WebSocket: http_get, http_post, fetch, Server\n\n- `http_get(url)` - GET request\n- `http_post(url, body)` - POST request\n- `Server::new()` - Create server",
            "json" => "**std::json** - JSON\n\nJSON parsing: parse, stringify, try_parse, validate\n\n- `parse(text)` - Parse JSON\n- `stringify(obj)` - JSON string\n- `try_parse(text)` - Safe parse",
            "async" => "**std::async** - Async\n\nAsynchronous operations: sleep, spawn, channel, join_all\n\n- `sleep(ms)` - Async sleep\n- `spawn(task)` - Spawn task\n- `channel()` - Create channel",
            "crypto" => "**std::crypto** - Cryptography\n\nHashingand signing: sha256, sign, verify, encrypt, decrypt\n\n- `sha256(data)` - Hash\n- `sign(key, msg)` - Sign data\n- `verify(key, msg, sig)` - Verify signature",
            "ai" => "**std::ai** - AI Operations\n\nText generation and classification: generate, classify, embed, sentiment\n\n- `generate(prompt)` - Generate text\n- `classify(text, cats)` - Classify\n- `embed(text)` - Get embedding",
            "web" => "**std::web** - Web Framework\n\nWeb server: Server, Router, middleware\n\n- `Server::new()` - Create server\n- `server.get(path, handler)` - GET route\n- `server.listen(port)` - Start server",
            _ => return None,
        };

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: hover_info.to_string(),
            }),
            range: None,
        })
    }
}
