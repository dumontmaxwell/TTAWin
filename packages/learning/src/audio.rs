use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat, StreamConfig,
};
use hound::{WavReader, WavWriter};
use serde::{Deserialize, Serialize};
use std::{
    path::Path,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::sync::mpsc;

/// Configuration for audio transcription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u16,
    pub buffer_size: usize,
    pub silence_threshold: f32,
    pub min_audio_length: Duration,
    pub noise_reduction: bool,
    pub vad_enabled: bool, // Voice Activity Detection
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            channels: 1,
            buffer_size: 4096,
            silence_threshold: 0.01,
            min_audio_length: Duration::from_millis(500),
            noise_reduction: true,
            vad_enabled: true,
        }
    }
}

/// Audio transcriber for converting audio to text
pub struct AudioTranscriber {
    config: AudioConfig,
    // In a real implementation, you'd have a speech recognition model here
    // For now, we'll simulate transcription with basic audio processing
}

impl AudioTranscriber {
    /// Create a new audio transcriber
    pub fn new() -> Result<Self> {
        let config = AudioConfig::default();
        Ok(Self { config })
    }

    /// Create audio transcriber with custom configuration
    pub fn with_config(config: AudioConfig) -> Self {
        Self { config }
    }

    /// Transcribe audio from a file
    pub async fn transcribe(&self, audio_path: &str) -> Result<String> {
        let audio_data = self.load_audio_file(audio_path)?;
        let processed_audio = self.preprocess_audio(&audio_data)?;
        self.transcribe_audio_data(&processed_audio).await
    }

    /// Transcribe audio from raw bytes
    pub async fn transcribe_bytes(&self, audio_bytes: &[u8]) -> Result<String> {
        let audio_data = self.load_audio_from_bytes(audio_bytes)?;
        let processed_audio = self.preprocess_audio(&audio_data)?;
        self.transcribe_audio_data(&processed_audio).await
    }

    /// Start real-time audio transcription
    pub async fn start_realtime_transcription(
        &self,
    ) -> Result<(mpsc::Receiver<String>, tokio::task::JoinHandle<()>)> {
        let (tx, rx) = mpsc::channel(100);
        let config = self.config.clone();

        let handle = tokio::spawn(async move {
            if let Err(e) = Self::run_realtime_transcription(config, tx).await {
                eprintln!("Real-time transcription error: {}", e);
            }
        });

        Ok((rx, handle))
    }

    /// Load audio file and convert to internal format
    fn load_audio_file(&self, audio_path: &str) -> Result<Vec<f32>> {
        let path = Path::new(audio_path);
        
        match path.extension().and_then(|s| s.to_str()) {
            Some("wav") => self.load_wav_file(audio_path),
            Some("mp3") => self.load_mp3_file(audio_path),
            Some("flac") => self.load_flac_file(audio_path),
            _ => Err(anyhow::anyhow!("Unsupported audio format")),
        }
    }

    /// Load audio from raw bytes
    fn load_audio_from_bytes(&self, audio_bytes: &[u8]) -> Result<Vec<f32>> {
        // Try to detect format and load accordingly
        // For now, assume WAV format
        self.load_wav_from_bytes(audio_bytes)
    }

    /// Load WAV file
    fn load_wav_file(&self, audio_path: &str) -> Result<Vec<f32>> {
        let reader = WavReader::open(audio_path)?;
        let samples: Vec<f32> = reader
            .into_samples::<i16>()
            .map(|s| s.map(|sample| sample as f32 / 32768.0))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(samples)
    }

    /// Load WAV from bytes
    fn load_wav_from_bytes(&self, audio_bytes: &[u8]) -> Result<Vec<f32>> {
        let reader = WavReader::new(std::io::Cursor::new(audio_bytes))?;
        let samples: Vec<f32> = reader
            .into_samples::<i16>()
            .map(|s| s.map(|sample| sample as f32 / 32768.0))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(samples)
    }

    /// Load MP3 file (placeholder - would need mp3 decoder)
    fn load_mp3_file(&self, _audio_path: &str) -> Result<Vec<f32>> {
        // In a real implementation, you'd use a crate like symphonia or rodio
        Err(anyhow::anyhow!("MP3 support not implemented yet"))
    }

    /// Load FLAC file (placeholder - would need flac decoder)
    fn load_flac_file(&self, _audio_path: &str) -> Result<Vec<f32>> {
        // In a real implementation, you'd use a crate like symphonia
        Err(anyhow::anyhow!("FLAC support not implemented yet"))
    }

    /// Preprocess audio for better transcription
    fn preprocess_audio(&self, audio_data: &[f32]) -> Result<Vec<f32>> {
        let mut processed = audio_data.to_vec();

        // Apply noise reduction if enabled
        if self.config.noise_reduction {
            processed = self.reduce_noise(&processed);
        }

        // Apply voice activity detection if enabled
        if self.config.vad_enabled {
            processed = self.apply_vad(&processed);
        }

        // Normalize audio
        processed = self.normalize_audio(&processed);

        Ok(processed)
    }

    /// Reduce noise in audio
    fn reduce_noise(&self, audio_data: &[f32]) -> Vec<f32> {
        let mut denoised = audio_data.to_vec();
        
        // Simple noise gate
        for sample in denoised.iter_mut() {
            if sample.abs() < self.config.silence_threshold {
                *sample = 0.0;
            }
        }

        // Simple low-pass filter to reduce high-frequency noise
        let mut filtered = vec![0.0; denoised.len()];
        let alpha = 0.1; // Filter coefficient
        
        if !denoised.is_empty() {
            filtered[0] = denoised[0];
            for i in 1..denoised.len() {
                filtered[i] = alpha * denoised[i] + (1.0 - alpha) * filtered[i - 1];
            }
        }

        filtered
    }

    /// Apply Voice Activity Detection
    fn apply_vad(&self, audio_data: &[f32]) -> Vec<f32> {
        let mut vad_processed = audio_data.to_vec();
        
        // Simple energy-based VAD
        let window_size = self.config.sample_rate as usize / 100; // 10ms windows
        let mut energy_threshold = 0.01;
        
        // Calculate energy threshold from first few seconds
        let calibration_samples = std::cmp::min(3 * self.config.sample_rate as usize, audio_data.len());
        let mut energies = Vec::new();
        
        for window_start in (0..calibration_samples).step_by(window_size) {
            let window_end = std::cmp::min(window_start + window_size, calibration_samples);
            let energy: f32 = audio_data[window_start..window_end]
                .iter()
                .map(|&x| x * x)
                .sum::<f32>() / (window_end - window_start) as f32;
            energies.push(energy);
        }
        
        if !energies.is_empty() {
            energies.sort_by(|a, b| a.partial_cmp(b).unwrap());
            energy_threshold = energies[energies.len() / 4] * 2.0; // Use 25th percentile * 2
        }

        // Apply VAD
        for window_start in (0..vad_processed.len()).step_by(window_size) {
            let window_end = std::cmp::min(window_start + window_size, vad_processed.len());
            let energy: f32 = audio_data[window_start..window_end]
                .iter()
                .map(|&x| x * x)
                .sum::<f32>() / (window_end - window_start) as f32;
            
            if energy < energy_threshold {
                // Mark as silence
                for i in window_start..window_end {
                    vad_processed[i] = 0.0;
                }
            }
        }

        vad_processed
    }

    /// Normalize audio levels
    fn normalize_audio(&self, audio_data: &[f32]) -> Vec<f32> {
        if audio_data.is_empty() {
            return vec![];
        }

        let max_amplitude = audio_data.iter().map(|&x| x.abs()).fold(0.0, f32::max);
        
        if max_amplitude > 0.0 {
            let scale_factor = 0.95 / max_amplitude; // Leave some headroom
            audio_data.iter().map(|&x| x * scale_factor).collect()
        } else {
            audio_data.to_vec()
        }
    }

    /// Transcribe audio data to text
    async fn transcribe_audio_data(&self, audio_data: &[f32]) -> Result<String> {
        // In a real implementation, this would use a speech recognition model
        // For now, we'll simulate transcription based on audio characteristics
        
        let duration = audio_data.len() as f32 / self.config.sample_rate as f32;
        
        if duration < self.config.min_audio_length.as_secs_f32() {
            return Ok("Audio too short to transcribe".to_string());
        }

        // Calculate audio features for "transcription"
        let energy = self.calculate_energy(audio_data);
        let zero_crossings = self.calculate_zero_crossings(audio_data);
        let spectral_centroid = self.calculate_spectral_centroid(audio_data);

        // Generate simulated transcription based on audio features
        let transcription = self.generate_simulated_transcription(energy, zero_crossings, spectral_centroid, duration);
        
        Ok(transcription)
    }

    /// Calculate audio energy
    fn calculate_energy(&self, audio_data: &[f32]) -> f32 {
        audio_data.iter().map(|&x| x * x).sum::<f32>() / audio_data.len() as f32
    }

    /// Calculate zero crossing rate
    fn calculate_zero_crossings(&self, audio_data: &[f32]) -> f32 {
        if audio_data.len() < 2 {
            return 0.0;
        }

        let crossings = audio_data.windows(2)
            .filter(|window| (window[0] >= 0.0) != (window[1] >= 0.0))
            .count();

        crossings as f32 / (audio_data.len() - 1) as f32
    }

    /// Calculate spectral centroid (simplified)
    fn calculate_spectral_centroid(&self, audio_data: &[f32]) -> f32 {
        // Simplified spectral centroid calculation
        // In a real implementation, you'd use FFT
        let high_freq_energy: f32 = audio_data.iter()
            .enumerate()
            .map(|(i, &x)| x * x * (i as f32 / audio_data.len() as f32))
            .sum();
        
        let total_energy: f32 = audio_data.iter().map(|&x| x * x).sum();
        
        if total_energy > 0.0 {
            high_freq_energy / total_energy
        } else {
            0.0
        }
    }

    /// Generate simulated transcription based on audio features
    fn generate_simulated_transcription(
        &self,
        energy: f32,
        zero_crossings: f32,
        spectral_centroid: f32,
        duration: f32,
    ) -> String {
        // This is a placeholder that generates text based on audio characteristics
        // In a real implementation, you'd use a trained speech recognition model
        
        let mut transcription = String::new();
        
        // Estimate speech rate based on zero crossings
        let estimated_words = if zero_crossings > 0.1 {
            (duration * 150.0) as usize // Fast speech
        } else if zero_crossings > 0.05 {
            (duration * 120.0) as usize // Normal speech
        } else {
            (duration * 80.0) as usize // Slow speech
        };

        // Generate placeholder text based on energy and duration
        if energy > 0.01 {
            transcription.push_str("Detected speech content. ");
            transcription.push_str(&format!("Duration: {:.1} seconds. ", duration));
            transcription.push_str(&format!("Estimated words: {}. ", estimated_words));
            
            if spectral_centroid > 0.5 {
                transcription.push_str("High-frequency speech detected. ");
            } else {
                transcription.push_str("Low-frequency speech detected. ");
            }
        } else {
            transcription.push_str("Silence or very low audio detected. ");
        }

        transcription
    }

    /// Run real-time transcription
    async fn run_realtime_transcription(
        config: AudioConfig,
        tx: mpsc::Sender<String>,
    ) -> Result<()> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device found"))?;

        let config_stream = StreamConfig {
            channels: config.channels,
            sample_rate: cpal::SampleRate(config.sample_rate),
            buffer_size: cpal::BufferSize::Fixed(config.buffer_size as u32),
        };

        let audio_buffer = Arc::new(Mutex::new(Vec::new()));
        let audio_buffer_clone = Arc::clone(&audio_buffer);

        let stream = device.build_input_stream(
            &config_stream,
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut buffer = audio_buffer_clone.lock().unwrap();
                buffer.extend_from_slice(data);
                
                // Process buffer when it's large enough
                if buffer.len() >= config.sample_rate as usize {
                    let audio_data = buffer.drain(..).collect::<Vec<_>>();
                    // In a real implementation, you'd send this to a background task for processing
                }
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )?;

        stream.play()?;

        // Keep the stream alive
        std::thread::sleep(Duration::from_secs(1));
        
        Ok(())
    }

    /// Save audio data to WAV file
    pub fn save_audio(&self, audio_data: &[f32], output_path: &str) -> Result<()> {
        let spec = hound::WavSpec {
            channels: self.config.channels,
            sample_rate: self.config.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = WavWriter::create(output_path, spec)?;
        
        for &sample in audio_data {
            let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
            writer.write_sample(sample_i16)?;
        }

        writer.finalize()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_config_default() {
        let config = AudioConfig::default();
        assert_eq!(config.sample_rate, 16000);
        assert_eq!(config.channels, 1);
        assert_eq!(config.buffer_size, 4096);
        assert_eq!(config.silence_threshold, 0.01);
    }

    #[tokio::test]
    async fn test_audio_transcriber_creation() {
        let transcriber = AudioTranscriber::new();
        assert!(transcriber.is_ok());
    }

    #[test]
    fn test_audio_normalization() {
        let transcriber = AudioTranscriber::new().unwrap();
        let audio_data = vec![0.5, -0.3, 0.8, -0.1];
        let normalized = transcriber.normalize_audio(&audio_data);
        
        assert!(!normalized.is_empty());
        assert!(normalized.iter().all(|&x| x.abs() <= 1.0));
    }
} 