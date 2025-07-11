pub mod stripe;
pub mod crypto;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

// Re-export main types for easy access
pub use stripe::{
    StripePaymentService, StripeConfig, StripePaymentRequest, StripePaymentResult, 
    StripePaymentStatus, StripeFees
};

pub use crypto::{
    CryptoPaymentService, CryptoConfig, CryptoPaymentRequest, CryptoPaymentResult,
    CryptoPaymentDetails, CryptoCurrency, BlockchainNetwork, ConfirmationStatus,
    CryptoFees, CryptoTransaction, SmartContractPayment, WalletManager, WalletInfo
};

/// Unified payment configuration
#[derive(Debug, Clone)]
pub struct PaymentConfig {
    pub stripe_config: StripeConfig,
    pub crypto_config: CryptoConfig,
}

impl Default for PaymentConfig {
    fn default() -> Self {
        Self {
            stripe_config: StripeConfig::default(),
            crypto_config: CryptoConfig::default(),
        }
    }
}

/// Payment method types
#[derive(Debug, Serialize, Deserialize)]
pub enum PaymentMethod {
    Stripe,
    Crypto(CryptoPaymentDetails),
}

/// Unified payment request
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub amount: u64, // Amount in cents
    pub currency: String, // e.g., "usd" or "btc"
    pub description: Option<String>,
    pub customer_email: Option<String>,
    pub payment_method: PaymentMethod,
    pub metadata: HashMap<String, String>,
}

/// Unified payment result
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResult {
    pub success: bool,
    pub payment_intent_id: Option<String>,
    pub transaction_hash: Option<String>,
    pub client_secret: Option<String>,
    pub error_message: Option<String>,
    pub payment_method: PaymentMethod,
    pub confirmation_status: ConfirmationStatus,
    pub fees: PaymentFees,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Unified payment fees
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentFees {
    pub network_fee: f64,
    pub processing_fee: f64,
    pub international_fee: f64,
    pub total_fee: f64,
    pub currency: String,
}

/// Unified payment service that handles both Stripe and crypto payments
pub struct PaymentService {
    stripe_service: Arc<StripePaymentService>,
    crypto_service: Arc<CryptoPaymentService>,
    config: PaymentConfig,
}

impl PaymentService {
    /// Create a new unified payment service
    pub fn new(config: PaymentConfig) -> Self {
        let stripe_service = Arc::new(StripePaymentService::new(config.stripe_config.clone()));
        let crypto_service = Arc::new(CryptoPaymentService::new(config.crypto_config.clone()));
        
        Self {
            stripe_service,
            crypto_service,
            config,
        }
    }

    /// Create payment service with default configuration
    pub fn new_with_stripe_key(stripe_secret_key: String) -> Self {
        let config = PaymentConfig {
            stripe_config: StripeConfig {
                secret_key: stripe_secret_key,
                ..Default::default()
            },
            crypto_config: CryptoConfig::default(),
        };
        Self::new(config)
    }

    /// Process a payment using the specified method
    pub async fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResult, anyhow::Error> {
        match request.payment_method {
            PaymentMethod::Stripe => self.process_stripe_payment(request).await,
            PaymentMethod::Crypto(crypto_details) => self.process_crypto_payment(request, crypto_details).await,
        }
    }

    /// Process Stripe payment
    async fn process_stripe_payment(&self, request: PaymentRequest) -> Result<PaymentResult, anyhow::Error> {
        let stripe_request = stripe::StripePaymentRequest {
            amount: request.amount,
            currency: request.currency,
            description: request.description,
            customer_email: request.customer_email,
            metadata: request.metadata,
            payment_method_types: Some(vec!["card".to_string()]),
            capture_method: Some("automatic".to_string()),
        };

        let stripe_result = self.stripe_service.process_payment(stripe_request).await?;

        Ok(PaymentResult {
            success: stripe_result.success,
            payment_intent_id: stripe_result.payment_intent_id,
            transaction_hash: None,
            client_secret: stripe_result.client_secret,
            error_message: stripe_result.error_message,
            payment_method: PaymentMethod::Stripe,
            confirmation_status: match stripe_result.status {
                StripePaymentStatus::Succeeded => ConfirmationStatus::Confirmed,
                StripePaymentStatus::Canceled => ConfirmationStatus::Failed,
                _ => ConfirmationStatus::Pending,
            },
            fees: PaymentFees {
                network_fee: 0.0,
                processing_fee: stripe_result.fees.processing_fee,
                international_fee: stripe_result.fees.international_fee,
                total_fee: stripe_result.fees.total_fee,
                currency: stripe_result.fees.currency,
            },
            created_at: stripe_result.created_at,
        })
    }

    /// Process crypto payment
    async fn process_crypto_payment(
        &self,
        request: PaymentRequest,
        crypto_details: CryptoPaymentDetails,
    ) -> Result<PaymentResult, anyhow::Error> {
        let crypto_request = crypto::CryptoPaymentRequest {
            amount: request.amount,
            currency: request.currency,
            description: request.description,
            customer_email: request.customer_email,
            crypto_details: crypto_details.clone(),
            metadata: request.metadata,
        };

        let crypto_result = self.crypto_service.process_payment(crypto_request).await?;

        Ok(PaymentResult {
            success: crypto_result.success,
            payment_intent_id: None,
            transaction_hash: crypto_result.transaction_hash,
            client_secret: None,
            error_message: crypto_result.error_message,
            payment_method: PaymentMethod::Crypto(crypto_details),
            confirmation_status: crypto_result.confirmation_status,
            fees: PaymentFees {
                network_fee: crypto_result.fees.network_fee,
                processing_fee: crypto_result.fees.processing_fee,
                international_fee: 0.0,
                total_fee: crypto_result.fees.total_fee,
                currency: crypto_result.fees.currency,
            },
            created_at: crypto_result.created_at,
        })
    }

    /// Check payment status
    pub async fn check_payment_status(&self, payment_result: &PaymentResult) -> Result<ConfirmationStatus, anyhow::Error> {
        match &payment_result.payment_method {
            PaymentMethod::Stripe => {
                if let Some(payment_intent_id) = &payment_result.payment_intent_id {
                    let status = self.stripe_service.get_payment_status(payment_intent_id).await?;
                    Ok(match status {
                        StripePaymentStatus::Succeeded => ConfirmationStatus::Confirmed,
                        StripePaymentStatus::Canceled => ConfirmationStatus::Failed,
                        _ => ConfirmationStatus::Pending,
                    })
                } else {
                    Ok(ConfirmationStatus::Failed)
                }
            }
            PaymentMethod::Crypto(_) => {
                if let Some(transaction_hash) = &payment_result.transaction_hash {
                    self.crypto_service.check_payment_status(transaction_hash).await
                } else {
                    Ok(ConfirmationStatus::Failed)
                }
            }
        }
    }

    /// Create a subscription payment
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
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("payment_type".to_string(), "subscription".to_string());
                meta
            },
        };

        self.process_payment(request).await
    }

    /// Refund a payment
    pub async fn refund_payment(&self, payment_result: &PaymentResult) -> Result<PaymentResult, anyhow::Error> {
        match &payment_result.payment_method {
            PaymentMethod::Stripe => {
                if let Some(payment_intent_id) = &payment_result.payment_intent_id {
                    let stripe_result = self.stripe_service.refund_payment(payment_intent_id, None).await?;
                    
                    Ok(PaymentResult {
                        success: stripe_result.success,
                        payment_intent_id: stripe_result.payment_intent_id,
                        transaction_hash: None,
                        client_secret: stripe_result.client_secret,
                        error_message: stripe_result.error_message,
                        payment_method: PaymentMethod::Stripe,
                        confirmation_status: ConfirmationStatus::Confirmed,
                        fees: PaymentFees {
                            network_fee: 0.0,
                            processing_fee: 0.0,
                            international_fee: 0.0,
                            total_fee: 0.0,
                            currency: stripe_result.fees.currency,
                        },
                        created_at: stripe_result.created_at,
                    })
                } else {
                    Err(anyhow::anyhow!("No payment intent ID found for refund"))
                }
            }
            PaymentMethod::Crypto(_) => {
                if let Some(transaction_hash) = &payment_result.transaction_hash {
                    let crypto_result = self.crypto_service.refund_payment(transaction_hash).await?;
                    
                    Ok(PaymentResult {
                        success: crypto_result.success,
                        payment_intent_id: None,
                        transaction_hash: crypto_result.transaction_hash,
                        client_secret: None,
                        error_message: crypto_result.error_message,
                        payment_method: payment_result.payment_method.clone(),
                        confirmation_status: crypto_result.confirmation_status,
                        fees: PaymentFees {
                            network_fee: crypto_result.fees.network_fee,
                            processing_fee: crypto_result.fees.processing_fee,
                            international_fee: 0.0,
                            total_fee: crypto_result.fees.total_fee,
                            currency: crypto_result.fees.currency,
                        },
                        created_at: crypto_result.created_at,
                    })
                } else {
                    Err(anyhow::anyhow!("No transaction hash found for refund"))
                }
            }
        }
    }

    /// Get Stripe service reference
    pub fn stripe_service(&self) -> Arc<StripePaymentService> {
        Arc::clone(&self.stripe_service)
    }

    /// Get crypto service reference
    pub fn crypto_service(&self) -> Arc<CryptoPaymentService> {
        Arc::clone(&self.crypto_service)
    }

    /// Get supported payment methods
    pub fn get_supported_payment_methods() -> Vec<String> {
        vec![
            "stripe".to_string(),
            "bitcoin".to_string(),
            "ethereum".to_string(),
            "usdc".to_string(),
            "usdt".to_string(),
            "dai".to_string(),
        ]
    }

    /// Get supported currencies
    pub fn get_supported_currencies() -> Vec<String> {
        let mut currencies = StripePaymentService::get_supported_currencies();
        currencies.extend(vec![
            "btc".to_string(),
            "eth".to_string(),
            "usdc".to_string(),
            "usdt".to_string(),
            "dai".to_string(),
        ]);
        currencies
    }
}

// Legacy function for backward compatibility
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, Duration};

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_payment_service_creation() {
        let config = PaymentConfig::default();
        let service = PaymentService::new(config);
        assert!(service.stripe_service().client.as_ref().secret_key().starts_with("sk_test_"));
    }

    #[tokio::test]
    async fn test_stripe_payment() {
        let config = PaymentConfig::default();
        let service = PaymentService::new(config);
        
        let request = PaymentRequest {
            amount: 1000, // $10.00
            currency: "usd".to_string(),
            description: Some("Test payment".to_string()),
            customer_email: Some("test@example.com".to_string()),
            payment_method: PaymentMethod::Stripe,
            metadata: HashMap::new(),
        };

        let result = service.process_payment(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_crypto_payment() {
        let config = PaymentConfig::default();
        let service = PaymentService::new(config);
        
        let crypto_details = CryptoPaymentDetails {
            currency: CryptoCurrency::Bitcoin,
            network: BlockchainNetwork::Bitcoin,
            wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            amount_crypto: 0.001,
            exchange_rate: 45000.0,
            expires_at: Utc::now() + Duration::hours(1),
        };

        let request = PaymentRequest {
            amount: 45000, // $450.00
            currency: "usd".to_string(),
            description: Some("Test crypto payment".to_string()),
            customer_email: Some("test@example.com".to_string()),
            payment_method: PaymentMethod::Crypto(crypto_details),
            metadata: HashMap::new(),
        };

        let result = service.process_payment(request).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_supported_payment_methods() {
        let methods = PaymentService::get_supported_payment_methods();
        assert!(methods.contains(&"stripe".to_string()));
        assert!(methods.contains(&"bitcoin".to_string()));
        assert!(methods.contains(&"ethereum".to_string()));
    }

    #[test]
    fn test_supported_currencies() {
        let currencies = PaymentService::get_supported_currencies();
        assert!(currencies.contains(&"usd".to_string()));
        assert!(currencies.contains(&"btc".to_string()));
        assert!(currencies.contains(&"eth".to_string()));
    }
}
