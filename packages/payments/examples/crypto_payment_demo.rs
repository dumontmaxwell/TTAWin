use payments::{
    PaymentService, PaymentConfig, PaymentRequest, PaymentMethod,
    StripePaymentService, StripeConfig, StripePaymentRequest,
    CryptoPaymentService, CryptoConfig, CryptoPaymentRequest,
    CryptoPaymentDetails, CryptoCurrency, BlockchainNetwork,
    ConfirmationStatus, WalletManager, WalletInfo
};
use std::collections::HashMap;
use chrono::{Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 TTAWin Payments Demo - Unified Payment Processing");
    println!("==================================================\n");

    // Initialize payment services
    let config = PaymentConfig::default();
    let payment_service = PaymentService::new(config);

    // Demo 1: Traditional Stripe Payment
    println!("1️⃣ Traditional Stripe Payment");
    println!("----------------------------");
    
    let stripe_request = PaymentRequest {
        amount: 2500, // $25.00
        currency: "usd".to_string(),
        description: Some("Premium subscription".to_string()),
        customer_email: Some("customer@example.com".to_string()),
        payment_method: PaymentMethod::Stripe,
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("product_id".to_string(), "premium_sub".to_string());
            meta.insert("customer_tier".to_string(), "gold".to_string());
            meta
        },
    };

    match payment_service.process_payment(stripe_request).await {
        Ok(result) => {
            println!("✅ Stripe payment processed successfully!");
            println!("   Payment Intent ID: {}", result.payment_intent_id.as_deref().unwrap_or("N/A"));
            println!("   Client Secret: {}", result.client_secret.as_deref().unwrap_or("N/A"));
            println!("   Status: {:?}", result.confirmation_status);
            println!("   Fees: ${:.2}", result.fees.total_fee);
        }
        Err(e) => println!("❌ Stripe payment failed: {}", e),
    }

    println!();

    // Demo 2: Bitcoin Payment
    println!("2️⃣ Bitcoin Payment");
    println!("-----------------");
    
    let crypto_details = CryptoPaymentDetails {
        currency: CryptoCurrency::Bitcoin,
        network: BlockchainNetwork::Bitcoin,
        wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        amount_crypto: 0.001, // 0.001 BTC
        exchange_rate: 45000.0, // $45,000 per BTC
        expires_at: Utc::now() + Duration::hours(1),
    };

    let crypto_request = PaymentRequest {
        amount: 45000, // $450.00
        currency: "usd".to_string(),
        description: Some("Crypto payment for services".to_string()),
        customer_email: Some("crypto@example.com".to_string()),
        payment_method: PaymentMethod::Crypto(crypto_details),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("payment_type".to_string(), "crypto".to_string());
            meta.insert("crypto_currency".to_string(), "bitcoin".to_string());
            meta
        },
    };

    match payment_service.process_payment(crypto_request).await {
        Ok(result) => {
            println!("✅ Bitcoin payment processed successfully!");
            println!("   Transaction Hash: {}", result.transaction_hash.as_deref().unwrap_or("N/A"));
            println!("   Status: {:?}", result.confirmation_status);
            println!("   Network Fee: {:.6} BTC", result.fees.network_fee);
            println!("   Processing Fee: {:.6} BTC", result.fees.processing_fee);
            println!("   Total Fee: {:.6} BTC", result.fees.total_fee);
        }
        Err(e) => println!("❌ Bitcoin payment failed: {}", e),
    }

    println!();

    // Demo 3: Ethereum Payment
    println!("3️⃣ Ethereum Payment");
    println!("-------------------");
    
    let eth_crypto_details = CryptoPaymentDetails {
        currency: CryptoCurrency::Ethereum,
        network: BlockchainNetwork::Ethereum,
        wallet_address: "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6".to_string(),
        amount_crypto: 0.1, // 0.1 ETH
        exchange_rate: 3000.0, // $3,000 per ETH
        expires_at: Utc::now() + Duration::hours(1),
    };

    let eth_request = PaymentRequest {
        amount: 300000, // $3,000.00
        currency: "usd".to_string(),
        description: Some("Ethereum payment for premium features".to_string()),
        customer_email: Some("eth@example.com".to_string()),
        payment_method: PaymentMethod::Crypto(eth_crypto_details),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("payment_type".to_string(), "crypto".to_string());
            meta.insert("crypto_currency".to_string(), "ethereum".to_string());
            meta
        },
    };

    match payment_service.process_payment(eth_request).await {
        Ok(result) => {
            println!("✅ Ethereum payment processed successfully!");
            println!("   Transaction Hash: {}", result.transaction_hash.as_deref().unwrap_or("N/A"));
            println!("   Status: {:?}", result.confirmation_status);
            println!("   Network Fee: {:.6} ETH", result.fees.network_fee);
            println!("   Processing Fee: {:.6} ETH", result.fees.processing_fee);
            println!("   Total Fee: {:.6} ETH", result.fees.total_fee);
        }
        Err(e) => println!("❌ Ethereum payment failed: {}", e),
    }

    println!();

    // Demo 4: USDC Payment (Stablecoin)
    println!("4️⃣ USDC Payment (Stablecoin)");
    println!("----------------------------");
    
    let usdc_crypto_details = CryptoPaymentDetails {
        currency: CryptoCurrency::USDC,
        network: BlockchainNetwork::Polygon, // Using Polygon for lower fees
        wallet_address: "0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6".to_string(),
        amount_crypto: 100.0, // 100 USDC
        exchange_rate: 1.0, // 1:1 with USD
        expires_at: Utc::now() + Duration::hours(1),
    };

    let usdc_request = PaymentRequest {
        amount: 10000, // $100.00
        currency: "usd".to_string(),
        description: Some("USDC payment for micro-transactions".to_string()),
        customer_email: Some("usdc@example.com".to_string()),
        payment_method: PaymentMethod::Crypto(usdc_crypto_details),
        metadata: {
            let mut meta = HashMap::new();
            meta.insert("payment_type".to_string(), "stablecoin".to_string());
            meta.insert("crypto_currency".to_string(), "usdc".to_string());
            meta.insert("network".to_string(), "polygon".to_string());
            meta
        },
    };

    match payment_service.process_payment(usdc_request).await {
        Ok(result) => {
            println!("✅ USDC payment processed successfully!");
            println!("   Transaction Hash: {}", result.transaction_hash.as_deref().unwrap_or("N/A"));
            println!("   Status: {:?}", result.confirmation_status);
            println!("   Network Fee: {:.6} MATIC", result.fees.network_fee);
            println!("   Processing Fee: {:.2} USDC", result.fees.processing_fee);
            println!("   Total Fee: {:.2} USDC", result.fees.total_fee);
        }
        Err(e) => println!("❌ USDC payment failed: {}", e),
    }

    println!();

    // Demo 5: Subscription Payment (Stripe)
    println!("5️⃣ Subscription Payment (Stripe)");
    println!("--------------------------------");
    
    match payment_service.create_subscription_payment(
        1999, // $19.99
        "usd",
        "subscriber@example.com",
        PaymentMethod::Stripe,
    ).await {
        Ok(result) => {
            println!("✅ Subscription payment created successfully!");
            println!("   Payment Intent ID: {}", result.payment_intent_id.as_deref().unwrap_or("N/A"));
            println!("   Status: {:?}", result.confirmation_status);
            println!("   Processing Fee: ${:.2}", result.fees.processing_fee);
            println!("   International Fee: ${:.2}", result.fees.international_fee);
            println!("   Total Fee: ${:.2}", result.fees.total_fee);
        }
        Err(e) => println!("❌ Subscription payment failed: {}", e),
    }

    println!();

    // Demo 6: Payment Status Checking
    println!("6️⃣ Payment Status Checking");
    println!("---------------------------");
    
    // Create a test payment for status checking
    let test_request = PaymentRequest {
        amount: 1000, // $10.00
        currency: "usd".to_string(),
        description: Some("Status check test".to_string()),
        customer_email: Some("status@example.com".to_string()),
        payment_method: PaymentMethod::Stripe,
        metadata: HashMap::new(),
    };

    if let Ok(result) = payment_service.process_payment(test_request).await {
        println!("📊 Checking payment status...");
        
        match payment_service.check_payment_status(&result).await {
            Ok(status) => {
                println!("   Current Status: {:?}", status);
                match status {
                    ConfirmationStatus::Confirmed => println!("   ✅ Payment confirmed!"),
                    ConfirmationStatus::Pending => println!("   ⏳ Payment pending confirmation..."),
                    ConfirmationStatus::Failed => println!("   ❌ Payment failed"),
                    ConfirmationStatus::Expired => println!("   ⏰ Payment expired"),
                }
            }
            Err(e) => println!("   ❌ Error checking status: {}", e),
        }
    }

    println!();

    // Demo 7: Wallet Management
    println!("7️⃣ Wallet Management");
    println!("-------------------");
    
    let crypto_service = payment_service.crypto_service();
    let wallet_manager = &crypto_service.wallet_manager;

    // Add a Bitcoin wallet
    let btc_wallet = WalletInfo {
        address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        private_key: None, // In production, use secure key management
        balance: 0.5,
        currency: CryptoCurrency::Bitcoin,
        network: BlockchainNetwork::Bitcoin,
    };

    if let Ok(()) = wallet_manager.add_wallet(btc_wallet).await {
        println!("✅ Bitcoin wallet added successfully!");
        
        // Get wallet balance
        match wallet_manager.get_balance(&CryptoCurrency::Bitcoin).await {
            Ok(balance) => println!("   Balance: {:.6} BTC", balance),
            Err(e) => println!("   ❌ Error getting balance: {}", e),
        }

        // Get wallet address
        match wallet_manager.get_wallet_address(&CryptoCurrency::Bitcoin).await {
            Ok(address) => println!("   Address: {}", address),
            Err(e) => println!("   ❌ Error getting address: {}", e),
        }
    }

    println!();

    // Demo 8: Exchange Rate and Fee Calculation
    println!("8️⃣ Exchange Rate and Fee Calculation");
    println!("------------------------------------");
    
    let crypto_service = payment_service.crypto_service();
    
    // Get Bitcoin exchange rate
    match crypto_service.get_exchange_rate(&CryptoCurrency::Bitcoin).await {
        Ok(rate) => println!("   Bitcoin Exchange Rate: ${:.2} USD", rate),
        Err(e) => println!("   ❌ Error getting Bitcoin rate: {}", e),
    }

    // Get Ethereum exchange rate
    match crypto_service.get_exchange_rate(&CryptoCurrency::Ethereum).await {
        Ok(rate) => println!("   Ethereum Exchange Rate: ${:.2} USD", rate),
        Err(e) => println!("   ❌ Error getting Ethereum rate: {}", e),
    }

    // Calculate fees for a Bitcoin payment
    let fee_calc_details = CryptoPaymentDetails {
        currency: CryptoCurrency::Bitcoin,
        network: BlockchainNetwork::Bitcoin,
        wallet_address: "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa".to_string(),
        amount_crypto: 0.01, // 0.01 BTC
        exchange_rate: 45000.0,
        expires_at: Utc::now() + Duration::hours(1),
    };

    match crypto_service.calculate_fees(&fee_calc_details).await {
        Ok(fees) => {
            println!("   Bitcoin Payment Fees (0.01 BTC):");
            println!("     Network Fee: {:.6} BTC", fees.network_fee);
            println!("     Processing Fee: {:.6} BTC", fees.processing_fee);
            println!("     Total Fee: {:.6} BTC", fees.total_fee);
            println!("     Total Fee USD: ${:.2}", fees.total_fee * 45000.0);
        }
        Err(e) => println!("   ❌ Error calculating fees: {}", e),
    }

    println!();

    // Demo 9: Supported Currencies and Networks
    println!("9️⃣ Supported Currencies and Networks");
    println!("-----------------------------------");
    
    let supported_currencies = crypto_service.get_supported_currencies();
    let supported_networks = crypto_service.get_supported_networks();
    
    println!("   Supported Cryptocurrencies:");
    for currency in supported_currencies {
        println!("     - {}", currency);
    }
    
    println!("   Supported Blockchain Networks:");
    for network in supported_networks {
        println!("     - {}", network);
    }

    println!();

    // Demo 10: Payment Methods Summary
    println!("🔟 Payment Methods Summary");
    println!("-------------------------");
    
    let payment_methods = PaymentService::get_supported_payment_methods();
    let currencies = PaymentService::get_supported_currencies();
    
    println!("   Supported Payment Methods:");
    for method in payment_methods {
        println!("     - {}", method);
    }
    
    println!("   Supported Currencies:");
    for currency in currencies {
        println!("     - {}", currency);
    }

    println!("\n🎉 Payment processing demo completed successfully!");
    println!("The TTAWin payments system supports both traditional and crypto payments with:");
    println!("   • Secure Stripe integration for traditional payments");
    println!("   • Multi-cryptocurrency support (Bitcoin, Ethereum, USDC, USDT, DAI)");
    println!("   • Multiple blockchain networks (Bitcoin, Ethereum, Polygon, BSC)");
    println!("   • Real-time exchange rates and fee calculations");
    println!("   • Wallet management and transaction tracking");
    println!("   • Subscription and micro-payment support");

    Ok(())
} 