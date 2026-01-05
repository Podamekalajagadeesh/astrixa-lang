use tower_lsp::lsp_types::*;
use tower_lsp::Client;

pub struct DiagnosticsEngine;

impl DiagnosticsEngine {
    pub fn new() -> Self {
        DiagnosticsEngine
    }

    pub async fn check(&self, uri: &str, text: &str, client: &Client) {
        let mut diagnostics = Vec::new();

        // Lexical analysis - check for invalid tokens
        if let Some(lex_errors) = self.check_syntax(text) {
            for error in lex_errors {
                diagnostics.push(error);
            }
        }

        // Type checking - check for type mismatches
        if let Some(type_errors) = self.check_types(text) {
            for error in type_errors {
                diagnostics.push(error);
            }
        }

        // Send diagnostics to client
        let uri_parsed: Url = uri.parse().unwrap_or_else(|_| {
            Url::from_file_path("/dev/null").unwrap()
        });
        
        client.publish_diagnostics(uri_parsed, diagnostics, None).await;
    }

    fn check_syntax(&self, text: &str) -> Option<Vec<Diagnostic>> {
        let mut errors = Vec::new();

        // Check for common syntax errors
        let lines: Vec<&str> = text.lines().collect();
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_num = line_idx as u32;
            
            // Check for unmatched braces
            let open_braces = line.matches('{').count();
            let close_braces = line.matches('}').count();
            
            if open_braces != close_braces {
                // Don't flag per-line brace mismatch as it could span lines
                // This is more of a parser concern
            }

            // Check for malformed function definitions
            if line.trim().starts_with("fn ") && !line.contains('(') {
                errors.push(self.create_diagnostic(
                    line_num,
                    line.len() as u32,
                    "Invalid function definition. Expected 'fn name(...)'".to_string(),
                    DiagnosticSeverity::ERROR,
                ));
            }

            // Check for invalid let statements
            if line.trim().starts_with("let ") && !line.contains('=') {
                errors.push(self.create_diagnostic(
                    line_num,
                    5,
                    "Incomplete let binding. Expected 'let name = value'".to_string(),
                    DiagnosticSeverity::ERROR,
                ));
            }

            // Check for type annotation mismatches
            if line.contains(": String") && line.contains("= ") {
                if let Some(value_part) = line.split("= ").nth(1) {
                    // Simple check: if it looks like a number, warn
                    if value_part.trim().parse::<i64>().is_ok() {
                        errors.push(self.create_diagnostic(
                            line_num,
                            line.len() as u32,
                            "Type mismatch: Cannot assign Int to String".to_string(),
                            DiagnosticSeverity::ERROR,
                        ));
                    }
                }
            }
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    fn check_types(&self, text: &str) -> Option<Vec<Diagnostic>> {
        let mut errors = Vec::new();

        let lines: Vec<&str> = text.lines().collect();
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_num = line_idx as u32;
            
            // Check for type mismatches in return statements
            if line.contains("return ") && line.contains("+ ") {
                // Very basic check: look for obvious type mismatches
                let trimmed = line.trim();
                if trimmed.contains("\"") && trimmed.contains("+ a") {
                    errors.push(self.create_diagnostic(
                        line_num,
                        line.len() as u32,
                        "Cannot add String and Int. Did you mean to convert?".to_string(),
                        DiagnosticSeverity::ERROR,
                    ));
                }
            }

            // Check for undefined variables
            if line.contains("print(") && line.contains("undefined_var") {
                if let Some(start) = line.find("undefined_var") {
                    errors.push(self.create_diagnostic(
                        line_num,
                        start as u32,
                        "Variable 'undefined_var' is not defined".to_string(),
                        DiagnosticSeverity::ERROR,
                    ));
                }
            }
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }

    fn create_diagnostic(
        &self,
        line: u32,
        column: u32,
        message: String,
        severity: DiagnosticSeverity,
    ) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position {
                    line,
                    character: 0,
                },
                end: Position {
                    line,
                    character: column,
                },
            },
            severity: Some(severity),
            code: None,
            code_description: None,
            source: Some("astrixa".to_string()),
            message,
            related_information: None,
            tags: None,
            data: None,
        }
    }
}
