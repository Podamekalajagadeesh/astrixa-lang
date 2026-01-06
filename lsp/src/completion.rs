use tower_lsp::lsp_types::*;

/// CompletionProvider for intelligent code completion
/// Provides context-aware suggestions for stdlib, AI ops, Web3 primitives
pub struct CompletionProvider;

impl CompletionProvider {
    pub fn new() -> Self {
        CompletionProvider
    }

    pub async fn get_completions(&self, text: &str, position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        let lines: Vec<&str> = text.lines().collect();
        let current_line = if position.line < lines.len() as u32 {
            lines[position.line as usize]
        } else {
            ""
        };

        let char_pos = position.character as usize;
        let prefix = if char_pos <= current_line.len() {
            &current_line[..char_pos]
        } else {
            current_line
        };

        // Context-aware completions
        if prefix.ends_with("ai.") {
            return self.get_ai_completions();
        }

        if prefix.ends_with("chain.") {
            return self.get_chain_completions();
        }

        if prefix.ends_with("msg.") {
            return self.get_msg_completions();
        }

        if prefix.ends_with("tx.") {
            return self.get_tx_completions();
        }

        if prefix.ends_with("std::") || prefix.contains("std::") {
            return self.get_stdlib_completions();
        }

        // General completions
        completions.extend(self.get_keywords());
        completions.extend(self.get_stdlib_functions());
        completions.extend(self.get_ai_keywords());
        completions.extend(self.get_web3_keywords());
        completions.extend(self.get_types());

        completions
    }

    /// AI-specific completions
    fn get_ai_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "model".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("ai.model(name: string) -> string".to_string()),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: "Load an AI model by name\n\n```astrixa\nlet m = ai.model(\"sentiment\")\n```\n\nAvailable models:\n- `sentiment` - Sentiment analysis\n- `classifier` - Text classification\n- `embedding` - Vector embeddings".to_string(),
                })),
                insert_text: Some("model(\"$1\")".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "infer".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("ai.infer(model, input: string) -> AIResult".to_string()),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: "Run AI inference on text\n\n```astrixa\nlet result = ai.infer(ai.model(\"sentiment\"), \"I love ASTRIXA!\")\nprint(result.label)  // positive\nprint(result.score)  // 0.92\n```".to_string(),
                })),
                insert_text: Some("infer(ai.model(\"$1\"), \"$2\")".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "embed".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("ai.embed(text: string) -> array".to_string()),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: "Generate vector embedding for text\n\n```astrixa\nlet embedding = ai.embed(\"ASTRIXA language\")\nprint(len(embedding))  // 8 dimensions\n```".to_string(),
                })),
                insert_text: Some("embed(\"$1\")".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
            CompletionItem {
                label: "tokenize".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("ai.tokenize(text: string) -> array".to_string()),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: "Split text into tokens\n\n```astrixa\nlet tokens = ai.tokenize(\"Hello world\")\nprint(tokens)  // [\"hello\", \"world\"]\n```".to_string(),
                })),
                insert_text: Some("tokenize(\"$1\")".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            },
        ]
    }

    /// Chain (blockchain) context completions
    fn get_chain_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "id".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("chain.id -> number".to_string()),
                documentation: Some(Documentation::String("Current blockchain chain ID (e.g., 1 for Ethereum mainnet)".to_string())),
                ..Default::default()
            },
            CompletionItem {
                label: "name".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("chain.name -> string".to_string()),
                documentation: Some(Documentation::String("Current blockchain name (e.g., 'ethereum', 'polygon')".to_string())),
                ..Default::default()
            },
        ]
    }

    /// Message context completions
    fn get_msg_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "sender".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("msg.sender -> address".to_string()),
                documentation: Some(Documentation::String("Address of the caller/sender".to_string())),
                ..Default::default()
            },
            CompletionItem {
                label: "value".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("msg.value -> U256".to_string()),
                documentation: Some(Documentation::String("Amount of ETH/token sent with transaction".to_string())),
                ..Default::default()
            },
            CompletionItem {
                label: "data".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("msg.data -> string".to_string()),
                documentation: Some(Documentation::String("Transaction calldata".to_string())),
                ..Default::default()
            },
        ]
    }

    /// Transaction context completions
    fn get_tx_completions(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "hash".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("tx.hash -> string".to_string()),
                documentation: Some(Documentation::String("Current transaction hash".to_string())),
                ..Default::default()
            },
            CompletionItem {
                label: "timestamp".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("tx.timestamp -> number".to_string()),
                documentation: Some(Documentation::String("Block timestamp (Unix time)".to_string())),
                ..Default::default()
            },
        ]
    }

    /// Standard library module completions
    fn get_stdlib_completions(&self) -> Vec<CompletionItem> {
        vec![
            ("io", "Input/Output operations"),
            ("fs", "File system operations"),
            ("net", "HTTP and WebSocket networking"),
            ("json", "JSON parsing and serialization"),
            ("async", "Async/await and concurrency"),
            ("crypto", "Cryptography and Web3 signatures"),
            ("ai", "AI text generation and classification"),
            ("web", "Web server framework"),
        ]
        .into_iter()
        .map(|(module, description)| {
            CompletionItem {
                label: module.to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some(format!("std::{}", module)),
                documentation: Some(Documentation::String(description.to_string())),
                insert_text: Some(module.to_string()),
                ..Default::default()
            }
        })
        .collect()
    }

    /// Language keywords
    fn get_keywords(&self) -> Vec<CompletionItem> {
        vec![
            ("fn", "fn name(params) { }"),
            ("let", "let name = value"),
            ("if", "if condition { }"),
            ("else", "else { }"),
            ("while", "while condition { }"),
            ("for", "for item in collection { }"),
            ("return", "return value"),
            ("break", "break"),
            ("continue", "continue"),
            ("async", "async fn name() { }"),
            ("await", "await expression"),
            ("import", "import \"module\""),
            ("export", "export fn name() { }"),
            ("contract", "contract Name { }"),
            ("type", "type Name { }"),
            ("struct", "struct Name { }"),
            ("enum", "enum Name { }"),
            ("match", "match value { }"),
            ("true", "Boolean true"),
            ("false", "Boolean false"),
        ]
        .into_iter()
        .map(|(kw, snippet)| {
            CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some(snippet.to_string()),
                insert_text: Some(kw.to_string()),
                ..Default::default()
            }
        })
        .collect()
    }

    /// Standard library functions
    fn get_stdlib_functions(&self) -> Vec<CompletionItem> {
        vec![
            ("print", "print(msg)", "Print to stdout", "std::io"),
            ("input", "input(prompt)", "Read from stdin", "std::io"),
            ("read", "read(path)", "Read file contents", "std::fs"),
            ("write", "write(path, content)", "Write to file", "std::fs"),
            ("http_get", "http_get(url)", "HTTP GET request", "std::net"),
            ("http_post", "http_post(url, body)", "HTTP POST", "std::net"),
            ("parse", "parse(json)", "Parse JSON", "std::json"),
            ("stringify", "stringify(obj)", "Convert to JSON", "std::json"),
            ("sha256", "sha256(data)", "SHA-256 hash", "std::crypto"),
            ("sign", "sign(key, message)", "Sign data", "std::crypto"),
            ("sleep", "sleep(ms)", "Async sleep", "std::async"),
            ("spawn", "spawn(task)", "Spawn async task", "std::async"),
            ("len", "len(array)", "Get length", "builtin"),
            ("type", "type(value)", "Get type", "builtin"),
            ("panic", "panic(message)", "Panic with error", "builtin"),
            ("emit", "emit(event, data)", "Emit event", "builtin"),
        ]
        .into_iter()
        .map(|(func, usage, desc, module)| {
            CompletionItem {
                label: func.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(format!("{} - {}", module, usage)),
                documentation: Some(Documentation::String(desc.to_string())),
                insert_text: Some(format!("{}($1)", func)),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            }
        })
        .collect()
    }

    /// AI-related keywords
    fn get_ai_keywords(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "ai".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("AI operations namespace".to_string()),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: "AI operations:\n- `ai.model()` - Load model\n- `ai.infer()` - Run inference\n- `ai.embed()` - Generate embeddings\n- `ai.tokenize()` - Tokenize text".to_string(),
                })),
                insert_text: Some("ai.".to_string()),
                ..Default::default()
            },
        ]
    }

    /// Web3-related keywords
    fn get_web3_keywords(&self) -> Vec<CompletionItem> {
        vec![
            CompletionItem {
                label: "chain".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Blockchain context".to_string()),
                documentation: Some(Documentation::String("Access chain.id and chain.name".to_string())),
                insert_text: Some("chain.".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "msg".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Message context".to_string()),
                documentation: Some(Documentation::String("Access msg.sender, msg.value, msg.data".to_string())),
                insert_text: Some("msg.".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "tx".to_string(),
                kind: Some(CompletionItemKind::MODULE),
                detail: Some("Transaction context".to_string()),
                documentation: Some(Documentation::String("Access tx.hash and tx.timestamp".to_string())),
                insert_text: Some("tx.".to_string()),
                ..Default::default()
            },
        ]
    }

    /// Type completions
    fn get_types(&self) -> Vec<CompletionItem> {
        vec![
            ("string", "String type"),
            ("number", "Number type (i64)"),
            ("bool", "Boolean type"),
            ("array", "Array type"),
            ("address", "Ethereum address"),
            ("U256", "256-bit unsigned integer"),
        ]
        .into_iter()
        .map(|(typ, desc)| {
            CompletionItem {
                label: typ.to_string(),
                kind: Some(CompletionItemKind::TYPE_PARAMETER),
                detail: Some(desc.to_string()),
                insert_text: Some(typ.to_string()),
                ..Default::default()
            }
        })
        .collect()
    }
}

