use serde::{Deserialize, Serialize};

/// Anonymes Tauschangebot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnonymousOffer {
    /// 🆔 UUID für anonyme Identifikation
    pub offer_id: String,
    /// 👤 Temporäre Pseudo-Identität
    pub pseudo_id: String,
    /// 💰 EUR-Betrag
    pub amount_eur: f64,
    /// ₿ Bitcoin-Betrag
    pub amount_btc: f64,
    /// 📈 Buy/Sell
    pub offer_type: OfferType,
    /// 📊 Active, Matched, Closed
    pub status: OfferStatus,
    /// ⏰ Unix timestamp
    pub created_at: u64,
    /// ⏰ Ablaufzeit
    pub expires_at: u64,
    /// 👥 Liste der Interessenten-IDs
    pub interested_users: Vec<String>,
}

/// Private Interessensbekundung
#[derive(Debug, Clone)]
pub struct PrivateInterest {
    /// 🔗 Referenz zum Angebot
    pub offer_id: String,
    /// 🔑 Public Key des Interessenten
    pub interested_user_pubkey: String,
    /// 💬 Optionale Nachricht
    pub message: String,
    /// ⏰ Zeitstempel
    pub timestamp: u64,
}

/// Angebots-Typ für Typsicherheit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OfferType {
    /// 🟢 Bitcoin kaufen (User möchte BTC für EUR)
    Buy,
    /// 🔴 Bitcoin verkaufen (User möchte EUR für BTC)
    Sell,
}

/// Angebots-Status Lifecycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OfferStatus {
    /// 🟢 Aktiv und sichtbar
    Active,
    /// 🤝 Partner gefunden
    Matched,
    /// ✅ Erfolgreich abgeschlossen
    Closed,
    /// ⏰ Abgelaufen
    Expired,
}

impl AnonymousOffer {
    /// Erstelle ein neues anonymes Angebot
    pub fn new(
        amount_eur: f64,
        amount_btc: f64,
        offer_type: OfferType,
        expires_in_hours: u64,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            offer_id: uuid::Uuid::new_v4().to_string(),
            pseudo_id: format!("anon_{}", uuid::Uuid::new_v4().simple()),
            amount_eur,
            amount_btc,
            offer_type,
            status: OfferStatus::Active,
            created_at: now,
            expires_at: now + (expires_in_hours * 3600),
            interested_users: Vec::new(),
        }
    }

    /// Füge einen Interessenten hinzu
    pub fn add_interested_user(&mut self, user_pubkey: String) {
        if !self.interested_users.contains(&user_pubkey) {
            self.interested_users.push(user_pubkey);
        }
    }

    /// Prüfe ob das Angebot abgelaufen ist
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now > self.expires_at
    }

    /// Berechne den Bitcoin-Preis in EUR
    pub fn btc_price_eur(&self) -> f64 {
        if self.amount_btc > 0.0 {
            self.amount_eur / self.amount_btc
        } else {
            0.0
        }
    }
}

impl PrivateInterest {
    /// Erstelle eine neue Interessensbekundung
    pub fn new(offer_id: String, user_pubkey: String, message: String) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            offer_id,
            interested_user_pubkey: user_pubkey,
            message,
            timestamp: now,
        }
    }
}

impl std::fmt::Display for OfferType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OfferType::Buy => write!(f, "BUY"),
            OfferType::Sell => write!(f, "SELL"),
        }
    }
}

impl std::fmt::Display for OfferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OfferStatus::Active => write!(f, "ACTIVE"),
            OfferStatus::Matched => write!(f, "MATCHED"),
            OfferStatus::Closed => write!(f, "CLOSED"),
            OfferStatus::Expired => write!(f, "EXPIRED"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_anonymous_offer() {
        let offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Buy, 24);
        
        assert_eq!(offer.amount_eur, 1000.0);
        assert_eq!(offer.amount_btc, 0.025);
        assert!(matches!(offer.offer_type, OfferType::Buy));
        assert!(matches!(offer.status, OfferStatus::Active));
        assert!(offer.offer_id.len() > 0);
        assert!(offer.pseudo_id.starts_with("anon_"));
    }

    #[test]
    fn test_btc_price_calculation() {
        let offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Buy, 24);
        assert_eq!(offer.btc_price_eur(), 40000.0); // 1000 / 0.025 = 40000
    }

    #[test]
    fn test_add_interested_user() {
        let mut offer = AnonymousOffer::new(1000.0, 0.025, OfferType::Buy, 24);
        let user_pubkey = "npub1234...".to_string();
        
        offer.add_interested_user(user_pubkey.clone());
        assert_eq!(offer.interested_users.len(), 1);
        
        // Adding same user again should not duplicate
        offer.add_interested_user(user_pubkey.clone());
        assert_eq!(offer.interested_users.len(), 1);
    }

    #[test]
    fn test_private_interest() {
        let interest = PrivateInterest::new(
            "offer123".to_string(),
            "npub1234...".to_string(),
            "I'm interested in this offer".to_string(),
        );
        
        assert_eq!(interest.offer_id, "offer123");
        assert!(interest.timestamp > 0);
    }
}
