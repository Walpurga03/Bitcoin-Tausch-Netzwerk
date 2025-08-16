# 🔐 Bitcoin-Tausch-Netzwerk

**Anonymes Nostr-basiertes Bitcoin-Tausch-Netzwerk für sichere Kontaktvermittlung**

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Nostr](https://img.shields.io/badge/nostr-protocol-purple.svg)](https://nostr.com)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-production--ready-green.svg)](MASTERPLAN.md)

> ⚠️ **WICHTIGER DISCLAIMER**: Diese Plattform vermittelt **NUR** Kontakte zwischen Bitcoin-Käufern und -Verkäufern. Es werden **KEINE** Transaktionen durchgeführt!

## 🎯 Was ist das Bitcoin-Tausch-Netzwerk?

Ein dezentrales System auf Basis des Nostr-Protokolls, das es ermöglicht:

- **🎭 Anonyme Angebotserstellung** für Bitcoin-Käufe und -Verkäufe
- **🔐 Sichere Kontaktvermittlung** zwischen Interessenten
- **🛡️ Maximale Privatsphäre** durch NIP-59 Gift Wrapping
- **🌍 Dezentrale Infrastruktur** ohne Single Point of Failure

## ✨ Features

### 🔐 **Sicherheit & Anonymität**
- **NIP-59 Gift Wrapping** für vollständige Anonymität
- **Temporäre Identitäten** für jede Transaktion
- **Pseudo-ID System** ohne Rückverfolgbarkeit
- **Verschlüsselte Kommunikation** via NIP-44

### 💰 **Bitcoin-Angebote**
- **BUY/SELL Angebote** mit flexibler Preisgestaltung
- **Automatisches Matching** von Käufern/Verkäufern
- **Intelligente Ablaufzeiten**
- **Thread-sichere Angebotsverwaltung**

### 📡 **Nostr Integration**
- **Vollständige NIP-Kompatibilität** (NIP-01, 12, 29, 44, 59)
- **Multi-Relay Support** für Redundanz
- **Eigenes Relay** für maximale Kontrolle
- **Hochperformante Verbindungen**

## 🚀 Schnellstart

### Voraussetzungen

- [Rust](https://rustup.rs/) 1.70 oder höher
- Git

### Installation

```bash
# Repository klonen
git clone https://github.com/[username]/Bitcoin-Tausch-Netzwerk.git
cd Bitcoin-Tausch-Netzwerk

# Abhängigkeiten installieren und testen
cargo test

# Anwendung ausführen
cargo run
```

### Erste Schritte

1. **Relay-Kompatibilität testen**
   ```bash
   cargo run  # Führt automatisch alle Tests durch
   ```

2. **Eigenes Relay konfigurieren**
   ```rust
   // In src/main.rs die Relay-URL anpassen
   let relay_url = "wss://your-relay.example.com";
   ```

3. **Angebote erstellen**
   ```rust
   let offer = AnonymousOffer::new(
       OfferType::Buy,
       0.025,      // BTC Amount
       1000.0,     // EUR Price
       3600        // Expires in 1 hour
   );
   ```

## 🏗️ Architektur

```
src/
├── main.rs              # 🏠 Hauptanwendung & Integration Tests
├── types.rs             # 📊 Datenstrukturen (AnonymousOffer, PrivateInterest)
├── offer_manager.rs     # 🗄️ Thread-sichere Angebotsverwaltung
├── nostr_client.rs      # 📡 Nostr-Protokoll Integration
├── anonymous_publisher.rs # 🎭 NIP-59 Anonymisierung
└── relay_tester.rs      # 🧪 Relay-Kompatibilitätstests
```

### Technologie-Stack

- **[Rust](https://www.rust-lang.org/)** - Hochperformante, sichere Systemprogrammierung
- **[nostr-sdk](https://github.com/rust-nostr/nostr)** - Nostr-Protokoll Implementation
- **[tokio](https://tokio.rs/)** - Async Runtime für hohe Performance
- **[serde](https://serde.rs/)** - Serialisierung/Deserialisierung
- **[uuid](https://docs.rs/uuid/)** - Eindeutige Identifikatoren

## 🧪 Tests

Das Projekt verfügt über umfassende Tests:

```bash
# Alle Tests ausführen
cargo test

# Tests mit Output
cargo test -- --nocapture

# Spezifische Tests
cargo test test_offer_creation
cargo test test_anonymous_publishing
```

### Test Coverage

- ✅ **Datenstrukturen** - Vollständig getestet
- ✅ **Offer Management** - Alle CRUD-Operationen
- ✅ **Nostr Integration** - Relay-Verbindungen und Events
- ✅ **Anonymisierung** - NIP-59 Gift Wrapping
- ✅ **Relay Kompatibilität** - Alle NIPs verifiziert

## 📡 Nostr NIPs

Das System unterstützt folgende Nostr Improvement Proposals:

| NIP | Beschreibung | Status |
|-----|-------------|--------|
| [NIP-01](https://github.com/nostr-protocol/nips/blob/master/01.md) | Basic protocol flow | ✅ Vollständig |
| [NIP-12](https://github.com/nostr-protocol/nips/blob/master/12.md) | Generic tag queries | ✅ Vollständig |
| [NIP-29](https://github.com/nostr-protocol/nips/blob/master/29.md) | Simple Groups | ✅ Vollständig |
| [NIP-44](https://github.com/nostr-protocol/nips/blob/master/44.md) | Versioned Encryption | ✅ Vollständig |
| [NIP-59](https://github.com/nostr-protocol/nips/blob/master/59.md) | Gift Wrap | ✅ Vollständig |

## 🔐 Sicherheit

### Anonymität-Features

- **🎭 Temporäre Schlüssel** - Neue Identität für jede Session
- **🎁 Gift Wrapping** - NIP-59 verschleiert Event-Inhalte
- **🛡️ Pseudo-IDs** - UUID-basierte anonyme Identifikatoren
- **🔒 Verschlüsselung** - End-to-end verschlüsselte Kommunikation

### Best Practices

1. **Verwenden Sie Ihr eigenes Relay** für maximale Kontrolle
2. **Niemals echte Identitäten** in öffentlichen Events preisgeben
3. **Tor verwenden** für zusätzliche Anonymität
4. **Regelmäßige Updates** für Sicherheitspatches

## 🤝 Mitwirken

Beiträge sind willkommen! Bitte beachten Sie:

1. **Fork** das Repository
2. **Branch** für Ihr Feature erstellen (`git checkout -b feature/amazing-feature`)
3. **Commit** Ihre Änderungen (`git commit -m 'Add amazing feature'`)
4. **Push** zum Branch (`git push origin feature/amazing-feature`)
5. **Pull Request** öffnen

### Entwicklungsrichtlinien

- **Code-Qualität**: Alle Tests müssen bestehen
- **Dokumentation**: Neue Features dokumentieren
- **Sicherheit**: Security-relevante Änderungen reviewen lassen
- **Performance**: Benchmarks für kritische Pfade

## 📄 Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe [LICENSE](LICENSE) für Details.

## ⚠️ Rechtlicher Hinweis

Diese Software dient ausschließlich der **Kontaktvermittlung**. Die Entwickler übernehmen keine Verantwortung für:

- Tatsächliche Bitcoin-Transaktionen zwischen Nutzern
- Finanzielle Verluste oder Betrug
- Rechtliche Probleme aus der Nutzung
- Verletzungen lokaler Gesetze und Vorschriften

**Nutzen Sie diese Software verantwortungsvoll und auf eigene Gefahr.**

## 🔗 Links

- **[Masterplan](MASTERPLAN.md)** - Detaillierte Projektdokumentation
- **[Nostr Protocol](https://nostr.com)** - Mehr über das Nostr-Protokoll
- **[Rust Language](https://www.rust-lang.org/)** - Programmiersprache
- **[Bitcoin](https://bitcoin.org)** - Kryptowährung

---

**🎉 Erstellt mit ❤️ für die Bitcoin-Community**

*Made with Rust 🦀 and Nostr 💜*
