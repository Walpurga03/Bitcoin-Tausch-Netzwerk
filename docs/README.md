# 📱 Bitcoin-Tausch-Netzwerk - Web App

Eine glassmorphic Web-Anwendung für ein privates Bitcoin-Handels-Netzwerk mit nsec-basierter Authentifizierung.

## 🚀 Quick Start

```bash
# 1. Server starten
cd docs
python3 -m http.server 8001

# 2. App öffnen
http://localhost:8001/start.html
```

## 📁 Projekt-Struktur

### 🎯 Hauptanwendung
- **`start.html`** - Login-Seite mit nsec-Authentifizierung
- **`index.html`** - Dashboard mit Bitcoin-Angeboten
- **`my-offers.html`** - Interesse-Management für Angebote
- **`config.js`** - Konfiguration und autorisierte Mitglieder

### 📚 Dokumentation
- **`DEMO-ACCOUNTS.md`** - Test-Accounts für Login
- **`README.md`** - Diese Dokumentation

### 🎨 Design
- **`styles/`** - CSS-Dateien mit Glassmorphism-Design
- **`manifest.json`** - PWA-Konfiguration

### 🛠️ Entwicklung
- **`dev-tools/`** - Entwicklungstools und Key-Generatoren
- **`package.json`** - NPM-Dependencies (nostr-tools)

## 🔑 Authentication

Die App nutzt **Nostr nsec/npub Keys** für die Authentifizierung:

- **nsec (Private Key):** Für Login (geheim halten!)
- **npub (Public Key):** In `config.js` für Autorisierung

### Demo-Accounts (siehe `DEMO-ACCOUNTS.md`):
- Demo User, Alice Bitcoin, Bob Lightning, Charlie Hodler, Diana Trader

## 🎨 Design System

### Glassmorphism Theme
- **Primärfarben:** Lila-Violett (#6b46c1, #8b5cf6)
- **Akzentfarbe:** Pink (#ec4899)
- **Hintergrund:** Dunkle Gradients
- **Effekte:** Blur, Transparenz, Glow

### UI-Komponenten
- Bootstrap 5 Framework
- Bootstrap Icons
- Custom glassmorphic Cards
- Responsive Design

## 🔧 Features

### 🏠 Dashboard (`index.html`)
- Anonyme Bitcoin-Angebote durchstöbern
- Live-Statistiken (Angebote, Mitglieder, Volumen)
- "Interesse zeigen" Funktion
- Filterung nach Kategorien

### 👤 Meine Angebote (`my-offers.html`)
- Interesse-Management für eigene Angebote
- Chat-Initiierung mit interessierten Usern
- Angebotsstatus verwalten

### 🔐 Login (`start.html`)
- nsec-Key Authentifizierung
- Debug-Logging für Entwicklung
- Sessionverwaltung

## 🛠️ Entwicklung

### Dependencies
```bash
npm install nostr-tools
```

### Key-Generierung
```bash
cd dev-tools
python3 simple_key_gen.py
```

### Server starten
```bash
python3 -m http.server 8001
```

## 📱 Progressive Web App

Die App ist als PWA konfiguriert:
- Offline-Funktionalität vorbereitet
- App-Installation möglich
- Responsive Design

---

## 🎯 Nächste Schritte

- [ ] Backend-Integration für echte Bitcoin-Transaktionen
- [ ] Chat-System implementieren
- [ ] Push-Benachrichtigungen
- [ ] Offline-Funktionalität
- [ ] Mobile App (React Native)
