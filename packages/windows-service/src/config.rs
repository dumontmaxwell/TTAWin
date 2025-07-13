use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub server: ServerConfig,
    pub learning: LearningConfig,
    pub payments: PaymentConfig,
    pub settings: SettingsConfig,
    pub stream: StreamConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub cors_origins: Vec<String>,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    pub model_path: PathBuf,
    pub cache_dir: PathBuf,
    pub max_concurrent_analyses: usize,
    pub enable_gpu: bool,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentConfig {
    pub stripe_secret_key: Option<String>,
    pub crypto_enabled: bool,
    pub supported_currencies: Vec<String>,
    pub webhook_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsConfig {
    pub data_dir: PathBuf,
    pub backup_enabled: bool,
    pub auto_sync: bool,
    pub sync_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamConfig {
    pub enabled: bool,
    pub buffer_size: usize,
    pub sample_rate: u32,
    pub channels: u16,
}

impl ServiceConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let config_content = std::fs::read_to_string(&config_path)?;
            let config: ServiceConfig = toml::from_str(&config_content)?;
            Ok(config)
        } else {
            // Create default configuration
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let config_content = toml::to_string_pretty(self)?;
        std::fs::write(&config_path, config_content)?;
        
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let mut config_dir = std::env::current_exe()?;
        config_dir.pop(); // Remove executable name
        config_dir.push("config");
        config_dir.push("service.toml");
        Ok(config_dir)
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        let mut data_dir = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
        data_dir.pop();
        data_dir.push("data");

        let mut model_path = data_dir.clone();
        model_path.push("models");

        let mut cache_dir = data_dir.clone();
        cache_dir.push("cache");

        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                cors_origins: vec![
                    "http://localhost:3000".to_string(),
                    "http://127.0.0.1:3000".to_string(),
                ],
                max_connections: 1000,
            },
            learning: LearningConfig {
                model_path,
                cache_dir,
                max_concurrent_analyses: 4,
                enable_gpu: false,
                log_level: "info".to_string(),
            },
            payments: PaymentConfig {
                stripe_secret_key: None,
                crypto_enabled: true,
                supported_currencies: vec![
                    "usd".to_string(),
                    "btc".to_string(),
                    "eth".to_string(),
                    "usdc".to_string(),
                ],
                webhook_url: None,
            },
            settings: SettingsConfig {
                data_dir,
                backup_enabled: true,
                auto_sync: true,
                sync_interval_seconds: 300, // 5 minutes
            },
            stream: StreamConfig {
                enabled: true,
                buffer_size: 4096,
                sample_rate: 16000,
                channels: 1,
            },
        }
    }
} 