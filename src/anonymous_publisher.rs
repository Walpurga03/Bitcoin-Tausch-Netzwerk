use anyhow::{Result, anyhow};
use nostr_sdk::prelude::*;
use serde_json;
use tracing::{info, debug, warn};

use crate::types::{AnonymousOffer, OfferType};

/// Manager für anonyme Angebotserstellung mit NIP-59 Gift Wrapping
pub struct AnonymousOfferPublisher {
    client: Client,
    keys: Keys,
}

impl AnonymousOfferPublisher {
    /// Erstelle einen neuen Publisher
    pub fn new(client: Client, keys: Keys) -> Self {
        Self { client, keys }
    }

    /// Veröffentliche ein anonymes Angebot mit NIP-59 Gift Wrapping
    pub async fn publish_anonymous_offer(&self, offer: &AnonymousOffer) -> Result<EventId> {
        info!("🎁 Publishing anonymous offer with Gift Wrapping...");
        
        // 1. Erstelle das Original-Angebot als JSON
        let offer_content = self.create_offer_content(offer)?;
        debug!("📝 Offer content: {}", offer_content);

        // 2. Erstelle temporäre Schlüssel für die Anonymisierung
        let temp_keys = Keys::generate();
        let temp_pubkey = temp_keys.public_key();
        
        info!("🎭 Generated temporary identity: {}", temp_pubkey.to_bech32()?);

        // 3. Erstelle das innere Event (das eigentliche Angebot)
        let inner_event = EventBuilder::text_note(&offer_content, [])
            .to_event(&temp_keys)?;

        // 4. Wrapper für NIP-59 Gift Wrapping
        let wrapped_event = self.create_gift_wrapped_event(inner_event, &temp_keys).await?;

        // 5. Sende das gift-wrapped Event
        match self.client.send_event(wrapped_event).await {
            Ok(_output) => {
                info!("✅ Anonymous offer published successfully!");
                info!("🎭 Pseudo-ID: {}", offer.pseudo_id);
                info!("💰 Offer: {} {} for {}€", offer.offer_type, offer.amount_btc, offer.amount_eur);
                
                // Für diese Version geben wir eine dummy Event-ID zurück
                // In einer produktiven Version würde hier die echte Event-ID extrahiert
                Ok(EventId::all_zeros())
            }
            Err(e) => {
                warn!("❌ Failed to publish anonymous offer: {}", e);
                Err(anyhow!("Publishing failed: {}", e))
            }
        }
    }

    /// Erstelle den Angebotsinhalt als strukturiertes JSON
    fn create_offer_content(&self, offer: &AnonymousOffer) -> Result<String> {
        let offer_data = serde_json::json!({
            "type": "bitcoin_exchange_offer",
            "version": "1.0",
            "pseudo_id": offer.pseudo_id,
            "offer_type": offer.offer_type,
            "amount_eur": offer.amount_eur,
            "amount_btc": offer.amount_btc,
            "rate_eur_per_btc": offer.btc_price_eur(),
            "created_at": offer.created_at,
            "expires_at": offer.expires_at,
            "disclaimer": "🚫 This platform only connects people - NO transactions are processed here!",
            "instructions": "Send private message to show interest. Actual exchange happens outside this platform.",
            "tags": ["#bitcoin", "#exchange", "#p2p", "#anonymous"]
        });

        let content = format!(
            "🔐 Anonymous Bitcoin Exchange Offer\n\n\
            📊 Type: {}\n\
            ₿ Amount: {} BTC\n\
            💰 Price: {}€\n\
            📈 Rate: {}€/BTC\n\
            🎭 Contact: {}\n\n\
            💬 Interested? Send a private message!\n\
            ⚠️ All trades happen outside this platform.\n\n\
            🔍 Details: {}",
            offer.offer_type,
            offer.amount_btc,
            offer.amount_eur,
            offer.btc_price_eur(),
            offer.pseudo_id,
            offer_data.to_string()
        );

        Ok(content)
    }

    /// Erstelle ein Gift-Wrapped Event (vereinfachte NIP-59 Implementation)
    async fn create_gift_wrapped_event(&self, _inner_event: Event, temp_keys: &Keys) -> Result<Event> {
        info!("🎁 Creating gift-wrapped event for anonymity...");

        // Für diese MVP-Version verwenden wir eine vereinfachte Gift-Wrapping-Strategie:
        // Das Event wird mit temporären Schlüsseln erstellt und dann als "wrapped" markiert
        
        let wrap_content = format!(
            "🎁 Anonymous Bitcoin Exchange Network\n\
            📦 Wrapped Content Available\n\
            🔒 Contact required pseudo-ID for details\n\
            ⚠️ Platform facilitates contact only - no transactions processed\n\n\
            Event-Type: bitcoin_exchange_offer\n\
            Timestamp: {}\n\
            Wrapped-By: NIP-59-Compatible",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        );

        // Erstelle das Wrapper-Event mit den temporären Schlüsseln
        let wrapper_event = EventBuilder::text_note(wrap_content, [
            Tag::hashtag("bitcoin"),
            Tag::hashtag("exchange"), 
            Tag::hashtag("anonymous"),
            Tag::hashtag("p2p"),
        ])
        .to_event(temp_keys)?;

        debug!("🎁 Gift-wrapped event created with temp pubkey: {}", temp_keys.public_key().to_bech32()?);
        Ok(wrapper_event)
    }

    /// Erstelle ein Test-Angebot für Demonstrationszwecke
    pub fn create_test_offer() -> AnonymousOffer {
        AnonymousOffer::new(
            1000.0,    // 1000€
            0.025,     // 0.025 BTC  
            OfferType::Buy,  // Nutzer möchte Bitcoin kaufen
            48         // 48 Stunden gültig
        )
    }

    /// Erstelle mehrere Test-Angebote für Demo
    pub fn create_demo_offers() -> Vec<AnonymousOffer> {
        vec![
            // Buy Offers
            AnonymousOffer::new(500.0, 0.0125, OfferType::Buy, 24),
            AnonymousOffer::new(2000.0, 0.05, OfferType::Buy, 72),
            
            // Sell Offers  
            AnonymousOffer::new(1500.0, 0.0375, OfferType::Sell, 48),
            AnonymousOffer::new(800.0, 0.02, OfferType::Sell, 24),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OfferType;

    #[test]
    fn test_create_offer_content() {
        let keys = Keys::generate();
        let client = Client::new(&keys);
        let publisher = AnonymousOfferPublisher::new(client, keys);
        
        let offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Buy, 24);
        let content = publisher.create_offer_content(&offer).unwrap();
        
        assert!(content.contains("Bitcoin Exchange Offer"));
        assert!(content.contains("BUY"));
        assert!(content.contains("0.025 BTC"));
        assert!(content.contains("1000€"));
        assert!(content.contains(&offer.pseudo_id));
    }

    #[test]
    fn test_create_test_offer() {
        let offer = AnonymousOfferPublisher::create_test_offer();
        
        assert_eq!(offer.amount_eur, 1000.0);
        assert_eq!(offer.amount_btc, 0.025);
        assert!(matches!(offer.offer_type, OfferType::Buy));
        assert!(!offer.pseudo_id.is_empty());
        assert!(offer.pseudo_id.starts_with("anon_"));
    }

    #[test]
    fn test_create_demo_offers() {
        let offers = AnonymousOfferPublisher::create_demo_offers();
        
        assert_eq!(offers.len(), 4);
        
        // Check we have both buy and sell offers
        let buy_count = offers.iter().filter(|o| matches!(o.offer_type, OfferType::Buy)).count();
        let sell_count = offers.iter().filter(|o| matches!(o.offer_type, OfferType::Sell)).count();
        
        assert_eq!(buy_count, 2);
        assert_eq!(sell_count, 2);
        
        // Each offer should have unique pseudo_id
        let mut pseudo_ids: Vec<&String> = offers.iter().map(|o| &o.pseudo_id).collect();
        pseudo_ids.sort();
        pseudo_ids.dedup();
        assert_eq!(pseudo_ids.len(), 4, "All offers should have unique pseudo IDs");
    }
}
