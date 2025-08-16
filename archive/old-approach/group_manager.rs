// =====================================
// 🏢 NIP-29 GROUPS MANAGER 
// Private Bitcoin-Tausch-Gruppe mit Invite-Links
// =====================================

use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateGroup {
    pub group_id: String,
    pub name: String,
    pub description: String,
    pub admin_pubkey: PublicKey,
    pub invite_code: String,
    pub created_at: u64,
    pub member_count: usize,
    pub is_trading_enabled: bool, // Erst nach Kennenlernphase aktiviert
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    pub pubkey: PublicKey,
    pub nickname: String,
    pub joined_at: u64,
    pub is_verified: bool,     // Vom Admin bestätigt
    pub reputation_score: u32, // Community-Bewertung
    pub introduction_sent: bool,
    pub can_create_offers: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupInvite {
    pub invite_code: String,
    pub group_id: String,
    pub created_by: PublicKey,
    pub expires_at: Option<u64>,
    pub max_uses: Option<u32>,
    pub current_uses: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMessage {
    pub id: String,
    pub group_id: String,
    pub sender: PublicKey,
    pub content: String,
    pub message_type: GroupMessageType,
    pub created_at: u64,
    pub reply_to: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupMessageType {
    Introduction,     // Neue Mitglieder stellen sich vor
    General,         // Allgemeine Diskussion
    Announcement,    // Admin-Ankündigungen
    TradingRequest,  // Bitcoin-Handel (nur für verifizierte)
    Verification,    // Admin verifiziert Mitglieder
}

pub struct GroupManager {
    groups: HashMap<String, PrivateGroup>,
    members: HashMap<String, Vec<GroupMember>>, // group_id -> members
    invites: HashMap<String, GroupInvite>,      // invite_code -> invite
    messages: HashMap<String, Vec<GroupMessage>>, // group_id -> messages
    nostr_client: Client,
}

impl GroupManager {
    pub fn new(nostr_client: Client) -> Self {
        Self {
            groups: HashMap::new(),
            members: HashMap::new(),
            invites: HashMap::new(),
            messages: HashMap::new(),
            nostr_client,
        }
    }

    // 🏗️ GRUPPE ERSTELLEN (nur für Admins)
    pub async fn create_private_group(
        &mut self,
        admin_keys: &Keys,
        name: String,
        description: String,
    ) -> Result<(String, String)> {
        let group_id = Uuid::new_v4().to_string();
        let invite_code = self.generate_invite_code();
        
        let group = PrivateGroup {
            group_id: group_id.clone(),
            name: name.clone(),
            description: description.clone(),
            admin_pubkey: admin_keys.public_key(),
            invite_code: invite_code.clone(),
            created_at: Timestamp::now().as_u64(),
            member_count: 1,
            is_trading_enabled: false, // Erst später aktivieren
        };

        // Admin als erstes Mitglied hinzufügen
        let admin_member = GroupMember {
            pubkey: admin_keys.public_key(),
            nickname: "Admin".to_string(),
            joined_at: Timestamp::now().as_u64(),
            is_verified: true,
            reputation_score: 100,
            introduction_sent: true,
            can_create_offers: true,
        };

        self.groups.insert(group_id.clone(), group);
        self.members.insert(group_id.clone(), vec![admin_member]);
        
        // NIP-29 Group Creation Event
        self.publish_group_creation_event(&admin_keys, &group_id, &name).await?;
        
        Ok((group_id, invite_code))
    }

    // 🔗 INVITE-LINK GENERIEREN
    fn generate_invite_code(&self) -> String {
        format!("btn_{}", Uuid::new_v4().to_string()[..8].to_uppercase())
    }

    // 📨 INVITE-LINK ERSTELLEN
    pub fn create_invite_link(&mut self, group_id: &str, created_by: PublicKey) -> Option<String> {
        if let Some(_group) = self.groups.get(group_id) {
            let invite_code = self.generate_invite_code();
            let invite = GroupInvite {
                invite_code: invite_code.clone(),
                group_id: group_id.to_string(),
                created_by,
                expires_at: Some(Timestamp::now().as_u64() + 7 * 24 * 60 * 60), // 7 Tage
                max_uses: Some(50), // Maximal 50 Personen
                current_uses: 0,
            };
            
            self.invites.insert(invite_code.clone(), invite);
            
            // Vollständiger Invite-Link
            Some(format!(
                "https://walpurga03.github.io/Bitcoin-Tausch-Netzwerk/#join/{}",
                invite_code
            ))
        } else {
            None
        }
    }

    // 🚪 GRUPPE BEITRETEN (mit Invite-Code)
    pub async fn join_group_with_invite(
        &mut self,
        user_keys: &Keys,
        invite_code: &str,
        nickname: String,
    ) -> Result<String> {
        let invite = self.invites.get_mut(invite_code)
            .ok_or_else(|| Error::msg("Ungültiger Invite-Code"))?;

        // Invite validieren
        if let Some(expires_at) = invite.expires_at {
            if Timestamp::now().as_u64() > expires_at {
                return Err(Error::msg("Invite-Code ist abgelaufen"));
            }
        }

        if let Some(max_uses) = invite.max_uses {
            if invite.current_uses >= max_uses {
                return Err(Error::msg("Invite-Code wurde zu oft verwendet"));
            }
        }

        let group_id = invite.group_id.clone();
        let mut group = self.groups.get_mut(&group_id)
            .ok_or_else(|| Error::msg("Gruppe nicht gefunden"))?;

        // Neues Mitglied erstellen
        let new_member = GroupMember {
            pubkey: user_keys.public_key(),
            nickname: nickname.clone(),
            joined_at: Timestamp::now().as_u64(),
            is_verified: false, // Muss vom Admin bestätigt werden
            reputation_score: 0,
            introduction_sent: false,
            can_create_offers: false, // Erst nach Verifikation
        };

        // Zu Gruppe hinzufügen
        if let Some(members) = self.members.get_mut(&group_id) {
            members.push(new_member);
        } else {
            self.members.insert(group_id.clone(), vec![new_member]);
        }

        group.member_count += 1;
        invite.current_uses += 1;

        // NIP-29 Join Event publizieren
        self.publish_group_join_event(user_keys, &group_id, &nickname).await?;

        // Willkommensnachricht senden
        self.send_welcome_message(&group_id, &nickname).await?;

        Ok(group_id)
    }

    // 👋 VORSTELLUNG SENDEN (Kennenlern-Phase)
    pub async fn send_introduction(
        &mut self,
        user_keys: &Keys,
        group_id: &str,
        introduction: String,
    ) -> Result<()> {
        // Prüfen ob Mitglied in Gruppe
        let members = self.members.get_mut(group_id)
            .ok_or_else(|| Error::msg("Gruppe nicht gefunden"))?;

        let member = members.iter_mut()
            .find(|m| m.pubkey == user_keys.public_key())
            .ok_or_else(|| Error::msg("Du bist nicht Mitglied dieser Gruppe"))?;

        if member.introduction_sent {
            return Err(Error::msg("Du hast dich bereits vorgestellt"));
        }

        // Vorstellungs-Nachricht senden
        let message = GroupMessage {
            id: Uuid::new_v4().to_string(),
            group_id: group_id.to_string(),
            sender: user_keys.public_key(),
            content: format!("👋 Hallo! Ich bin {}.\n\n{}", member.nickname, introduction),
            message_type: GroupMessageType::Introduction,
            created_at: Timestamp::now().as_u64(),
            reply_to: None,
        };

        self.add_group_message(group_id, message.clone());
        self.publish_group_message_event(user_keys, message).await?;

        member.introduction_sent = true;
        
        Ok(())
    }

    // ✅ MITGLIED VERIFIZIEREN (nur Admin)
    pub async fn verify_member(
        &mut self,
        admin_keys: &Keys,
        group_id: &str,
        member_pubkey: PublicKey,
    ) -> Result<()> {
        let group = self.groups.get(group_id)
            .ok_or_else(|| Error::msg("Gruppe nicht gefunden"))?;

        // Prüfen ob Admin
        if group.admin_pubkey != admin_keys.public_key() {
            return Err(Error::msg("Nur der Admin kann Mitglieder verifizieren"));
        }

        let members = self.members.get_mut(group_id)
            .ok_or_else(|| Error::msg("Mitgliederliste nicht gefunden"))?;

        let member = members.iter_mut()
            .find(|m| m.pubkey == member_pubkey)
            .ok_or_else(|| Error::msg("Mitglied nicht gefunden"))?;

        member.is_verified = true;
        member.can_create_offers = true;
        member.reputation_score = 50; // Basis-Reputation

        // Verifikations-Nachricht senden
        let message = GroupMessage {
            id: Uuid::new_v4().to_string(),
            group_id: group_id.to_string(),
            sender: admin_keys.public_key(),
            content: format!("✅ {} wurde verifiziert und kann jetzt Bitcoin-Angebote erstellen!", member.nickname),
            message_type: GroupMessageType::Verification,
            created_at: Timestamp::now().as_u64(),
            reply_to: None,
        };

        self.add_group_message(group_id, message.clone());
        self.publish_group_message_event(admin_keys, message).await?;

        Ok(())
    }

    // 🚀 BITCOIN-HANDEL AKTIVIEREN (nach Kennenlernphase)
    pub async fn enable_trading(
        &mut self,
        admin_keys: &Keys,
        group_id: &str,
    ) -> Result<()> {
        let group = self.groups.get_mut(group_id)
            .ok_or_else(|| Error::msg("Gruppe nicht gefunden"))?;

        if group.admin_pubkey != admin_keys.public_key() {
            return Err(Error::msg("Nur der Admin kann den Handel aktivieren"));
        }

        group.is_trading_enabled = true;

        // Ankündigung senden
        let message = GroupMessage {
            id: Uuid::new_v4().to_string(),
            group_id: group_id.to_string(),
            sender: admin_keys.public_key(),
            content: "🚀 Bitcoin-Handel ist jetzt aktiviert! Verifizierte Mitglieder können Angebote erstellen.".to_string(),
            message_type: GroupMessageType::Announcement,
            created_at: Timestamp::now().as_u64(),
            reply_to: None,
        };

        self.add_group_message(group_id, message.clone());
        self.publish_group_message_event(admin_keys, message).await?;

        Ok(())
    }

    // 📊 GRUPPEN-STATISTIKEN
    pub fn get_group_stats(&self, group_id: &str) -> Option<GroupStats> {
        let group = self.groups.get(group_id)?;
        let members = self.members.get(group_id)?;
        
        let verified_count = members.iter().filter(|m| m.is_verified).count();
        let introduced_count = members.iter().filter(|m| m.introduction_sent).count();
        
        Some(GroupStats {
            total_members: members.len(),
            verified_members: verified_count,
            introduced_members: introduced_count,
            trading_enabled: group.is_trading_enabled,
            created_at: group.created_at,
        })
    }

    // 🔄 Helper Methods
    fn add_group_message(&mut self, group_id: &str, message: GroupMessage) {
        if let Some(messages) = self.messages.get_mut(group_id) {
            messages.push(message);
        } else {
            self.messages.insert(group_id.to_string(), vec![message]);
        }
    }

    async fn send_welcome_message(&mut self, group_id: &str, nickname: &str) -> Result<()> {
        let message = GroupMessage {
            id: Uuid::new_v4().to_string(),
            group_id: group_id.to_string(),
            sender: PublicKey::from_hex("0000000000000000000000000000000000000000000000000000000000000000")?, // System
            content: format!(
                "🎉 Willkommen {}!\n\n\
                📋 Nächste Schritte:\n\
                1. Stelle dich der Gruppe vor\n\
                2. Lerne andere Mitglieder kennen\n\
                3. Warte auf Admin-Verifikation\n\
                4. Dann kannst du Bitcoin-Angebote erstellen\n\n\
                💡 Tipp: Je besser wir uns kennen, desto sicherer der Handel!", 
                nickname
            ),
            message_type: GroupMessageType::Announcement,
            created_at: Timestamp::now().as_u64(),
            reply_to: None,
        };

        self.add_group_message(group_id, message);
        Ok(())
    }

    // NIP-29 Event Publishing
    async fn publish_group_creation_event(&self, admin_keys: &Keys, group_id: &str, name: &str) -> Result<()> {
        let event = EventBuilder::new(
            Kind::Custom(39000), // NIP-29 Group Creation
            format!("Neue private Bitcoin-Tausch-Gruppe: {}", name),
            [
                Tag::generic(TagKind::Custom("d".into()), [group_id]),
                Tag::generic(TagKind::Custom("name".into()), [name]),
                Tag::generic(TagKind::Custom("about".into()), ["Private Bitcoin-Handels-Community"]),
                Tag::generic(TagKind::Custom("private".into()), ["true"]),
            ]
        ).to_event(admin_keys)?;

        self.nostr_client.send_event(event).await?;
        Ok(())
    }

    async fn publish_group_join_event(&self, user_keys: &Keys, group_id: &str, nickname: &str) -> Result<()> {
        let event = EventBuilder::new(
            Kind::Custom(39001), // NIP-29 Group Join
            format!("{} ist der Gruppe beigetreten", nickname),
            [
                Tag::generic(TagKind::Custom("d".into()), [group_id]),
                Tag::generic(TagKind::Custom("nickname".into()), [nickname]),
            ]
        ).to_event(user_keys)?;

        self.nostr_client.send_event(event).await?;
        Ok(())
    }

    async fn publish_group_message_event(&self, user_keys: &Keys, message: GroupMessage) -> Result<()> {
        let event = EventBuilder::new(
            Kind::Custom(39002), // NIP-29 Group Message
            message.content,
            [
                Tag::generic(TagKind::Custom("d".into()), [&message.group_id]),
                Tag::generic(TagKind::Custom("msg_type".into()), [&format!("{:?}", message.message_type)]),
            ]
        ).to_event(user_keys)?;

        self.nostr_client.send_event(event).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupStats {
    pub total_members: usize,
    pub verified_members: usize,
    pub introduced_members: usize,
    pub trading_enabled: bool,
    pub created_at: u64,
}
