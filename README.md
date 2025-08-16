# 🔐 Bitcoin-Tausch-Netzwerk
## Private Nostr-Gruppe mit nsec-basierter Zugriffskontrolle

### 🎯 **Das Konzept**

Eine **private Bitcoin-Handels-Gruppe** für bekannte Personen, basierend auf:
- ✅ **Nsec-basierte Authentifizierung** (einfach & sicher)
- ✅ **Allowlist-System** (nur eingeladene Members)
- ✅ **Dezentrale Architektur** (keine Server, keine Datenbank)
- ✅ **Nostr-Integration** (zukunftssicher & privat)

---

## 🚀 **Wie es funktioniert**

### 1. **👤 Member-Login**
```
User gibt nsec ein → System konvertiert zu npub → Prüfung gegen Allowlist
✅ Authorized → Sofortiger Zugang zur Handels-Gruppe
❌ Not Authorized → "Zugang beantragen" Option
```

### 2. **🔐 Allowlist-Verwaltung**
```javascript
// 🔐 AUTHORIZED MEMBERS (npub format)
const AUTHORIZED_MEMBERS = [
    'npub1max...example1',    // Max - Bitcoin OG
    'npub1lisa...example2',   // Lisa - Lightning Expert  
    'npub1tom...example3',    // Tom - Hodler
    'npub1sarah...example4',  // Sarah - DeFi Queen
    'npub1admin...example6',  // Admin (du)
];
```

### 3. **📝 Neue Member-Anfragen**
- User gibt npub + Nachricht ein
- Admin bekommt DM über Nostr
- Admin entscheidet und fügt npub zur Liste hinzu
- User kann sich dann einloggen

---

## 💰 **Features der Handels-Gruppe**

### **Für Members verfügbar:**
- 👀 **Aktive Bitcoin-Angebote** anzeigen
- 📊 **Kauf/Verkauf-Angebote** erstellen  
- 💬 **Direkter Chat** mit anderen Tradern
- 👥 **Member-Liste** mit Online-Status
- 🔔 **Push-Notifications** für neue Deals

### **Bitcoin-Handel Features:**
- 💸 **Verkaufen**: Menge, Preis, Zahlungsart
- 💰 **Kaufen**: Gesuchte Menge, Maximalpreis
- 🏦 **Zahlungsarten**: SEPA, Cash, Lightning
- 📍 **Location-Filter**: München, Berlin, etc.
- ⭐ **Reputation-System**: Bewertungen nach Handel

---

## 🛡️ **Security & Privacy**

### **Was ist sicher:**
- ✅ **Nsec bleibt lokal** - wird nie übertragen
- ✅ **Nur npub wird geprüft** - öffentlich sicher
- ✅ **Allowlist-Kontrolle** - keine unbekannten User
- ✅ **Dezentrales System** - kein Single Point of Failure
- ✅ **End-to-End-Chat** - via Nostr DMs

### **Admin-Kontrolle:**
- 🎛️ **Vollständige Member-Kontrolle**
- 🚫 **Sofortige Sperrung** möglich
- 📊 **Activity-Monitoring**
- 🔄 **Backup-Admins** definierbar

---

## 📁 **Projekt-Struktur**

```
Bitcoin-Tausch-Netzwerk/
├── docs/
│   ├── index.html           # 🔐 Haupt-App (nsec-Login)
│   ├── styles/              # 🎨 SCSS mit Nostr-Farben
│   │   ├── main.css         # Kompilierte Styles
│   │   ├── _variables.scss  # Farben & Variablen
│   │   └── _components.scss # UI-Komponenten
│   ├── manifest.json        # 📱 PWA-Manifest
│   └── nsec-login-demo.html # 🎭 Demo-Version
├── src/
│   ├── main.rs              # 🦀 Rust Backend (optional)
│   ├── offer_manager.rs     # 💰 Bitcoin-Angebote
│   └── nostr_client.rs      # 📡 Nostr-Integration
├── archive/old-approach/    # 📦 Alte Ansätze (Backup)
└── README.md                # 📖 Diese Dokumentation
```

---

## 🎨 **Design-System**

### **Nostr-Farben:**
- 🟣 **Primary**: `#6b46c1` (Lila)
- 🟢 **Secondary**: `#8b5cf6` (Violett)  
- 🌸 **Accent**: `#ec4899` (Pink)
- ⚫ **Dark**: `#1a1a1a` (Fast Schwarz)

### **UI-Features:**
- 🔮 **Glass Morphism** Effekte
- 🌈 **Gradient Buttons**
- ✨ **Neon Glow** Animationen
- 📱 **Mobile-First** Responsive

---

## 🚀 **Deployment**

### **GitHub Pages:**
```bash
# Automatisch via GitHub Actions
git push origin main
# → Live unter: https://username.github.io/Bitcoin-Tausch-Netzwerk
```

### **Als PWA:**
- 📱 **Installierbar** auf Handy/Desktop
- 🔔 **Push-Notifications** 
- 💾 **Offline-fähig**
- ⚡ **App-ähnliche** Performance

---

## 🎯 **Next Steps**

### **Phase 1: MVP** ✅
- [x] Nsec-Login System
- [x] Allowlist-Verwaltung  
- [x] Basic UI mit Nostr-Design
- [x] Demo-Angebote anzeigen

### **Phase 2: Erweitert** 🚧
- [ ] Echte Nostr-Integration
- [ ] Bitcoin-Angebote CRUD
- [ ] Chat-System (DMs)
- [ ] Push-Notifications

### **Phase 3: Pro** 📈
- [ ] Lightning-Integration
- [ ] Reputation-System
- [ ] Mobile App (Tauri)
- [ ] Multi-Sprachen Support

---

## ⚡ **Schnellstart**

1. **Clone & Setup:**
   ```bash
   git clone https://github.com/username/Bitcoin-Tausch-Netzwerk
   cd Bitcoin-Tausch-Netzwerk/docs
   python -m http.server 8000
   ```

2. **Test-Login:**
   - Öffne `http://localhost:8000`
   - Verwende Test-nsec: `nsec1test...` (siehe Code)
   - Zugang zur Demo-Handels-Gruppe

3. **Member hinzufügen:**
   - Editiere `AUTHORIZED_MEMBERS` Array
   - Füge npub des neuen Members hinzu
   - Deploy → User kann sich einloggen

---

**🏢 Perfekt für deine private Bitcoin-Handels-Crew!** 🚀

*Einfach, sicher, dezentral - genau wie Bitcoin selbst.* ₿
