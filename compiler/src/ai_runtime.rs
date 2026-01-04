// ASTRIXA AI Runtime - Abstraction for AI inference
// Allows different backends: local, GPU, remote, on-chain

use crate::interpreter::Value;

/// Represents an AI model handle
#[derive(Clone, Debug)]
pub struct AIModel {
    pub name: String,
    pub model_type: ModelType,
}

#[derive(Clone, Debug)]
pub enum ModelType {
    TextClassifier,
    Sentiment,
    Embedding,
    NamedEntityRecognition,
    QuestionAnswering,
    Custom(String),
}

impl ModelType {
    pub fn from_string(s: &str) -> Self {
        match s {
            "text-classifier" | "classifier" => ModelType::TextClassifier,
            "sentiment" => ModelType::Sentiment,
            "embedding" | "embeddings" => ModelType::Embedding,
            "ner" | "named-entity" => ModelType::NamedEntityRecognition,
            "qa" | "question-answering" => ModelType::QuestionAnswering,
            custom => ModelType::Custom(custom.to_string()),
        }
    }
}

/// AI Runtime trait - implement to add different backends
pub trait AIRuntime {
    /// Load or get a model by name
    fn model(&self, name: &str) -> Result<AIModel, String>;

    /// Run inference on input
    fn infer(&self, model: &AIModel, input: &str) -> Result<Value, String>;

    /// Tokenize text
    fn tokenize(&self, text: &str) -> Result<Vec<String>, String>;

    /// Generate embeddings
    fn embed(&self, text: &str) -> Result<Vec<f64>, String>;
}

/// Default local AI runtime - uses heuristics/mocks
/// In production, would integrate with ML frameworks
pub struct LocalAIRuntime;

impl AIRuntime for LocalAIRuntime {
    fn model(&self, name: &str) -> Result<AIModel, String> {
        Ok(AIModel {
            name: name.to_string(),
            model_type: ModelType::from_string(name),
        })
    }

    fn infer(&self, model: &AIModel, input: &str) -> Result<Value, String> {
        // Deterministic heuristic-based inference for now
        // In production: call actual ML model
        
        match model.model_type {
            ModelType::Sentiment => {
                // Simple sentiment analysis based on keywords
                let sentiment_score = calculate_sentiment(input);
                let label = if sentiment_score > 0.5 {
                    "positive".to_string()
                } else if sentiment_score < -0.5 {
                    "negative".to_string()
                } else {
                    "neutral".to_string()
                };
                
                Ok(Value::AIResult {
                    label,
                    score: (sentiment_score.abs()).min(1.0),
                })
            }
            ModelType::TextClassifier => {
                // Generic text classification
                let class = classify_text(input);
                Ok(Value::AIResult {
                    label: class,
                    score: 0.85, // Default confidence
                })
            }
            ModelType::Embedding => {
                // Create a simple deterministic embedding
                let embedding = simple_embedding(input);
                Ok(Value::Array(
                    embedding
                        .iter()
                        .map(|&f| Value::Number((f * 100.0) as i64))
                        .collect(),
                ))
            }
            _ => Err(format!("Model type {:?} not supported in local runtime", model.model_type)),
        }
    }

    fn tokenize(&self, text: &str) -> Result<Vec<String>, String> {
        // Simple whitespace tokenization
        Ok(text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect())
    }

    fn embed(&self, text: &str) -> Result<Vec<f64>, String> {
        // Generate deterministic embedding
        Ok(simple_embedding(text))
    }
}

// Helper functions for deterministic AI

fn calculate_sentiment(text: &str) -> f64 {
    let text_lower = text.to_lowercase();

    // Positive keywords
    let positive_words = vec![
        "love", "amazing", "wonderful", "excellent", "great", "good", "awesome", "fantastic",
        "beautiful", "perfect", "best", "incredible",
    ];
    let positive_count = positive_words.iter().filter(|w| text_lower.contains(*w)).count();

    // Negative keywords
    let negative_words = vec![
        "hate", "terrible", "awful", "bad", "horrible", "worst", "worst", "ugly", "poor",
        "disappointing", "useless", "worst",
    ];
    let negative_count = negative_words.iter().filter(|w| text_lower.contains(*w)).count();

    let total = (positive_count + negative_count) as f64;
    if total == 0.0 {
        0.0
    } else {
        ((positive_count as f64) - (negative_count as f64)) / total
    }
}

fn classify_text(text: &str) -> String {
    let text_lower = text.to_lowercase();

    if text_lower.contains("question") || text_lower.contains("what") || text_lower.contains("how")
    {
        "question".to_string()
    } else if text_lower.contains("statement") || text_lower.contains("is") {
        "statement".to_string()
    } else if text_lower.contains("exclaim") || text_lower.contains("!") {
        "exclamation".to_string()
    } else {
        "unknown".to_string()
    }
}

fn simple_embedding(text: &str) -> Vec<f64> {
    // Deterministic embedding based on text hash
    let mut hash: u64 = 5381;
    for byte in text.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
    }

    // Generate 8-dimensional embedding from hash
    let mut embedding = vec![0.0; 8];
    for i in 0..8 {
        let shifted = hash.wrapping_shr((i * 8) as u32);
        embedding[i] = ((shifted as f64) / 256.0) - 0.5;
    }
    embedding
}
