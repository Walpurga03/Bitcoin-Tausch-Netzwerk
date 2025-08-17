use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use anyhow::{Result, anyhow};
use tracing::{info, debug};

use crate::types::{AnonymousOffer, PrivateInterest, OfferStatus, OfferType};

/// Thread-sicherer Manager für alle Angebote und Interessen
#[derive(Clone)]
pub struct OfferManager {
    /// Alle aktiven Angebote, indiziert nach offer_id
    offers: Arc<RwLock<HashMap<String, AnonymousOffer>>>,
    /// Alle Interessensbekundungen, indiziert nach offer_id
    interests: Arc<RwLock<HashMap<String, Vec<PrivateInterest>>>>,
}

impl OfferManager {
    /// Erstelle einen neuen OfferManager
    pub fn new() -> Self {
        info!("🗄️ Initializing OfferManager...");
        Self {
            offers: Arc::new(RwLock::new(HashMap::new())),
            interests: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Füge ein neues Angebot hinzu
    pub fn add_offer(&self, offer: AnonymousOffer) -> Result<()> {
        let mut offers = self.offers.write().map_err(|_| anyhow!("Failed to acquire write lock"))?;
        
        info!("📝 Adding new offer: {} ({} {} for {}€)", 
              offer.pseudo_id, offer.offer_type, offer.amount_btc, offer.amount_eur);
        
        offers.insert(offer.offer_id.clone(), offer);
        Ok(())
    }

    /// Hole ein Angebot per ID
    #[allow(dead_code)]
    pub fn get_offer(&self, offer_id: &str) -> Result<Option<AnonymousOffer>> {
        let offers = self.offers.read().map_err(|_| anyhow!("Failed to acquire read lock"))?;
        Ok(offers.get(offer_id).cloned())
    }

    /// Hole alle aktiven Angebote (für öffentliche Anzeige)
    pub fn get_active_offers(&self) -> Result<Vec<AnonymousOffer>> {
        let offers = self.offers.read().map_err(|_| anyhow!("Failed to acquire read lock"))?;
        
        let active_offers: Vec<AnonymousOffer> = offers
            .values()
            .filter(|offer| matches!(offer.status, OfferStatus::Active))
            .filter(|offer| !offer.is_expired())
            .cloned()
            .collect();

        debug!("📊 Found {} active offers", active_offers.len());
        Ok(active_offers)
    }

    /// Füge eine Interessensbekundung hinzu
    pub fn add_interest(&self, interest: PrivateInterest) -> Result<()> {
        // Prüfe ob das Angebot existiert
        {
            let offers = self.offers.read().map_err(|_| anyhow!("Failed to acquire read lock"))?;
            if !offers.contains_key(&interest.offer_id) {
                return Err(anyhow!("Offer {} not found", interest.offer_id));
            }
        }

        // Füge Interesse hinzu
        let mut interests = self.interests.write().map_err(|_| anyhow!("Failed to acquire write lock"))?;
        let offer_interests = interests.entry(interest.offer_id.clone()).or_insert_with(Vec::new);
        
        info!("👋 Adding interest from {} for offer {}", 
              interest.interested_user_pubkey, interest.offer_id);
        
        offer_interests.push(interest);

        // Aktualisiere auch das Angebot
        {
            let mut offers = self.offers.write().map_err(|_| anyhow!("Failed to acquire write lock"))?;
            if let Some(offer) = offers.get_mut(&offer_interests.last().unwrap().offer_id) {
                offer.add_interested_user(offer_interests.last().unwrap().interested_user_pubkey.clone());
            }
        }

        Ok(())
    }

    /// Hole alle Interessen für ein Angebot (nur für Angebotsersteller)
    pub fn get_interests_for_offer(&self, offer_id: &str) -> Result<Vec<PrivateInterest>> {
        let interests = self.interests.read().map_err(|_| anyhow!("Failed to acquire read lock"))?;
        
        let offer_interests = interests.get(offer_id).cloned().unwrap_or_default();
        debug!("💬 Found {} interests for offer {}", offer_interests.len(), offer_id);
        
        Ok(offer_interests)
    }

    /// Markiere ein Angebot als "Matched" und entferne es aus der öffentlichen Liste
    pub fn match_offer(&self, offer_id: &str, selected_user: &str) -> Result<()> {
        let mut offers = self.offers.write().map_err(|_| anyhow!("Failed to acquire write lock"))?;
        
        if let Some(offer) = offers.get_mut(offer_id) {
            offer.status = OfferStatus::Matched;
            info!("🤝 Offer {} matched with user {}", offer_id, selected_user);
            Ok(())
        } else {
            Err(anyhow!("Offer {} not found", offer_id))
        }
    }

    /// Entferne abgelaufene Angebote
    #[allow(dead_code)]
    pub fn cleanup_expired_offers(&self) -> Result<u32> {
        let mut offers = self.offers.write().map_err(|_| anyhow!("Failed to acquire write lock"))?;
        let mut interests = self.interests.write().map_err(|_| anyhow!("Failed to acquire write lock"))?;
        
        let mut removed_count = 0;
        let expired_ids: Vec<String> = offers
            .iter()
            .filter(|(_, offer)| offer.is_expired())
            .map(|(id, _)| id.clone())
            .collect();

        for offer_id in expired_ids {
            offers.remove(&offer_id);
            interests.remove(&offer_id);
            removed_count += 1;
        }

        if removed_count > 0 {
            info!("🗑️ Cleaned up {} expired offers", removed_count);
        }

        Ok(removed_count)
    }

    /// Hole Statistiken über den aktuellen Zustand
    pub fn get_stats(&self) -> Result<OfferStats> {
        let offers = self.offers.read().map_err(|_| anyhow!("Failed to acquire read lock"))?;
        let interests = self.interests.read().map_err(|_| anyhow!("Failed to acquire read lock"))?;

        let total_offers = offers.len();
        let active_offers = offers.values().filter(|o| matches!(o.status, OfferStatus::Active)).count();
        let matched_offers = offers.values().filter(|o| matches!(o.status, OfferStatus::Matched)).count();
        let buy_offers = offers.values().filter(|o| matches!(o.offer_type, OfferType::Buy)).count();
        let sell_offers = offers.values().filter(|o| matches!(o.offer_type, OfferType::Sell)).count();
        let total_interests = interests.values().map(|v| v.len()).sum();

        Ok(OfferStats {
            total_offers,
            active_offers,
            matched_offers,
            buy_offers,
            sell_offers,
            total_interests,
        })
    }
}

/// Statistiken über den OfferManager
#[derive(Debug)]
pub struct OfferStats {
    pub total_offers: usize,
    pub active_offers: usize,
    pub matched_offers: usize,
    pub buy_offers: usize,
    pub sell_offers: usize,
    pub total_interests: usize,
}

impl std::fmt::Display for OfferStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "📊 Stats: {} total offers ({} active, {} matched) | {} buy, {} sell | {} interests",
               self.total_offers, self.active_offers, self.matched_offers,
               self.buy_offers, self.sell_offers, self.total_interests)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OfferType;

    #[tokio::test]
    async fn test_offer_manager_basic_operations() {
        let manager = OfferManager::new();
        
        // Create and add offer
        let offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Buy, 24);
        let offer_id = offer.offer_id.clone();
        
        manager.add_offer(offer).unwrap();
        
        // Check offer exists
        let retrieved = manager.get_offer(&offer_id).unwrap();
        assert!(retrieved.is_some());
        
        // Check active offers
        let active = manager.get_active_offers().unwrap();
        assert_eq!(active.len(), 1);
    }

    #[tokio::test]
    async fn test_interest_management() {
        let manager = OfferManager::new();
        
        // Add offer
        let offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Buy, 24);
        let offer_id = offer.offer_id.clone();
        manager.add_offer(offer).unwrap();
        
        // Add interest
        let interest = PrivateInterest::new(
            offer_id.clone(),
            "npub1test".to_string(),
            "I'm interested!".to_string(),
        );
        
        manager.add_interest(interest).unwrap();
        
        // Check interests
        let interests = manager.get_interests_for_offer(&offer_id).unwrap();
        assert_eq!(interests.len(), 1);
    }

    #[tokio::test]
    async fn test_match_offer() {
        let manager = OfferManager::new();
        
        // Add offer
        let offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Buy, 24);
        let offer_id = offer.offer_id.clone();
        manager.add_offer(offer).unwrap();
        
        // Match offer
        manager.match_offer(&offer_id, "npub1selected").unwrap();
        
        // Check status changed
        let updated_offer = manager.get_offer(&offer_id).unwrap().unwrap();
        assert!(matches!(updated_offer.status, OfferStatus::Matched));
        
        // Should not appear in active offers
        let active = manager.get_active_offers().unwrap();
        assert_eq!(active.len(), 0);
    }

    #[tokio::test]
    async fn test_stats() {
        let manager = OfferManager::new();
        
        // Add some offers
        let buy_offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Buy, 24);
        let sell_offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Sell, 24);
        
        manager.add_offer(buy_offer).unwrap();
        manager.add_offer(sell_offer).unwrap();
        
        let stats = manager.get_stats().unwrap();
        assert_eq!(stats.total_offers, 2);
        assert_eq!(stats.active_offers, 2);
        assert_eq!(stats.buy_offers, 1);
        assert_eq!(stats.sell_offers, 1);
    }
}
