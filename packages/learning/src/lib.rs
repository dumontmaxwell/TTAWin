pub mod ocr;
pub mod audio;
pub mod llm;
pub mod analysis;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Main learning service that orchestrates all components
pub struct LearningService {
    ocr_engine: Arc<ocr::OCREngine>,
    audio_transcriber: Arc<audio::AudioTranscriber>,
    llm_engine: Arc<llm::LocalLLM>,
    analysis_engine: Arc<analysis::AnalysisEngine>,
}

impl LearningService {
    /// Create a new learning service with all components initialized
    pub async fn new() -> Result<Self, anyhow::Error> {
        let ocr_engine = Arc::new(ocr::OCREngine::new()?);
        let audio_transcriber = Arc::new(audio::AudioTranscriber::new()?);
        let llm_engine = Arc::new(llm::LocalLLM::new().await?);
        let analysis_engine = Arc::new(analysis::AnalysisEngine::new(
            Arc::clone(&llm_engine),
        ));

        Ok(Self {
            ocr_engine,
            audio_transcriber,
            llm_engine,
            analysis_engine,
        })
    }

    /// Analyze a screenshot and extract relevant information
    pub async fn analyze_screenshot(&self, image_path: &str) -> Result<analysis::AnalysisResult, anyhow::Error> {
        let text_content = self.ocr_engine.extract_text(image_path).await?;
        self.analysis_engine.analyze_text(&text_content).await
    }

    /// Transcribe audio and analyze the content
    pub async fn analyze_audio(&self, audio_path: &str) -> Result<analysis::AnalysisResult, anyhow::Error> {
        let text_content = self.audio_transcriber.transcribe(audio_path).await?;
        self.analysis_engine.analyze_text(&text_content).await
    }

    /// Get the underlying LLM engine for direct access
    pub fn llm_engine(&self) -> Arc<llm::LocalLLM> {
        Arc::clone(&self.llm_engine)
    }

    /// Get the analysis engine for custom analysis
    pub fn analysis_engine(&self) -> Arc<analysis::AnalysisEngine> {
        Arc::clone(&self.analysis_engine)
    }
}

/// Configuration for the learning service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    pub ocr_config: ocr::OCRConfig,
    pub audio_config: audio::AudioConfig,
    pub llm_config: llm::LLMConfig,
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            ocr_config: ocr::OCRConfig::default(),
            audio_config: audio::AudioConfig::default(),
            llm_config: llm::LLMConfig::default(),
        }
    }
}

// Legacy function for backward compatibility
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_learning_service_creation() {
        let service = LearningService::new().await;
        assert!(service.is_ok());
    }
}
