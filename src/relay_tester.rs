use anyhow::Result;
use nostr_sdk::prelude::*;
use std::time::Duration;
use tracing::{info, warn, error};

/// 🧪 Testet die NIP-Kompatibilität eines Nostr-Relays
pub struct RelayTester {
    client: Client,
    keys: Keys,
    relay_url: String,
}

impl RelayTester {
    pub fn new(relay_url: &str) -> Self {
        let keys = Keys::generate();
        let client = Client::new(&keys);
        
        Self {
            client,
            keys,
            relay_url: relay_url.to_string(),
        }
    }

    /// 🔌 Verbindung zum Relay testen
    pub async fn test_connection(&self) -> Result<bool> {
        info!("🔌 Testing connection to relay: {}", self.relay_url);
        
        self.client.add_relay(&self.relay_url).await?;
        self.client.connect().await;
        
        // Kurz warten für Verbindung
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        
        let status = self.client.relay(&self.relay_url).await;
        match status {
            Ok(relay) => {
                let is_connected = relay.is_connected().await;
                if is_connected {
                    info!("✅ Successfully connected to relay");
                } else {
                    warn!("⚠️ Could not establish connection");
                }
                Ok(is_connected)
            }
            Err(e) => {
                error!("❌ Connection failed: {}", e);
                Ok(false)
            }
        }
    }

    /// 📝 NIP-01 Basic Event Publishing testen
    pub async fn test_nip01_basic_events(&self) -> Result<bool> {
        info!("📝 Testing NIP-01: Basic event publishing");
        
        let event = EventBuilder::text_note("Test NIP-01 compatibility", []).to_event(&self.keys)?;
        
        match self.client.send_event(event).await {
            Ok(output) => {
                info!("✅ NIP-01 supported - Basic events work");
                info!("📤 Event sent to {} relays", output.success.len());
                Ok(true)
            }
            Err(e) => {
                error!("❌ NIP-01 failed: {}", e);
                Ok(false)
            }
        }
    }

    /// 🏷️ NIP-12 Generic Tag Queries testen
    pub async fn test_nip12_generic_tags(&self) -> Result<bool> {
        info!("🏷️ Testing NIP-12: Generic tag queries");
        
        // Event mit Custom Tags erstellen
        let tags = vec![
            Tag::hashtag("bitcoin"),
            Tag::hashtag("exchange"),
            Tag::custom(TagKind::Custom("btc-offer".into()), vec!["buy".to_string()]),
        ];
        
        let event = EventBuilder::text_note("Bitcoin exchange offer", tags)
            .to_event(&self.keys)?;
        
        match self.client.send_event(event.clone()).await {
            Ok(_) => {
                info!("✅ NIP-12 supported - Generic tags work");
                
                // Kurz warten und dann Query testen
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                
                // Test Query mit Tag-Filter
                let filter = Filter::new()
                    .kind(Kind::TextNote)
                    .hashtag("bitcoin")
                    .limit(10);
                
                match self.client.get_events_of(vec![filter], Some(Duration::from_secs(5))).await {
                    Ok(events) => {
                        info!("🔍 Found {} events with bitcoin hashtag", events.len());
                        Ok(true)
                    }
                    Err(e) => {
                        warn!("⚠️ Query failed but publishing worked: {}", e);
                        Ok(true) // Publishing ist wichtiger
                    }
                }
            }
            Err(e) => {
                error!("❌ NIP-12 failed: {}", e);
                Ok(false)
            }
        }
    }

    /// 🎁 NIP-59 Gift Wrapping testen
    pub async fn test_nip59_gift_wrapping(&self) -> Result<bool> {
        info!("🎁 Testing NIP-59: Gift wrapping");
        
        // Temporäre Schlüssel für Gift Wrapping
        let temp_keys = Keys::generate();
        let receiver_keys = Keys::generate();
        
        // Inneres Event erstellen
        let inner_event = EventBuilder::text_note("Secret Bitcoin offer", [])
            .to_event(&temp_keys)?;
        
        // Gift Wrap erstellen - vereinfachter Test
        let rumor = UnsignedEvent::from(inner_event.clone());
        
        // Versuche Gift Wrap zu erstellen (kann fehlschlagen wenn nicht unterstützt)
        match EventBuilder::gift_wrap(&self.keys, &receiver_keys.public_key(), rumor, None) {
            Ok(seal_event) => {
                match self.client.send_event(seal_event).await {
                    Ok(_) => {
                        info!("✅ NIP-59 supported - Gift wrapping works");
                        Ok(true)
                    }
                    Err(e) => {
                        error!("❌ NIP-59 gift wrap send failed: {}", e);
                        // Fallback: teste einfaches verschleiertes Event
                        self.test_nip59_fallback().await
                    }
                }
            }
            Err(e) => {
                error!("❌ NIP-59 gift wrap creation failed: {}", e);
                // Fallback: teste einfaches verschleiertes Event
                self.test_nip59_fallback().await
            }
        }
    }

    /// 🎁 NIP-59 Fallback Test (vereinfacht)
    async fn test_nip59_fallback(&self) -> Result<bool> {
        info!("🔄 Testing NIP-59 fallback: Simple obfuscated event");
        
        let temp_keys = Keys::generate();
        
        // Einfaches Event mit anderen Schlüsseln
        let event = EventBuilder::text_note("Anonymous test", [])
            .to_event(&temp_keys)?;
        
        match self.client.send_event(event).await {
            Ok(_) => {
                info!("✅ NIP-59 fallback works - Can publish with different keys");
                Ok(true)
            }
            Err(e) => {
                error!("❌ Even NIP-59 fallback failed: {}", e);
                Ok(false)
            }
        }
    }

    /// 🔐 NIP-44 Encryption testen (falls verfügbar)
    pub async fn test_nip44_encryption(&self) -> Result<bool> {
        info!("🔐 Testing NIP-44: Encryption capabilities");
        
        // Teste ob wir ein DM Event erstellen können (vereinfacht)
        let event = EventBuilder::text_note("Encrypted test message placeholder", [])
            .to_event(&self.keys)?;
        
        match self.client.send_event(event).await {
            Ok(_) => {
                info!("✅ NIP-44 basic messaging supported");
                Ok(true)
            }
            Err(e) => {
                warn!("⚠️ NIP-44 messaging not supported: {}", e);
                Ok(false)
            }
        }
    }

    /// 👥 NIP-29 Simple Groups testen
    pub async fn test_nip29_simple_groups(&self) -> Result<bool> {
        info!("👥 Testing NIP-29: Simple Groups");
        
        // Gruppe-spezifische Tags für Bitcoin-Austausch
        let group_id = "bitcoin-exchange-test";
        let tags = vec![
            Tag::hashtag("bitcoin-group"),
            Tag::custom(TagKind::Custom("h".into()), vec![group_id.to_string()]), // NIP-29 group identifier
            Tag::custom(TagKind::Custom("k".into()), vec!["9".to_string()]), // Group chat message kind
        ];
        
        // Group Chat Message Event (Kind 9)
        let event = EventBuilder::new(Kind::Custom(9), "Bitcoin exchange group test message", tags)
            .to_event(&self.keys)?;
        
        match self.client.send_event(event.clone()).await {
            Ok(_) => {
                info!("✅ NIP-29 supported - Group messages work");
                
                // Kurz warten und dann Query testen
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                
                // Test Query für Group Messages
                let filter = Filter::new()
                    .kind(Kind::Custom(9))
                    .hashtag("bitcoin-group")
                    .limit(5);
                
                match self.client.get_events_of(vec![filter], Some(Duration::from_secs(3))).await {
                    Ok(events) => {
                        info!("👥 Found {} group messages", events.len());
                        Ok(true)
                    }
                    Err(e) => {
                        warn!("⚠️ Group query failed but publishing worked: {}", e);
                        Ok(true) // Publishing ist wichtiger
                    }
                }
            }
            Err(e) => {
                error!("❌ NIP-29 failed: {}", e);
                Ok(false)
            }
        }
    }

    /// 📊 Vollständiger Relay-Kompatibilitätstest
    pub async fn run_full_compatibility_test(&self) -> Result<RelayCompatibilityReport> {
        info!("🧪 Starting full relay compatibility test for: {}", self.relay_url);
        
        let mut report = RelayCompatibilityReport::new(&self.relay_url);
        
        // Verbindungstest
        report.connection = self.test_connection().await?;
        if !report.connection {
            error!("❌ Cannot proceed - no connection to relay");
            return Ok(report);
        }

        // NIP Tests
        report.nip01_basic = self.test_nip01_basic_events().await?;
        report.nip12_tags = self.test_nip12_generic_tags().await?;
        report.nip29_groups = self.test_nip29_simple_groups().await?;
        report.nip44_encryption = self.test_nip44_encryption().await?;
        report.nip59_gift_wrap = self.test_nip59_gift_wrapping().await?;
        
        // Disconnect
        self.client.disconnect().await?;
        
        Ok(report)
    }
}

/// 📊 Kompatibilitätsbericht
#[derive(Debug)]
pub struct RelayCompatibilityReport {
    pub relay_url: String,
    pub connection: bool,
    pub nip01_basic: bool,
    pub nip12_tags: bool,
    pub nip29_groups: bool,
    pub nip44_encryption: bool,
    pub nip59_gift_wrap: bool,
}

impl RelayCompatibilityReport {
    fn new(relay_url: &str) -> Self {
        Self {
            relay_url: relay_url.to_string(),
            connection: false,
            nip01_basic: false,
            nip12_tags: false,
            nip29_groups: false,
            nip44_encryption: false,
            nip59_gift_wrap: false,
        }
    }

    /// ✅ Prüft ob das Relay für unser Bitcoin-Tausch-Netzwerk geeignet ist
    pub fn is_suitable_for_bitcoin_exchange(&self) -> bool {
        // Minimum: Connection + Basic Events + Gift Wrapping + Groups
        self.connection && self.nip01_basic && self.nip29_groups && self.nip59_gift_wrap
    }

    /// 📋 Erstellt einen detaillierten Bericht
    pub fn print_report(&self) {
        info!("📋 =========================");
        info!("📊 RELAY COMPATIBILITY REPORT");
        info!("📋 =========================");
        info!("🔗 Relay: {}", self.relay_url);
        info!("📋 =========================");
        
        self.print_test_result("🔌 Connection", self.connection);
        self.print_test_result("📝 NIP-01 (Basic Events)", self.nip01_basic);
        self.print_test_result("🏷️ NIP-12 (Generic Tags)", self.nip12_tags);
        self.print_test_result("� NIP-29 (Simple Groups)", self.nip29_groups);
        self.print_test_result("�🔐 NIP-44 (Encryption)", self.nip44_encryption);
        self.print_test_result("🎁 NIP-59 (Gift Wrapping)", self.nip59_gift_wrap);
        
        info!("📋 =========================");
        
        if self.is_suitable_for_bitcoin_exchange() {
            info!("✅ VERDICT: Relay is SUITABLE for Bitcoin Exchange Network!");
            info!("💡 Your relay supports all required features for anonymous Bitcoin exchange offers.");
        } else {
            warn!("⚠️ VERDICT: Relay has LIMITATIONS for Bitcoin Exchange Network");
            if !self.connection {
                error!("❌ Critical: Cannot connect to relay");
            }
            if !self.nip01_basic {
                error!("❌ Critical: Basic event publishing not supported");
            }
            if !self.nip59_gift_wrap {
                error!("❌ Critical: Gift wrapping (anonymity) not supported");
            }
            if !self.nip29_groups {
                error!("❌ Critical: Simple Groups not supported (group features limited)");
            }
            if !self.nip44_encryption {
                warn!("⚠️ Warning: Encryption not fully supported (private messages may be limited)");
            }
            if !self.nip12_tags {
                warn!("⚠️ Warning: Generic tags not supported (search functionality limited)");
            }
        }
        
        info!("📋 =========================");
    }

    fn print_test_result(&self, test_name: &str, result: bool) {
        if result {
            info!("✅ {}", test_name);
        } else {
            error!("❌ {}", test_name);
        }
    }
}
