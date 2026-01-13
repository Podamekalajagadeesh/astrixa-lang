// STEP 50: New Project Command

use colored::*;
use std::fs;
use std::path::Path;
use crate::config::Config;
use crate::templates;

pub fn create_project(name: &str, template: &str) -> Result<(), String> {
    println!("{}", format!("✨ Creating new ASTRIXA project '{}'", name).green().bold());
    
    // Check if directory exists
    if Path::new(name).exists() {
        return Err(format!("Directory '{}' already exists", name));
    }
    
    // Get template
    let tmpl = templates::get_template(template)
        .ok_or_else(|| format!("Unknown template: {}", template))?;
    
    println!("   {} {}", "Template:".cyan(), tmpl.description);
    
    // Create directory structure
    create_directory_structure(name)?;
    
    // Create config file
    let config = Config::new(name.to_string());
    config.save(format!("{}/astrixa.toml", name))?;
    println!("   {} astrixa.toml", "Created".green());
    
    // Create main.ax
    fs::write(format!("{}/src/main.ax", name), tmpl.main_content)
        .map_err(|e| format!("Failed to create main.ax: {}", e))?;
    println!("   {} src/main.ax", "Created".green());
    
    // Create additional files
    for (path, content) in &tmpl.additional_files {
        fs::write(format!("{}/{}", name, path), content)
            .map_err(|e| format!("Failed to create {}: {}", path, e))?;
        println!("   {} {}", "Created".green(), path);
    }
    
    // Create .gitignore
    let gitignore = r#"# Build artifacts
build/
*.wat
*.wasm

# Dependencies
modules/

# Editor
.vscode/
.idea/
*.swp
*~

# OS
.DS_Store
Thumbs.db
"#;
    fs::write(format!("{}/.gitignore", name), gitignore)
        .map_err(|e| format!("Failed to create .gitignore: {}", e))?;
    println!("   {} .gitignore", "Created".green());
    
    println!();
    println!("{}", "✅ Project created successfully!".green().bold());
    println!();
    println!("Next steps:");
    println!("  {} {}", "cd".cyan(), name);
    println!("  {} {}", "astrixa".cyan(), "run");
    println!();
    
    Ok(())
}

pub fn init_project() -> Result<(), String> {
    println!("{}", "✨ Initializing ASTRIXA project in current directory".green().bold());
    
    // Check if already initialized
    if Path::new("astrixa.toml").exists() {
        return Err("Project already initialized (astrixa.toml exists)".to_string());
    }
    
    // Create directory structure
    create_directory_structure(".")?;
    
    // Get current directory name
    let current_dir = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    let project_name = current_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("my-project");
    
    // Create config file
    let config = Config::new(project_name.to_string());
    config.save("astrixa.toml")?;
    println!("   {} astrixa.toml", "Created".green());
    
    // Create main.ax if it doesn't exist
    if !Path::new("src/main.ax").exists() {
        let tmpl = templates::get_template("default").unwrap();
        fs::write("src/main.ax", tmpl.main_content)
            .map_err(|e| format!("Failed to create main.ax: {}", e))?;
        println!("   {} src/main.ax", "Created".green());
    }
    
    println!();
    println!("{}", "✅ Project initialized!".green().bold());
    println!();
    println!("Next steps:");
    println!("  {} {}", "astrixa".cyan(), "run");
    println!();
    
    Ok(())
}

fn create_directory_structure(base: &str) -> Result<(), String> {
    let dirs = vec![
        format!("{}/src", base),
        format!("{}/modules", base),
        format!("{}/build", base),
    ];
    
    for dir in dirs {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create directory {}: {}", dir, e))?;
    }
    
    Ok(())
}
