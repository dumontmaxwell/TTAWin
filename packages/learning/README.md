# TTAWin Learning Package

A comprehensive Rust package for intelligent content analysis, featuring local OCR, audio transcription, and a custom local LLM for interview session analysis and insights.

## 🚀 Features

### 📸 OCR (Optical Character Recognition)
- **Local text extraction** from screenshots and images
- **Advanced preprocessing** with noise reduction and contrast enhancement
- **Structured data extraction** (URLs, emails, phone numbers)
- **Multi-language support** with configurable language packs
- **Confidence scoring** for extracted text quality

### 🎤 Audio Transcription
- **Real-time audio processing** with Voice Activity Detection (VAD)
- **Noise reduction** and audio normalization
- **Multiple format support** (WAV, MP3, FLAC)
- **Streaming transcription** for live audio analysis
- **Audio feature extraction** for content analysis

### 🧠 Local LLM (Large Language Model)
- **Custom-built local model** without external API dependencies
- **Text generation** and analysis capabilities
- **Sentiment analysis** with confidence scoring
- **Zero-shot classification** for topic identification
- **Question answering** based on context
- **Context window management** for conversation tracking

### 📊 Analysis Engine
- **Comprehensive content analysis** combining all components
- **Session management** with conversation history
- **Intelligent recommendations** based on content analysis
- **Caching system** for performance optimization
- **Real-time insights** and actionable feedback

## 📦 Installation

Add the learning package to your `Cargo.toml`:

```toml
[dependencies]
learning = { path = "packages/learning" }
```

## 🔧 Quick Start

### Basic Usage

```rust
use learning::LearningService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the learning service
    let learning_service = LearningService::new().await?;
    
    // Analyze a screenshot
    let screenshot_analysis = learning_service.analyze_screenshot("path/to/screenshot.png").await?;
    println!("Screenshot analysis: {:?}", screenshot_analysis);
    
    // Transcribe and analyze audio
    let audio_analysis = learning_service.analyze_audio("path/to/audio.wav").await?;
    println!("Audio analysis: {:?}", audio_analysis);
    
    Ok(())
}
```

### Individual Component Usage

#### OCR Engine

```rust
use learning::ocr::OCREngine;

let ocr_engine = OCREngine::new()?;

// Extract text from image
let text = ocr_engine.extract_text("screenshot.png").await?;
println!("Extracted text: {}", text);

// Get confidence score
let confidence = ocr_engine.get_confidence("screenshot.png").await?;
println!("Confidence: {:.2}", confidence);
```

#### Audio Transcriber

```rust
use learning::audio::AudioTranscriber;

let transcriber = AudioTranscriber::new()?;

// Transcribe audio file
let transcription = transcriber.transcribe("audio.wav").await?;
println!("Transcription: {}", transcription);

// Start real-time transcription
let (rx, handle) = transcriber.start_realtime_transcription().await?;
// Process incoming transcriptions...
```

#### Local LLM

```rust
use learning::llm::LocalLLM;

let mut llm = LocalLLM::new().await?;

// Generate text
let generated = llm.generate_text("Complete this sentence: The best way to learn is").await?;
println!("Generated: {}", generated);

// Analyze sentiment
let sentiment = llm.analyze_sentiment("I'm excited about this project!").await?;
println!("Sentiment: {} (score: {:.2})", sentiment.label, sentiment.score);

// Classify text
let labels = vec!["technology".to_string(), "business".to_string(), "education".to_string()];
let classification = llm.classify_text("This is about machine learning", &labels).await?;
println!("Classification: {:?}", classification);
```

#### Analysis Engine

```rust
use learning::analysis::{AnalysisEngine, ContentType, ExtractedText};

let analysis_engine = AnalysisEngine::new(Arc::new(llm));

// Analyze text
let analysis = analysis_engine.analyze_text("User is working on a complex coding project").await?;
println!("Analysis: {:?}", analysis);

// Get session summary
let summary = analysis_engine.get_session_summary().await?;
println!("Session summary: {:?}", summary);
```

## 🎯 Use Cases

### Interview Session Analysis
- **Real-time transcription** of interview conversations
- **Sentiment tracking** throughout the session
- **Topic identification** and trend analysis
- **Actionable insights** for interview improvement

### Learning Progress Tracking
- **Screenshot analysis** of coding sessions
- **Code quality assessment** through text analysis
- **Learning pattern recognition**
- **Personalized recommendations**

### Productivity Enhancement
- **Content summarization** for quick review
- **Focus area identification**
- **Time management insights**
- **Goal tracking and progress monitoring**

## ⚙️ Configuration

### OCR Configuration

```rust
use learning::ocr::OCRConfig;

let config = OCRConfig {
    language: "eng".to_string(),
    confidence_threshold: 0.7,
    preprocess_image: true,
    extract_structured_data: true,
};

let ocr_engine = OCREngine::with_config(config)?;
```

### Audio Configuration

```rust
use learning::audio::AudioConfig;
use std::time::Duration;

let config = AudioConfig {
    sample_rate: 16000,
    channels: 1,
    buffer_size: 4096,
    silence_threshold: 0.01,
    min_audio_length: Duration::from_millis(500),
    noise_reduction: true,
    vad_enabled: true,
};

let transcriber = AudioTranscriber::with_config(config);
```

### LLM Configuration

```rust
use learning::llm::LLMConfig;

let config = LLMConfig {
    model_name: "gpt2".to_string(),
    max_length: 512,
    temperature: 0.7,
    top_p: 0.9,
    top_k: 50,
    use_gpu: false,
    batch_size: 1,
    context_window_size: 2048,
};

let llm = LocalLLM::with_config(config).await?;
```

## 🔍 Advanced Features

### Custom Analysis Pipelines

```rust
use learning::analysis::{AnalysisEngine, ExtractedText, ContentType};

// Create custom extracted text
let extracted_text = ExtractedText {
    source: ContentType::Text,
    text: "Custom content for analysis".to_string(),
    confidence: 0.95,
    timestamp: chrono::Utc::now(),
    metadata: std::collections::HashMap::new(),
};

// Analyze with custom pipeline
let analysis = analysis_engine.analyze_extracted_text(extracted_text).await?;
```

### Session Management

```rust
// Get comprehensive session summary
let summary = analysis_engine.get_session_summary().await?;

// Clear session for new conversation
analysis_engine.clear_session().await;
```

### Real-time Processing

```rust
// Process multiple inputs in real-time
let inputs = vec![
    "User started coding session",
    "Found a bug in authentication",
    "Successfully implemented feature",
];

for input in inputs {
    let analysis = analysis_engine.analyze_text(input).await?;
    println!("Real-time analysis: {:?}", analysis);
}
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo test
```

Run the example:

```bash
cargo run --example basic_usage
```

## 📊 Performance

- **OCR Processing**: ~100-500ms per image (depending on size and complexity)
- **Audio Transcription**: ~50-200ms per second of audio
- **LLM Analysis**: ~10-100ms per text input
- **Memory Usage**: ~50-200MB (depending on model size and configuration)

## 🔒 Privacy & Security

- **100% Local Processing**: No data sent to external services
- **No Internet Required**: All analysis happens on-device
- **Configurable Caching**: Optional caching with local storage
- **Session Isolation**: Each session is independent and isolated

## 🛠️ Development

### Building from Source

```bash
cd packages/learning
cargo build --release
```

### Running Tests

```bash
cargo test --all-features
```

### Code Coverage

```bash
cargo tarpaulin --out Html
```

## 📚 API Reference

### Core Types

- `LearningService`: Main service orchestrating all components
- `OCREngine`: Text extraction from images
- `AudioTranscriber`: Audio to text conversion
- `LocalLLM`: Local language model for analysis
- `AnalysisEngine`: Comprehensive analysis coordination

### Key Structs

- `AnalysisResult`: Complete analysis output
- `ExtractedText`: Text extracted from various sources
- `SessionContext`: Conversation and analysis history
- `Recommendation`: Actionable insights and suggestions

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

For questions, issues, or feature requests:

1. Check the [documentation](docs/)
2. Search existing [issues](../../issues)
3. Create a new issue with detailed information

## 🎯 Roadmap

- [ ] GPU acceleration for LLM processing
- [ ] Additional language support for OCR
- [ ] Advanced audio processing features
- [ ] Integration with external learning platforms
- [ ] Mobile platform support
- [ ] Cloud synchronization (optional) 