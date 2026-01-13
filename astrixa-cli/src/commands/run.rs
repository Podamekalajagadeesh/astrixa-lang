// STEP 50: Run Command

use colored::*;
use std::path::PathBuf;
use std::process::Command;
use crate::config::{Config, find_project_root};
use super::build;

pub fn run_project(release: bool) -> Result<(), String> {
    let root = find_project_root()?;
    let config = Config::load(root.join("astrixa.toml"))?;
    
    println!("{} {}", "Running".green().bold(), config.package.name);
    println!();
    
    // Build the project first
    build::build_project(release, None)?;
    
    // Determine WASM file path
    let wasm_file = root.join("build").join(format!("{}.wat", config.package.name));
    
    if !wasm_file.exists() {
        return Err("Build artifact not found. Build may have failed.".to_string());
    }
    
    println!();
    println!("{}", "─".repeat(50).dimmed());
    println!("{}", "Output:".cyan().bold());
    println!();
    
    // Execute the WASM module
    execute_wasm(&wasm_file)?;
    
    println!();
    println!("{}", "─".repeat(50).dimmed());
    
    Ok(())
}

fn execute_wasm(wasm_file: &PathBuf) -> Result<(), String> {
    // For now, we'll create a simple Node.js runner
    // In a production system, you'd use wasmtime or wasmer
    
    let js_runner = format!(r#"
const fs = require('fs');
const wabt = require('wabt')();

async function run() {{
    try {{
        // Read WAT file
        const watContent = fs.readFileSync('{}', 'utf8');
        
        // Convert WAT to WASM
        const module = wabt.parseWat('module.wat', watContent);
        const {{ buffer }} = module.toBinary({{}});
        
        // Runtime environment
        const memory = new WebAssembly.Memory({{ initial: 1 }});
        const imports = {{
            env: {{
                memory: memory,
                print_str: (ptr, len) => {{
                    const bytes = new Uint8Array(memory.buffer, ptr, len);
                    const str = new TextDecoder().decode(bytes);
                    console.log(str);
                }},
                println_str: (ptr, len) => {{
                    const bytes = new Uint8Array(memory.buffer, ptr, len);
                    const str = new TextDecoder().decode(bytes);
                    console.log(str);
                }},
                panic: (ptr, len) => {{
                    const bytes = new Uint8Array(memory.buffer, ptr, len);
                    const str = new TextDecoder().decode(bytes);
                    throw new Error('Panic: ' + str);
                }},
            }}
        }};
        
        // Instantiate and run
        const instance = await WebAssembly.instantiate(buffer, imports);
        
        if (instance.instance.exports.main) {{
            const result = instance.instance.exports.main();
            if (result !== undefined) {{
                console.log('Program returned:', result);
            }}
        }} else {{
            console.error('No main function found');
            process.exit(1);
        }}
    }} catch (error) {{
        console.error('Runtime error:', error.message);
        process.exit(1);
    }}
}}

run();
"#, wasm_file.display());
    
    // Write temporary JS file
    let temp_js = wasm_file.parent().unwrap().join("_run_temp.js");
    std::fs::write(&temp_js, js_runner)
        .map_err(|e| format!("Failed to create runner: {}", e))?;
    
    // Execute with Node.js
    let output = Command::new("node")
        .arg(&temp_js)
        .output();
    
    // Clean up temp file
    let _ = std::fs::remove_file(&temp_js);
    
    match output {
        Ok(output) => {
            // Print stdout
            if !output.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&output.stdout));
            }
            
            // Print stderr
            if !output.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }
            
            if !output.status.success() {
                return Err("Program exited with error".to_string());
            }
            
            Ok(())
        }
        Err(e) => {
            Err(format!("Failed to execute: {}. Is Node.js installed?", e))
        }
    }
}
