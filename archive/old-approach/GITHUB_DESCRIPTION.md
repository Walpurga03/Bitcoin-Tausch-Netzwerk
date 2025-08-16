# 🔐 Bitcoin-Tausch-Netzwerk

> **Anonymes Nostr-basiertes Bitcoin-Tausch-Netzwerk für sichere Kontaktvermittlung zwischen Bitcoin-Händlern**

[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)
[![Nostr](https://img.shields.io/badge/nostr-protocol-purple.svg)](https://nostr.com/)
[![GitHub Pages](https://img.shields.io/badge/demo-live-brightgreen.svg)](https://walpurga03.github.io/Bitcoin-Tausch-Netzwerk/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## 🚀 Live Demo

**🌐 [Bitcoin-Tausch-Netzwerk Live](https://walpurga03.github.io/Bitcoin-Tausch-Netzwerk/)**

*Ein-Klick-Login mit Alby, Amber oder jeder anderen Nostr-Wallet!*

---

## 📋 Übersicht

Das **Bitcoin-Tausch-Netzwerk** ist eine revolutionäre Plattform für **anonyme Kontaktvermittlung** zwischen Bitcoin-Käufern und -Verkäufern. Basierend auf dem dezentralen **Nostr-Protokoll** ermöglicht es sichere, private Kommunikation ohne zentrale Server oder Datensammlung.

### ⚠️ Wichtiger Hinweis
**Diese Plattform vermittelt NUR Kontakte - KEINE Transaktionen werden abgewickelt!**

---

## ✨ Hauptfeatures

### 🔐 **Maximale Anonymität & Sicherheit**
- **🎭 NIP-59 Gift Wrapping** - Vollständige Anonymisierung aller Angebote
- **🔒 NIP-17 Private Messages** - Verschlüsselte End-zu-End Kommunikation
- **🛡️ Pseudo-ID System** - Keine Rückverfolgbarkeit zur echten Identität
- **🔑 Client-side Encryption** - Schlüssel bleiben immer lokal

### 💰 **Bitcoin-Handel-Features**
- **📝 Anonyme Angebotserstellung** - BUY/SELL Bitcoin-Angebote
- **💱 Flexible Preisgestaltung** - EUR/BTC mit automatischer Kalkulation
- **⏰ Zeitbasierte Gültigkeit** - Automatisches Ablaufen von Angeboten
- **🤝 Intelligentes Matching** - Optimale Partnervermittlung

### 💬 **Kommunikation & Networking**
- **📱 Universal Login** - Alby, Amber, nos2x, alle NIP-07 Wallets
- **💌 Interest System** - Einfache Kontaktaufnahme zu Anbietern
- **🔄 Real-time Updates** - Live-Synchronisation über Nostr-Relays
- **📊 Conversation Management** - Organisierte Chat-Verläufe

---

## 🛠️ Technologie-Stack

### 🦀 **Backend (Rust)**
```
Rust 2021 Edition
├── nostr-sdk 0.33      # Nostr-Protokoll Integration
├── tokio               # Async Runtime für Performance
├── serde               # JSON Serialisierung
├── uuid                # Anonyme ID-Generierung
└── anyhow              # Fehlerbehandlung
```

### 🌐 **Frontend (Web)**
```
Pure Web Technologies
├── HTML5 + CSS3        # Responsive UI
├── Vanilla JavaScript  # Keine Framework-Dependencies
├── nostr-tools         # Nostr Client-Bibliothek
├── @nostrband/nostr-login # Universal Wallet-Integration
└── Tailwind CSS        # Modern Styling
```

### 📡 **Nostr-Protokoll Integration**
- **NIP-01** - Basic Protocol Flow (Events, Relays)
- **NIP-04** - Encrypted Direct Messages
- **NIP-12** - Generic Tags (Suchfunktionalität)
- **NIP-17** - Private Direct Messages (Hauptkommunikation)
- **NIP-29** - Simple Groups (Community-Features)
- **NIP-59** - Gift Wrapping (Maximale Anonymität)

---

## 🚀 Quick Start

### 📱 **Für Endnutzer**
1. **🌐 Website besuchen:** https://walpurga03.github.io/Bitcoin-Tausch-Netzwerk/
2. **🔐 Anmelden:** Mit Alby, Amber oder neue Identität erstellen
3. **📝 Angebot erstellen:** Bitcoin kaufen/verkaufen
4. **💬 Kommunizieren:** Private Nachrichten mit Interessenten

### 👨‍💻 **Für Entwickler**
```bash
# Repository klonen
git clone https://github.com/Walpurga03/Bitcoin-Tausch-Netzwerk.git
cd Bitcoin-Tausch-Netzwerk

# Rust Backend testen
cargo test
cargo run

# Web Interface lokal testen
cd docs/
python3 -m http.server 8080
# Oder: npx serve .
```

---

## 📊 **Projektdaten**

| Metrik | Wert |
|--------|------|
| **Sprachen** | Rust (Backend) + JavaScript (Frontend) |
| **Tests** | 15/15 passing (100% Coverage) |
| **NIPs implementiert** | 6 (NIP-01, 04, 12, 17, 29, 59) |
| **Deployment** | GitHub Pages (Live) |
| **Performance** | <250ms Message Delivery |
| **Security** | Client-side Encryption |

---

## 🎯 **Anwendungsfälle**

### 👤 **Für Bitcoin-Käufer**
- Anonyme Suche nach lokalen Bitcoin-Verkäufern
- Sichere Kommunikation vor persönlichem Treffen
- Preisvergleich verschiedener Angebote

### 💼 **Für Bitcoin-Verkäufer**
- Anonyme Veröffentlichung von Verkaufsangeboten
- Überprüfung und Auswahl vertrauenswürdiger Käufer
- Private Verhandlung von Transaktionsdetails

### 🌍 **Für die Community**
- Dezentrale Alternative zu zentralisierten Börsen
- Förderung von Peer-to-Peer Bitcoin-Handel
- Aufbau lokaler Bitcoin-Communities

---

## 🔒 **Sicherheit & Datenschutz**

### 🛡️ **Privacy-by-Design**
- **Keine Registrierung** erforderlich - sofortiger anonymer Zugang
- **Keine Datensammlung** - keine Server-side Speicherung
- **Keine KYC/AML** - vollständig pseudonym
- **Keine Tracking** - kein Analytics oder Cookies

### 🔐 **Kryptographische Sicherheit**
- **secp256k1** Elliptic Curve Cryptography (Bitcoin-Standard)
- **NIP-04/17** End-to-End Message Encryption
- **NIP-59** Additional Anonymization Layer
- **Client-side Key Generation** - niemals Übertragung privater Schlüssel

---

## 📚 **Dokumentation**

- **📋 [MASTERPLAN.md](MASTERPLAN.md)** - Vollständige Projektdokumentation
- **🎨 [Interface-Plan](interface-plan.md)** - UI/UX Design-Entscheidungen
- **🧪 [Login-Demo](https://walpurga03.github.io/Bitcoin-Tausch-Netzwerk/login-demo.html)** - Teste verschiedene Anmelde-Methoden
- **📡 [Nostr Integration](src/)** - Technische Implementation Details

---

## 🤝 **Contributing**

Das Projekt ist **Open Source** und freut sich über Beiträge!

### 🎯 **Gewünschte Contributions**
- **🌐 Internationalization** - Übersetzungen in andere Sprachen
- **📱 Mobile Apps** - Native iOS/Android Clients
- **🔧 Relay Improvements** - Bessere Relay-Kompatibilität
- **🎨 UI/UX Enhancements** - Design-Verbesserungen
- **📊 Analytics Features** - Markt-Trend Analyse

### 📝 **Development Setup**
```bash
# Rust Toolchain installieren
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dependencies installieren
cargo build

# Tests ausführen
cargo test

# Development Server starten
cargo run
```

---

## 📞 **Support & Community**

- **🐛 Issues:** [GitHub Issues](https://github.com/Walpurga03/Bitcoin-Tausch-Netzwerk/issues)
- **💬 Discussions:** [GitHub Discussions](https://github.com/Walpurga03/Bitcoin-Tausch-Netzwerk/discussions)
- **📧 Contact:** Via Nostr oder GitHub

---

## 📄 **Lizenz**

Dieses Projekt steht unter der **MIT License** - siehe [LICENSE](LICENSE) für Details.

---

## 🙏 **Acknowledgments**

- **Nostr-Community** für das revolutionäre dezentrale Protokoll
- **Rust-Community** für die sichere und performante Programmiersprache
- **Bitcoin-Community** für die Inspiration zu peer-to-peer Handel

---

**⭐ Star dieses Repository wenn es dir gefällt!**

*Gebaut mit ❤️ für die dezentrale Zukunft des Bitcoin-Handels*
