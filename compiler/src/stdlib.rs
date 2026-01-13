/// ASTRIXA Standard Library Registry
/// 
/// This module defines all built-in functions available in ASTRIXA.
/// These functions are provided by the runtime and compiled as WASM imports.
/// 
/// Design principles:
/// - Minimal: Only essential functions
/// - Fast: Zero overhead (direct WASM imports)
/// - Explicit: Clear function names and purposes
/// - Web3-ready: Includes crypto functions for blockchain
/// - WASM-friendly: All signatures compatible with WASM types

/// Standard library function category
#[derive(Debug, Clone, PartialEq)]
pub enum StdlibCategory {
    Core,      // Essential I/O: print, input, len, exit
    Math,      // Math operations: abs, pow, sqrt, rand
    Time,      // Time functions: time, sleep
    Crypto,    // Web3 crypto: hash, keccak, sha256
    String,    // String operations: len, concat, substr
    AI,        // STEP 52: AI-native functions: generate, embed, classify
}

/// Standard library function signature
#[derive(Debug, Clone)]
pub struct StdlibFunction {
    pub name: &'static str,
    pub category: StdlibCategory,
    pub description: &'static str,
    pub param_count: usize,
    pub return_type: &'static str,
}

/// Check if a function name is a standard library function
pub fn is_stdlib(name: &str) -> bool {
    STDLIB_FUNCTIONS.iter().any(|f| f.name == name)
}

/// STEP 52: Check if a function is an AI function
pub fn is_ai(name: &str) -> bool {
    matches!(name,
        "ai.generate" |
        "ai.embed" |
        "ai.classify"
    )
}

/// STEP 53: Check if a function is a Web3 function
pub fn is_web3(name: &str) -> bool {
    matches!(name,
        "web3.wallet" |
        "web3.sign" |
        "web3.verify" |
        "web3.keccak" |
        "web3.balance" |
        "web3.send"
    )
}

/// Get information about a stdlib function
pub fn get_stdlib_info(name: &str) -> Option<&'static StdlibFunction> {
    STDLIB_FUNCTIONS.iter().find(|f| f.name == name)
}

/// Get all stdlib functions by category
pub fn get_by_category(category: StdlibCategory) -> Vec<&'static StdlibFunction> {
    STDLIB_FUNCTIONS
        .iter()
        .filter(|f| f.category == category)
        .collect()
}

/// Registry of all standard library functions
pub static STDLIB_FUNCTIONS: &[StdlibFunction] = &[
    // ==========================================
    // CORE FUNCTIONS (Always available)
    // ==========================================
    StdlibFunction {
        name: "print",
        category: StdlibCategory::Core,
        description: "Print a value to stdout (no newline)",
        param_count: 1,
        return_type: "void",
    },
    StdlibFunction {
        name: "println",
        category: StdlibCategory::Core,
        description: "Print a value to stdout with newline",
        param_count: 1,
        return_type: "void",
    },
    StdlibFunction {
        name: "input",
        category: StdlibCategory::Core,
        description: "Read a line from stdin",
        param_count: 0,
        return_type: "string",
    },
    StdlibFunction {
        name: "len",
        category: StdlibCategory::Core,
        description: "Get length of a string or array",
        param_count: 1,
        return_type: "int",
    },
    StdlibFunction {
        name: "exit",
        category: StdlibCategory::Core,
        description: "Exit program with status code",
        param_count: 1,
        return_type: "void",
    },
    
    // ==========================================
    // MATH FUNCTIONS
    // ==========================================
    StdlibFunction {
        name: "abs",
        category: StdlibCategory::Math,
        description: "Absolute value of a number",
        param_count: 1,
        return_type: "int",
    },
    StdlibFunction {
        name: "pow",
        category: StdlibCategory::Math,
        description: "Raise base to exponent (base^exp)",
        param_count: 2,
        return_type: "int",
    },
    StdlibFunction {
        name: "sqrt",
        category: StdlibCategory::Math,
        description: "Square root of a number",
        param_count: 1,
        return_type: "int",
    },
    StdlibFunction {
        name: "min",
        category: StdlibCategory::Math,
        description: "Minimum of two numbers",
        param_count: 2,
        return_type: "int",
    },
    StdlibFunction {
        name: "max",
        category: StdlibCategory::Math,
        description: "Maximum of two numbers",
        param_count: 2,
        return_type: "int",
    },
    StdlibFunction {
        name: "rand",
        category: StdlibCategory::Math,
        description: "Random integer in range [0, max)",
        param_count: 1,
        return_type: "int",
    },
    
    // ==========================================
    // TIME FUNCTIONS
    // ==========================================
    StdlibFunction {
        name: "time",
        category: StdlibCategory::Time,
        description: "Current Unix timestamp in milliseconds",
        param_count: 0,
        return_type: "int",
    },
    StdlibFunction {
        name: "sleep",
        category: StdlibCategory::Time,
        description: "Sleep for specified milliseconds",
        param_count: 1,
        return_type: "void",
    },
    
    // ==========================================
    // CRYPTO FUNCTIONS (Web3-ready)
    // ==========================================
    StdlibFunction {
        name: "hash",
        category: StdlibCategory::Crypto,
        description: "Generic hash function (defaults to keccak256)",
        param_count: 1,
        return_type: "string",
    },
    StdlibFunction {
        name: "keccak",
        category: StdlibCategory::Crypto,
        description: "Keccak-256 hash (Ethereum standard)",
        param_count: 1,
        return_type: "string",
    },
    StdlibFunction {
        name: "sha256",
        category: StdlibCategory::Crypto,
        description: "SHA-256 hash",
        param_count: 1,
        return_type: "string",
    },    
    // ==========================================
    // AI FUNCTIONS (STEP 52)
    // ==========================================
    StdlibFunction {
        name: "ai.generate",
        category: StdlibCategory::AI,
        description: "Generate text using AI (prompt in, text out)",
        param_count: 1,
        return_type: "string",
    },
    StdlibFunction {
        name: "ai.embed",
        category: StdlibCategory::AI,
        description: "Generate embeddings for text",
        param_count: 1,
        return_type: "string",
    },
    StdlibFunction {
        name: "ai.classify",
        category: StdlibCategory::AI,
        description: "Classify text into categories",
        param_count: 1,
        return_type: "string",
    },];

/// Generate human-readable documentation for stdlib
pub fn generate_docs() -> String {
    let mut docs = String::new();
    
    docs.push_str("# ASTRIXA Standard Library Reference\n\n");
    docs.push_str("Built-in functions available in all ASTRIXA programs.\n\n");
    
    for category in &[
        StdlibCategory::Core,
        StdlibCategory::Math,
        StdlibCategory::Time,
        StdlibCategory::Crypto,
        StdlibCategory::AI,
    ] {
        docs.push_str(&format!("## {:?} Functions\n\n", category));
        
        let funcs = get_by_category(category.clone());
        for func in funcs {
            docs.push_str(&format!("### `{}()`\n", func.name));
            docs.push_str(&format!("{}\n\n", func.description));
            docs.push_str(&format!("**Params:** {}\n", func.param_count));
            docs.push_str(&format!("**Returns:** `{}`\n\n", func.return_type));
        }
    }
    
    docs
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_stdlib() {
        assert!(is_stdlib("print"));
        assert!(is_stdlib("time"));
        assert!(is_stdlib("hash"));
        assert!(!is_stdlib("custom_func"));
    }
    
    #[test]
    fn test_get_stdlib_info() {
        let info = get_stdlib_info("print").unwrap();
        assert_eq!(info.name, "print");
        assert_eq!(info.category, StdlibCategory::Core);
    }
    
    #[test]
    fn test_get_by_category() {
        let core_funcs = get_by_category(StdlibCategory::Core);
        assert!(core_funcs.len() >= 3);
        
        let math_funcs = get_by_category(StdlibCategory::Math);
        assert!(math_funcs.len() >= 3);
    }
    
    #[test]
    fn test_all_categories_covered() {
        // Ensure we have functions in all categories
        assert!(!get_by_category(StdlibCategory::Core).is_empty());
        assert!(!get_by_category(StdlibCategory::Math).is_empty());
        assert!(!get_by_category(StdlibCategory::Time).is_empty());
        assert!(!get_by_category(StdlibCategory::Crypto).is_empty());
    }
}
