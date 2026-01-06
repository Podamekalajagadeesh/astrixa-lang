# Standard Library: AI

First-class AI operations in ASTRIXA - no Python dependency hell.

## Quick Start

```astrixa
use std::ai

let text = "ASTRIXA is an amazing programming language!"
let sentiment = ai.sentiment(text)

print("Sentiment: " + sentiment)  // "positive"
```

## Text Analysis

### Sentiment Analysis

Analyze emotional tone of text:

```astrixa
// Basic sentiment
let positive = ai.sentiment("This is great!")           // "positive"
let negative = ai.sentiment("This is terrible")         // "negative"
let neutral = ai.sentiment("The sky is blue")           // "neutral"

// Detailed sentiment scores
let scores = ai.sentimentScores("I love ASTRIXA!")
print("Positive: " + scores.positive)  // 0.95
print("Negative: " + scores.negative)  // 0.02
print("Neutral: " + scores.neutral)    // 0.03
```

### Text Classification

Categorize text into predefined labels:

```astrixa
let text = "How do I reset my password?"

let category = ai.classify(text, [
    "technical_support",
    "billing",
    "general_inquiry"
])

print(category)  // "technical_support"

// With confidence scores
let result = ai.classifyWithScores(text, categories)
print(result.category)     // "technical_support"
print(result.confidence)   // 0.89
```

### Named Entity Recognition

Extract entities from text:

```astrixa
let text = "Alice works at Google in San Francisco"

let entities = ai.extractEntities(text)

for entity in entities {
    print(entity.text + " (" + entity.type + ")")
}
// Output:
// Alice (PERSON)
// Google (ORGANIZATION)
// San Francisco (LOCATION)
```

### Text Summarization

Generate concise summaries:

```astrixa
let longText = """
    ASTRIXA is a modern programming language designed for Web, Web3, and AI.
    It provides first-class support for building web applications, smart contracts,
    and AI-powered features. The language focuses on simplicity, safety, and
    developer productivity.
"""

let summary = ai.summarize(longText, maxLength: 50)
print(summary)
// Output: "ASTRIXA is a language for Web, Web3, and AI development."
```

## Embeddings

Generate vector representations for semantic search:

```astrixa
// Generate embedding
let text = "artificial intelligence"
let embedding = ai.embedding(text)

print(embedding.length)  // 384 (or model-specific dimension)
print(embedding[0])      // 0.123... (first dimension)

// Compare similarity
let text1 = "machine learning"
let text2 = "deep learning"
let text3 = "cooking recipes"

let emb1 = ai.embedding(text1)
let emb2 = ai.embedding(text2)
let emb3 = ai.embedding(text3)

let sim12 = ai.cosineSimilarity(emb1, emb2)  // ~0.85 (very similar)
let sim13 = ai.cosineSimilarity(emb1, emb3)  // ~0.12 (not similar)
```

## Text Generation (coming soon)

Generate text with AI models:

```astrixa
// Simple generation
let response = ai.generate("Write a haiku about programming")
print(response)

// With options
let options = {
    maxTokens: 100,
    temperature: 0.7,
    model: "gpt-3.5-turbo"
}

let response = ai.generateWithOptions(prompt, options)
```

## Tokenization

Convert text to tokens:

```astrixa
let text = "Hello, ASTRIXA!"
let tokens = ai.tokenize(text)

print(tokens)  // ["Hello", ",", "ASTRIXA", "!"]
print(tokens.length)  // 4

// Count tokens (for API limits)
let count = ai.countTokens(longText)
print("Token count: " + count)
```

## Language Detection

Detect the language of text:

```astrixa
let lang1 = ai.detectLanguage("Hello, world!")          // "en"
let lang2 = ai.detectLanguage("Bonjour le monde!")      // "fr"
let lang3 = ai.detectLanguage("こんにちは世界!")           // "ja"

// With confidence
let result = ai.detectLanguageWithConfidence(text)
print(result.language)    // "en"
print(result.confidence)  // 0.98
```

## Practical Examples

### Email Classifier

```astrixa
use std::ai

fn classifyEmail(email: map) -> string {
    let subject = email.subject
    let body = email.body
    let text = subject + " " + body
    
    let category = ai.classify(text, [
        "spam",
        "important",
        "newsletter",
        "personal"
    ])
    
    return category
}

// Usage
let email = {
    subject: "Meeting tomorrow at 3pm",
    body: "Don't forget our team meeting..."
}

let category = classifyEmail(email)
print("Category: " + category)  // "important"
```

### Smart Search

```astrixa
use std::ai

let documents = [
    "ASTRIXA is a programming language for Web3 and AI",
    "Python is popular for machine learning",
    "Rust provides memory safety without garbage collection",
    "JavaScript runs in web browsers"
]

// Generate embeddings for all documents
let docEmbeddings = []
for doc in documents {
    docEmbeddings.push(ai.embedding(doc))
}

fn search(query: string) -> [string] {
    let queryEmb = ai.embedding(query)
    let results = []
    
    for i in 0..documents.length {
        let similarity = ai.cosineSimilarity(queryEmb, docEmbeddings[i])
        results.push({
            doc: documents[i],
            score: similarity
        })
    }
    
    // Sort by relevance
    results.sort(fn(a, b) { return b.score - a.score })
    
    return results[0..3]  // Top 3 results
}

// Search
let results = search("blockchain programming")
for result in results {
    print(result.doc + " (score: " + result.score + ")")
}
```

### Content Moderation

```astrixa
use std::ai

fn moderateContent(text: string) -> map {
    // Check toxicity
    let toxic = ai.classify(text, ["toxic", "safe"]) == "toxic"
    
    // Check sentiment
    let sentiment = ai.sentiment(text)
    
    // Detect language
    let language = ai.detectLanguage(text)
    
    return {
        approved: !toxic && sentiment != "negative",
        reason: toxic ? "Toxic content" : "OK",
        sentiment: sentiment,
        language: language
    }
}

let comment = "This is a great product!"
let moderation = moderateContent(comment)

if moderation.approved {
    print("✅ Comment approved")
} else {
    print("❌ Comment rejected: " + moderation.reason)
}
```

### Auto-tagging

```astrixa
use std::ai

fn generateTags(article: string) -> [string] {
    // Extract key entities
    let entities = ai.extractEntities(article)
    let tags = []
    
    for entity in entities {
        if entity.type == "ORGANIZATION" || entity.type == "TECHNOLOGY" {
            tags.push(entity.text.toLowerCase())
        }
    }
    
    // Add sentiment as tag
    let sentiment = ai.sentiment(article)
    tags.push(sentiment)
    
    return tags
}

let article = """
    Apple announced a new iPhone with advanced AI capabilities.
    The device uses machine learning for enhanced photography.
"""

let tags = generateTags(article)
print("Tags: " + tags.join(", "))
// Output: "apple, iphone, ai, machine learning, positive"
```

## Smart Contract AI (Deterministic)

Use AI in smart contracts with deterministic behavior:

```astrixa
contract SentimentGovernance {
    state proposals: map<int, Proposal>
    state nextId: int
    
    struct Proposal {
        id: int,
        text: string,
        sentiment: string,
        approved: bool
    }
    
    fn submitProposal(text: string) {
        // Deterministic sentiment analysis
        let sentiment = ai.sentiment(text)
        
        let proposal = Proposal {
            id: nextId,
            text: text,
            sentiment: sentiment,
            approved: sentiment == "positive"
        }
        
        proposals[nextId] = proposal
        nextId += 1
    }
    
    fn getProposal(id: int) -> Proposal {
        return proposals[id]
    }
}
```

## Performance Considerations

### Caching Results

```astrixa
let sentimentCache: map<string, string> = {}

fn getSentiment(text: string) -> string {
    if sentimentCache.has(text) {
        return sentimentCache[text]
    }
    
    let sentiment = ai.sentiment(text)
    sentimentCache[text] = sentiment
    return sentiment
}
```

### Batch Processing

```astrixa
// Process multiple texts efficiently
let texts = ["text1", "text2", "text3"]
let results = ai.sentimentBatch(texts)

for i in 0..texts.length {
    print(texts[i] + ": " + results[i])
}
```

## Best Practices

### ✅ Do: Validate Input Length

```astrixa
fn analyzeSafely(text: string) -> string {
    if text.length > 10000 {
        text = text.substring(0, 10000)  // Truncate
    }
    
    return ai.sentiment(text)
}
```

### ✅ Do: Handle Edge Cases

```astrixa
fn getSentiment(text: string) -> string {
    if text.length == 0 {
        return "neutral"
    }
    
    if text.length < 10 {
        return "neutral"  // Too short to analyze
    }
    
    return ai.sentiment(text)
}
```

### ✅ Do: Use Appropriate Models

```astrixa
// For speed: use simple classifiers
let quickResult = ai.sentiment(text)

// For accuracy: use advanced models (coming soon)
let detailedResult = ai.sentimentAdvanced(text, {
    model: "bert-large"
})
```

## Next Steps

- [Web Module →](web.md)
- [Web3 Module →](web3.md)
- [AI Examples →](../../examples/)

---

**Note:** All AI operations in ASTRIXA are deterministic and safe for smart contracts.
