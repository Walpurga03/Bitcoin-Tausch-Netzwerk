use anyhow::{Result, anyhow};
use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn, debug};
use tokio::time::{timeout, Duration};

/// NIP-17 Private Direct Message für Bitcoin-Tausch-Netzwerk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateMessage {
    /// 🆔 Eindeutige Message-ID
    pub message_id: String,
    /// 👤 Sender Public Key
    pub sender_pubkey: String,
    /// 👤 Empfänger Public Key  
    pub recipient_pubkey: String,
    /// 💬 Nachrichteninhalt
    pub content: String,
    /// 🔗 Optional: Referenz zu einem Angebot
    pub offer_id: Option<String>,
    /// 📧 Message Type für bessere Organisation
    pub message_type: MessageType,
    /// ⏰ Zeitstempel
    pub timestamp: u64,
    /// ✅ Gelesen-Status
    pub read: bool,
}

/// Verschiedene Nachrichtentypen für bessere UX
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// 👋 Interesse an einem Angebot
    Interest,
    /// 💬 Normale private Nachricht
    Chat,
    /// 🤝 Transaktionsverhandlung
    Negotiation,
    /// ✅ Bestätigung/Abschluss
    Confirmation,
    /// ❌ Ablehnung
    Rejection,
}

/// Conversation Thread für bessere Organisation
#[derive(Debug, Clone)]
pub struct Conversation {
    /// 🆔 Conversation ID (abgeleitet von beiden Pubkeys)
    pub conversation_id: String,
    /// 👤 Teilnehmer Pubkeys
    pub participants: Vec<String>,
    /// 💬 Alle Nachrichten in chronologischer Reihenfolge
    pub messages: Vec<PrivateMessage>,
    /// ⏰ Letztes Update
    pub last_updated: u64,
    /// 🔗 Optional: Verknüpftes Angebot
    pub linked_offer_id: Option<String>,
}

/// NIP-17 Messenger für sichere private Kommunikation
pub struct Nip17Messenger {
    client: Client,
    keys: Keys,
    /// 💾 In-Memory Storage für empfangene Nachrichten
    conversations: Arc<RwLock<HashMap<String, Conversation>>>,
    /// 📨 Ungelesene Nachrichten Counter
    unread_count: Arc<RwLock<u32>>,
}

impl Nip17Messenger {
    /// Erstelle einen neuen NIP-17 Messenger
    pub fn new(client: Client, keys: Keys) -> Self {
        info!("💬 Initializing NIP-17 Messenger...");
        
        Self {
            client,
            keys,
            conversations: Arc::new(RwLock::new(HashMap::new())),
            unread_count: Arc::new(RwLock::new(0)),
        }
    }

    /// Sende eine private Nachricht an einen anderen User
    pub async fn send_private_message(
        &self,
        recipient_pubkey: &str,
        content: &str,
        message_type: MessageType,
        offer_id: Option<String>,
    ) -> Result<EventId> {
        info!("📤 Sending private message to: {}", recipient_pubkey);

        // Parse recipient public key
        let recipient_pk = PublicKey::from_bech32(recipient_pubkey)
            .or_else(|_| PublicKey::from_hex(recipient_pubkey))?;

        // Erstelle private Nachricht
        let message = PrivateMessage {
            message_id: uuid::Uuid::new_v4().to_string(),
            sender_pubkey: self.keys.public_key().to_hex(),
            recipient_pubkey: recipient_pk.to_hex(),
            content: content.to_string(),
            offer_id,
            message_type,
            timestamp: chrono::Utc::now().timestamp() as u64,
            read: false,
        };

        // Serialisiere Message als JSON
        let message_json = serde_json::to_string(&message)?;

        // Erstelle NIP-04 Encrypted Direct Message (vorerst, bis echtes NIP-17 verfügbar)
        let dm_event = EventBuilder::encrypted_direct_msg(&self.keys, recipient_pk, message_json, None)?
            .to_event(&self.keys)?;

        // Sende Event
        match timeout(Duration::from_secs(10), self.client.send_event(dm_event)).await {
            Ok(Ok(output)) => {
                info!("✅ Private message sent successfully: {:?}", output);
                
                // Füge zur eigenen Conversation hinzu
                self.add_message_to_conversation(message).await?;
                
                // Für jetzt geben wir eine Dummy EventId zurück
                let event_id = EventId::all_zeros();
                Ok(event_id)
            }
            Ok(Err(e)) => {
                error!("❌ Failed to send private message: {}", e);
                Err(anyhow!("Failed to send message: {}", e))
            }
            Err(_) => {
                error!("⏰ Send message timeout");
                Err(anyhow!("Send timeout"))
            }
        }
    }

    /// Sende Interesse an einem Angebot
    pub async fn send_interest_message(
        &self,
        offer_creator_pubkey: &str,
        offer_id: &str,
        personal_message: Option<&str>,
    ) -> Result<EventId> {
        let content = match personal_message {
            Some(msg) => format!("🎯 Interesse an Angebot {}: {}", offer_id, msg),
            None => format!("🎯 Ich interessiere mich für Ihr Angebot {}", offer_id),
        };

        self.send_private_message(
            offer_creator_pubkey,
            &content,
            MessageType::Interest,
            Some(offer_id.to_string()),
        ).await
    }

    /// Antworte auf eine Interessensbekundung
    pub async fn respond_to_interest(
        &self,
        interested_user_pubkey: &str,
        offer_id: &str,
        accepted: bool,
        response_message: Option<&str>,
    ) -> Result<EventId> {
        let content = if accepted {
            match response_message {
                Some(msg) => format!("✅ Angebot {} akzeptiert! {}", offer_id, msg),
                None => format!("✅ Ihr Interesse an Angebot {} wurde akzeptiert! Lassen Sie uns die Details besprechen.", offer_id),
            }
        } else {
            match response_message {
                Some(msg) => format!("❌ Angebot {} abgelehnt. {}", offer_id, msg),
                None => format!("❌ Entschuldigung, Ihr Interesse an Angebot {} kann derzeit nicht berücksichtigt werden.", offer_id),
            }
        };

        let message_type = if accepted {
            MessageType::Confirmation
        } else {
            MessageType::Rejection
        };

        self.send_private_message(
            interested_user_pubkey,
            &content,
            message_type,
            Some(offer_id.to_string()),
        ).await
    }

    /// Starte Event-Listener für eingehende private Nachrichten
    pub async fn start_message_listener(&self) -> Result<()> {
        info!("👂 Starting NIP-17 message listener...");

        // Filter für private messages (NIP-04 DMs)
        let filter = Filter::new()
            .kind(Kind::EncryptedDirectMessage)
            .pubkey(self.keys.public_key());

        // Subscribe zu eingehenden Messages
        let _ = self.client.subscribe(vec![filter], None).await;

        // Handle incoming events
        let mut notifications = self.client.notifications();
        let conversations = Arc::clone(&self.conversations);
        let unread_count = Arc::clone(&self.unread_count);
        let keys = self.keys.clone();

        tokio::spawn(async move {
            while let Ok(notification) = notifications.recv().await {
                if let RelayPoolNotification::Event { event, .. } = notification {
                    if event.kind() == Kind::EncryptedDirectMessage {
                        match Self::process_incoming_message(&event, &keys, &conversations, &unread_count).await {
                            Ok(_) => debug!("✅ Processed incoming message"),
                            Err(e) => error!("❌ Failed to process message: {}", e),
                        }
                    }
                }
            }
        });

        info!("✅ Message listener started successfully");
        Ok(())
    }

    /// Verarbeite eingehende private Nachrichten
    async fn process_incoming_message(
        event: &Event,
        keys: &Keys,
        conversations: &Arc<RwLock<HashMap<String, Conversation>>>,
        unread_count: &Arc<RwLock<u32>>,
    ) -> Result<()> {
        // Entschlüssele die Nachricht (NIP-04 Style für jetzt)
        let decrypted_content = nip04::decrypt(keys.secret_key()?, &event.pubkey, &event.content)?;
        
        // Parse JSON
        let message: PrivateMessage = serde_json::from_str(&decrypted_content)?;
        
        info!("📨 Received private message from: {}", message.sender_pubkey);
        debug!("💬 Message content: {}", message.content);

        // Füge zur Conversation hinzu
        let conversation_id = Self::create_conversation_id(&message.sender_pubkey, &keys.public_key().to_hex());
        
        let mut conversations_lock = conversations.write().await;
        let conversation = conversations_lock
            .entry(conversation_id.clone())
            .or_insert_with(|| Conversation {
                conversation_id: conversation_id.clone(),
                participants: vec![message.sender_pubkey.clone(), keys.public_key().to_hex()],
                messages: Vec::new(),
                last_updated: message.timestamp,
                linked_offer_id: message.offer_id.clone(),
            });

        conversation.messages.push(message);
        conversation.last_updated = chrono::Utc::now().timestamp() as u64;
        
        // Increment unread counter
        let mut unread = unread_count.write().await;
        *unread += 1;

        info!("✅ Message added to conversation: {}", conversation_id);
        Ok(())
    }

    /// Füge eine Nachricht zur entsprechenden Conversation hinzu
    async fn add_message_to_conversation(&self, message: PrivateMessage) -> Result<()> {
        let conversation_id = Self::create_conversation_id(&message.sender_pubkey, &message.recipient_pubkey);
        
        let mut conversations = self.conversations.write().await;
        let conversation = conversations
            .entry(conversation_id.clone())
            .or_insert_with(|| Conversation {
                conversation_id: conversation_id.clone(),
                participants: vec![message.sender_pubkey.clone(), message.recipient_pubkey.clone()],
                messages: Vec::new(),
                last_updated: message.timestamp,
                linked_offer_id: message.offer_id.clone(),
            });

        conversation.messages.push(message);
        conversation.last_updated = chrono::Utc::now().timestamp() as u64;
        
        Ok(())
    }

    /// Erstelle eindeutige Conversation ID aus zwei Pubkeys
    fn create_conversation_id(pubkey1: &str, pubkey2: &str) -> String {
        let mut keys = vec![pubkey1, pubkey2];
        keys.sort();
        format!("conv_{}_{}", &keys[0][..8], &keys[1][..8])
    }

    /// Hole alle Conversations
    pub async fn get_all_conversations(&self) -> HashMap<String, Conversation> {
        self.conversations.read().await.clone()
    }

    /// Hole eine spezifische Conversation
    pub async fn get_conversation(&self, conversation_id: &str) -> Option<Conversation> {
        self.conversations.read().await.get(conversation_id).cloned()
    }

    /// Markiere alle Nachrichten in einer Conversation als gelesen
    pub async fn mark_conversation_as_read(&self, conversation_id: &str) -> Result<()> {
        let mut conversations = self.conversations.write().await;
        
        if let Some(conversation) = conversations.get_mut(conversation_id) {
            let mut unread_count = 0u32;
            for message in &mut conversation.messages {
                if !message.read {
                    message.read = true;
                    unread_count += 1;
                }
            }
            
            // Update global unread counter
            let mut global_unread = self.unread_count.write().await;
            *global_unread = global_unread.saturating_sub(unread_count);
            
            info!("✅ Marked {} messages as read in conversation: {}", unread_count, conversation_id);
        }
        
        Ok(())
    }

    /// Hole Anzahl ungelesener Nachrichten
    pub async fn get_unread_count(&self) -> u32 {
        *self.unread_count.read().await
    }

    /// Hole alle Conversations für ein bestimmtes Angebot
    pub async fn get_conversations_for_offer(&self, offer_id: &str) -> Vec<Conversation> {
        self.conversations
            .read()
            .await
            .values()
            .filter(|conv| conv.linked_offer_id.as_ref() == Some(&offer_id.to_string()))
            .cloned()
            .collect()
    }

    /// Lösche eine Conversation (nur lokal)
    pub async fn delete_conversation(&self, conversation_id: &str) -> Result<()> {
        let mut conversations = self.conversations.write().await;
        
        if let Some(conversation) = conversations.remove(conversation_id) {
            // Update unread counter
            let unread_in_conversation = conversation.messages.iter()
                .filter(|msg| !msg.read)
                .count() as u32;
                
            let mut global_unread = self.unread_count.write().await;
            *global_unread = global_unread.saturating_sub(unread_in_conversation);
            
            info!("🗑️ Deleted conversation: {}", conversation_id);
        }
        
        Ok(())
    }

    /// Hole die eigene Public Key
    pub fn get_own_pubkey(&self) -> String {
        self.keys.public_key().to_hex()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_conversation_id_creation() {
        let pubkey1 = "npub1alice123";
        let pubkey2 = "npub1bob456";
        
        let id1 = Nip17Messenger::create_conversation_id(pubkey1, pubkey2);
        let id2 = Nip17Messenger::create_conversation_id(pubkey2, pubkey1);
        
        // Should be the same regardless of order
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_message_creation() {
        let message = PrivateMessage {
            message_id: "test-id".to_string(),
            sender_pubkey: "sender123".to_string(),
            recipient_pubkey: "recipient456".to_string(),
            content: "Hello Bitcoin trader!".to_string(),
            offer_id: Some("offer-123".to_string()),
            message_type: MessageType::Interest,
            timestamp: 1692185000,
            read: false,
        };

        assert_eq!(message.content, "Hello Bitcoin trader!");
        assert_eq!(message.offer_id, Some("offer-123".to_string()));
        assert!(!message.read);
    }
}