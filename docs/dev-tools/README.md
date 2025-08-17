# 🛠️ Development Tools

Diese Verzeichnis enthält Entwicklungstools für das Bitcoin-Tausch-Netzwerk:

## 🔑 Key Generation Tools

- **`simple_key_gen.py`** - Python Script zum Generieren gültiger nsec/npub Demo-Keys
- **`generate_demo_keys.py`** - Erweiterte Key-Generierung mit Base58 Encoding
- **`account-generator.html`** - Web-basierter Account Generator
- **`valid-key-generator.html`** - Validierung und Generierung von Nostr Keys
- **`npub-generator.html`** - Speziell für npub Key-Generierung

## 🔧 Utilities

- **`debug.js`** - Debug-Funktionen für Entwicklung
- **`convert.html`** - Key-Format Konvertierung zwischen nsec/npub

## 📝 Generierte Keys

Die aktuell verwendeten Demo-Keys wurden mit `simple_key_gen.py` generiert und sind in:
- `/docs/config.js` (npub Keys für Authorization)
- `/docs/DEMO-ACCOUNTS.md` (nsec/npub Paare für Testing)

## 🚀 Verwendung

```bash
# Key-Generator starten
python3 simple_key_gen.py

# Web-basierte Tools (Server muss laufen)
http://localhost:8001/dev-tools/account-generator.html
```

---
*Diese Tools sind nur für die Entwicklung und sollten nicht in Produktion verwendet werden.*
