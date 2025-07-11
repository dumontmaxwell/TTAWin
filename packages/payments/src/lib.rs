use async_stripe::{
    Client, CreatePaymentIntent, CreatePaymentIntentPaymentMethodData, Currency, PaymentIntent,
    PaymentMethodType,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

/// Payment configuration
#[derive(Debug, Clone)]
pub struct PaymentConfig {
    pub stripe_secret_key: String,
    pub crypto_config: CryptoConfig,
}

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

/// Payment request data
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub amount: u64, // Amount in cents
    pub currency: String, // e.g., "usd" or "btc"
    pub description: Option<String>,
    pub customer_email: Option<String>,
    pub payment_method: PaymentMethod,
    pub metadata: HashMap<String, String>,
}

/// Payment methods
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentMethod {
    Stripe,
    Crypto(CryptoPaymentDetails),
}

/// Crypto payment details
#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoPaymentDetails {
    pub currency: CryptoCurrency,
    pub network: BlockchainNetwork,
    pub wallet_address: String,
    pub amount_crypto: f64,
    pub exchange_rate: f64,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Payment result
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResult {
    pub success: bool,
    pub payment_intent_id: Option<String>,
    pub transaction_hash: Option<String>,
    pub error_message: Option<String>,
    pub payment_method: PaymentMethod,
    pub confirmation_status: ConfirmationStatus,
    pub fees: PaymentFees,
}

/// Payment confirmation status
#[derive(Debug, Serialize, Deserialize)]
pub enum ConfirmationStatus {
    Pending,
    Confirmed,
    Failed,
    Expired,
}

/// Payment fees
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentFees {
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
    pub timestamp: chrono::DateTime<chrono::Utc>,
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

/// Payment service for handling both traditional and crypto payments
pub struct PaymentService {
    client: Arc<Client>,
    crypto_service: Arc<CryptoPaymentService>,
    config: PaymentConfig,
    transaction_cache: Arc<Mutex<HashMap<String, CryptoTransaction>>>,
}

/// Crypto payment service
pub struct CryptoPaymentService {
    config: CryptoConfig,
    price_feeds: Arc<Mutex<HashMap<String, f64>>>,
    wallet_manager: Arc<WalletManager>,
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

impl PaymentService {
    /// Create a new payment service instance
    pub fn new(config: PaymentConfig) -> Self {
        let client = Arc::new(Client::new(&config.stripe_secret_key));
        let crypto_service = Arc::new(CryptoPaymentService::new(config.crypto_config.clone()));
        
        Self {
            client,
            crypto_service,
            config,
            transaction_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Process a payment using the specified method
    pub async fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResult, anyhow::Error> {
        match request.payment_method {
            PaymentMethod::Stripe => self.process_stripe_payment(request).await,
            PaymentMethod::Crypto(crypto_details) => self.process_crypto_payment(request, crypto_details).await,
        }
    }

    /// Process traditional Stripe payment
    async fn process_stripe_payment(&self, request: PaymentRequest) -> Result<PaymentResult, anyhow::Error> {
        // Create payment intent
        let mut create_payment_intent = CreatePaymentIntent::new(request.amount);
        create_payment_intent.currency = Some(Currency::from(request.currency.as_str()));
        
        if let Some(description) = request.description {
            create_payment_intent.description = Some(description);
        }

        // Create the payment intent
        let payment_intent = PaymentIntent::create(&self.client, create_payment_intent).await?;

        Ok(PaymentResult {
            success: payment_intent.status == async_stripe::PaymentIntentStatus::RequiresPaymentMethod,
            payment_intent_id: Some(payment_intent.id.to_string()),
            transaction_hash: None,
            error_message: None,
            payment_method: PaymentMethod::Stripe,
            confirmation_status: ConfirmationStatus::Pending,
            fees: PaymentFees {
                network_fee: 0.0,
                processing_fee: 0.029, // 2.9% Stripe fee
                total_fee: request.amount as f64 * 0.029 / 100.0,
                currency: request.currency,
            },
        })
    }

    /// Process crypto payment
    async fn process_crypto_payment(
        &self,
        request: PaymentRequest,
        crypto_details: CryptoPaymentDetails,
    ) -> Result<PaymentResult, anyhow::Error> {
        // Validate crypto payment details
        self.validate_crypto_payment(&crypto_details).await?;

        // Get current exchange rate
        let exchange_rate = self.crypto_service.get_exchange_rate(&crypto_details.currency).await?;

        // Calculate fees
        let fees = self.crypto_service.calculate_fees(&crypto_details).await?;

        // Create crypto transaction
        let transaction = self.crypto_service.create_transaction(&crypto_details).await?;

        // Store transaction in cache
        let mut cache = self.transaction_cache.lock().await;
        cache.insert(transaction.transaction_hash.clone(), transaction.clone());

        Ok(PaymentResult {
            success: true,
            payment_intent_id: None,
            transaction_hash: Some(transaction.transaction_hash),
            error_message: None,
            payment_method: PaymentMethod::Crypto(crypto_details),
            confirmation_status: ConfirmationStatus::Pending,
            fees,
        })
    }

    /// Validate crypto payment details
    async fn validate_crypto_payment(&self, crypto_details: &CryptoPaymentDetails) -> Result<(), anyhow::Error> {
        // Check if currency is supported
        if !self.config.crypto_config.supported_currencies.contains(&crypto_details.currency) {
            return Err(anyhow::anyhow!("Unsupported cryptocurrency: {:?}", crypto_details.currency));
        }

        // Check if network is supported
        if !self.config.crypto_config.blockchain_networks.contains(&crypto_details.network) {
            return Err(anyhow::anyhow!("Unsupported blockchain network: {:?}", crypto_details.network));
        }

        // Validate wallet address format
        if !self.is_valid_wallet_address(&crypto_details.wallet_address, &crypto_details.currency) {
            return Err(anyhow::anyhow!("Invalid wallet address for currency: {:?}", crypto_details.currency));
        }

        // Check if payment has expired
        if crypto_details.expires_at < chrono::Utc::now() {
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
                (address.starts_with('1') || address.starts_with('3') || address.starts_with('bc1'))
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

    /// Check payment confirmation status
    pub async fn check_payment_status(&self, payment_result: &PaymentResult) -> Result<ConfirmationStatus, anyhow::Error> {
        match &payment_result.payment_method {
            PaymentMethod::Stripe => self.check_stripe_status(payment_result).await,
            PaymentMethod::Crypto(_) => self.check_crypto_status(payment_result).await,
        }
    }

    /// Check Stripe payment status
    async fn check_stripe_status(&self, payment_result: &PaymentResult) -> Result<ConfirmationStatus, anyhow::Error> {
        if let Some(payment_intent_id) = &payment_result.payment_intent_id {
            let payment_intent = PaymentIntent::retrieve(&self.client, payment_intent_id).await?;
            
            match payment_intent.status {
                async_stripe::PaymentIntentStatus::Succeeded => Ok(ConfirmationStatus::Confirmed),
                async_stripe::PaymentIntentStatus::Canceled => Ok(ConfirmationStatus::Failed),
                async_stripe::PaymentIntentStatus::RequiresPaymentMethod => Ok(ConfirmationStatus::Pending),
                _ => Ok(ConfirmationStatus::Pending),
            }
        } else {
            Ok(ConfirmationStatus::Failed)
        }
    }

    /// Check crypto payment status
    async fn check_crypto_status(&self, payment_result: &PaymentResult) -> Result<ConfirmationStatus, anyhow::Error> {
        if let Some(transaction_hash) = &payment_result.transaction_hash {
            let cache = self.transaction_cache.lock().await;
            
            if let Some(transaction) = cache.get(transaction_hash) {
                let required_confirmations = self.config.crypto_config.confirmation_blocks;
                
                if transaction.confirmation_count >= required_confirmations {
                    Ok(ConfirmationStatus::Confirmed)
                } else {
                    Ok(ConfirmationStatus::Pending)
                }
            } else {
                Ok(ConfirmationStatus::Failed)
            }
        } else {
            Ok(ConfirmationStatus::Failed)
        }
    }

    /// Create a subscription payment (supports both traditional and crypto)
    pub async fn create_subscription_payment(
        &self,
        amount: u64,
        currency: &str,
        customer_email: &str,
        payment_method: PaymentMethod,
    ) -> Result<PaymentResult, anyhow::Error> {
        let request = PaymentRequest {
            amount,
            currency: currency.to_string(),
            description: Some("Subscription payment".to_string()),
            customer_email: Some(customer_email.to_string()),
            payment_method,
            metadata: HashMap::new(),
        };

        self.process_payment(request).await
    }

    /// Get payment history
    pub async fn get_payment_history(&self, customer_email: &str) -> Result<Vec<PaymentResult>, anyhow::Error> {
        // In a real implementation, this would query a database
        // For now, return empty vector
        Ok(Vec::new())
    }

    /// Refund a payment
    pub async fn refund_payment(&self, payment_result: &PaymentResult) -> Result<PaymentResult, anyhow::Error> {
        match &payment_result.payment_method {
            PaymentMethod::Stripe => self.refund_stripe_payment(payment_result).await,
            PaymentMethod::Crypto(_) => self.refund_crypto_payment(payment_result).await,
        }
    }

    /// Refund Stripe payment
    async fn refund_stripe_payment(&self, payment_result: &PaymentResult) -> Result<PaymentResult, anyhow::Error> {
        // Stripe refund implementation
        // This would use Stripe's refund API
        Ok(PaymentResult {
            success: true,
            payment_intent_id: payment_result.payment_intent_id.clone(),
            transaction_hash: None,
            error_message: None,
            payment_method: PaymentMethod::Stripe,
            confirmation_status: ConfirmationStatus::Confirmed,
            fees: PaymentFees {
                network_fee: 0.0,
                processing_fee: 0.0,
                total_fee: 0.0,
                currency: "usd".to_string(),
            },
        })
    }

    /// Refund crypto payment (if possible)
    async fn refund_crypto_payment(&self, payment_result: &PaymentResult) -> Result<PaymentResult, anyhow::Error> {
        // Crypto refunds are more complex and depend on the specific implementation
        // Some cryptocurrencies support refunds through smart contracts
        Err(anyhow::anyhow!("Crypto refunds are not supported in this implementation"))
    }
}

impl CryptoPaymentService {
    /// Create a new crypto payment service
    pub fn new(config: CryptoConfig) -> Self {
        Self {
            config,
            price_feeds: Arc::new(Mutex::new(HashMap::new())),
            wallet_manager: Arc::new(WalletManager::new()),
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
    pub async fn calculate_fees(&self, crypto_details: &CryptoPaymentDetails) -> Result<PaymentFees, anyhow::Error> {
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

        Ok(PaymentFees {
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
            timestamp: chrono::Utc::now(),
            gas_used: Some(21000),
            gas_price: Some(20000000000), // 20 Gwei
        })
    }

    /// Update transaction confirmations
    pub async fn update_transaction_confirmations(&self, transaction_hash: &str, confirmations: u32) -> Result<(), anyhow::Error> {
        // In a real implementation, this would update the transaction in a database
        // For now, just log the update
        println!("Updated transaction {} with {} confirmations", transaction_hash, confirmations);
        Ok(())
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
    async fn test_payment_service_creation() {
        let config = PaymentConfig {
            stripe_secret_key: "sk_test_dummy_key".to_string(),
            crypto_config: CryptoConfig::default(),
        };
        let service = PaymentService::new(config);
        assert!(service.client.as_ref().secret_key().starts_with("sk_test_"));
    }

    #[tokio::test]
    async fn test_crypto_payment_validation() {
        let config = PaymentConfig {
            stripe_secret_key: "sk_test_dummy_key".to_string(),
            crypto_config: CryptoConfig::default(),
        };
        let service = PaymentService::new(config);
        
        let crypto_details = CryptoPaymentDetails {
            currency: CryptoCurrency::Bitcoin,
            network: BlockchainNetwork::Bitcoin,
            wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            amount_crypto: 0.001,
            exchange_rate: 45000.0,
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
        };
        
        let result = service.validate_crypto_payment(&crypto_details).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_wallet_address_validation() {
        let config = PaymentConfig {
            stripe_secret_key: "sk_test_dummy_key".to_string(),
            crypto_config: CryptoConfig::default(),
        };
        let service = PaymentService::new(config);
        
        // Valid Bitcoin address
        assert!(service.is_valid_wallet_address("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", &CryptoCurrency::Bitcoin));
        
        // Valid Ethereum address
        assert!(service.is_valid_wallet_address("0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6", &CryptoCurrency::Ethereum));
        
        // Invalid addresses
        assert!(!service.is_valid_wallet_address("invalid", &CryptoCurrency::Bitcoin));
        assert!(!service.is_valid_wallet_address("0xinvalid", &CryptoCurrency::Ethereum));
    }
}
