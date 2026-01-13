// STEP 50: Project Configuration (astrixa.toml)

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub package: Package,
    #[serde(default)]
    pub dependencies: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub dev_dependencies: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    #[serde(default)]
    pub license: Option<String>,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read astrixa.toml: {}", e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse astrixa.toml: {}", e))
    }
    
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(path, content)
            .map_err(|e| format!("Failed to write astrixa.toml: {}", e))
    }
    
    pub fn new(name: String) -> Self {
        Self {
            package: Package {
                name,
                version: "0.1.0".to_string(),
                description: None,
                authors: vec![],
                license: Some("MIT".to_string()),
            },
            dependencies: std::collections::HashMap::new(),
            dev_dependencies: std::collections::HashMap::new(),
        }
    }
    
    pub fn add_dependency(&mut self, name: String, version: String) {
        self.dependencies.insert(name, version);
    }
}

pub fn find_project_root() -> Result<std::path::PathBuf, String> {
    let mut current = std::env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;
    
    loop {
        let config_path = current.join("astrixa.toml");
        if config_path.exists() {
            return Ok(current);
        }
        
        if !current.pop() {
            return Err("Not in an ASTRIXA project (no astrixa.toml found)".to_string());
        }
    }
}
