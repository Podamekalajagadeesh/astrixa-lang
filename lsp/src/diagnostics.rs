use tower_lsp::lsp_types::*;
use tower_lsp::Client;
use std::collections::HashSet;

/// DiagnosticsEngine provides human-friendly, actionable error messages
/// Philosophy: Errors should be calm, clear, and helpful
pub struct DiagnosticsEngine;

impl DiagnosticsEngine {
    pub fn new() -> Self {
        DiagnosticsEngine
    }

    /// Check document for errors and send diagnostics to client
    pub async fn check(&self, uri: &str, text: &str, client: &Client) {
        let mut diagnostics = Vec::new();

        // Lexical and syntax analysis
        if let Some(syntax_errors) = self.check_syntax(text) {
            diagnostics.extend(syntax_errors);
        }

        // Semantic analysis - type checking and undefined variables
        if let Some(semantic_errors) = self.check_semantics(text) {
            diagnostics.extend(semantic_errors);
        }

        // Best practices and warnings
        if let Some(warnings) = self.check_best_practices(text) {
            diagnostics.extend(warnings);
        }

        // Send diagnostics to client
        let uri_parsed: Url = uri.parse().unwrap_or_else(|_| {
            Url::from_file_path("/dev/null").unwrap()
        });
        
        client.publish_diagnostics(uri_parsed, diagnostics, None).await;
    }

    /// Check for syntax errors with helpful messages
    fn check_syntax(&self, text: &str) -> Option<Vec<Diagnostic>> {
        let mut errors = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_num = line_idx as u32;
            let trimmed = line.trim();
            
            // Malformed function definitions
            if trimmed.starts_with("fn ") {
                if !trimmed.contains('(') {
                    let col = line.find("fn ").unwrap_or(0) as u32;
                    errors.push(self.create_diagnostic(
                        line_num,
                        col,
                        line.len() as u32,
                        "Function needs parentheses. Try: fn name(...) { }".to_string(),
                        DiagnosticSeverity::ERROR,
                    ));
                } else if trimmed.contains('(') && !trimmed.contains(')') && !trimmed.ends_with(',') {
                    let col = line.find('(').unwrap_or(0) as u32;
                    errors.push(self.create_diagnostic(
                        line_num,
                        col,
                        line.len() as u32,
                        "Missing closing parenthesis ')' in function definition".to_string(),
                        DiagnosticSeverity::ERROR,
                    ));
                }
            }

            // Incomplete let statements
            if trimmed.starts_with("let ") && !trimmed.contains('=') && !line_idx + 1 < lines.len() {
                let col = line.find("let ").unwrap_or(0) as u32;
                errors.push(self.create_diagnostic(
                    line_num,
                    col,
                    5,
                    "Let binding needs assignment. Try: let name = value".to_string(),
                    DiagnosticSeverity::ERROR,
                ));
            }

            // Type annotation mismatches
            if trimmed.contains(": string") && trimmed.contains("= ") {
                if let Some(value_part) = trimmed.split("= ").nth(1) {
                    let value_clean = value_part.trim().trim_end_matches(';');
                    if value_clean.parse::<i64>().is_ok() {
                        let col = line.find(value_clean).unwrap_or(0) as u32;
                        errors.push(self.create_diagnostic(
                            line_num,
                            col,
                            value_clean.len() as u32,
                            "Cannot assign number to string. Remove ': string' or wrap in quotes".to_string(),
                            DiagnosticSeverity::ERROR,
                        ));
                    }
                }
            }

            // Missing semicolons in certain contexts
            if (trimmed.starts_with("let ") || trimmed.starts_with("return ")) 
                && !trimmed.ends_with(';') 
                && !trimmed.ends_with('{') 
                && !trimmed.ends_with('}') 
                && !trimmed.is_empty() {
                // This is a warning, not an error (ASTRIXA may allow optional semicolons)
                errors.push(self.create_diagnostic(
                    line_num,
                    line.len() as u32,
                    line.len() as u32,
                    "Consider adding semicolon for clarity".to_string(),
                    DiagnosticSeverity::HINT,
                ));
            }

            // Unmatched quotes
            let quote_count = trimmed.matches('"').count();
            if quote_count % 2 != 0 {
                if let Some(col) = line.rfind('"') {
                    errors.push(self.create_diagnostic(
                        line_num,
                        col as u32,
                        (col + 1) as u32,
                        "Unclosed string literal. Missing closing quote".to_string(),
                        DiagnosticSeverity::ERROR,
                    ));
                }
            }

            // Common typos in keywords
            if trimmed.starts_with("fucntion ") || trimmed.starts_with("funct ") {
                errors.push(self.create_diagnostic(
                    line_num,
                    0,
                    8,
                    "Did you mean 'fn'? ASTRIXA uses 'fn' for functions".to_string(),
                    DiagnosticSeverity::ERROR,
                ));
            }

            if trimmed.contains(" var ") && !trimmed.starts_with("//") {
                if let Some(col) = line.find(" var ") {
                    errors.push(self.create_diagnostic(
                        line_num,
                        col as u32,
                        (col + 4) as u32,
                        "Use 'let' instead of 'var' in ASTRIXA".to_string(),
                        DiagnosticSeverity::WARNING,
                    ));
                }
            }
        }

        if errors.is_empty() { None } else { Some(errors) }
    }

    /// Check for semantic errors (types, undefined variables)
    fn check_semantics(&self, text: &str) -> Option<Vec<Diagnostic>> {
        let mut errors = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        
        // Track defined variables
        let mut defined_vars: HashSet<String> = HashSet::new();
        defined_vars.insert("chain".to_string());
        defined_vars.insert("msg".to_string());
        defined_vars.insert("tx".to_string());
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_num = line_idx as u32;
            let trimmed = line.trim();
            
            // Track let bindings
            if trimmed.starts_with("let ") {
                if let Some(name_part) = trimmed.strip_prefix("let ") {
                    if let Some(name) = name_part.split_whitespace().next() {
                        defined_vars.insert(name.trim_end_matches(':').to_string());
                    }
                }
            }

            // Track function parameters
            if trimmed.starts_with("fn ") {
                if let Some(params_start) = trimmed.find('(') {
                    if let Some(params_end) = trimmed.find(')') {
                        let params = &trimmed[params_start + 1..params_end];
                        for param in params.split(',') {
                            if let Some(name) = param.trim().split(':').next() {
                                if !name.is_empty() {
                                    defined_vars.insert(name.trim().to_string());
                                }
                            }
                        }
                    }
                }
            }

            // Type mismatch: String + Int
            if trimmed.contains(" + ") {
                let parts: Vec<&str> = trimmed.split(" + ").collect();
                if parts.len() >= 2 {
                    let left_has_quote = parts[0].contains('"');
                    let right_has_quote = parts[1].contains('"');
                    
                    if left_has_quote != right_has_quote {
                        if let Some(col) = line.find(" + ") {
                            errors.push(self.create_diagnostic(
                                line_num,
                                col as u32,
                                (col + 3) as u32,
                                "Cannot mix strings and numbers. Convert one to match the other".to_string(),
                                DiagnosticSeverity::ERROR,
                            ));
                        }
                    }
                }
            }

            // Division by zero check
            if trimmed.contains(" / 0") {
                if let Some(col) = line.find(" / 0") {
                    errors.push(self.create_diagnostic(
                        line_num,
                        col as u32 + 3,
                        col as u32 + 4,
                        "Division by zero will cause a runtime error".to_string(),
                        DiagnosticSeverity::WARNING,
                    ));
                }
            }

            // AI operations validation
            if trimmed.contains("ai.infer") && !trimmed.contains("ai.model(") {
                if let Some(col) = line.find("ai.infer") {
                    errors.push(self.create_diagnostic(
                        line_num,
                        col as u32,
                        (col + 8) as u32,
                        "ai.infer() needs a model. Use ai.infer(ai.model(\"name\"), text)".to_string(),
                        DiagnosticSeverity::ERROR,
                    ));
                }
            }
        }

        if errors.is_empty() { None } else { Some(errors) }
    }

    /// Check for best practices and provide helpful hints
    fn check_best_practices(&self, text: &str) -> Option<Vec<Diagnostic>> {
        let mut warnings = Vec::new();
        let lines: Vec<&str> = text.lines().collect();
        
        for (line_idx, line) in lines.iter().enumerate() {
            let line_num = line_idx as u32;
            let trimmed = line.trim();
            
            // Unused imports
            if trimmed.starts_with("import ") {
                // Future enhancement: track import usage for unused import hints
            }

            // Long function detection
            if trimmed.starts_with("fn ") {
                let mut fn_lines = 0;
                for i in line_idx..lines.len() {
                    fn_lines += 1;
                    if lines[i].trim() == "}" {
                        break;
                    }
                }
                if fn_lines > 50 {
                    warnings.push(self.create_diagnostic(
                        line_num,
                        0,
                        line.len() as u32,
                        "Consider breaking this function into smaller pieces".to_string(),
                        DiagnosticSeverity::HINT,
                    ));
                }
            }

            // Naming conventions
            if trimmed.starts_with("fn ") {
                if let Some(name_start) = trimmed.find("fn ") {
                    if let Some(name_end) = trimmed.find('(') {
                        let fn_name = &trimmed[name_start + 3..name_end].trim();
                        if fn_name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                            warnings.push(self.create_diagnostic(
                                line_num,
                                (name_start + 3) as u32,
                                name_end as u32,
                                "Function names should start with lowercase in ASTRIXA".to_string(),
                                DiagnosticSeverity::HINT,
                            ));
                        }
                    }
                }
            }
        }

        if warnings.is_empty() { None } else { Some(warnings) }
    }

    /// Create a diagnostic with proper range
    fn create_diagnostic(
        &self,
        line: u32,
        start_char: u32,
        end_char: u32,
        message: String,
        severity: DiagnosticSeverity,
    ) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position {
                    line,
                    character: start_char,
                },
                end: Position {
                    line,
                    character: end_char,
                },
            },
            severity: Some(severity),
            message,
            source: Some("astrixa".to_string()),
            ..Default::default()
        }
    }
}
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
