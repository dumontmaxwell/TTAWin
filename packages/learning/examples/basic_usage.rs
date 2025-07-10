use learning::{
    audio::AudioTranscriber,
    llm::LocalLLM,
    ocr::OCREngine,
    analysis::{AnalysisEngine, ContentType, ExtractedText},
    LearningService,
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 TTAWin Learning Package Demo");
    println!("================================\n");

    // Initialize the learning service
    println!("📦 Initializing Learning Service...");
    let learning_service = LearningService::new().await?;
    println!("✅ Learning Service initialized successfully!\n");

    // Example 1: OCR Analysis
    println!("🔍 Example 1: OCR Text Extraction");
    println!("----------------------------------");
    
    // Note: In a real scenario, you would provide an actual image path
    // For this demo, we'll show the structure
    let image_path = "path/to/screenshot.png";
    
    // Extract text from screenshot
    match learning_service.analyze_screenshot(image_path).await {
        Ok(analysis) => {
            println!("📸 Screenshot Analysis Results:");
            println!("   Analysis ID: {}", analysis.analysis_id);
            println!("   Confidence: {:.2}", analysis.confidence);
            println!("   Summary: {}", analysis.summary);
            println!("   Topics: {:?}", analysis.topics);
            println!("   Insights: {:?}", analysis.insights);
            println!("   Recommendations: {}", analysis.recommendations.len());
        }
        Err(e) => {
            println!("❌ OCR Analysis failed: {}", e);
            println!("   (This is expected if no image file is provided)");
        }
    }
    println!();

    // Example 2: Audio Transcription
    println!("🎤 Example 2: Audio Transcription");
    println!("---------------------------------");
    
    let audio_path = "path/to/audio.wav";
    
    // Transcribe and analyze audio
    match learning_service.analyze_audio(audio_path).await {
        Ok(analysis) => {
            println!("🎵 Audio Analysis Results:");
            println!("   Analysis ID: {}", analysis.analysis_id);
            println!("   Confidence: {:.2}", analysis.confidence);
            println!("   Summary: {}", analysis.summary);
            println!("   Sentiment: {:?}", analysis.sentiment);
            println!("   Topics: {:?}", analysis.topics);
        }
        Err(e) => {
            println!("❌ Audio Analysis failed: {}", e);
            println!("   (This is expected if no audio file is provided)");
        }
    }
    println!();

    // Example 3: Direct LLM Usage
    println!("🧠 Example 3: Direct LLM Analysis");
    println!("---------------------------------");
    
    let llm_engine = learning_service.llm_engine();
    let text_to_analyze = "The user is working on a complex coding project and seems to be struggling with debugging. They've been working for several hours and the code still has issues.";
    
    match llm_engine.analyze_with_context(text_to_analyze).await {
        Ok(analysis) => {
            println!("💭 LLM Analysis Results:");
            println!("   Sentiment: {} (score: {:.2})", analysis.sentiment.label, analysis.sentiment.score);
            println!("   Summary: {}", analysis.summary);
            println!("   Insights: {:?}", analysis.insights);
            println!("   Context Used: {}", analysis.context_used);
        }
        Err(e) => {
            println!("❌ LLM Analysis failed: {}", e);
        }
    }
    println!();

    // Example 4: Custom Analysis Engine
    println!("⚙️  Example 4: Custom Analysis Engine");
    println!("-------------------------------------");
    
    let analysis_engine = learning_service.analysis_engine();
    
    // Create custom extracted text
    let extracted_text = ExtractedText {
        source: ContentType::Text,
        text: "The user completed a successful coding session. They solved 3 bugs and implemented 2 new features. The code quality is good and they're making progress.".to_string(),
        confidence: 0.95,
        timestamp: chrono::Utc::now(),
        metadata: std::collections::HashMap::new(),
    };
    
    match analysis_engine.analyze_extracted_text(extracted_text).await {
        Ok(analysis) => {
            println!("📊 Custom Analysis Results:");
            println!("   Analysis ID: {}", analysis.analysis_id);
            println!("   Content Type: {:?}", analysis.content_type);
            println!("   Confidence: {:.2}", analysis.confidence);
            println!("   Summary: {}", analysis.summary);
            println!("   Topics: {:?}", analysis.topics);
            println!("   Recommendations: {}", analysis.recommendations.len());
            
            for (i, rec) in analysis.recommendations.iter().enumerate() {
                println!("     {}. {} (Priority: {:?})", i + 1, rec.title, rec.priority);
            }
        }
        Err(e) => {
            println!("❌ Custom Analysis failed: {}", e);
        }
    }
    println!();

    // Example 5: Session Management
    println!("📋 Example 5: Session Management");
    println!("--------------------------------");
    
    // Get session summary
    match analysis_engine.get_session_summary().await {
        Ok(summary) => {
            println!("📈 Session Summary:");
            println!("   Session ID: {}", summary.session_id);
            println!("   Total Turns: {}", summary.total_turns);
            println!("   Total Texts: {}", summary.total_texts);
            println!("   Created: {}", summary.created_at);
            println!("   Last Updated: {}", summary.last_updated);
            println!("   Key Insights: {:?}", summary.key_insights);
        }
        Err(e) => {
            println!("❌ Session Summary failed: {}", e);
        }
    }
    println!();

    // Example 6: Individual Component Usage
    println!("🔧 Example 6: Individual Component Usage");
    println!("----------------------------------------");
    
    // OCR Engine
    println!("📸 OCR Engine:");
    let ocr_engine = OCREngine::new()?;
    println!("   ✅ OCR Engine created successfully");
    
    // Audio Transcriber
    println!("🎤 Audio Transcriber:");
    let audio_transcriber = AudioTranscriber::new()?;
    println!("   ✅ Audio Transcriber created successfully");
    
    // Local LLM
    println!("🧠 Local LLM:");
    let mut llm = LocalLLM::new().await?;
    println!("   ✅ Local LLM created successfully");
    
    // Test simple text generation
    match llm.generate_text("Hello, how are you?").await {
        Ok(response) => {
            println!("   📝 Generated text: {}", response);
        }
        Err(e) => {
            println!("   ❌ Text generation failed: {}", e);
        }
    }
    
    // Test sentiment analysis
    match llm.analyze_sentiment("I'm feeling great about this project!").await {
        Ok(sentiment) => {
            println!("   😊 Sentiment: {} (score: {:.2})", sentiment.label, sentiment.score);
        }
        Err(e) => {
            println!("   ❌ Sentiment analysis failed: {}", e);
        }
    }
    println!();

    // Example 7: Real-time Processing Simulation
    println!("⏱️  Example 7: Real-time Processing Simulation");
    println!("----------------------------------------------");
    
    let analysis_engine = learning_service.analysis_engine();
    
    // Simulate processing multiple inputs
    let inputs = vec![
        "User is working on a React component",
        "Found a bug in the authentication logic",
        "Successfully implemented the new feature",
        "Need to refactor the database queries",
    ];
    
    for (i, input) in inputs.iter().enumerate() {
        println!("   Processing input {}: {}", i + 1, input);
        
        match analysis_engine.analyze_text(input).await {
            Ok(analysis) => {
                println!("     ✅ Analyzed - Topics: {:?}", analysis.topics);
            }
            Err(e) => {
                println!("     ❌ Analysis failed: {}", e);
            }
        }
    }
    println!();

    println!("🎉 Demo completed successfully!");
    println!("📚 The learning package provides:");
    println!("   • OCR for screenshot text extraction");
    println!("   • Audio transcription and analysis");
    println!("   • Local LLM for text analysis and generation");
    println!("   • Comprehensive analysis engine with session management");
    println!("   • Real-time processing capabilities");
    println!("   • Caching and optimization features");

    Ok(())
} 