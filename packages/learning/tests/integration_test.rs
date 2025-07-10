use learning::{
    LearningService,
    ocr::OCREngine,
    audio::AudioTranscriber,
    llm::LocalLLM,
    analysis::{AnalysisEngine, ContentType, ExtractedText},
};
use std::sync::Arc;

#[tokio::test]
async fn test_learning_service_integration() {
    // Test that the learning service can be created
    let service = LearningService::new().await;
    assert!(service.is_ok(), "Learning service should be created successfully");
}

#[tokio::test]
async fn test_ocr_engine_creation() {
    // Test OCR engine creation
    let ocr_engine = OCREngine::new();
    assert!(ocr_engine.is_ok(), "OCR engine should be created successfully");
}

#[tokio::test]
async fn test_audio_transcriber_creation() {
    // Test audio transcriber creation
    let transcriber = AudioTranscriber::new();
    assert!(transcriber.is_ok(), "Audio transcriber should be created successfully");
}

#[tokio::test]
async fn test_llm_creation() {
    // Test LLM creation
    let llm = LocalLLM::new().await;
    assert!(llm.is_ok(), "Local LLM should be created successfully");
}

#[tokio::test]
async fn test_analysis_engine_creation() {
    // Test analysis engine creation
    let llm = LocalLLM::new().await.unwrap();
    let analysis_engine = AnalysisEngine::new(Arc::new(llm));
    assert!(analysis_engine.config.enable_sentiment_analysis, "Analysis engine should have default config");
}

#[tokio::test]
async fn test_text_analysis_pipeline() {
    // Test the complete text analysis pipeline
    let llm = LocalLLM::new().await.unwrap();
    let analysis_engine = AnalysisEngine::new(Arc::new(llm));
    
    let test_text = "The user is working on a complex coding project and making good progress.";
    
    let result = analysis_engine.analyze_text(test_text).await;
    assert!(result.is_ok(), "Text analysis should succeed");
    
    let analysis = result.unwrap();
    assert_eq!(analysis.content_type, ContentType::Text);
    assert!(!analysis.analysis_id.is_empty());
    assert!(analysis.confidence > 0.0);
}

#[tokio::test]
async fn test_extracted_text_analysis() {
    // Test analysis of extracted text
    let llm = LocalLLM::new().await.unwrap();
    let analysis_engine = AnalysisEngine::new(Arc::new(llm));
    
    let extracted_text = ExtractedText {
        source: ContentType::Text,
        text: "Successfully implemented authentication system".to_string(),
        confidence: 0.95,
        timestamp: chrono::Utc::now(),
        metadata: std::collections::HashMap::new(),
    };
    
    let result = analysis_engine.analyze_extracted_text(extracted_text).await;
    assert!(result.is_ok(), "Extracted text analysis should succeed");
    
    let analysis = result.unwrap();
    assert_eq!(analysis.content_type, ContentType::Text);
    assert_eq!(analysis.confidence, 0.95);
}

#[tokio::test]
async fn test_session_management() {
    // Test session management functionality
    let llm = LocalLLM::new().await.unwrap();
    let analysis_engine = AnalysisEngine::new(Arc::new(llm));
    
    // Get initial session summary
    let initial_summary = analysis_engine.get_session_summary().await;
    assert!(initial_summary.is_ok(), "Should get initial session summary");
    
    let initial = initial_summary.unwrap();
    assert_eq!(initial.total_turns, 0, "Initial session should have 0 turns");
    
    // Add some content to the session
    let _ = analysis_engine.analyze_text("First analysis").await;
    let _ = analysis_engine.analyze_text("Second analysis").await;
    
    // Get updated session summary
    let updated_summary = analysis_engine.get_session_summary().await;
    assert!(updated_summary.is_ok(), "Should get updated session summary");
    
    let updated = updated_summary.unwrap();
    assert_eq!(updated.total_turns, 2, "Session should have 2 turns after analysis");
    
    // Clear session
    analysis_engine.clear_session().await;
    
    let final_summary = analysis_engine.get_session_summary().await.unwrap();
    assert_eq!(final_summary.total_turns, 0, "Session should be cleared");
}

#[tokio::test]
async fn test_llm_text_generation() {
    // Test LLM text generation
    let mut llm = LocalLLM::new().await.unwrap();
    
    let result = llm.generate_text("Hello").await;
    assert!(result.is_ok(), "Text generation should succeed");
    
    let generated = result.unwrap();
    assert!(!generated.is_empty(), "Generated text should not be empty");
}

#[tokio::test]
async fn test_llm_sentiment_analysis() {
    // Test LLM sentiment analysis
    let llm = LocalLLM::new().await.unwrap();
    
    let positive_text = "I'm very happy with the results!";
    let result = llm.analyze_sentiment(positive_text).await;
    assert!(result.is_ok(), "Sentiment analysis should succeed");
    
    let sentiment = result.unwrap();
    assert!(!sentiment.label.is_empty(), "Sentiment label should not be empty");
    assert!(sentiment.score >= 0.0 && sentiment.score <= 1.0, "Sentiment score should be between 0 and 1");
}

#[tokio::test]
async fn test_llm_classification() {
    // Test LLM text classification
    let llm = LocalLLM::new().await.unwrap();
    
    let labels = vec!["technology".to_string(), "business".to_string()];
    let result = llm.classify_text("This is about programming", &labels).await;
    assert!(result.is_ok(), "Text classification should succeed");
    
    let classification = result.unwrap();
    assert!(!classification.is_empty(), "Classification should return results");
    assert!(classification.len() <= labels.len(), "Classification results should not exceed label count");
}

#[tokio::test]
async fn test_context_management() {
    // Test LLM context management
    let llm = LocalLLM::new().await.unwrap();
    
    // Add text to context
    let result = llm.add_to_context("First context item").await;
    assert!(result.is_ok(), "Should add to context");
    
    // Get context
    let context = llm.get_context().await;
    assert_eq!(context.len(), 1, "Context should have one item");
    assert_eq!(context[0], "First context item", "Context should contain the added text");
    
    // Clear context
    llm.clear_context().await;
    let empty_context = llm.get_context().await;
    assert!(empty_context.is_empty(), "Context should be empty after clearing");
}

#[tokio::test]
async fn test_analysis_with_context() {
    // Test LLM analysis with context
    let llm = LocalLLM::new().await.unwrap();
    
    // Add context
    llm.add_to_context("Previous conversation about coding").await.unwrap();
    
    // Analyze new text with context
    let result = llm.analyze_with_context("Current coding session").await;
    assert!(result.is_ok(), "Analysis with context should succeed");
    
    let analysis = result.unwrap();
    assert!(analysis.context_used, "Analysis should indicate context was used");
    assert!(!analysis.summary.is_empty(), "Analysis should provide a summary");
    assert!(!analysis.insights.is_empty(), "Analysis should provide insights");
}

#[tokio::test]
async fn test_recommendation_generation() {
    // Test that recommendations are generated
    let llm = LocalLLM::new().await.unwrap();
    let analysis_engine = AnalysisEngine::new(Arc::new(llm));
    
    let technical_text = "Found a critical bug in the authentication system that needs immediate attention.";
    
    let result = analysis_engine.analyze_text(technical_text).await;
    assert!(result.is_ok(), "Analysis should succeed");
    
    let analysis = result.unwrap();
    assert!(!analysis.recommendations.is_empty(), "Analysis should generate recommendations");
    
    // Check that recommendations have required fields
    for recommendation in &analysis.recommendations {
        assert!(!recommendation.title.is_empty(), "Recommendation should have a title");
        assert!(!recommendation.description.is_empty(), "Recommendation should have a description");
        assert!(!recommendation.action_items.is_empty(), "Recommendation should have action items");
        assert!(recommendation.confidence >= 0.0 && recommendation.confidence <= 1.0, "Recommendation confidence should be between 0 and 1");
    }
}

#[tokio::test]
async fn test_metadata_extraction() {
    // Test metadata extraction from text
    let llm = LocalLLM::new().await.unwrap();
    let analysis_engine = AnalysisEngine::new(Arc::new(llm));
    
    let text_with_metadata = "This is a test message with 123 numbers and http://example.com URL.";
    
    let result = analysis_engine.analyze_text(text_with_metadata).await;
    assert!(result.is_ok(), "Analysis should succeed");
    
    let analysis = result.unwrap();
    assert!(!analysis.metadata.is_empty(), "Analysis should extract metadata");
    
    // Check for specific metadata fields
    assert!(analysis.metadata.contains_key("word_count"), "Should extract word count");
    assert!(analysis.metadata.contains_key("char_count"), "Should extract character count");
    assert!(analysis.metadata.contains_key("contains_numbers"), "Should detect numbers");
    assert!(analysis.metadata.contains_key("contains_urls"), "Should detect URLs");
}

#[tokio::test]
async fn test_error_handling() {
    // Test error handling for invalid inputs
    let llm = LocalLLM::new().await.unwrap();
    let analysis_engine = AnalysisEngine::new(Arc::new(llm));
    
    // Test with empty text
    let result = analysis_engine.analyze_text("").await;
    assert!(result.is_ok(), "Empty text should be handled gracefully");
    
    // Test with very long text (should still work)
    let long_text = "This is a very long text. ".repeat(1000);
    let result = analysis_engine.analyze_text(&long_text).await;
    assert!(result.is_ok(), "Long text should be handled gracefully");
} 