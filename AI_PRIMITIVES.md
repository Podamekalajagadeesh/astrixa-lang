# AI-Native Primitives

The ASTRIXA language includes AI as a first-class language feature, not relegated to libraries. This enables native, deterministic AI operations in smart contracts and applications.

## Overview

AI operations are accessed through the `ai` namespace with four primary methods:
- `ai.model(name)` - Load or create an AI model
- `ai.infer(model, input)` - Run inference on a model
- `ai.embed(text)` - Generate vector embeddings
- `ai.tokenize(text)` - Split text into tokens

## API Reference

### ai.model(name: string) → string

Loads or retrieves an AI model by name.

**Supported Models:**
- `"sentiment"` - Sentiment analysis (positive/negative/neutral)
- `"classifier"` - General text classification
- `"embedding"` - Text-to-vector conversion
- `"ner"` - Named entity recognition
- `"qa"` - Question answering

**Example:**
```astrixa
let m = ai.model("sentiment");
```

### ai.infer(model: string, input: string) → AIResult

Runs inference on a model with the given input text.

**Returns:** An `AIResult` object with `label` (string) and `score` (number) properties.

**Example:**
```astrixa
let result = ai.infer(ai.model("sentiment"), "I love this!");
print(result);  // Output: positive: 0.92
```

### ai.embed(text: string) → array

Generates a vector embedding for the given text.

**Returns:** Array of numbers representing the embedding

**Example:**
```astrixa
let embedding = ai.embed("ASTRIXA language");
print(len(embedding));  // Outputs embedding dimension
```

### ai.tokenize(text: string) → array

Splits text into tokens.

**Returns:** Array of strings (tokens)

**Example:**
```astrixa
let tokens = ai.tokenize("Hello world");
print(tokens);  // ["hello", "world"]
```

## Determinism Guarantee

AI operations in ASTRIXA are **deterministic**, making them safe for blockchain smart contracts:

1. **No floating-point randomness**: All AI operations use heuristic algorithms with deterministic outputs
2. **Hash-based embeddings**: Embeddings are computed from text hash, ensuring reproducibility
3. **Keyword matching**: Sentiment and classification use rule-based keyword matching, not neural networks
4. **Simple algorithms**: Tokenization uses basic whitespace splitting

This means:
- Same input always produces same output
- Results are reproducible across different machines
- Safe for contract state transitions and deterministic execution

## Type System

### AIResult Type

Returned by `ai.infer()`, contains:
- `label` (string): The predicted label/classification
- `score` (number): Confidence score (0.0 to 1.0)

**Type checking:**
```astrixa
let result = ai.infer(ai.model("sentiment"), "Hello");
print(type(result));  // Outputs: "ai_result"
```

## Example: Sentiment Analysis Contract

```astrixa
contract SentimentAnalyzer {
    state: ["last_sentiment"]
    
    fn analyze(text: string) {
        let result = ai.infer(ai.model("sentiment"), text);
        state["last_sentiment"] = result.label;
        emit("SentimentAnalyzed", result);
    }
    
    fn get_last() {
        return state["last_sentiment"];
    }
}
```

## Example: Semantic Search

```astrixa
fn find_similar(query: string, documents: array) {
    let query_embedding = ai.embed(query);
    
    // Compare embeddings (simplified cosine similarity)
    let best_match = null;
    let best_score = -1.0;
    
    for doc in documents {
        let doc_embedding = ai.embed(doc);
        // In real implementation, compute cosine similarity
        // For now, just track matches
    }
    
    return best_match;
}
```

## Example: Text Classification

```astrixa
fn classify_content(text: string) {
    let model = ai.model("classifier");
    let result = ai.infer(model, text);
    
    if result.score > 0.8 {
        print("High confidence: " + result.label);
    } else {
        print("Low confidence: " + result.label);
    }
}
```

## Performance Characteristics

| Operation | Time Complexity | Space Complexity |
|-----------|-----------------|------------------|
| model() | O(1) | O(1) |
| infer() | O(n) where n = text length | O(1) |
| embed() | O(n) where n = text length | O(k) where k = embedding dim |
| tokenize() | O(n) where n = text length | O(m) where m = token count |

## Integration with Gas Model

AI operations have assigned gas costs:
- `ai.model()` - 10 gas
- `ai.infer()` - 50 gas
- `ai.embed()` - 100 gas
- `ai.tokenize()` - 30 gas

These ensure AI contracts don't consume excessive computation.

## Future Extensions

### 1. Remote AI Backend
```rust
// Future: Support calling remote AI services with deterministic caching
let result = ai.infer_remote("openai:gpt4", prompt);
```

### 2. Fine-tuned Models
```rust
// Future: Load fine-tuned models
let model = ai.model("custom:sentiment_finance");
```

### 3. Multi-modal AI
```rust
// Future: Support image, audio, and multimodal inputs
let result = ai.infer(model, image);
```

### 4. Proof-of-Inference
```rust
// Future: Generate cryptographic proofs of AI execution
let proof = ai.prove_infer(model, input);
```

## Best Practices

1. **Cache model loads**: Load models once and reuse
   ```astrixa
   let model = ai.model("sentiment");  // Load once
   let r1 = ai.infer(model, text1);
   let r2 = ai.infer(model, text2);
   ```

2. **Check confidence scores**: Don't blindly trust results
   ```astrixa
   if result.score < 0.5 {
       return error("Inconclusive result");
   }
   ```

3. **Monitor gas usage**: AI operations are expensive
   ```astrixa
   // Check remaining gas before intensive operations
   ```

4. **Use deterministic models**: Always prefer rule-based over learned models in contracts
   - Good: Keyword-based sentiment analysis
   - Risky: Neural network inference without proofs

## Limitations

- No neural network execution (for determinism)
- No image/audio input (text-only)
- No GPU acceleration (deterministic CPU only)
- Embeddings limited to 128 dimensions
- Token limit of 10,000 tokens per call

## Error Handling

```astrixa
fn safe_infer(model_name: string, text: string) {
    try {
        let m = ai.model(model_name);
        let result = ai.infer(m, text);
        print(result);
    } catch (error) {
        print("Error: " + error);
    }
}
```

## See Also

- [Gas Model Documentation](GAS_MODEL.md)
- [Smart Contracts Guide](CONTRACT_GUIDE.md)
- [Language Reference](LANGUAGE_REF.md)
