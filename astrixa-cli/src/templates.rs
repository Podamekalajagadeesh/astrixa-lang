pub struct Template {
    pub name: &'static str,
    pub description: &'static str,
    pub main_content: &'static str,
    pub additional_files: Vec<(&'static str, &'static str)>,
}

pub fn get_template(name: &str) -> Option<Template> {
    match name {
        "default" => Some(default_template()),
        "lib" => Some(library_template()),
        "web3" => Some(web3_template()),
        "ai" => Some(ai_template()),
        _ => None,
    }
}

fn default_template() -> Template {
    Template {
        name: "default",
        description: "Basic ASTRIXA application",
        main_content: r#"// ASTRIXA Hello World

fn main {
    print("Hello, ASTRIXA!")
    print("Welcome to modern programming!")
}
"#,
        additional_files: vec![],
    }
}

fn library_template() -> Template {
    Template {
        name: "lib",
        description: "ASTRIXA library template",
        main_content: r#"// ASTRIXA Library

export fn add(a, b) {
    return a + b
}

export fn multiply(a, b) {
    return a * b
}

export fn power(base, exp) {
    let result = 1
    let i = 0
    while i < exp {
        result = result * base
        i = i + 1
    }
    return result
}
"#,
        additional_files: vec![
            ("README.md", r#"# ASTRIXA Library

A reusable ASTRIXA library.

## Usage

```astrixa
import mylib

fn main {
    let sum = mylib.add(10, 20)
    print(sum)
}
```
"#),
        ],
    }
}

fn web3_template() -> Template {
    Template {
        name: "web3",
        description: "Web3/Blockchain application template",
        main_content: r#"// ASTRIXA Web3 Application

import crypto

fn main {
    print("🔐 ASTRIXA Web3 App")
    
    // Example: Hash some data
    let data = "Hello, Blockchain!"
    let hash = keccak(data)
    
    print("Data hash:")
    print(hash)
}

export fn sign_transaction(tx_data) {
    let hash = keccak(tx_data)
    return hash
}

export fn verify_signature(signature, message) {
    let expected = keccak(message)
    return signature == expected
}
"#,
        additional_files: vec![
            ("README.md", r#"# ASTRIXA Web3 Project

Blockchain-ready ASTRIXA application.

## Features

- Cryptographic hashing
- Transaction signing
- Signature verification
"#),
        ],
    }
}

fn ai_template() -> Template {
    Template {
        name: "ai",
        description: "AI/ML application template",
        main_content: r#"// ASTRIXA AI Application

fn main {
    print("🤖 ASTRIXA AI App")
    
    // Example: Simple prediction
    let data = [1, 2, 3, 4, 5]
    let prediction = predict(data)
    
    print("Prediction:")
    print(prediction)
}

export fn predict(data) {
    // Simple linear prediction
    let sum = 0
    let count = 0
    
    while count < 5 {
        sum = sum + data
        count = count + 1
    }
    
    return sum / count
}

export fn train_model(training_data) {
    print("Training model...")
    return "model_v1"
}
"#,
        additional_files: vec![
            ("README.md", r#"# ASTRIXA AI Project

AI/ML application built with ASTRIXA.

## Features

- Data processing
- Model training
- Predictions
"#),
        ],
    }
}

pub fn list_templates() -> Vec<Template> {
    vec![
        default_template(),
        library_template(),
        web3_template(),
        ai_template(),
    ]
}
