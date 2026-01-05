use tower_lsp::lsp_types::*;
use std::collections::HashMap;

pub struct SymbolProvider;

impl SymbolProvider {
    pub fn new() -> Self {
        SymbolProvider
    }

    pub async fn get_definition_location(&self, text: &str, position: Position) -> Option<Location> {
        let lines: Vec<&str> = text.lines().collect();
        
        if position.line as usize >= lines.len() {
            return None;
        }

        let line = lines[position.line as usize];
        let character = position.character as usize;

        // Find word at cursor
        if let Some(word) = self.get_word_at_position(line, character) {
            // Find definition of this word in the document
            self.find_definition(&word, &lines)
        } else {
            None
        }
    }

    pub async fn get_document_symbols(&self, text: &str) -> Vec<SymbolInformation> {
        let mut symbols = Vec::new();
        let lines: Vec<&str> = text.lines().collect();

        for (line_idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Find function definitions
            if trimmed.starts_with("fn ") {
                if let Some(name) = self.extract_function_name(trimmed) {
                    symbols.push(SymbolInformation {
                        name,
                        kind: SymbolKind::FUNCTION,
                        location: Location {
                            uri: Url::from_file_path("/dev/null").unwrap(),
                            range: Range {
                                start: Position {
                                    line: line_idx as u32,
                                    character: 0,
                                },
                                end: Position {
                                    line: line_idx as u32,
                                    character: line.len() as u32,
                                },
                            },
                        },
                        container_name: None,
                        deprecated: None,
                        tags: None,
                    });
                }
            }

            // Find variable definitions
            if trimmed.starts_with("let ") {
                if let Some(name) = self.extract_variable_name(trimmed) {
                    symbols.push(SymbolInformation {
                        name,
                        kind: SymbolKind::VARIABLE,
                        location: Location {
                            uri: Url::from_file_path("/dev/null").unwrap(),
                            range: Range {
                                start: Position {
                                    line: line_idx as u32,
                                    character: 0,
                                },
                                end: Position {
                                    line: line_idx as u32,
                                    character: line.len() as u32,
                                },
                            },
                        },
                        container_name: None,
                        deprecated: None,
                        tags: None,
                    });
                }
            }

            // Find type/struct definitions
            if trimmed.starts_with("type ") || trimmed.starts_with("struct ") {
                if let Some(name) = self.extract_type_name(trimmed) {
                    symbols.push(SymbolInformation {
                        name,
                        kind: SymbolKind::CLASS,
                        location: Location {
                            uri: Url::from_file_path("/dev/null").unwrap(),
                            range: Range {
                                start: Position {
                                    line: line_idx as u32,
                                    character: 0,
                                },
                                end: Position {
                                    line: line_idx as u32,
                                    character: line.len() as u32,
                                },
                            },
                        },
                        container_name: None,
                        deprecated: None,
                        tags: None,
                    });
                }
            }
        }

        symbols
    }

    fn get_word_at_position(&self, line: &str, pos: usize) -> Option<String> {
        if pos > line.len() {
            return None;
        }

        let mut start = pos;
        let mut end = pos;

        // Expand left
        while start > 0 && (line.chars().nth(start - 1).unwrap().is_alphanumeric() || 
                           line.chars().nth(start - 1).unwrap() == '_') {
            start -= 1;
        }

        // Expand right
        while end < line.len() && (line.chars().nth(end).unwrap().is_alphanumeric() || 
                                   line.chars().nth(end).unwrap() == '_') {
            end += 1;
        }

        if start < end {
            Some(line[start..end].to_string())
        } else {
            None
        }
    }

    fn find_definition(&self, word: &str, lines: &[&str]) -> Option<Location> {
        for (line_idx, line) in lines.iter().enumerate() {
            // Check function definitions
            if line.trim().starts_with("fn ") && line.contains(word) {
                if let Some(fn_name) = self.extract_function_name(line.trim()) {
                    if fn_name == word {
                        return Some(Location {
                            uri: Url::from_file_path("/dev/null").unwrap(),
                            range: Range {
                                start: Position {
                                    line: line_idx as u32,
                                    character: 0,
                                },
                                end: Position {
                                    line: line_idx as u32,
                                    character: line.len() as u32,
                                },
                            },
                        });
                    }
                }
            }

            // Check variable definitions
            if line.trim().starts_with("let ") && line.contains(word) {
                if let Some(var_name) = self.extract_variable_name(line.trim()) {
                    if var_name == word {
                        return Some(Location {
                            uri: Url::from_file_path("/dev/null").unwrap(),
                            range: Range {
                                start: Position {
                                    line: line_idx as u32,
                                    character: 0,
                                },
                                end: Position {
                                    line: line_idx as u32,
                                    character: line.len() as u32,
                                },
                            },
                        });
                    }
                }
            }
        }

        None
    }

    fn extract_function_name(&self, line: &str) -> Option<String> {
        // "fn add(a: Int, b: Int) -> Int {" -> "add"
        let after_fn = line.strip_prefix("fn ")?;
        let before_paren = after_fn.split('(').next()?;
        Some(before_paren.trim().to_string())
    }

    fn extract_variable_name(&self, line: &str) -> Option<String> {
        // "let x = 42" -> "x"
        let after_let = line.strip_prefix("let ")?;
        let before_eq = after_let.split('=').next()?;
        Some(before_eq.trim().to_string())
    }

    fn extract_type_name(&self, line: &str) -> Option<String> {
        // "type Point = {x: Int, y: Int}" -> "Point"
        if let Some(after_type) = line.strip_prefix("type ") {
            let before_eq = after_type.split('=').next()?;
            return Some(before_eq.trim().to_string());
        }
        
        if let Some(after_struct) = line.strip_prefix("struct ") {
            let before_brace = after_struct.split('{').next()?;
            return Some(before_brace.trim().to_string());
        }

        None
    }
}
