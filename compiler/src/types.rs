#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Text,
    Void,
    Unknown,
}

impl Type {
    pub fn to_string(&self) -> String {
        match self {
            Type::Int => "Int".to_string(),
            Type::Float => "Float".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::String => "String".to_string(),
            Type::Text => "Text".to_string(),
            Type::Void => "Void".to_string(),
            Type::Unknown => "Unknown".to_string(),
        }
    }
}
