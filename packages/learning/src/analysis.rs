use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::llm::{AnalysisResult as LLMAnalysisResult, LocalLLM, SentimentResult};

/// Configuration for the analysis engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    pub enable_sentiment_analysis: bool,
    pub enable_topic_extraction: bool,
    pub enable_summarization: bool,
    pub enable_insight_generation: bool,
    pub max_context_length: usize,
    pub confidence_threshold: f32,
    pub enable_caching: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            enable_sentiment_analysis: true,
            enable_topic_extraction: true,
            enable_summarization: true,
            enable_insight_generation: true,
            max_context_length: 10000,
            confidence_threshold: 0.6,
            enable_caching: true,
        }
    }
}

/// Analysis engine that coordinates OCR, audio, and LLM analysis
pub struct AnalysisEngine {
    llm: Arc<LocalLLM>,
    config: AnalysisConfig,
    cache: Arc<Mutex<std::collections::HashMap<String, AnalysisResult>>>,
    session_context: Arc<Mutex<SessionContext>>,
}

/// Session context for maintaining conversation state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: String,
    pub conversation_history: Vec<ConversationTurn>,
    pub extracted_texts: Vec<ExtractedText>,
    pub analysis_summary: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// A single turn in the conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationTurn {
    pub turn_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub content_type: ContentType,
    pub content: String,
    pub analysis: Option<AnalysisResult>,
    pub user_feedback: Option<String>,
}

/// Type of content being analyzed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Screenshot,
    Audio,
    Text,
    Combined,
}

/// Extracted text from various sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedText {
    pub source: ContentType,
    pub text: String,
    pub confidence: f32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Comprehensive analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub analysis_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub content_type: ContentType,
    pub original_content: String,
    pub extracted_text: Option<ExtractedText>,
    pub sentiment: Option<SentimentResult>,
    pub topics: Vec<String>,
    pub summary: String,
    pub insights: Vec<String>,
    pub confidence: f32,
    pub context_used: bool,
    pub recommendations: Vec<Recommendation>,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
}

/// Actionable recommendation based on analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub category: RecommendationCategory,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub action_items: Vec<String>,
    pub confidence: f32,
}

/// Categories of recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    Learning,
    Productivity,
    Communication,
    Technical,
    General,
}

/// Priority levels for recommendations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl AnalysisEngine {
    /// Create a new analysis engine
    pub fn new(llm: Arc<LocalLLM>) -> Self {
        let config = AnalysisConfig::default();
        let cache = Arc::new(Mutex::new(std::collections::HashMap::new()));
        let session_context = Arc::new(Mutex::new(SessionContext::new()));

        Self {
            llm,
            config,
            cache,
            session_context,
        }
    }

    /// Create analysis engine with custom configuration
    pub fn with_config(llm: Arc<LocalLLM>, config: AnalysisConfig) -> Self {
        let cache = Arc::new(Mutex::new(std::collections::HashMap::new()));
        let session_context = Arc::new(Mutex::new(SessionContext::new()));

        Self {
            llm,
            config,
            cache,
            session_context,
        }
    }

    /// Analyze text content
    pub async fn analyze_text(&self, text: &str) -> Result<AnalysisResult> {
        // Check cache first
        if self.config.enable_caching {
            let cache_key = format!("text:{}", text);
            let cache = self.cache.lock().await;
            if let Some(cached_result) = cache.get(&cache_key) {
                return Ok(cached_result.clone());
            }
        }

        // Add to LLM context
        self.llm.add_to_context(text).await?;

        // Perform LLM analysis
        let llm_analysis = self.llm.analyze_with_context(text).await?;

        // Create comprehensive analysis result
        let analysis_result = AnalysisResult {
            analysis_id: self.generate_analysis_id(),
            timestamp: chrono::Utc::now(),
            content_type: ContentType::Text,
            original_content: text.to_string(),
            extracted_text: None,
            sentiment: Some(llm_analysis.sentiment),
            topics: llm_analysis.topics,
            summary: llm_analysis.summary,
            insights: llm_analysis.insights,
            confidence: self.calculate_confidence(text),
            context_used: llm_analysis.context_used,
            recommendations: self.generate_recommendations(text, &llm_analysis).await?,
            metadata: self.extract_metadata(text),
        };

        // Cache the result
        if self.config.enable_caching {
            let cache_key = format!("text:{}", text);
            let mut cache = self.cache.lock().await;
            cache.insert(cache_key, analysis_result.clone());
        }

        // Update session context
        self.update_session_context(&analysis_result).await?;

        Ok(analysis_result)
    }

    /// Analyze extracted text from OCR or audio
    pub async fn analyze_extracted_text(&self, extracted_text: ExtractedText) -> Result<AnalysisResult> {
        let text = &extracted_text.text;
        
        // Check cache first
        if self.config.enable_caching {
            let cache_key = format!("extracted:{}", text);
            let cache = self.cache.lock().await;
            if let Some(cached_result) = cache.get(&cache_key) {
                return Ok(cached_result.clone());
            }
        }

        // Add to LLM context
        self.llm.add_to_context(text).await?;

        // Perform LLM analysis
        let llm_analysis = self.llm.analyze_with_context(text).await?;

        // Create comprehensive analysis result
        let analysis_result = AnalysisResult {
            analysis_id: self.generate_analysis_id(),
            timestamp: chrono::Utc::now(),
            content_type: extracted_text.source.clone(),
            original_content: text.to_string(),
            extracted_text: Some(extracted_text.clone()),
            sentiment: Some(llm_analysis.sentiment),
            topics: llm_analysis.topics,
            summary: llm_analysis.summary,
            insights: llm_analysis.insights,
            confidence: extracted_text.confidence,
            context_used: llm_analysis.context_used,
            recommendations: self.generate_recommendations(text, &llm_analysis).await?,
            metadata: self.extract_metadata(text),
        };

        // Cache the result
        if self.config.enable_caching {
            let cache_key = format!("extracted:{}", text);
            let mut cache = self.cache.lock().await;
            cache.insert(cache_key, analysis_result.clone());
        }

        // Update session context
        self.update_session_context(&analysis_result).await?;

        Ok(analysis_result)
    }

    /// Analyze multiple content sources together
    pub async fn analyze_combined(&self, contents: Vec<ExtractedText>) -> Result<AnalysisResult> {
        if contents.is_empty() {
            return Err(anyhow::anyhow!("No content provided for analysis"));
        }

        // Combine all texts
        let combined_text = contents
            .iter()
            .map(|content| content.text.clone())
            .collect::<Vec<_>>()
            .join("\n\n");

        // Add to LLM context
        self.llm.add_to_context(&combined_text).await?;

        // Perform LLM analysis
        let llm_analysis = self.llm.analyze_with_context(&combined_text).await?;

        // Calculate overall confidence
        let overall_confidence = contents
            .iter()
            .map(|content| content.confidence)
            .sum::<f32>() / contents.len() as f32;

        // Create comprehensive analysis result
        let analysis_result = AnalysisResult {
            analysis_id: self.generate_analysis_id(),
            timestamp: chrono::Utc::now(),
            content_type: ContentType::Combined,
            original_content: combined_text.clone(),
            extracted_text: None,
            sentiment: Some(llm_analysis.sentiment),
            topics: llm_analysis.topics,
            summary: llm_analysis.summary,
            insights: llm_analysis.insights,
            confidence: overall_confidence,
            context_used: llm_analysis.context_used,
            recommendations: self.generate_recommendations(&combined_text, &llm_analysis).await?,
            metadata: self.extract_metadata(&combined_text),
        };

        // Update session context
        self.update_session_context(&analysis_result).await?;

        Ok(analysis_result)
    }

    /// Get session summary
    pub async fn get_session_summary(&self) -> Result<SessionSummary> {
        let context = self.session_context.lock().await;
        
        let total_turns = context.conversation_history.len();
        let total_texts = context.extracted_texts.len();
        
        let sentiment_summary = self.calculate_session_sentiment(&context.conversation_history).await?;
        let topic_summary = self.calculate_session_topics(&context.conversation_history).await?;
        
        let summary = SessionSummary {
            session_id: context.session_id.clone(),
            total_turns,
            total_texts,
            sentiment_summary,
            topic_summary,
            created_at: context.created_at,
            last_updated: context.last_updated,
            key_insights: self.extract_key_insights(&context.conversation_history).await?,
        };

        Ok(summary)
    }

    /// Clear session context
    pub async fn clear_session(&self) {
        let mut context = self.session_context.lock().await;
        *context = SessionContext::new();
        self.llm.clear_context().await;
    }

    /// Generate analysis ID
    fn generate_analysis_id(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        format!("analysis_{}", timestamp)
    }

    /// Calculate confidence score for analysis
    fn calculate_confidence(&self, text: &str) -> f32 {
        // Simple confidence calculation based on text characteristics
        let word_count = text.split_whitespace().count();
        let char_count = text.chars().count();
        
        if word_count == 0 {
            return 0.0;
        }

        let avg_word_length = char_count as f32 / word_count as f32;
        let length_confidence = (word_count as f32 / 100.0).min(1.0);
        let complexity_confidence = (avg_word_length / 8.0).min(1.0);
        
        (length_confidence + complexity_confidence) / 2.0
    }

    /// Generate recommendations based on analysis
    async fn generate_recommendations(
        &self,
        text: &str,
        llm_analysis: &LLMAnalysisResult,
    ) -> Result<Vec<Recommendation>> {
        let mut recommendations = Vec::new();

        // Analyze sentiment for recommendations
        if let Some(sentiment) = &llm_analysis.sentiment {
            match sentiment.label.as_str() {
                "NEGATIVE" => {
                    recommendations.push(Recommendation {
                        category: RecommendationCategory::Communication,
                        title: "Address Negative Sentiment".to_string(),
                        description: "Consider addressing the negative aspects mentioned in the content.".to_string(),
                        priority: Priority::High,
                        action_items: vec![
                            "Review the content for improvement opportunities".to_string(),
                            "Consider alternative approaches".to_string(),
                        ],
                        confidence: sentiment.score,
                    });
                }
                "POSITIVE" => {
                    recommendations.push(Recommendation {
                        category: RecommendationCategory::Learning,
                        title: "Build on Positive Aspects".to_string(),
                        description: "Leverage the positive elements identified in the content.".to_string(),
                        priority: Priority::Medium,
                        action_items: vec![
                            "Document successful approaches".to_string(),
                            "Share positive insights with team".to_string(),
                        ],
                        confidence: sentiment.score,
                    });
                }
                _ => {}
            }
        }

        // Analyze topics for recommendations
        for topic in &llm_analysis.topics {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Learning,
                title: format!("Explore {}", topic),
                description: format!("Consider diving deeper into the topic of {}.", topic),
                priority: Priority::Medium,
                action_items: vec![
                    format!("Research more about {}", topic),
                    "Document key learnings".to_string(),
                ],
                confidence: 0.7,
            });
        }

        // Generate content-specific recommendations
        if text.contains("error") || text.contains("problem") {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Technical,
                title: "Technical Issue Identified".to_string(),
                description: "Technical problems or errors were mentioned in the content.".to_string(),
                priority: Priority::High,
                action_items: vec![
                    "Investigate the technical issue".to_string(),
                    "Document the problem and solution".to_string(),
                    "Consider preventive measures".to_string(),
                ],
                confidence: 0.8,
            });
        }

        if text.contains("learn") || text.contains("study") {
            recommendations.push(Recommendation {
                category: RecommendationCategory::Learning,
                title: "Learning Opportunity".to_string(),
                description: "Learning-related content was identified.".to_string(),
                priority: Priority::Medium,
                action_items: vec![
                    "Create study materials".to_string(),
                    "Schedule review sessions".to_string(),
                    "Track learning progress".to_string(),
                ],
                confidence: 0.7,
            });
        }

        Ok(recommendations)
    }

    /// Extract metadata from text
    fn extract_metadata(&self, text: &str) -> std::collections::HashMap<String, serde_json::Value> {
        let mut metadata = std::collections::HashMap::new();
        
        // Extract basic statistics
        metadata.insert("word_count".to_string(), serde_json::Value::Number(
            serde_json::Number::from(text.split_whitespace().count())
        ));
        metadata.insert("char_count".to_string(), serde_json::Value::Number(
            serde_json::Number::from(text.chars().count())
        ));
        metadata.insert("sentence_count".to_string(), serde_json::Value::Number(
            serde_json::Number::from(text.split('.').count())
        ));

        // Extract language indicators
        let has_urls = text.contains("http") || text.contains("www");
        metadata.insert("contains_urls".to_string(), serde_json::Value::Bool(has_urls));
        
        let has_numbers = text.chars().any(|c| c.is_numeric());
        metadata.insert("contains_numbers".to_string(), serde_json::Value::Bool(has_numbers));

        metadata
    }

    /// Update session context with new analysis
    async fn update_session_context(&self, analysis: &AnalysisResult) -> Result<()> {
        let mut context = self.session_context.lock().await;
        
        // Add to conversation history
        let turn = ConversationTurn {
            turn_id: analysis.analysis_id.clone(),
            timestamp: analysis.timestamp,
            content_type: analysis.content_type.clone(),
            content: analysis.original_content.clone(),
            analysis: Some(analysis.clone()),
            user_feedback: None,
        };
        
        context.conversation_history.push(turn);
        context.last_updated = chrono::Utc::now();

        // Add extracted text if available
        if let Some(extracted_text) = &analysis.extracted_text {
            context.extracted_texts.push(extracted_text.clone());
        }

        Ok(())
    }

    /// Calculate session sentiment summary
    async fn calculate_session_sentiment(&self, history: &[ConversationTurn]) -> Result<SentimentSummary> {
        let mut positive_count = 0;
        let mut negative_count = 0;
        let mut neutral_count = 0;
        let mut total_score = 0.0;

        for turn in history {
            if let Some(analysis) = &turn.analysis {
                if let Some(sentiment) = &analysis.sentiment {
                    total_score += sentiment.score;
                    match sentiment.label.as_str() {
                        "POSITIVE" => positive_count += 1,
                        "NEGATIVE" => negative_count += 1,
                        _ => neutral_count += 1,
                    }
                }
            }
        }

        let total_turns = history.len();
        let avg_score = if total_turns > 0 { total_score / total_turns as f32 } else { 0.0 };

        Ok(SentimentSummary {
            positive_count,
            negative_count,
            neutral_count,
            average_score: avg_score,
            dominant_sentiment: if positive_count > negative_count && positive_count > neutral_count {
                "POSITIVE".to_string()
            } else if negative_count > positive_count && negative_count > neutral_count {
                "NEGATIVE".to_string()
            } else {
                "NEUTRAL".to_string()
            },
        })
    }

    /// Calculate session topic summary
    async fn calculate_session_topics(&self, history: &[ConversationTurn]) -> Result<TopicSummary> {
        let mut topic_counts = std::collections::HashMap::new();

        for turn in history {
            if let Some(analysis) = &turn.analysis {
                for topic in &analysis.topics {
                    *topic_counts.entry(topic.clone()).or_insert(0) += 1;
                }
            }
        }

        let mut sorted_topics: Vec<_> = topic_counts.into_iter().collect();
        sorted_topics.sort_by(|a, b| b.1.cmp(&a.1));

        Ok(TopicSummary {
            top_topics: sorted_topics.into_iter().take(5).collect(),
            total_unique_topics: topic_counts.len(),
        })
    }

    /// Extract key insights from session
    async fn extract_key_insights(&self, history: &[ConversationTurn]) -> Result<Vec<String>> {
        let mut insights = Vec::new();

        // Extract insights from recent turns
        let recent_turns: Vec<_> = history.iter().rev().take(5).collect();
        
        for turn in recent_turns {
            if let Some(analysis) = &turn.analysis {
                insights.extend(analysis.insights.clone());
            }
        }

        // Deduplicate and limit insights
        insights = insights.into_iter().collect::<std::collections::HashSet<_>>().into_iter().collect();
        insights.truncate(10);

        Ok(insights)
    }
}

impl SessionContext {
    fn new() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            conversation_history: Vec::new(),
            extracted_texts: Vec::new(),
            analysis_summary: None,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
        }
    }
}

/// Summary of session sentiment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentSummary {
    pub positive_count: usize,
    pub negative_count: usize,
    pub neutral_count: usize,
    pub average_score: f32,
    pub dominant_sentiment: String,
}

/// Summary of session topics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicSummary {
    pub top_topics: Vec<(String, usize)>,
    pub total_unique_topics: usize,
}

/// Complete session summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub session_id: String,
    pub total_turns: usize,
    pub total_texts: usize,
    pub sentiment_summary: SentimentSummary,
    pub topic_summary: TopicSummary,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub key_insights: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LocalLLM;

    #[tokio::test]
    async fn test_analysis_engine_creation() {
        let llm = LocalLLM::new().await.unwrap();
        let engine = AnalysisEngine::new(Arc::new(llm));
        assert!(engine.config.enable_sentiment_analysis);
    }

    #[tokio::test]
    async fn test_text_analysis() {
        let llm = LocalLLM::new().await.unwrap();
        let engine = AnalysisEngine::new(Arc::new(llm));
        
        let result = engine.analyze_text("This is a test message for analysis.").await;
        assert!(result.is_ok());
        
        let analysis = result.unwrap();
        assert_eq!(analysis.content_type, ContentType::Text);
        assert!(!analysis.analysis_id.is_empty());
    }

    #[tokio::test]
    async fn test_session_management() {
        let llm = LocalLLM::new().await.unwrap();
        let engine = AnalysisEngine::new(Arc::new(llm));
        
        // Test session summary
        let summary = engine.get_session_summary().await;
        assert!(summary.is_ok());
        
        // Test clearing session
        engine.clear_session().await;
        let summary_after_clear = engine.get_session_summary().await.unwrap();
        assert_eq!(summary_after_clear.total_turns, 0);
    }
} 