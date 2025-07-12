use std::sync::Arc;
use tokio::sync::Mutex;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, StreamConfig};

use crate::api_response::ApiResponse;

pub struct InputStream {
    device: Option<cpal::Device>,
    config: Option<StreamConfig>,
    is_recording: Arc<Mutex<bool>>,
}

impl InputStream {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or("No default input device found")?;
        
        let config = device.default_input_config()?;
        
        Ok(Self {
            device: Some(device),
            config: Some(config.into()),
            is_recording: Arc::new(Mutex::new(false)),
        })
    }

    pub fn get_device_name(&self) -> Option<String> {
        self.device.as_ref().and_then(|d| d.name().ok())
    }

    pub fn get_config(&self) -> Option<&StreamConfig> {
        self.config.as_ref()
    }
}

pub struct OutputStream {
    buffer: Arc<Mutex<Vec<f32>>>,
    is_processing: Arc<Mutex<bool>>,
}

impl OutputStream {
    pub fn new() -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            is_processing: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn add_audio_data(&self, data: Vec<f32>) {
        let mut buffer = self.buffer.lock().await;
        buffer.extend(data);
        
        // Process data in chunks (e.g., every 1024 samples)
        if buffer.len() >= 1024 {
            let chunk: Vec<f32> = buffer.drain(..1024).collect();
            self.process_chunk(chunk).await;
        }
    }

    async fn process_chunk(&self, chunk: Vec<f32>) {
        // TODO: Send chunk to ML system
        println!("Processing audio chunk: {} samples", chunk.len());
        
        // For now, just log the audio data
        let rms = (chunk.iter().map(|&x| x * x).sum::<f32>() / chunk.len() as f32).sqrt();
        println!("Audio RMS: {:.4}", rms);
    }
}

pub struct AudioStream {
    input_stream: InputStream,
    output_stream: OutputStream,
}

impl AudioStream {
    pub fn default() -> Self {
        Self::new().expect("Failed to create audio stream")
    }

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let input_stream = InputStream::new()?;
        let output_stream = OutputStream::new();
        
        Ok(Self {
            input_stream,
            output_stream,
        })
    }

    pub async fn start_stream(&self) -> ApiResponse<()> {
        let mut is_recording = self.input_stream.is_recording.lock().await;
        if *is_recording {
            return ApiResponse::error("Stream is already running".to_string());
        }
        *is_recording =! *is_recording;
        drop(is_recording);

        let device = match self.input_stream.device.as_ref() {
            Some(d) => d,
            None => return ApiResponse::error("No input device available".to_string()),
        };
        let config = match self.input_stream.config.as_ref() {
            Some(c) => c,
            None => return ApiResponse::error("No stream config available".to_string()),
        };

        let output_stream = self.output_stream.clone();

        let data_fn = move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let output_stream = output_stream.clone();
            let audio_data: Vec<f32> = data.to_vec();
            tokio::spawn(async move {
                output_stream.add_audio_data(audio_data).await;
            });
        };

        let error_fn = |err| {
            eprintln!("Audio stream error: {}", err);
        };

        match device.build_input_stream(config, data_fn, error_fn, None) {
            Ok(stream) => {
                if let Err(e) = stream.play() {
                    return ApiResponse::error(format!("Failed to start stream: {}", e));
                }
                
                println!("Audio stream started successfully");
                ApiResponse::success(())
            }
            Err(e) => ApiResponse::error(format!("Failed to build stream: {}", e)),
        }
    }

    pub async fn stop_stream(&self) -> ApiResponse<()> {
        let mut is_recording = self.input_stream.is_recording.lock().await;
        if !*is_recording {
            return ApiResponse::error("Stream is not running".to_string());
        }
        *is_recording = false;
        drop(is_recording);

        println!("Audio stream stopped successfully");
        ApiResponse::success(())
    }
}

impl Clone for AudioStream {
    fn clone(&self) -> Self {
        Self {
            input_stream: InputStream {
                device: self.input_stream.device.clone(),
                config: self.input_stream.config.clone(),
                is_recording: self.input_stream.is_recording.clone(),
            },
            output_stream: self.output_stream.clone(),
        }
    }
}

impl Clone for InputStream {
    fn clone(&self) -> Self {
        Self {
            device: self.device.clone(),
            config: self.config.clone(),
            is_recording: self.is_recording.clone(),
        }
    }
}

impl Clone for OutputStream {
    fn clone(&self) -> Self {
        Self {
            buffer: self.buffer.clone(),
            is_processing: self.is_processing.clone(),
        }
    }
}