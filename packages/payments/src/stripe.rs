use async_stripe::{
    Client, CreatePaymentIntent, Currency, PaymentIntent,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Stripe payment configuration
#[derive(Debug, Clone)]
pub struct StripeConfig {
    pub secret_key: String,
    pub webhook_secret: Option<String>,
    pub api_version: Option<String>,
}

impl Default for StripeConfig {
    fn default() -> Self {
        Self {
            secret_key: "sk_test_dummy_key".to_string(),
            webhook_secret: None,
            api_version: None,
        }
    }
}

/// Stripe payment request
#[derive(Debug, Serialize, Deserialize)]
pub struct StripePaymentRequest {
    pub amount: u64, // Amount in cents
    pub currency: String, // e.g., "usd"
    pub description: Option<String>,
    pub customer_email: Option<String>,
    pub metadata: HashMap<String, String>,
    pub payment_method_types: Option<Vec<String>>,
    pub capture_method: Option<String>,
}

/// Stripe payment result
#[derive(Debug, Serialize, Deserialize)]
pub struct StripePaymentResult {
    pub success: bool,
    pub payment_intent_id: Option<String>,
    pub client_secret: Option<String>,
    pub status: StripePaymentStatus,
    pub error_message: Option<String>,
    pub fees: StripeFees,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Stripe payment status
#[derive(Debug, Serialize, Deserialize)]
pub enum StripePaymentStatus {
    RequiresPaymentMethod,
    RequiresConfirmation,
    RequiresAction,
    Processing,
    RequiresCapture,
    Canceled,
    Succeeded,
    Failed,
}

/// Stripe fees
#[derive(Debug, Serialize, Deserialize)]
pub struct StripeFees {
    pub processing_fee: f64,
    pub international_fee: f64,
    pub total_fee: f64,
    pub currency: String,
}

/// Stripe payment service
pub struct StripePaymentService {
    pub client: Arc<Client>,
    config: StripeConfig,
}

impl StripePaymentService {
    /// Create a new Stripe payment service
    pub fn new(config: StripeConfig) -> Self {
        let client = Arc::new(Client::new(&config.secret_key));
        Self { client, config }
    }

    /// Create a new Stripe payment service with default config
    pub fn new_with_key(secret_key: String) -> Self {
        let config = StripeConfig {
            secret_key,
            ..Default::default()
        };
        Self::new(config)
    }

    /// Process a Stripe payment
    pub async fn process_payment(&self, request: StripePaymentRequest) -> Result<StripePaymentResult, anyhow::Error> {
        // Create payment intent
        let mut create_payment_intent = CreatePaymentIntent::new(request.amount);
        create_payment_intent.currency = Some(Currency::from(request.currency.as_str()));
        
        if let Some(description) = request.description {
            create_payment_intent.description = Some(description);
        }

        if let Some(payment_method_types) = request.payment_method_types {
            create_payment_intent.payment_method_types = Some(payment_method_types);
        }

        if let Some(capture_method) = request.capture_method {
            create_payment_intent.capture_method = Some(capture_method);
        }

        // Add metadata if provided
        if !request.metadata.is_empty() {
            create_payment_intent.metadata = Some(request.metadata);
        }

        // Create the payment intent
        let payment_intent = PaymentIntent::create(&self.client, create_payment_intent).await?;

        // Calculate fees
        let fees = self.calculate_fees(request.amount, &request.currency);

        // Convert status
        let status = match payment_intent.status {
            async_stripe::PaymentIntentStatus::RequiresPaymentMethod => StripePaymentStatus::RequiresPaymentMethod,
            async_stripe::PaymentIntentStatus::RequiresConfirmation => StripePaymentStatus::RequiresConfirmation,
            async_stripe::PaymentIntentStatus::RequiresAction => StripePaymentStatus::RequiresAction,
            async_stripe::PaymentIntentStatus::Processing => StripePaymentStatus::Processing,
            async_stripe::PaymentIntentStatus::RequiresCapture => StripePaymentStatus::RequiresCapture,
            async_stripe::PaymentIntentStatus::Canceled => StripePaymentStatus::Canceled,
            async_stripe::PaymentIntentStatus::Succeeded => StripePaymentStatus::Succeeded,
        };

        Ok(StripePaymentResult {
            success: payment_intent.status == async_stripe::PaymentIntentStatus::RequiresPaymentMethod,
            payment_intent_id: Some(payment_intent.id.to_string()),
            client_secret: Some(payment_intent.client_secret.unwrap_or_default()),
            status,
            error_message: None,
            fees,
            created_at: chrono::Utc::now(),
        })
    }

    /// Confirm a payment intent
    pub async fn confirm_payment(&self, payment_intent_id: &str) -> Result<StripePaymentResult, anyhow::Error> {
        let payment_intent = PaymentIntent::retrieve(&self.client, payment_intent_id).await?;
        
        let fees = self.calculate_fees(payment_intent.amount, &payment_intent.currency.to_string());

        let status = match payment_intent.status {
            async_stripe::PaymentIntentStatus::RequiresPaymentMethod => StripePaymentStatus::RequiresPaymentMethod,
            async_stripe::PaymentIntentStatus::RequiresConfirmation => StripePaymentStatus::RequiresConfirmation,
            async_stripe::PaymentIntentStatus::RequiresAction => StripePaymentStatus::RequiresAction,
            async_stripe::PaymentIntentStatus::Processing => StripePaymentStatus::Processing,
            async_stripe::PaymentIntentStatus::RequiresCapture => StripePaymentStatus::RequiresCapture,
            async_stripe::PaymentIntentStatus::Canceled => StripePaymentStatus::Canceled,
            async_stripe::PaymentIntentStatus::Succeeded => StripePaymentStatus::Succeeded,
        };

        Ok(StripePaymentResult {
            success: payment_intent.status == async_stripe::PaymentIntentStatus::Succeeded,
            payment_intent_id: Some(payment_intent.id.to_string()),
            client_secret: Some(payment_intent.client_secret.unwrap_or_default()),
            status,
            error_message: None,
            fees,
            created_at: chrono::Utc::now(),
        })
    }

    /// Cancel a payment intent
    pub async fn cancel_payment(&self, payment_intent_id: &str) -> Result<StripePaymentResult, anyhow::Error> {
        let mut payment_intent = PaymentIntent::retrieve(&self.client, payment_intent_id).await?;
        payment_intent = payment_intent.cancel(&self.client).await?;

        let fees = self.calculate_fees(payment_intent.amount, &payment_intent.currency.to_string());

        Ok(StripePaymentResult {
            success: false,
            payment_intent_id: Some(payment_intent.id.to_string()),
            client_secret: None,
            status: StripePaymentStatus::Canceled,
            error_message: Some("Payment was canceled".to_string()),
            fees,
            created_at: chrono::Utc::now(),
        })
    }

    /// Refund a payment
    pub async fn refund_payment(&self, payment_intent_id: &str, amount: Option<u64>) -> Result<StripePaymentResult, anyhow::Error> {
        // In a real implementation, you would use Stripe's refund API
        // For now, we'll simulate a refund
        
        let payment_intent = PaymentIntent::retrieve(&self.client, payment_intent_id).await?;
        let refund_amount = amount.unwrap_or(payment_intent.amount);

        let fees = self.calculate_fees(refund_amount, &payment_intent.currency.to_string());

        Ok(StripePaymentResult {
            success: true,
            payment_intent_id: Some(payment_intent.id.to_string()),
            client_secret: None,
            status: StripePaymentStatus::Succeeded,
            error_message: None,
            fees: StripeFees {
                processing_fee: 0.0,
                international_fee: 0.0,
                total_fee: 0.0,
                currency: payment_intent.currency.to_string(),
            },
            created_at: chrono::Utc::now(),
        })
    }

    /// Get payment status
    pub async fn get_payment_status(&self, payment_intent_id: &str) -> Result<StripePaymentStatus, anyhow::Error> {
        let payment_intent = PaymentIntent::retrieve(&self.client, payment_intent_id).await?;
        
        let status = match payment_intent.status {
            async_stripe::PaymentIntentStatus::RequiresPaymentMethod => StripePaymentStatus::RequiresPaymentMethod,
            async_stripe::PaymentIntentStatus::RequiresConfirmation => StripePaymentStatus::RequiresConfirmation,
            async_stripe::PaymentIntentStatus::RequiresAction => StripePaymentStatus::RequiresAction,
            async_stripe::PaymentIntentStatus::Processing => StripePaymentStatus::Processing,
            async_stripe::PaymentIntentStatus::RequiresCapture => StripePaymentStatus::RequiresCapture,
            async_stripe::PaymentIntentStatus::Canceled => StripePaymentStatus::Canceled,
            async_stripe::PaymentIntentStatus::Succeeded => StripePaymentStatus::Succeeded,
        };

        Ok(status)
    }

    /// Create a subscription payment
    pub async fn create_subscription_payment(
        &self,
        amount: u64,
        currency: &str,
        customer_email: &str,
    ) -> Result<StripePaymentResult, anyhow::Error> {
        let request = StripePaymentRequest {
            amount,
            currency: currency.to_string(),
            description: Some("Subscription payment".to_string()),
            customer_email: Some(customer_email.to_string()),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("payment_type".to_string(), "subscription".to_string());
                meta
            },
            payment_method_types: Some(vec!["card".to_string()]),
            capture_method: Some("automatic".to_string()),
        };

        self.process_payment(request).await
    }

    /// Calculate Stripe fees
    fn calculate_fees(&self, amount: u64, currency: &str) -> StripeFees {
        let amount_f64 = amount as f64 / 100.0; // Convert cents to dollars
        
        // Standard Stripe fees: 2.9% + 30¢ for US cards
        let processing_fee = (amount_f64 * 0.029) + 0.30;
        
        // International fee: additional 1% for international cards
        let international_fee = amount_f64 * 0.01;
        
        let total_fee = processing_fee + international_fee;

        StripeFees {
            processing_fee,
            international_fee,
            total_fee,
            currency: currency.to_string(),
        }
    }

    /// Validate payment method
    pub fn validate_payment_method(&self, payment_method: &str) -> bool {
        // In a real implementation, you would validate against Stripe's payment method types
        matches!(payment_method, "card" | "bank_transfer" | "sepa_debit" | "sofort")
    }

    /// Get supported currencies
    pub fn get_supported_currencies() -> Vec<String> {
        vec![
            "usd".to_string(),
            "eur".to_string(),
            "gbp".to_string(),
            "cad".to_string(),
            "aud".to_string(),
            "jpy".to_string(),
            "chf".to_string(),
            "sek".to_string(),
            "nok".to_string(),
            "dkk".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stripe_config_default() {
        let config = StripeConfig::default();
        assert_eq!(config.secret_key, "sk_test_dummy_key");
        assert!(config.webhook_secret.is_none());
    }

    #[tokio::test]
    async fn test_stripe_service_creation() {
        let config = StripeConfig::default();
        let service = StripePaymentService::new(config);
        assert!(service.client.as_ref().secret_key().starts_with("sk_test_"));
    }

    #[test]
    fn test_fee_calculation() {
        let config = StripeConfig::default();
        let service = StripePaymentService::new(config);
        
        let fees = service.calculate_fees(1000, "usd"); // $10.00
        assert_eq!(fees.processing_fee, 0.59); // 2.9% of $10 + $0.30
        assert_eq!(fees.international_fee, 0.10); // 1% of $10
        assert_eq!(fees.total_fee, 0.69);
    }

    #[test]
    fn test_payment_method_validation() {
        let config = StripeConfig::default();
        let service = StripePaymentService::new(config);
        
        assert!(service.validate_payment_method("card"));
        assert!(service.validate_payment_method("bank_transfer"));
        assert!(!service.validate_payment_method("invalid"));
    }

    #[test]
    fn test_supported_currencies() {
        let currencies = StripePaymentService::get_supported_currencies();
        assert!(currencies.contains(&"usd".to_string()));
        assert!(currencies.contains(&"eur".to_string()));
        assert!(!currencies.contains(&"invalid".to_string()));
    }
} 