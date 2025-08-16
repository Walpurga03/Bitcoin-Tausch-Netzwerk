use anyhow::{Result, anyhow};
use nostr_sdk::prelude::*;
use std::time::Duration;
use tracing::{info, error, warn, debug};
use tokio::time::timeout;

use crate::types::AnonymousOffer;
use crate::anonymous_publisher::AnonymousOfferPublisher;

/// Nostr Client für das Bitcoin-Tausch-Netzwerk
pub struct NostrClient {
    client: Client,
    keys: Keys,
    relay_url: String,
}

impl NostrClient {
    /// Erstelle einen neuen Nostr Client
    pub async fn new(relay_url: String) -> Result<Self> {
        info!("🔌 Initializing Nostr client for relay: {}", relay_url);
        
        // Generiere neue Schlüssel für diese Session
        let keys = Keys::generate();
        let public_key = keys.public_key();
        
        info!("🔑 Generated new keypair - Public Key: {}", public_key.to_bech32()?);
        
        // Erstelle Client
        let client = Client::new(&keys);
        
        Ok(Self {
            client,
            keys,
            relay_url,
        })
    }

    /// Verbinde zum Relay
    pub async fn connect(&self) -> Result<()> {
        info!("🔗 Connecting to relay: {}", self.relay_url);
        
        // Füge Relay hinzu
        self.client.add_relay(&self.relay_url).await?;
        
        // Verbinde mit Timeout
        match timeout(Duration::from_secs(10), self.client.connect()).await {
            Ok(_) => {
                info!("✅ Successfully connected to relay: {}", self.relay_url);
                Ok(())
            }
            Err(_) => {
                error!("⏰ Connection timeout after 10 seconds");
                Err(anyhow!("Connection timeout"))
            }
        }
    }

    /// Teste die Verbindung durch Senden eines Test-Events
    pub async fn test_connection(&self) -> Result<()> {
        info!("🧪 Testing connection with a test note...");
        
        let test_content = format!(
            "🔐 Bitcoin-Tausch-Netzwerk Test - Connection established at {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        );
        
        let test_event = EventBuilder::text_note(test_content, [])
            .to_event(&self.keys)?;
        
        match timeout(Duration::from_secs(5), self.client.send_event(test_event)).await {
            Ok(Ok(output)) => {
                info!("✅ Test event sent successfully: {:?}", output);
                Ok(())
            }
            Ok(Err(e)) => {
                error!("❌ Failed to send test event: {}", e);
                Err(anyhow!("Test event failed: {}", e))
            }
            Err(_) => {
                error!("⏰ Test event timeout");
                Err(anyhow!("Test event timeout"))
            }
        }
    }

    /// Hole die Public Key für diese Session
    pub fn get_public_key(&self) -> PublicKey {
        self.keys.public_key()
    }

    /// Hole die Public Key als npub-String
    pub fn get_npub(&self) -> Result<String> {
        Ok(self.keys.public_key().to_bech32()?)
    }

    /// Sende ein anonymes Angebot ins Netzwerk mit NIP-59 Gift Wrapping
    pub async fn publish_anonymous_offer(&self, offer: &AnonymousOffer) -> Result<EventId> {
        info!("📝 Publishing anonymous offer to relay...");
        
        let publisher = AnonymousOfferPublisher::new(self.client.clone(), self.keys.clone());
        publisher.publish_anonymous_offer(offer).await
    }

    /// Sende eine private Interessensbekundung (später implementiert)  
    pub async fn send_private_interest(&self, _target_pubkey: &str, _message: &str) -> Result<EventId> {
        // TODO: Implementiere NIP-17 private messages
        warn!("🚧 send_private_interest not yet implemented");
        Err(anyhow!("Not implemented yet"))
    }

    /// Lausche auf Events (später implementiert)
    pub async fn start_listening(&self) -> Result<()> {
        // TODO: Implementiere Event-Listener für Angebote und Nachrichten
        warn!("🚧 start_listening not yet implemented");
        Ok(())
    }

    /// Trenne die Verbindung
    pub async fn disconnect(&self) -> Result<()> {
        info!("🔌 Disconnecting from relay...");
        self.client.disconnect().await?;
        info!("✅ Disconnected successfully");
        Ok(())
    }

    /// Hole Relay-Status-Informationen
    pub async fn get_relay_status(&self) -> Result<RelayStatus> {
        let relays = self.client.relays().await;
        
        if let Some((url, relay)) = relays.iter().next() {
            let is_connected = relay.is_connected().await;
            
            Ok(RelayStatus {
                url: url.to_string(),
                is_connected,
                events_sent: 0, // Stats not easily accessible in current nostr-sdk version
                events_received: 0,
            })
        } else {
            Err(anyhow!("No relays configured"))
        }
    }
}

/// Status-Informationen über das Relay
#[derive(Debug)]
pub struct RelayStatus {
    pub url: String,
    pub is_connected: bool,
    pub events_sent: usize,
    pub events_received: usize,
}

impl std::fmt::Display for RelayStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "📡 Relay: {} | Connected: {} | Sent: {} | Received: {}", 
               self.url, self.is_connected, self.events_sent, self.events_received)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nostr_client_creation() {
        let client = NostrClient::new("wss://test-relay.example".to_string()).await;
        assert!(client.is_ok());
        
        let client = client.unwrap();
        assert!(client.get_npub().is_ok());
        
        let npub = client.get_npub().unwrap();
        assert!(npub.starts_with("npub1"));
    }

    #[tokio::test]
    async fn test_key_generation() {
        let client = NostrClient::new("wss://test-relay.example".to_string()).await.unwrap();
        
        let pubkey1 = client.get_public_key();
        
        // Create another client and verify different keys
        let client2 = NostrClient::new("wss://test-relay.example".to_string()).await.unwrap();
        let pubkey2 = client2.get_public_key();
        
        assert_ne!(pubkey1, pubkey2, "Each client should have unique keys");
    }
}
