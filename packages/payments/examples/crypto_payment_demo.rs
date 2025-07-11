use payments::{
    PaymentService, PaymentConfig, PaymentRequest, PaymentMethod, CryptoPaymentDetails,
    CryptoCurrency, BlockchainNetwork, ConfirmationStatus,
};
use std::collections::HashMap;
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 TTAWin Crypto Payment Demo");
    println!("=============================\n");

    // Initialize payment service with crypto support
    let config = PaymentConfig {
        stripe_secret_key: "sk_test_dummy_key".to_string(),
        crypto_config: payments::CryptoConfig::default(),
    };
    
    let payment_service = PaymentService::new(config);
    println!("✅ Payment service initialized with crypto support!\n");

    // Demo 1: E-commerce Crypto Payment
    println!("🛒 Demo 1: E-commerce Crypto Payment");
    println!("------------------------------------");
    
    let ecommerce_payment = PaymentRequest {
        amount: 5000, // $50.00
        currency: "usd".to_string(),
        description: Some("Premium Interview Coaching Session".to_string()),
        customer_email: Some("user@example.com".to_string()),
        payment_method: PaymentMethod::Crypto(CryptoPaymentDetails {
            currency: CryptoCurrency::Bitcoin,
            network: BlockchainNetwork::Bitcoin,
            wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            amount_crypto: 0.001, // BTC
            exchange_rate: 45000.0,
            expires_at: Utc::now() + Duration::hours(1),
        }),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("order_id".to_string(), "ORD-12345".to_string());
            meta.insert("product_type".to_string(), "coaching".to_string());
            meta
        },
    };

    match payment_service.process_payment(ecommerce_payment).await {
        Ok(result) => {
            println!("📊 E-commerce Payment Result:");
            println!("   Success: {}", result.success);
            println!("   Transaction Hash: {:?}", result.transaction_hash);
            println!("   Status: {:?}", result.confirmation_status);
            println!("   Network Fee: {:.6} BTC", result.fees.network_fee);
            println!("   Processing Fee: {:.6} BTC", result.fees.processing_fee);
            println!("   Total Fee: {:.6} BTC", result.fees.total_fee);
        }
        Err(e) => {
            println!("❌ E-commerce payment failed: {}", e);
        }
    }
    println!();

    // Demo 2: Subscription Payment with Stablecoin
    println!("🔄 Demo 2: Subscription Payment with USDC");
    println!("-------------------------------------------");
    
    let subscription_payment = PaymentRequest {
        amount: 2500, // $25.00
        currency: "usd".to_string(),
        description: Some("Monthly Premium Membership".to_string()),
        customer_email: Some("subscriber@example.com".to_string()),
        payment_method: PaymentMethod::Crypto(CryptoPaymentDetails {
            currency: CryptoCurrency::USDC,
            network: BlockchainNetwork::Ethereum,
            wallet_address: "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6".to_string(),
            amount_crypto: 25.0, // USDC
            exchange_rate: 1.0,
            expires_at: Utc::now() + Duration::hours(24),
        }),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("subscription_id".to_string(), "SUB-67890".to_string());
            meta.insert("billing_cycle".to_string(), "monthly".to_string());
            meta
        },
    };

    match payment_service.process_payment(subscription_payment).await {
        Ok(result) => {
            println!("📊 Subscription Payment Result:");
            println!("   Success: {}", result.success);
            println!("   Transaction Hash: {:?}", result.transaction_hash);
            println!("   Status: {:?}", result.confirmation_status);
            println!("   Network Fee: {:.6} ETH", result.fees.network_fee);
            println!("   Processing Fee: {:.2} USDC", result.fees.processing_fee);
            println!("   Total Fee: {:.2} USDC", result.fees.total_fee);
        }
        Err(e) => {
            println!("❌ Subscription payment failed: {}", e);
        }
    }
    println!();

    // Demo 3: Micro-payment with Ethereum
    println!("💎 Demo 3: Micro-payment with Ethereum");
    println!("--------------------------------------");
    
    let micropayment = PaymentRequest {
        amount: 100, // $1.00
        currency: "usd".to_string(),
        description: Some("Pay-per-view Interview Question".to_string()),
        customer_email: Some("viewer@example.com".to_string()),
        payment_method: PaymentMethod::Crypto(CryptoPaymentDetails {
            currency: CryptoCurrency::Ethereum,
            network: BlockchainNetwork::Polygon, // Lower fees for micro-payments
            wallet_address: "0x1234567890123456789012345678901234567890".to_string(),
            amount_crypto: 0.0005, // ETH
            exchange_rate: 3000.0,
            expires_at: Utc::now() + Duration::minutes(30),
        }),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("content_id".to_string(), "CONTENT-111".to_string());
            meta.insert("payment_type".to_string(), "micro".to_string());
            meta
        },
    };

    match payment_service.process_payment(micropayment).await {
        Ok(result) => {
            println!("📊 Micro-payment Result:");
            println!("   Success: {}", result.success);
            println!("   Transaction Hash: {:?}", result.transaction_hash);
            println!("   Status: {:?}", result.confirmation_status);
            println!("   Network Fee: {:.6} MATIC", result.fees.network_fee);
            println!("   Processing Fee: {:.6} ETH", result.fees.processing_fee);
            println!("   Total Fee: {:.6} ETH", result.fees.total_fee);
        }
        Err(e) => {
            println!("❌ Micro-payment failed: {}", e);
        }
    }
    println!();

    // Demo 4: Traditional Stripe Payment (for comparison)
    println!("💳 Demo 4: Traditional Stripe Payment");
    println!("-------------------------------------");
    
    let stripe_payment = PaymentRequest {
        amount: 10000, // $100.00
        currency: "usd".to_string(),
        description: Some("Enterprise Interview Package".to_string()),
        customer_email: Some("enterprise@example.com".to_string()),
        payment_method: PaymentMethod::Stripe,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("package_type".to_string(), "enterprise".to_string());
            meta.insert("customer_tier".to_string(), "premium".to_string());
            meta
        },
    };

    match payment_service.process_payment(stripe_payment).await {
        Ok(result) => {
            println!("📊 Stripe Payment Result:");
            println!("   Success: {}", result.success);
            println!("   Payment Intent ID: {:?}", result.payment_intent_id);
            println!("   Status: {:?}", result.confirmation_status);
            println!("   Processing Fee: ${:.2}", result.fees.processing_fee);
            println!("   Total Fee: ${:.2}", result.fees.total_fee);
        }
        Err(e) => {
            println!("❌ Stripe payment failed: {}", e);
        }
    }
    println!();

    // Demo 5: Payment Status Checking
    println!("🔍 Demo 5: Payment Status Checking");
    println!("----------------------------------");
    
    // Simulate checking payment status
    let crypto_payment = PaymentRequest {
        amount: 5000,
        currency: "usd".to_string(),
        description: Some("Test Payment".to_string()),
        customer_email: Some("test@example.com".to_string()),
        payment_method: PaymentMethod::Crypto(CryptoPaymentDetails {
            currency: CryptoCurrency::Bitcoin,
            network: BlockchainNetwork::Bitcoin,
            wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            amount_crypto: 0.001,
            exchange_rate: 45000.0,
            expires_at: Utc::now() + Duration::hours(1),
        }),
        metadata: HashMap::new(),
    };

    if let Ok(result) = payment_service.process_payment(crypto_payment).await {
        println!("📊 Initial Payment Status: {:?}", result.confirmation_status);
        
        // Simulate checking status after some time
        match payment_service.check_payment_status(&result).await {
            Ok(status) => {
                println!("📊 Updated Payment Status: {:?}", status);
                match status {
                    ConfirmationStatus::Confirmed => println!("   ✅ Payment confirmed! Order can be fulfilled."),
                    ConfirmationStatus::Pending => println!("   ⏳ Payment pending confirmation..."),
                    ConfirmationStatus::Failed => println!("   ❌ Payment failed or expired."),
                    ConfirmationStatus::Expired => println!("   ⏰ Payment request expired."),
                }
            }
            Err(e) => {
                println!("❌ Status check failed: {}", e);
            }
        }
    }
    println!();

    // Demo 6: Subscription Management
    println!("🔄 Demo 6: Subscription Management");
    println!("----------------------------------");
    
    let subscription_result = payment_service.create_subscription_payment(
        2500, // $25.00
        "usd",
        "subscriber@example.com",
        PaymentMethod::Crypto(CryptoPaymentDetails {
            currency: CryptoCurrency::USDC,
            network: BlockchainNetwork::Ethereum,
            wallet_address: "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6".to_string(),
            amount_crypto: 25.0,
            exchange_rate: 1.0,
            expires_at: Utc::now() + Duration::hours(24),
        }),
    ).await;

    match subscription_result {
        Ok(result) => {
            println!("📊 Subscription Created:");
            println!("   Success: {}", result.success);
            println!("   Transaction Hash: {:?}", result.transaction_hash);
            println!("   Status: {:?}", result.confirmation_status);
            println!("   Next billing: Monthly recurring");
        }
        Err(e) => {
            println!("❌ Subscription creation failed: {}", e);
        }
    }
    println!();

    // Demo 7: Payment History
    println!("📋 Demo 7: Payment History");
    println!("---------------------------");
    
    match payment_service.get_payment_history("user@example.com").await {
        Ok(history) => {
            println!("📊 Payment History for user@example.com:");
            println!("   Total payments: {}", history.len());
            
            for (i, payment) in history.iter().enumerate() {
                println!("   {}. {} - {:?} - ${:.2}", 
                    i + 1,
                    payment.payment_method.to_string(),
                    payment.confirmation_status,
                    payment.fees.total_fee
                );
            }
        }
        Err(e) => {
            println!("❌ Failed to retrieve payment history: {}", e);
        }
    }
    println!();

    // Demo 8: Real-world Payment Flow Simulation
    println!("🌐 Demo 8: Real-world Payment Flow Simulation");
    println!("---------------------------------------------");
    
    simulate_real_world_payment_flow(&payment_service).await;
    println!();

    println!("🎉 Crypto Payment Demo Completed!");
    println!("📚 Key Benefits of Crypto Payments:");
    println!("   • Lower fees (especially for international payments)");
    println!("   • Faster settlement (minutes vs days)");
    println!("   • No chargebacks (reduces fraud risk)");
    println!("   • Global accessibility");
    println!("   • Programmable money (smart contracts)");
    println!("   • Privacy options available");

    Ok(())
}

async fn simulate_real_world_payment_flow(payment_service: &PaymentService) {
    println!("🔄 Simulating Real-world Payment Flow:");
    println!("   1. Customer selects crypto payment");
    println!("   2. System generates payment request");
    println!("   3. Customer receives wallet address");
    println!("   4. Customer sends payment from wallet");
    println!("   5. System monitors blockchain for confirmation");
    println!("   6. Payment confirmed, order fulfilled");
    println!("   7. Customer receives confirmation email");
    
    // Step 1: Customer selects crypto payment
    println!("   ✅ Step 1: Customer selects Bitcoin payment");
    
    // Step 2: System generates payment request
    let payment_request = PaymentRequest {
        amount: 7500, // $75.00
        currency: "usd".to_string(),
        description: Some("Advanced Interview Preparation Package".to_string()),
        customer_email: Some("customer@example.com".to_string()),
        payment_method: PaymentMethod::Crypto(CryptoPaymentDetails {
            currency: CryptoCurrency::Bitcoin,
            network: BlockchainNetwork::Bitcoin,
            wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
            amount_crypto: 0.0015, // BTC
            exchange_rate: 45000.0,
            expires_at: Utc::now() + Duration::hours(1),
        }),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("order_id".to_string(), "ORD-REAL-001".to_string());
            meta.insert("customer_id".to_string(), "CUST-123".to_string());
            meta
        },
    };
    
    println!("   ✅ Step 2: Payment request generated");
    println!("      Amount: 0.0015 BTC (≈ $75.00)");
    println!("      Wallet: 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa");
    println!("      Expires: {}", payment_request.payment_method.get_expiry().unwrap());
    
    // Step 3: Customer receives wallet address
    println!("   ✅ Step 3: Customer receives wallet address via QR code/email");
    
    // Step 4: Customer sends payment (simulated)
    println!("   ✅ Step 4: Customer sends payment from their wallet");
    
    // Step 5: System processes payment
    match payment_service.process_payment(payment_request).await {
        Ok(result) => {
            println!("   ✅ Step 5: Payment processed successfully");
            println!("      Transaction Hash: {}", result.transaction_hash.as_ref().unwrap());
            println!("      Status: {:?}", result.confirmation_status);
            
            // Step 6: Monitor confirmation
            println!("   ✅ Step 6: Monitoring blockchain for confirmations...");
            
            // Simulate confirmation process
            for i in 1..=6 {
                println!("      Block {}: {} confirmations", i, i);
                if i >= 6 {
                    println!("      ✅ Payment confirmed! (6+ confirmations)");
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            
            // Step 7: Order fulfillment
            println!("   ✅ Step 7: Order fulfilled automatically");
            println!("      - Interview materials sent to customer@example.com");
            println!("      - Access granted to premium content");
            println!("      - Confirmation email sent");
            
            println!("   🎉 Payment flow completed successfully!");
        }
        Err(e) => {
            println!("   ❌ Payment processing failed: {}", e);
        }
    }
}

// Helper trait for getting expiry time
trait PaymentMethodExt {
    fn get_expiry(&self) -> Option<chrono::DateTime<chrono::Utc>>;
}

impl PaymentMethodExt for PaymentMethod {
    fn get_expiry(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        match self {
            PaymentMethod::Crypto(details) => Some(details.expires_at),
            PaymentMethod::Stripe => None, // Stripe handles expiry internally
        }
    }
}

impl std::fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentMethod::Stripe => write!(f, "Stripe"),
            PaymentMethod::Crypto(details) => write!(f, "Crypto ({})", details.currency),
        }
    }
} 