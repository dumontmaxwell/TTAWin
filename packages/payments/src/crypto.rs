use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc, Duration};

/// Crypto payment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoConfig {
    pub supported_currencies: Vec<CryptoCurrency>,
    pub blockchain_networks: Vec<BlockchainNetwork>,
    pub smart_contract_address: Option<String>,
    pub gas_limit: u64,
    pub confirmation_blocks: u32,
    pub price_feed_urls: HashMap<String, String>,
}

/// Supported cryptocurrency types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CryptoCurrency {
    Bitcoin,
    Ethereum,
    USDC,
    USDT,
    DAI,
    Custom(String),
}

/// Blockchain networks
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BlockchainNetwork {
    Bitcoin,
    Ethereum,
    Polygon,
    BinanceSmartChain,
    Arbitrum,
    Optimism,
    Custom(String),
}

/// Crypto payment request
#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoPaymentRequest {
    pub amount: u64, // Amount in cents
    pub currency: String, // e.g., "usd"
    pub description: Option<String>,
    pub customer_email: Option<String>,
    pub crypto_details: CryptoPaymentDetails,
    pub metadata: HashMap<String, String>,
}

/// Crypto payment details
#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoPaymentDetails {
    pub currency: CryptoCurrency,
    pub network: BlockchainNetwork,
    pub wallet_address: String,
    pub amount_crypto: f64,
    pub exchange_rate: f64,
    pub expires_at: DateTime<Utc>,
}

/// Crypto payment result
#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoPaymentResult {
    pub success: bool,
    pub transaction_hash: Option<String>,
    pub error_message: Option<String>,
    pub confirmation_status: ConfirmationStatus,
    pub fees: CryptoFees,
    pub created_at: DateTime<Utc>,
}

/// Payment confirmation status
#[derive(Debug, Serialize, Deserialize)]
pub enum ConfirmationStatus {
    Pending,
    Confirmed,
    Failed,
    Expired,
}

/// Crypto payment fees
#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoFees {
    pub network_fee: f64,
    pub processing_fee: f64,
    pub total_fee: f64,
    pub currency: String,
}

/// Crypto transaction details
#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoTransaction {
    pub transaction_hash: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: f64,
    pub currency: CryptoCurrency,
    pub network: BlockchainNetwork,
    pub block_number: Option<u64>,
    pub confirmation_count: u32,
    pub timestamp: DateTime<Utc>,
    pub gas_used: Option<u64>,
    pub gas_price: Option<u64>,
}

/// Smart contract payment
#[derive(Debug, Serialize, Deserialize)]
pub struct SmartContractPayment {
    pub contract_address: String,
    pub function_name: String,
    pub parameters: Vec<String>,
    pub gas_limit: u64,
    pub gas_price: u64,
}

/// Crypto payment service
pub struct CryptoPaymentService {
    config: CryptoConfig,
    price_feeds: Arc<Mutex<HashMap<String, f64>>>,
    pub wallet_manager: Arc<WalletManager>,
    transaction_cache: Arc<Mutex<HashMap<String, CryptoTransaction>>>,
}

/// Wallet manager for handling crypto wallets
pub struct WalletManager {
    wallets: Arc<Mutex<HashMap<CryptoCurrency, WalletInfo>>>,
}

/// Wallet information
#[derive(Debug, Clone)]
pub struct WalletInfo {
    pub address: String,
    pub private_key: Option<String>, // In production, use secure key management
    pub balance: f64,
    pub currency: CryptoCurrency,
    pub network: BlockchainNetwork,
}

impl CryptoPaymentService {
    /// Create a new crypto payment service
    pub fn new(config: CryptoConfig) -> Self {
        Self {
            config,
            price_feeds: Arc::new(Mutex::new(HashMap::new())),
            wallet_manager: Arc::new(WalletManager::new()),
            transaction_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Process a crypto payment
    pub async fn process_payment(&self, request: CryptoPaymentRequest) -> Result<CryptoPaymentResult, anyhow::Error> {
        // Validate crypto payment details
        self.validate_crypto_payment(&request.crypto_details).await?;

        // Get current exchange rate
        let exchange_rate = self.get_exchange_rate(&request.crypto_details.currency).await?;

        // Calculate fees
        let fees = self.calculate_fees(&request.crypto_details).await?;

        // Create crypto transaction
        let transaction = self.create_transaction(&request.crypto_details).await?;

        // Store transaction in cache
        let mut cache = self.transaction_cache.lock().await;
        cache.insert(transaction.transaction_hash.clone(), transaction.clone());

        Ok(CryptoPaymentResult {
            success: true,
            transaction_hash: Some(transaction.transaction_hash),
            error_message: None,
            confirmation_status: ConfirmationStatus::Pending,
            fees,
            created_at: Utc::now(),
        })
    }

    /// Validate crypto payment details
    async fn validate_crypto_payment(&self, crypto_details: &CryptoPaymentDetails) -> Result<(), anyhow::Error> {
        // Check if currency is supported
        if !self.config.supported_currencies.contains(&crypto_details.currency) {
            return Err(anyhow::anyhow!("Unsupported cryptocurrency: {:?}", crypto_details.currency));
        }

        // Check if network is supported
        if !self.config.blockchain_networks.contains(&crypto_details.network) {
            return Err(anyhow::anyhow!("Unsupported blockchain network: {:?}", crypto_details.network));
        }

        // Validate wallet address format
        if !self.is_valid_wallet_address(&crypto_details.wallet_address, &crypto_details.currency) {
            return Err(anyhow::anyhow!("Invalid wallet address for currency: {:?}", crypto_details.currency));
        }

        // Check if payment has expired
        if crypto_details.expires_at < Utc::now() {
            return Err(anyhow::anyhow!("Payment request has expired"));
        }

        Ok(())
    }

    /// Validate wallet address format
    fn is_valid_wallet_address(&self, address: &str, currency: &CryptoCurrency) -> bool {
        match currency {
            CryptoCurrency::Bitcoin => {
                // Bitcoin address validation (simplified)
                address.len() >= 26 && address.len() <= 35 && 
                (address.starts_with("1") || address.starts_with("3") || address.starts_with("bc1"))
            }
            CryptoCurrency::Ethereum => {
                // Ethereum address validation
                address.len() == 42 && address.starts_with("0x") &&
                address.chars().skip(2).all(|c| c.is_ascii_hexdigit())
            }
            CryptoCurrency::USDC | CryptoCurrency::USDT | CryptoCurrency::DAI => {
                // ERC-20 tokens use Ethereum addresses
                self.is_valid_wallet_address(address, &CryptoCurrency::Ethereum)
            }
            CryptoCurrency::Custom(_) => {
                // For custom currencies, accept any non-empty string
                !address.is_empty()
            }
        }
    }

    /// Get current exchange rate for a cryptocurrency
    pub async fn get_exchange_rate(&self, currency: &CryptoCurrency) -> Result<f64, anyhow::Error> {
        let mut price_feeds = self.price_feeds.lock().await;
        
        if let Some(rate) = price_feeds.get(&currency.to_string()) {
            return Ok(*rate);
        }

        // In a real implementation, this would fetch from a price feed API
        let rate = match currency {
            CryptoCurrency::Bitcoin => 45000.0,
            CryptoCurrency::Ethereum => 3000.0,
            CryptoCurrency::USDC => 1.0,
            CryptoCurrency::USDT => 1.0,
            CryptoCurrency::DAI => 1.0,
            CryptoCurrency::Custom(_) => 1.0,
        };

        price_feeds.insert(currency.to_string(), rate);
        Ok(rate)
    }

    /// Calculate fees for crypto payment
    pub async fn calculate_fees(&self, crypto_details: &CryptoPaymentDetails) -> Result<CryptoFees, anyhow::Error> {
        let network_fee = match crypto_details.network {
            BlockchainNetwork::Bitcoin => 0.0001, // BTC
            BlockchainNetwork::Ethereum => 0.005, // ETH
            BlockchainNetwork::Polygon => 0.0001, // MATIC
            BlockchainNetwork::BinanceSmartChain => 0.0001, // BNB
            BlockchainNetwork::Arbitrum => 0.0001, // ETH
            BlockchainNetwork::Optimism => 0.0001, // ETH
            BlockchainNetwork::Custom(_) => 0.001,
        };

        let processing_fee = 0.01; // 1% processing fee
        let total_fee = network_fee + (crypto_details.amount_crypto * processing_fee);

        Ok(CryptoFees {
            network_fee,
            processing_fee: crypto_details.amount_crypto * processing_fee,
            total_fee,
            currency: crypto_details.currency.to_string(),
        })
    }

    /// Create a crypto transaction
    pub async fn create_transaction(&self, crypto_details: &CryptoPaymentDetails) -> Result<CryptoTransaction, anyhow::Error> {
        // In a real implementation, this would interact with the blockchain
        // For now, simulate transaction creation
        
        let transaction_hash = format!("0x{}", hex::encode(rand::random::<[u8; 32]>()));
        
        Ok(CryptoTransaction {
            transaction_hash,
            from_address: "0x0000000000000000000000000000000000000000".to_string(),
            to_address: crypto_details.wallet_address.clone(),
            amount: crypto_details.amount_crypto,
            currency: crypto_details.currency.clone(),
            network: crypto_details.network.clone(),
            block_number: Some(rand::random::<u64>()),
            confirmation_count: 0,
            timestamp: Utc::now(),
            gas_used: Some(21000),
            gas_price: Some(20000000000), // 20 Gwei
        })
    }

    /// Check payment confirmation status
    pub async fn check_payment_status(&self, transaction_hash: &str) -> Result<ConfirmationStatus, anyhow::Error> {
        let cache = self.transaction_cache.lock().await;
        
        if let Some(transaction) = cache.get(transaction_hash) {
            let required_confirmations = self.config.confirmation_blocks;
            
            if transaction.confirmation_count >= required_confirmations {
                Ok(ConfirmationStatus::Confirmed)
            } else {
                Ok(ConfirmationStatus::Pending)
            }
        } else {
            Ok(ConfirmationStatus::Failed)
        }
    }

    /// Update transaction confirmations
    pub async fn update_transaction_confirmations(&self, transaction_hash: &str, confirmations: u32) -> Result<(), anyhow::Error> {
        let mut cache = self.transaction_cache.lock().await;
        
        if let Some(transaction) = cache.get_mut(transaction_hash) {
            transaction.confirmation_count = confirmations;
        }

        Ok(())
    }

    /// Create a subscription payment
    pub async fn create_subscription_payment(
        &self,
        amount: u64,
        currency: &str,
        customer_email: &str,
        crypto_details: CryptoPaymentDetails,
    ) -> Result<CryptoPaymentResult, anyhow::Error> {
        let request = CryptoPaymentRequest {
            amount,
            currency: currency.to_string(),
            description: Some("Subscription payment".to_string()),
            customer_email: Some(customer_email.to_string()),
            crypto_details,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("payment_type".to_string(), "subscription".to_string());
                meta
            },
        };

        self.process_payment(request).await
    }

    /// Get payment history
    pub async fn get_payment_history(&self, customer_email: &str) -> Result<Vec<CryptoPaymentResult>, anyhow::Error> {
        // In a real implementation, this would query a database
        // For now, return empty vector
        Ok(Vec::new())
    }

    /// Refund a crypto payment (if possible)
    pub async fn refund_payment(&self, transaction_hash: &str) -> Result<CryptoPaymentResult, anyhow::Error> {
        // Crypto refunds are more complex and depend on the specific implementation
        // Some cryptocurrencies support refunds through smart contracts
        Err(anyhow::anyhow!("Crypto refunds are not supported in this implementation"))
    }

    /// Get supported cryptocurrencies
    pub fn get_supported_currencies(&self) -> Vec<CryptoCurrency> {
        self.config.supported_currencies.clone()
    }

    /// Get supported blockchain networks
    pub fn get_supported_networks(&self) -> Vec<BlockchainNetwork> {
        self.config.blockchain_networks.clone()
    }

    /// Get transaction details
    pub async fn get_transaction_details(&self, transaction_hash: &str) -> Result<Option<CryptoTransaction>, anyhow::Error> {
        let cache = self.transaction_cache.lock().await;
        Ok(cache.get(transaction_hash).cloned())
    }
}

impl WalletManager {
    /// Create a new wallet manager
    pub fn new() -> Self {
        Self {
            wallets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a wallet
    pub async fn add_wallet(&self, wallet_info: WalletInfo) -> Result<(), anyhow::Error> {
        let mut wallets = self.wallets.lock().await;
        wallets.insert(wallet_info.currency.clone(), wallet_info);
        Ok(())
    }

    /// Get wallet balance
    pub async fn get_balance(&self, currency: &CryptoCurrency) -> Result<f64, anyhow::Error> {
        let wallets = self.wallets.lock().await;
        
        if let Some(wallet) = wallets.get(currency) {
            Ok(wallet.balance)
        } else {
            Err(anyhow::anyhow!("Wallet not found for currency: {:?}", currency))
        }
    }

    /// Get wallet address
    pub async fn get_wallet_address(&self, currency: &CryptoCurrency) -> Result<String, anyhow::Error> {
        let wallets = self.wallets.lock().await;
        
        if let Some(wallet) = wallets.get(currency) {
            Ok(wallet.address.clone())
        } else {
            Err(anyhow::anyhow!("Wallet not found for currency: {:?}", currency))
        }
    }
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            supported_currencies: vec![
                CryptoCurrency::Bitcoin,
                CryptoCurrency::Ethereum,
                CryptoCurrency::USDC,
                CryptoCurrency::USDT,
                CryptoCurrency::DAI,
            ],
            blockchain_networks: vec![
                BlockchainNetwork::Bitcoin,
                BlockchainNetwork::Ethereum,
                BlockchainNetwork::Polygon,
                BlockchainNetwork::BinanceSmartChain,
            ],
            smart_contract_address: None,
            gas_limit: 21000,
            confirmation_blocks: 6,
            price_feed_urls: HashMap::new(),
        }
    }
}

impl std::fmt::Display for CryptoCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoCurrency::Bitcoin => write!(f, "BTC"),
            CryptoCurrency::Ethereum => write!(f, "ETH"),
            CryptoCurrency::USDC => write!(f, "USDC"),
            CryptoCurrency::USDT => write!(f, "USDT"),
            CryptoCurrency::DAI => write!(f, "DAI"),
            CryptoCurrency::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::fmt::Display for BlockchainNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockchainNetwork::Bitcoin => write!(f, "Bitcoin"),
            BlockchainNetwork::Ethereum => write!(f, "Ethereum"),
            BlockchainNetwork::Polygon => write!(f, "Polygon"),
            BlockchainNetwork::BinanceSmartChain => write!(f, "BSC"),
            BlockchainNetwork::Arbitrum => write!(f, "Arbitrum"),
            BlockchainNetwork::Optimism => write!(f, "Optimism"),
            BlockchainNetwork::Custom(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_config_default() {
        let config = CryptoConfig::default();
        assert!(config.supported_currencies.contains(&CryptoCurrency::Bitcoin));
        assert!(config.supported_currencies.contains(&CryptoCurrency::Ethereum));
        assert_eq!(config.confirmation_blocks, 6);
    }

    #[tokio::test]
    async fn test_crypto_service_creation() {
        let config = CryptoConfig::default();
        let service = CryptoPaymentService::new(config);
        assert!(!service.get_supported_currencies().is_empty());
    }

    #[tokio::test]
    async fn test_crypto_payment_validation() {
        let config = CryptoConfig::default();
        let service = CryptoPaymentService::new(config);
        
        let crypto_details = CryptoPaymentDetails {
            currency: CryptoCurrency::Bitcoin,
            network: BlockchainNetwork::Bitcoin,
            wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            amount_crypto: 0.001,
            exchange_rate: 45000.0,
            expires_at: Utc::now() + Duration::hours(1),
        };
        
        let result = service.validate_crypto_payment(&crypto_details).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_wallet_address_validation() {
        let config = CryptoConfig::default();
        let service = CryptoPaymentService::new(config);
        
        // Valid Bitcoin address
        assert!(service.is_valid_wallet_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", &CryptoCurrency::Bitcoin));
        
        // Valid Ethereum address
        assert!(service.is_valid_wallet_address("0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6", &CryptoCurrency::Ethereum));
        
        // Invalid addresses
        assert!(!service.is_valid_wallet_address("invalid", &CryptoCurrency::Bitcoin));
        assert!(!service.is_valid_wallet_address("0xinvalid", &CryptoCurrency::Ethereum));
    }

    #[tokio::test]
    async fn test_exchange_rate() {
        let config = CryptoConfig::default();
        let service = CryptoPaymentService::new(config);
        
        let rate = service.get_exchange_rate(&CryptoCurrency::Bitcoin).await.unwrap();
        assert_eq!(rate, 45000.0);
    }

    #[tokio::test]
    async fn test_fee_calculation() {
        let config = CryptoConfig::default();
        let service = CryptoPaymentService::new(config);
        
        let crypto_details = CryptoPaymentDetails {
            currency: CryptoCurrency::Bitcoin,
            network: BlockchainNetwork::Bitcoin,
            wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            amount_crypto: 0.001,
            exchange_rate: 45000.0,
            expires_at: Utc::now() + Duration::hours(1),
        };
        
        let fees = service.calculate_fees(&crypto_details).await.unwrap();
        assert!(fees.network_fee > 0.0);
        assert!(fees.processing_fee > 0.0);
        assert!(fees.total_fee > 0.0);
    }
} 