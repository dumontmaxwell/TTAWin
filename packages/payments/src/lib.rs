use async_stripe::{
    Client, CreatePaymentIntent, CreatePaymentIntentPaymentMethodData, Currency, PaymentIntent,
    PaymentMethodType,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Payment configuration
#[derive(Debug, Clone)]
pub struct PaymentConfig {
    pub stripe_secret_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PricingTier {
    pub name: String,
    pub price: u64,
    pub currency: String,
}

/// Payment request data
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub amount: u64, // Amount in cents
    pub currency: String, // e.g., "usd"
    pub description: Option<String>,
    pub customer_email: Option<String>,
}

/// Payment result
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentResult {
    pub success: bool,
    pub payment_intent_id: Option<String>,
    pub error_message: Option<String>,
}

/// Payment service for handling Stripe payments
pub struct PaymentService {
    client: Arc<Client>,
}

impl PaymentService {
    /// Create a new payment service instance
    pub fn new(config: PaymentConfig) -> Self {
        let client = Arc::new(Client::new(&config.stripe_secret_key));
        Self { client }
    }

    /// Process a payment using Stripe
    pub async fn process_payment(&self, request: PaymentRequest) -> Result<PaymentResult, anyhow::Error> {
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
            error_message: None,
        })
    }

    /// Create a subscription payment (basic implementation)
    pub async fn create_subscription_payment(
        &self,
        amount: u64,
        currency: &str,
        customer_email: &str,
    ) -> Result<PaymentResult, anyhow::Error> {
        let request = PaymentRequest {
            amount,
            currency: currency.to_string(),
            description: Some("Subscription payment".to_string()),
            customer_email: Some(customer_email.to_string()),
        };

        self.process_payment(request).await
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
        };
        let service = PaymentService::new(config);
        assert!(service.client.as_ref().secret_key().starts_with("sk_test_"));
    }
}
