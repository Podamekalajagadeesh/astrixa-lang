use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::ast::{Module, Stmt};
use crate::error::CompileError;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub struct ModuleLoader {
    loaded_modules: HashMap<String, Module>,
    search_paths: Vec<PathBuf>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            loaded_modules: HashMap::new(),
            search_paths: vec![
                PathBuf::from("."),
                PathBuf::from("./stdlib"),
            ],
        }
    }
    
    pub fn add_search_path(&mut self, path: PathBuf) {
        self.search_paths.push(path);
    }
    
    /// Load a module by name (e.g., "math" loads "math.ax")
    pub fn load_module(&mut self, name: &str) -> Result<&Module, CompileError> {
        if self.loaded_modules.contains_key(name) {
            return Ok(&self.loaded_modules[name]);
        }
        
        let module_file = self.find_module_file(name)?;
        
        let source = fs::read_to_string(&module_file)
            .map_err(|e| CompileError::new(
                &format!("Failed to read module '{}': {}", name, e),
                0, 0
            ))?;
        
        let module = self.parse_module(name, &source)?;
        
        self.loaded_modules.insert(name.to_string(), module);
        
        Ok(&self.loaded_modules[name])
    }
    
    /// Find the module file in search paths
    fn find_module_file(&self, name: &str) -> Result<PathBuf, CompileError> {
        let filename = format!("{}.ax", name);
        
        for search_path in &self.search_paths {
            let candidate = search_path.join(&filename);
            if candidate.exists() {
                return Ok(candidate);
            }
        }
        
        Err(CompileError::new(
            &format!("Module '{}' not found in search paths", name),
            0, 0
        ).help(&format!("Searched for '{}' in: {:?}", filename, self.search_paths)))
    }
    
    /// Parse module source into Module AST
    fn parse_module(&mut self, name: &str, source: &str) -> Result<Module, CompileError> {
        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);
        
        let statements = parser.parse()?;
        
        let mut imports = Vec::new();
        for stmt in &statements {
            if let Stmt::Import(module_name) = stmt {
                imports.push(module_name.clone());
            }
        }
        
        Ok(Module {
            name: name.to_string(),
            imports,
            statements,
        })
    }
    
    /// Get a loaded module
    pub fn get_module(&self, name: &str) -> Option<&Module> {
        self.loaded_modules.get(name)
    }
    
    /// Get all loaded modules
    pub fn get_all_modules(&self) -> &HashMap<String, Module> {
        &self.loaded_modules
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_loader_creation() {
        let loader = ModuleLoader::new();
        assert_eq!(loader.loaded_modules.len(), 0);
        assert!(loader.search_paths.len() >= 2);
    }
}
