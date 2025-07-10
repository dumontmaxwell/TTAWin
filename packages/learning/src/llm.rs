use anyhow::Result;
use rust_bert::{
    pipelines::{
        common::ModelType,
        text_generation::{TextGenerationConfig, TextGenerationModel},
        zero_shot_classification::{ZeroShotClassificationConfig, ZeroShotClassificationModel},
        sentiment_analysis::{SentimentAnalysisConfig, SentimentAnalysisModel},
        question_answering::{QuestionAnsweringConfig, QuestionAnsweringModel},
    },
    resources::RemoteResource,
    Config,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokenizers::Tokenizer;

/// Configuration for the local LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub model_name: String,
    pub max_length: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: usize,
    pub use_gpu: bool,
    pub batch_size: usize,
    pub context_window_size: usize,
}

impl Default for LLMConfig {
    fn default() -> Self {
        Self {
            model_name: "gpt2".to_string(),
            max_length: 512,
            temperature: 0.7,
            top_p: 0.9,
            top_k: 50,
            use_gpu: false,
            batch_size: 1,
            context_window_size: 2048,
        }
    }
}

/// Local LLM for text analysis and generation
pub struct LocalLLM {
    config: LLMConfig,
    text_generator: Option<TextGenerationModel>,
    zero_shot_classifier: Option<ZeroShotClassificationModel>,
    sentiment_analyzer: Option<SentimentAnalysisModel>,
    qa_model: Option<QuestionAnsweringModel>,
    tokenizer: Option<Tokenizer>,
    context_window: Arc<Mutex<Vec<String>>>,
}

impl LocalLLM {
    /// Create a new local LLM instance
    pub async fn new() -> Result<Self> {
        let config = LLMConfig::default();
        Self::with_config(config).await
    }

    /// Create local LLM with custom configuration
    pub async fn with_config(config: LLMConfig) -> Result<Self> {
        let context_window = Arc::new(Mutex::new(Vec::new()));
        
        Ok(Self {
            config,
            text_generator: None,
            zero_shot_classifier: None,
            sentiment_analyzer: None,
            qa_model: None,
            tokenizer: None,
            context_window,
        })
    }

    /// Initialize the text generation model
    pub async fn initialize_text_generator(&mut self) -> Result<()> {
        let config = TextGenerationConfig {
            model_type: ModelType::GPT2,
            max_length: Some(self.config.max_length),
            temperature: Some(self.config.temperature),
            top_p: Some(self.config.top_p),
            top_k: Some(self.config.top_k),
            do_sample: Some(true),
            ..Default::default()
        };

        self.text_generator = Some(TextGenerationModel::new(config)?);
        Ok(())
    }

    /// Initialize the zero-shot classification model
    pub async fn initialize_zero_shot_classifier(&mut self) -> Result<()> {
        let config = ZeroShotClassificationConfig {
            model_type: ModelType::BERT,
            ..Default::default()
        };

        self.zero_shot_classifier = Some(ZeroShotClassificationModel::new(config)?);
        Ok(())
    }

    /// Initialize the sentiment analysis model
    pub async fn initialize_sentiment_analyzer(&mut self) -> Result<()> {
        let config = SentimentAnalysisConfig {
            model_type: ModelType::BERT,
            ..Default::default()
        };

        self.sentiment_analyzer = Some(SentimentAnalysisModel::new(config)?);
        Ok(())
    }

    /// Initialize the question answering model
    pub async fn initialize_qa_model(&mut self) -> Result<()> {
        let config = QuestionAnsweringConfig {
            model_type: ModelType::BERT,
            ..Default::default()
        };

        self.qa_model = Some(QuestionAnsweringModel::new(config)?);
        Ok(())
    }

    /// Initialize the tokenizer
    pub async fn initialize_tokenizer(&mut self) -> Result<()> {
        let tokenizer_resource = Box::new(RemoteResource::from_pretrained(
            ("bert-base-uncased".to_string(), "tokenizer.json".to_string()),
        ));
        
        self.tokenizer = Some(Tokenizer::from_file(tokenizer_resource.get_local_path()?)?);
        Ok(())
    }

    /// Generate text based on a prompt
    pub async fn generate_text(&self, prompt: &str) -> Result<String> {
        if let Some(generator) = &self.text_generator {
            let output = generator.generate(&[prompt], None)?;
            Ok(output[0].text.clone())
        } else {
            // Fallback to simple text generation
            self.simple_text_generation(prompt).await
        }
    }

    /// Classify text using zero-shot learning
    pub async fn classify_text(&self, text: &str, labels: &[String]) -> Result<Vec<(String, f32)>> {
        if let Some(classifier) = &self.zero_shot_classifier {
            let output = classifier.predict_multilabel(&[text], labels, None, 128)?;
            let mut results = Vec::new();
            
            for (i, label) in labels.iter().enumerate() {
                results.push((label.clone(), output[0][i]));
            }
            
            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            Ok(results)
        } else {
            // Fallback to simple classification
            self.simple_classification(text, labels).await
        }
    }

    /// Analyze sentiment of text
    pub async fn analyze_sentiment(&self, text: &str) -> Result<SentimentResult> {
        if let Some(analyzer) = &self.sentiment_analyzer {
            let output = analyzer.predict(&[text])?;
            let sentiment = &output[0];
            
            Ok(SentimentResult {
                label: sentiment.label.clone(),
                score: sentiment.score,
                text: text.to_string(),
            })
        } else {
            // Fallback to simple sentiment analysis
            self.simple_sentiment_analysis(text).await
        }
    }

    /// Answer questions based on context
    pub async fn answer_question(&self, question: &str, context: &str) -> Result<AnswerResult> {
        if let Some(qa_model) = &self.qa_model {
            let output = qa_model.predict(&[(&question, &context)])?;
            let answer = &output[0];
            
            Ok(AnswerResult {
                answer: answer.answer.clone(),
                score: answer.score,
                start: answer.start,
                end: answer.end,
            })
        } else {
            // Fallback to simple question answering
            self.simple_question_answering(question, context).await
        }
    }

    /// Add text to the context window
    pub async fn add_to_context(&self, text: &str) -> Result<()> {
        let mut context = self.context_window.lock().await;
        
        // Tokenize the text to check length
        if let Some(tokenizer) = &self.tokenizer {
            let tokens = tokenizer.encode(text, true)?;
            let token_count = tokens.get_tokens().len();
            
            // If adding this text would exceed context window, remove oldest entries
            let mut current_tokens = 0;
            for existing_text in context.iter() {
                let existing_tokens = tokenizer.encode(existing_text, true)?.get_tokens().len();
                current_tokens += existing_tokens;
            }
            
            while current_tokens + token_count > self.config.context_window_size && !context.is_empty() {
                let removed = context.remove(0);
                let removed_tokens = tokenizer.encode(&removed, true)?.get_tokens().len();
                current_tokens -= removed_tokens;
            }
        }
        
        context.push(text.to_string());
        Ok(())
    }

    /// Get the current context window
    pub async fn get_context(&self) -> Vec<String> {
        self.context_window.lock().await.clone()
    }

    /// Clear the context window
    pub async fn clear_context(&self) {
        let mut context = self.context_window.lock().await;
        context.clear();
    }

    /// Analyze text using the context window
    pub async fn analyze_with_context(&self, text: &str) -> Result<AnalysisResult> {
        let context = self.get_context().await;
        let context_text = context.join("\n");
        
        // Combine context and current text
        let full_text = if context_text.is_empty() {
            text.to_string()
        } else {
            format!("Context: {}\n\nCurrent: {}", context_text, text)
        };

        // Perform various analyses
        let sentiment = self.analyze_sentiment(text).await?;
        let topics = self.extract_topics(&full_text).await?;
        let summary = self.generate_summary(&full_text).await?;
        let insights = self.generate_insights(&full_text).await?;

        Ok(AnalysisResult {
            sentiment,
            topics,
            summary,
            insights,
            context_used: !context.is_empty(),
        })
    }

    /// Simple text generation fallback
    async fn simple_text_generation(&self, prompt: &str) -> Result<String> {
        // Simple template-based generation
        let templates = [
            "Based on the prompt '{}', I can provide the following analysis:",
            "The text '{}' suggests the following insights:",
            "Analyzing '{}' reveals several key points:",
        ];
        
        let template = templates[prompt.len() % templates.len()];
        let response = format!("{} This is a placeholder response that would be generated by a more sophisticated model.", 
            template.replace("{}", prompt));
        
        Ok(response)
    }

    /// Simple classification fallback
    async fn simple_classification(&self, text: &str, labels: &[String]) -> Result<Vec<(String, f32)>> {
        let mut results = Vec::new();
        
        // Simple keyword-based classification
        let text_lower = text.to_lowercase();
        
        for label in labels {
            let label_lower = label.to_lowercase();
            let score = if text_lower.contains(&label_lower) {
                0.8
            } else {
                // Check for partial matches
                let words: Vec<&str> = label_lower.split_whitespace().collect();
                let matches = words.iter()
                    .filter(|word| text_lower.contains(word))
                    .count();
                
                if matches > 0 {
                    matches as f32 / words.len() as f32 * 0.6
                } else {
                    0.1
                }
            };
            
            results.push((label.clone(), score));
        }
        
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        Ok(results)
    }

    /// Simple sentiment analysis fallback
    async fn simple_sentiment_analysis(&self, text: &str) -> Result<SentimentResult> {
        let text_lower = text.to_lowercase();
        
        let positive_words = ["good", "great", "excellent", "amazing", "wonderful", "happy", "love", "like"];
        let negative_words = ["bad", "terrible", "awful", "hate", "dislike", "sad", "angry", "frustrated"];
        
        let positive_count = positive_words.iter()
            .filter(|word| text_lower.contains(word))
            .count();
        let negative_count = negative_words.iter()
            .filter(|word| text_lower.contains(word))
            .count();
        
        let (label, score) = if positive_count > negative_count {
            ("POSITIVE", 0.7 + (positive_count as f32 * 0.1).min(0.3))
        } else if negative_count > positive_count {
            ("NEGATIVE", 0.7 + (negative_count as f32 * 0.1).min(0.3))
        } else {
            ("NEUTRAL", 0.5)
        };
        
        Ok(SentimentResult {
            label: label.to_string(),
            score,
            text: text.to_string(),
        })
    }

    /// Simple question answering fallback
    async fn simple_question_answering(&self, question: &str, context: &str) -> Result<AnswerResult> {
        let question_lower = question.to_lowercase();
        let context_lower = context.to_lowercase();
        
        // Simple keyword matching
        let question_words: Vec<&str> = question_lower.split_whitespace().collect();
        let context_sentences: Vec<&str> = context_lower.split('.').collect();
        
        let mut best_sentence = "";
        let mut best_score = 0.0;
        
        for sentence in context_sentences {
            let mut score = 0.0;
            for word in &question_words {
                if sentence.contains(word) {
                    score += 1.0;
                }
            }
            score /= question_words.len() as f32;
            
            if score > best_score {
                best_score = score;
                best_sentence = sentence;
            }
        }
        
        let answer = if best_score > 0.3 {
            best_sentence.trim()
        } else {
            "I cannot find a specific answer in the provided context."
        };
        
        Ok(AnswerResult {
            answer: answer.to_string(),
            score: best_score,
            start: 0,
            end: answer.len(),
        })
    }

    /// Extract topics from text
    async fn extract_topics(&self, text: &str) -> Result<Vec<String>> {
        let topics = vec![
            "technology".to_string(),
            "business".to_string(),
            "education".to_string(),
            "health".to_string(),
            "entertainment".to_string(),
        ];
        
        let classification = self.classify_text(text, &topics).await?;
        let relevant_topics: Vec<String> = classification
            .into_iter()
            .filter(|(_, score)| *score > 0.3)
            .map(|(topic, _)| topic)
            .collect();
        
        Ok(relevant_topics)
    }

    /// Generate summary of text
    async fn generate_summary(&self, text: &str) -> Result<String> {
        let sentences: Vec<&str> = text.split('.').collect();
        let summary_length = (sentences.len() / 3).max(1).min(3);
        
        let summary: String = sentences
            .into_iter()
            .take(summary_length)
            .collect::<Vec<_>>()
            .join(". ");
        
        Ok(format!("{}.", summary))
    }

    /// Generate insights from text
    async fn generate_insights(&self, text: &str) -> Result<Vec<String>> {
        let mut insights = Vec::new();
        
        // Simple insight generation based on text characteristics
        let word_count = text.split_whitespace().count();
        if word_count > 100 {
            insights.push("The text is quite detailed and comprehensive.".to_string());
        } else if word_count < 20 {
            insights.push("The text is brief and concise.".to_string());
        }
        
        if text.contains('?') {
            insights.push("The text contains questions that may need addressing.".to_string());
        }
        
        if text.contains("http") || text.contains("www") {
            insights.push("The text contains references to external resources.".to_string());
        }
        
        if insights.is_empty() {
            insights.push("The text appears to be standard content without notable patterns.".to_string());
        }
        
        Ok(insights)
    }
}

/// Result of sentiment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentResult {
    pub label: String,
    pub score: f32,
    pub text: String,
}

/// Result of question answering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerResult {
    pub answer: String,
    pub score: f32,
    pub start: usize,
    pub end: usize,
}

/// Result of comprehensive analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub sentiment: SentimentResult,
    pub topics: Vec<String>,
    pub summary: String,
    pub insights: Vec<String>,
    pub context_used: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llm_config_default() {
        let config = LLMConfig::default();
        assert_eq!(config.model_name, "gpt2");
        assert_eq!(config.max_length, 512);
        assert_eq!(config.temperature, 0.7);
    }

    #[tokio::test]
    async fn test_llm_creation() {
        let llm = LocalLLM::new().await;
        assert!(llm.is_ok());
    }

    #[tokio::test]
    async fn test_simple_sentiment_analysis() {
        let llm = LocalLLM::new().await.unwrap();
        let result = llm.simple_sentiment_analysis("I love this amazing product!").await;
        assert!(result.is_ok());
        
        let sentiment = result.unwrap();
        assert_eq!(sentiment.label, "POSITIVE");
        assert!(sentiment.score > 0.5);
    }

    #[tokio::test]
    async fn test_context_management() {
        let llm = LocalLLM::new().await.unwrap();
        
        llm.add_to_context("This is some context.").await.unwrap();
        let context = llm.get_context().await;
        assert_eq!(context.len(), 1);
        assert_eq!(context[0], "This is some context.");
        
        llm.clear_context().await;
        let context = llm.get_context().await;
        assert!(context.is_empty());
    }
} 