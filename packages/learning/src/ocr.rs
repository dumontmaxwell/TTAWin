use anyhow::Result;
use image::{DynamicImage, ImageBuffer, Rgb};
use serde::{Deserialize, Serialize};
use std::path::Path;
use tesseract::{Tesseract, Image};

/// Configuration for OCR processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OCRConfig {
    pub language: String,
    pub confidence_threshold: f32,
    pub preprocess_image: bool,
    pub extract_structured_data: bool,
}

impl Default for OCRConfig {
    fn default() -> Self {
        Self {
            language: "eng".to_string(),
            confidence_threshold: 0.6,
            preprocess_image: true,
            extract_structured_data: true,
        }
    }
}

/// OCR Engine for extracting text from images
pub struct OCREngine {
    tesseract: Tesseract,
    config: OCRConfig,
}

impl OCREngine {
    /// Create a new OCR engine
    pub fn new() -> Result<Self> {
        let config = OCRConfig::default();
        let tesseract = Tesseract::new(None, Some(&config.language))?;
        
        Ok(Self { tesseract, config })
    }

    /// Create OCR engine with custom configuration
    pub fn with_config(config: OCRConfig) -> Result<Self> {
        let tesseract = Tesseract::new(None, Some(&config.language))?;
        Ok(Self { tesseract, config })
    }

    /// Extract text from an image file
    pub async fn extract_text(&self, image_path: &str) -> Result<String> {
        let image = self.load_and_preprocess_image(image_path)?;
        let text = self.tesseract.recognize(&image)?;
        
        if self.config.extract_structured_data {
            self.post_process_text(&text.text)
        } else {
            Ok(text.text)
        }
    }

    /// Extract text from raw image data
    pub async fn extract_text_from_bytes(&self, image_bytes: &[u8]) -> Result<String> {
        let image = self.preprocess_image_bytes(image_bytes)?;
        let text = self.tesseract.recognize(&image)?;
        
        if self.config.extract_structured_data {
            self.post_process_text(&text.text)
        } else {
            Ok(text.text)
        }
    }

    /// Load and preprocess image for better OCR results
    fn load_and_preprocess_image(&self, image_path: &str) -> Result<Image> {
        let img = image::open(Path::new(image_path))?;
        let processed_img = if self.config.preprocess_image {
            self.preprocess_image(&img)
        } else {
            img
        };
        
        Ok(Image::from_dynamic_image(&processed_img))
    }

    /// Preprocess image bytes for better OCR results
    fn preprocess_image_bytes(&self, image_bytes: &[u8]) -> Result<Image> {
        let img = image::load_from_memory(image_bytes)?;
        let processed_img = if self.config.preprocess_image {
            self.preprocess_image(&img)
        } else {
            img
        };
        
        Ok(Image::from_dynamic_image(&processed_img))
    }

    /// Apply image preprocessing techniques to improve OCR accuracy
    fn preprocess_image(&self, img: &DynamicImage) -> DynamicImage {
        // Convert to grayscale for better text recognition
        let gray = img.to_luma8();
        
        // Apply contrast enhancement
        let enhanced = self.enhance_contrast(&gray);
        
        // Apply noise reduction
        let denoised = self.reduce_noise(&enhanced);
        
        // Convert back to RGB for tesseract
        DynamicImage::ImageRgb8(denoised)
    }

    /// Enhance image contrast
    fn enhance_contrast(&self, img: &ImageBuffer<image::Luma<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut enhanced = ImageBuffer::new(img.width(), img.height());
        
        for (x, y, pixel) in img.enumerate_pixels() {
            let value = pixel[0] as f32;
            // Apply histogram equalization-like enhancement
            let enhanced_value = ((value / 255.0).powf(0.7) * 255.0) as u8;
            enhanced.put_pixel(x, y, Rgb([enhanced_value, enhanced_value, enhanced_value]));
        }
        
        enhanced
    }

    /// Reduce noise in the image
    fn reduce_noise(&self, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut denoised = ImageBuffer::new(img.width(), img.height());
        
        for (x, y, pixel) in img.enumerate_pixels() {
            if x > 0 && y > 0 && x < img.width() - 1 && y < img.height() - 1 {
                // Simple 3x3 median filter
                let mut neighbors = Vec::new();
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        let neighbor = img.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);
                        neighbors.push(neighbor[0]);
                    }
                }
                neighbors.sort();
                let median = neighbors[4]; // 5th element in sorted 9-element array
                denoised.put_pixel(x, y, Rgb([median, median, median]));
            } else {
                denoised.put_pixel(x, y, *pixel);
            }
        }
        
        denoised
    }

    /// Post-process extracted text to improve quality
    fn post_process_text(&self, text: &str) -> Result<String> {
        let mut processed = text.to_string();
        
        // Remove excessive whitespace
        processed = processed.replace("\n\n\n", "\n\n");
        
        // Fix common OCR errors
        processed = self.fix_common_ocr_errors(&processed);
        
        // Extract structured information if enabled
        if self.config.extract_structured_data {
            processed = self.extract_structured_information(&processed);
        }
        
        Ok(processed)
    }

    /// Fix common OCR errors
    fn fix_common_ocr_errors(&self, text: &str) -> String {
        let mut fixed = text.to_string();
        
        // Common OCR replacements
        let replacements = [
            ("0", "O"), // Often confused
            ("1", "l"), // Often confused
            ("5", "S"), // Often confused
            ("8", "B"), // Often confused
        ];
        
        for (wrong, correct) in replacements.iter() {
            // Only replace in context where it makes sense
            // This is a simplified version - in practice, you'd want more sophisticated logic
            if text.contains(wrong) {
                // Add context-aware replacement logic here
            }
        }
        
        fixed
    }

    /// Extract structured information from text
    fn extract_structured_information(&self, text: &str) -> String {
        let mut structured = String::new();
        
        // Extract URLs
        let url_pattern = regex::Regex::new(r"https?://[^\s]+").unwrap();
        for url in url_pattern.find_iter(text) {
            structured.push_str(&format!("URL: {}\n", url.as_str()));
        }
        
        // Extract email addresses
        let email_pattern = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
        for email in email_pattern.find_iter(text) {
            structured.push_str(&format!("Email: {}\n", email.as_str()));
        }
        
        // Extract phone numbers
        let phone_pattern = regex::Regex::new(r"(\+\d{1,3}[-.\s]?)?\(?\d{3}\)?[-.\s]?\d{3}[-.\s]?\d{4}").unwrap();
        for phone in phone_pattern.find_iter(text) {
            structured.push_str(&format!("Phone: {}\n", phone.as_str()));
        }
        
        if structured.is_empty() {
            text.to_string()
        } else {
            structured.push_str("\n--- Full Text ---\n");
            structured.push_str(text);
            structured
        }
    }

    /// Get confidence score for extracted text
    pub async fn get_confidence(&self, image_path: &str) -> Result<f32> {
        let image = self.load_and_preprocess_image(image_path)?;
        let result = self.tesseract.recognize(&image)?;
        
        // Calculate average confidence from all words
        let words = result.words();
        if words.is_empty() {
            return Ok(0.0);
        }
        
        let total_confidence: f32 = words.iter().map(|word| word.confidence()).sum();
        Ok(total_confidence / words.len() as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocr_config_default() {
        let config = OCRConfig::default();
        assert_eq!(config.language, "eng");
        assert_eq!(config.confidence_threshold, 0.6);
        assert!(config.preprocess_image);
        assert!(config.extract_structured_data);
    }

    #[tokio::test]
    async fn test_ocr_engine_creation() {
        let engine = OCREngine::new();
        assert!(engine.is_ok());
    }
} 