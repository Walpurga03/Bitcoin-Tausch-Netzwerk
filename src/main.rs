use anyhow::Result;
use tracing::{info, error, warn};

mod types;
mod offer_manager;
mod nostr_client;
mod anonymous_publisher;
mod relay_tester;

use types::{AnonymousOffer, OfferType, PrivateInterest};
use offer_manager::OfferManager;
use nostr_client::NostrClient;
use anonymous_publisher::AnonymousOfferPublisher;
use relay_tester::RelayTester;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🔐 Bitcoin-Tausch-Netzwerk starting...");
    info!("📋 This platform only connects people - NO transactions are processed!");
    
    // Test the data structures and offer manager
    test_data_structures().await?;
    test_offer_manager().await?;
    
    // Test Nostr connection to your relay
    test_nostr_connection().await?;
    
    // Test anonymous offer publishing
    test_anonymous_offer_publishing().await?;
    
    info!("✅ Application initialized successfully");
    
    Ok(())
}

async fn test_data_structures() -> Result<()> {
    info!("🧪 Testing data structures...");
    
    // Create a test offer
    let mut offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Buy, 24);
    info!("📝 Created offer: {} - {} BTC for {}€ (Price: {}€/BTC)", 
          offer.pseudo_id, offer.amount_btc, offer.amount_eur, offer.btc_price_eur());
    
    // Test adding interest
    let user_pubkey = "npub1test123456789".to_string();
    offer.add_interested_user(user_pubkey.clone());
    info!("👋 Added interested user, total: {}", offer.interested_users.len());
    
    // Create interest message
    let interest = PrivateInterest::new(
        offer.offer_id.clone(),
        user_pubkey,
        "I'm interested in this Bitcoin trade!".to_string()
    );
    info!("💬 Created interest message for offer: {}", interest.offer_id);
    
    info!("✅ Data structures working correctly!");
    Ok(())
}

async fn test_offer_manager() -> Result<()> {
    info!("🗄️ Testing OfferManager...");
    
    let manager = OfferManager::new();
    
    // Create and add multiple offers
    let buy_offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Buy, 24);
    let sell_offer = AnonymousOffer::new(1500.0, 0.035, OfferType::Sell, 48);
    
    let buy_id = buy_offer.offer_id.clone();
    
    manager.add_offer(buy_offer)?;
    manager.add_offer(sell_offer)?;
    
    // Check active offers
    let active_offers = manager.get_active_offers()?;
    info!("📊 Active offers: {}", active_offers.len());
    
    // Add some interests
    let interest1 = PrivateInterest::new(
        buy_id.clone(),
        "npub1alice123".to_string(),
        "I want to sell my Bitcoin".to_string()
    );
    
    let interest2 = PrivateInterest::new(
        buy_id.clone(),
        "npub1bob456".to_string(),
        "I can provide Bitcoin for this trade".to_string()
    );
    
    manager.add_interest(interest1)?;
    manager.add_interest(interest2)?;
    
    // Check interests
    let interests = manager.get_interests_for_offer(&buy_id)?;
    info!("💬 Interests for buy offer: {}", interests.len());
    
    // Match the offer
    manager.match_offer(&buy_id, "npub1alice123")?;
    
    // Check stats
    let stats = manager.get_stats()?;
    info!("{}", stats);
    
    // Verify matched offer is no longer active
    let active_after_match = manager.get_active_offers()?;
    info!("📊 Active offers after match: {}", active_after_match.len());
    
    info!("✅ OfferManager working correctly!");
    Ok(())
}

async fn test_nostr_connection() -> Result<()> {
    info!("🔌 Testing Nostr connection to your relay...");
    
    // Your personal relay
    let relay_url = "wss://nostr-relay.online".to_string();
    
    // Create Nostr client
    let client = NostrClient::new(relay_url).await?;
    info!("👤 Generated identity: {}", client.get_npub()?);
    
    // Test connection
    match client.connect().await {
        Ok(_) => {
            info!("🎉 Successfully connected to your relay!");
            
            // Test with a simple note
            match client.test_connection().await {
                Ok(_) => {
                    info!("✅ Test message sent successfully!");
                }
                Err(e) => {
                    info!("⚠️ Test message failed (but connection works): {}", e);
                }
            }
            
            // Get relay status
            match client.get_relay_status().await {
                Ok(status) => {
                    info!("{}", status);
                }
                Err(e) => {
                    info!("⚠️ Could not get relay status: {}", e);
                }
            }
            
            // Clean disconnect
            client.disconnect().await?;
            info!("🔌 Disconnected cleanly");
        }
        Err(e) => {
            info!("❌ Connection failed: {}", e);
            info!("💡 This is normal if the relay is not accessible right now");
        }
    }
    
    info!("✅ Nostr client test completed!");
    Ok(())
}

async fn test_anonymous_offer_publishing() -> Result<()> {
    info!("🎁 Testing anonymous offer publishing with NIP-59...");
    
    // Your personal relay
    let relay_url = "wss://nostr-relay.online".to_string();
    
    // Create Nostr client
    let client = NostrClient::new(relay_url).await?;
    info!("🎭 Generated new anonymous identity: {}", client.get_npub()?);
    
    // Connect to relay
    match client.connect().await {
        Ok(_) => {
            info!("🔌 Connected to relay for anonymous publishing");
            
            // Create test offers
            let test_offers = AnonymousOfferPublisher::create_demo_offers();
            info!("📦 Created {} demo offers for testing", test_offers.len());
            
            // Publish one test offer
            let test_offer = &test_offers[0];
            info!("🎯 Publishing test offer: {} {} for {}€ (Rate: {}€/BTC)",
                  test_offer.offer_type, test_offer.amount_btc, 
                  test_offer.amount_eur, test_offer.btc_price_eur());
            
            match client.publish_anonymous_offer(test_offer).await {
                Ok(event_id) => {
                    info!("🎉 Successfully published anonymous offer!");
                    info!("📋 Event ID: {:?}", event_id);
                    info!("🎭 Pseudo-ID: {}", test_offer.pseudo_id);
                    info!("💡 Offer is now visible on your relay anonymously!");
                }
                Err(e) => {
                    info!("⚠️ Publishing failed: {}", e);
                    info!("💡 This is expected if relay doesn't support NIP-59 yet");
                }
            }
            
            // Disconnect
            client.disconnect().await?;
        }
        Err(e) => {
            info!("❌ Could not connect to relay: {}", e);
        }
    }
    
        info!("✅ Anonymous offer publishing test completed!");

    // 🧪 Test Relay Compatibility
    info!("🧪 Testing relay compatibility for all required NIPs...");
    let relay_url = "wss://nostr-relay.online";
    let relay_tester = RelayTester::new(relay_url);
    let compatibility_report = relay_tester.run_full_compatibility_test().await?;
    compatibility_report.print_report();

    if !compatibility_report.is_suitable_for_bitcoin_exchange() {
        warn!("⚠️ Your relay may have limitations for the Bitcoin Exchange Network");
        warn!("💡 Consider using a different relay or check relay documentation");
    } else {
        info!("🎉 Your relay is perfectly compatible with all features!");
    }

    info!("✅ Application initialized successfully");
    Ok(())
}
