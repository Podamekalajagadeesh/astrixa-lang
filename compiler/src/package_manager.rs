use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use sha2::{Sha256, Digest};

/// Package manifest structure (astrixa.toml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub dependencies: Option<HashMap<String, String>>,
}

/// Lockfile structure (astrixa.lock)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lockfile {
    pub packages: HashMap<String, String>,
}

/// Package Manager - handles installation, resolution, and storage
pub struct PackageManager {
    packages_dir: PathBuf,
    registry_url: String,
}

impl PackageManager {
    /// Create a new PackageManager instance
    pub fn new() -> Result<Self, String> {
        let home = dirs::home_dir()
            .ok_or_else(|| "Could not find home directory".to_string())?;
        
        let packages_dir = home.join(".astrixa").join("packages");
        
        // Create packages directory if it doesn't exist
        if !packages_dir.exists() {
            fs::create_dir_all(&packages_dir)
                .map_err(|e| format!("Failed to create packages directory: {}", e))?;
        }
        
        Ok(PackageManager {
            packages_dir,
            registry_url: "https://registry.astrixa.org".to_string(), // Future registry
        })
    }
    
    /// Initialize a new ASTRIXA project
    pub fn init(project_name: &str) -> Result<(), String> {
        let manifest = PackageManifest {
            name: project_name.to_string(),
            version: "0.1.0".to_string(),
            description: Some(format!("A new ASTRIXA project")),
            dependencies: Some(HashMap::new()),
        };
        
        // Create astrixa.toml
        let toml_content = toml::to_string_pretty(&manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
        
        fs::write("astrixa.toml", toml_content)
            .map_err(|e| format!("Failed to write astrixa.toml: {}", e))?;
        
        // Create src directory
        fs::create_dir_all("src")
            .map_err(|e| format!("Failed to create src directory: {}", e))?;
        
        // Create main.ax
        let main_content = r#"// Welcome to ASTRIXA!
fn main() {
    print("Hello, ASTRIXA!");
}
"#;
        fs::write("src/main.ax", main_content)
            .map_err(|e| format!("Failed to create main.ax: {}", e))?;
        
        println!("✓ Initialized ASTRIXA project: {}", project_name);
        println!("✓ Created astrixa.toml");
        println!("✓ Created src/main.ax");
        
        Ok(())
    }
    
    /// Install a package
    pub fn install(&self, package_name: &str, version: Option<&str>) -> Result<(), String> {
        let version = version.unwrap_or("latest");
        
        println!("📦 Installing {}@{}...", package_name, version);
        
        // For MVP: check if package exists locally (simulate git-based registry)
        let package_dir = self.packages_dir.join(package_name);
        
        if package_dir.exists() {
            println!("✓ Package {} already installed", package_name);
            return Ok(());
        }
        
        // Create package directory
        fs::create_dir_all(&package_dir)
            .map_err(|e| format!("Failed to create package directory: {}", e))?;
        
        // MVP: create a basic package structure
        // Future versions will fetch from a registry or git
        self.create_skeleton_package(&package_dir, package_name, version)?;
        
        // Update lockfile
        self.update_lockfile(package_name, version)?;
        
        println!("✓ Installed {}@{}", package_name, version);
        
        Ok(())
    }
    
    /// Create a minimal local package (MVP - simulates registry fetch)
    fn create_skeleton_package(&self, package_dir: &Path, name: &str, version: &str) -> Result<(), String> {
        // Create package manifest
        let manifest = PackageManifest {
            name: name.to_string(),
            version: version.to_string(),
            description: Some(format!("{} package", name)),
            dependencies: None,
        };
        
        let toml_content = toml::to_string_pretty(&manifest)
            .map_err(|e| format!("Failed to serialize manifest: {}", e))?;
        
        fs::write(package_dir.join("astrixa.toml"), toml_content)
            .map_err(|e| format!("Failed to write package manifest: {}", e))?;
        
        // Create src directory
        let src_dir = package_dir.join("src");
        fs::create_dir_all(&src_dir)
            .map_err(|e| format!("Failed to create src directory: {}", e))?;
        
        // Create index.ax based on package type
        let index_content = match name {
            "math" => r#"// Math utilities
export fn add(a: int, b: int) -> int {
    return a + b;
}

export fn subtract(a: int, b: int) -> int {
    return a - b;
}

export fn multiply(a: int, b: int) -> int {
    return a * b;
}

export fn divide(a: int, b: int) -> int {
    if b == 0 {
        panic("Division by zero");
    }
    return a / b;
}

export fn power(base: int, exp: int) -> int {
    let result = 1;
    let i = 0;
    while i < exp {
        result = result * base;
        i = i + 1;
    }
    return result;
}
"#,
            "ai-tools" => r#"// AI utilities
export fn create_prompt(text: string) -> string {
    return "AI: " + text;
}

export fn analyze_sentiment(text: string) -> string {
    // Basic sentiment example; replace with real implementation when available
    return "neutral";
}
"#,
            _ => format!(r#"// {} package
export fn hello() -> string {{
    return "Hello from {}!";
}}
"#, name, name),
        };
        
        fs::write(src_dir.join("index.ax"), index_content)
            .map_err(|e| format!("Failed to write index.ax: {}", e))?;
        
        Ok(())
    }
    
    /// Update lockfile
    fn update_lockfile(&self, package_name: &str, version: &str) -> Result<(), String> {
        let lockfile_path = Path::new("astrixa.lock");
        
        // Read existing lockfile or create new
        let mut lockfile = if lockfile_path.exists() {
            let content = fs::read_to_string(lockfile_path)
                .map_err(|e| format!("Failed to read lockfile: {}", e))?;
            toml::from_str::<Lockfile>(&content)
                .map_err(|e| format!("Failed to parse lockfile: {}", e))?
        } else {
            Lockfile {
                packages: HashMap::new(),
            }
        };
        
        // Add/update package
        lockfile.packages.insert(package_name.to_string(), version.to_string());
        
        // Write lockfile
        let toml_content = toml::to_string_pretty(&lockfile)
            .map_err(|e| format!("Failed to serialize lockfile: {}", e))?;
        
        fs::write(lockfile_path, toml_content)
            .map_err(|e| format!("Failed to write lockfile: {}", e))?;
        
        Ok(())
    }
    
    /// List installed packages
    pub fn list(&self) -> Result<(), String> {
        println!("📦 Installed ASTRIXA packages:\n");
        
        if !self.packages_dir.exists() {
            println!("No packages installed yet.");
            return Ok(());
        }
        
        let entries = fs::read_dir(&self.packages_dir)
            .map_err(|e| format!("Failed to read packages directory: {}", e))?;
        
        let mut count = 0;
        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                let manifest_path = path.join("astrixa.toml");
                if manifest_path.exists() {
                    let content = fs::read_to_string(&manifest_path)
                        .map_err(|e| format!("Failed to read manifest: {}", e))?;
                    
                    let manifest: PackageManifest = toml::from_str(&content)
                        .map_err(|e| format!("Failed to parse manifest: {}", e))?;
                    
                    println!("  {} v{}", manifest.name, manifest.version);
                    if let Some(desc) = manifest.description {
                        println!("    {}", desc);
                    }
                    println!();
                    count += 1;
                }
            }
        }
        
        if count == 0 {
            println!("No packages installed yet.");
        } else {
            println!("Total: {} package(s)", count);
        }
        
        Ok(())
    }
    
    /// Get the path to an installed package
    pub fn get_package_path(&self, package_name: &str) -> Option<PathBuf> {
        let package_dir = self.packages_dir.join(package_name);
        if package_dir.exists() {
            Some(package_dir)
        } else {
            None
        }
    }
    
    /// Resolve package import path
    pub fn resolve_import(&self, import_path: &str) -> Option<PathBuf> {
        // Check if it's a package import (no ./ or ../)
        if !import_path.starts_with("./") && !import_path.starts_with("../") {
            let package_path = self.packages_dir.join(import_path).join("src").join("index.ax");
            if package_path.exists() {
                return Some(package_path);
            }
        }
        None
    }
    
    /// Calculate SHA256 checksum of a file
    pub fn calculate_checksum(path: &Path) -> Result<String, String> {
        let content = fs::read(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();
        
        Ok(format!("{:x}", result))
    }
    
    /// Read package manifest
    pub fn read_manifest(path: &Path) -> Result<PackageManifest, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse manifest: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_package_manifest_serialization() {
        let manifest = PackageManifest {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test package".to_string()),
            dependencies: None,
        };
        
        let toml = toml::to_string(&manifest).unwrap();
        assert!(toml.contains("name = \"test\""));
        assert!(toml.contains("version = \"1.0.0\""));
    }
}
