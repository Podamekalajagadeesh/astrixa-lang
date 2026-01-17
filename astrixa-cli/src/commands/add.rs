// STEP 50: Add Package Command

use colored::*;
use std::fs;
use std::path::Path;
use crate::config::{Config, find_project_root};

pub fn add_package(package: &str, version: &str) -> Result<(), String> {
    let root = find_project_root()?;
    let config_path = root.join("astrixa.toml");
    
    println!("{} package '{}'", "Adding".green().bold(), package);
    
    // Load config
    let mut config = Config::load(&config_path)?;
    
    // Check if already exists
    if config.dependencies.contains_key(package) {
        println!("   {} Package already in dependencies", "Warning:".yellow());
        println!("   Updating version...");
    }
    
    // Add dependency
    config.add_dependency(package.to_string(), version.to_string());
    
    // Save config
    config.save(&config_path)?;
    
    println!("   {} {} = \"{}\"", "Added".green(), package, version);
    
    // Install package (V1: local only)
    install_package(package, version, &root)?;
    
    println!();
    println!("{} Added {} v{}", "✅".green(), package, version);
    
    Ok(())
}

fn install_package(name: &str, version: &str, root: &Path) -> Result<(), String> {
    let modules_dir = root.join("modules");
    fs::create_dir_all(&modules_dir)
        .map_err(|e| format!("Failed to create modules directory: {}", e))?;
    
    // V1: Try to find package locally
    // Check standard locations
    let search_paths = vec![
        root.join("../stdlib").join(format!("{}.ax", name)),
        root.join("modules").join(format!("{}.ax", name)),
        Path::new("/usr/local/lib/astrixa/modules").join(format!("{}.ax", name)),
    ];
    
    for path in search_paths {
        if path.exists() {
            // Copy to project modules
            let dest = modules_dir.join(format!("{}.ax", name));
            fs::copy(&path, &dest)
                .map_err(|e| format!("Failed to install package: {}", e))?;
            
            println!("   {} Installed from {}", "Fetched".cyan(), path.display());
            return Ok(());
        }
    }
    
    // Package not found locally - create a module skeleton
    println!("   {} Package not found locally", "Warning:".yellow());
    println!("   Creating module skeleton...");
    
    let skeleton_code = format!(r#"// Module skeleton for {0}
// Registry installation planned; add implementation or install when available.

fn info {{
    print("Module '{0}' not installed; using local skeleton")
}}
"#, name);
    
    let skeleton_path = modules_dir.join(format!("{}.ax", name));
    fs::write(&skeleton_path, skeleton_code)
        .map_err(|e| format!("Failed to create module skeleton: {}", e))?;
    
    println!("   {} Skeleton created at modules/{}.ax", "Created".yellow(), name);
    println!();
    println!("   {} Package registry support is planned.", "Note:".cyan());
    println!("   Add your module implementation at modules/{}.ax.", name);
    
    Ok(())
}

pub fn remove_package(package: &str) -> Result<(), String> {
    let root = find_project_root()?;
    let config_path = root.join("astrixa.toml");
    
    println!("{} package '{}'", "Removing".yellow().bold(), package);
    
    // Load config
    let mut config = Config::load(&config_path)?;
    
    // Remove dependency
    if config.dependencies.remove(package).is_none() {
        return Err(format!("Package '{}' not found in dependencies", package));
    }
    
    // Save config
    config.save(&config_path)?;
    
    // Remove from modules
    let module_path = root.join("modules").join(format!("{}.ax", package));
    if module_path.exists() {
        fs::remove_file(&module_path)
            .map_err(|e| format!("Failed to remove module file: {}", e))?;
    }
    
    println!("{} Removed {}", "✅".green(), package);
    
    Ok(())
}

pub fn list_packages() -> Result<(), String> {
    let root = find_project_root()?;
    let config = Config::load(root.join("astrixa.toml"))?;
    
    if config.dependencies.is_empty() {
        println!("{}", "No dependencies".dimmed());
        return Ok(());
    }
    
    println!("{}", "Dependencies:".cyan().bold());
    for (name, version) in &config.dependencies {
        println!("  {} = {}", name.green(), version.dimmed());
    }
    
    Ok(())
}
