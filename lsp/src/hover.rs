use tower_lsp::lsp_types::*;

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
            self.get_hover_for_word(word)
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
        while start > 0 && (line.chars().nth(start - 1).unwrap().is_alphanumeric() || 
                           line.chars().nth(start - 1).unwrap() == '_' ||
                           line.chars().nth(start - 1).unwrap() == ':') {
            start -= 1;
        }

        // Expand right to find word end
        while end < line.len() && (line.chars().nth(end).unwrap().is_alphanumeric() || 
                                   line.chars().nth(end).unwrap() == '_' ||
                                   line.chars().nth(end).unwrap() == ':') {
            end += 1;
        }

        if start < end {
            Some(line[start..end].to_string())
        } else {
            None
        }
    }

    fn get_hover_for_word(&self, word: String) -> Option<Hover> {
        let hover_info = match word.as_str() {
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
