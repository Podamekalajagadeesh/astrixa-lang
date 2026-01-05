use tower_lsp::lsp_types::*;

pub struct CompletionProvider;

impl CompletionProvider {
    pub fn new() -> Self {
        CompletionProvider
    }

    pub async fn get_completions(&self, _text: &str, position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Get line and column context
        let line = position.line;
        let _character = position.character;

        // STDLIB completions
        let stdlib_modules = vec![
            ("std::io", "Input/Output operations"),
            ("std::fs", "File system operations"),
            ("std::net", "HTTP and WebSocket networking"),
            ("std::json", "JSON parsing and manipulation"),
            ("std::async", "Async/await and concurrency"),
            ("std::crypto", "Cryptography and Web3 operations"),
            ("std::ai", "AI text generation and classification"),
            ("std::web", "Web server framework"),
        ];

        // Check if we're completing a std:: module path
        if line < 100 {
            // Safe bounds check
            // Provide module completions for std::
            for (module, description) in stdlib_modules.iter() {
                completions.push(CompletionItem {
                    label: module.to_string(),
                    kind: Some(CompletionItemKind::MODULE),
                    detail: Some(description.to_string()),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("ASTRIXA stdlib module: {}\n\n{}", module, description),
                    })),
                    insert_text: Some(module.to_string()),
                    ..Default::default()
                });
            }
        }

        // Keywords
        let keywords = vec![
            "fn", "let", "if", "else", "for", "while", "return", "break", "continue",
            "async", "await", "import", "export", "type", "struct", "enum", "match",
        ];

        for kw in keywords {
            completions.push(CompletionItem {
                label: kw.to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                insert_text: Some(kw.to_string()),
                ..Default::default()
            });
        }

        // Common functions from stdlib
        let functions = vec![
            ("print", "Print to stdout", "std::io"),
            ("input", "Read from stdin", "std::io"),
            ("read", "Read file contents", "std::fs"),
            ("write", "Write to file", "std::fs"),
            ("http_get", "Make HTTP GET request", "std::net"),
            ("http_post", "Make HTTP POST request", "std::net"),
            ("parse", "Parse JSON", "std::json"),
            ("stringify", "Convert to JSON", "std::json"),
            ("generate", "Generate text with AI", "std::ai"),
            ("classify", "Classify text with AI", "std::ai"),
            ("sha256", "Hash with SHA256", "std::crypto"),
            ("sign", "Sign data", "std::crypto"),
            ("sleep", "Async sleep", "std::async"),
            ("spawn", "Spawn async task", "std::async"),
        ];

        for (func, desc, module) in functions {
            completions.push(CompletionItem {
                label: func.to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some(format!("({}) {}", module, desc)),
                insert_text: Some(format!("{}()", func)),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }

        completions
    }
}
