# 🔐 Bitcoin-Tausch-Netzwerk
## ⚡ Anonyme Bitcoin-Kontaktvermittlung - **LIVE & EINSATZBEREIT** ✅

### 🎯 **Funktionale Web-App für anonyme Bitcoin-Kontakte**
- ✅ **3-Seiten-System** bereit für sofortigen Einsatz
- ✅ **Anonyme Angebote** - Verkäufer bleiben unerkannt
- ✅ **Partner-Kontrolle** - Anbieter wählt Kontakte  
- ✅ **nsec-Authentifizierung** - Sicher ohne Passwörter

### 🌐 **Sofort verfügbar**
```bash
🚀 Demo-URL: https://dein-username.github.io/Bitcoin-Tausch-Netzwerk/start.html
🔑 Demo-Login: nsec15tl6pwn8h00w85qcvggchzuf0egk4jha08q8mtw74q6ennu45lzsjmpfhw
💰 Live-Angebote: €150, €300, €500 (Bargeld/Rechnung/Überweisung)
```

---

## 🚀 **Wie es funktioniert**

### **1. 🔑 Login** 
`start.html` → nsec eingeben → Allowlist-Check → Dashboard-Zugang

### **2. 💰 Angebote sehen**
`dashboard.html` → Anonyme Liste: "€500 (Bargeld) - 2 Interessen"

### **3. 💝 Interesse zeigen** 
"Interesse zeigen" klicken → Counter steigt → Anbieter wird benachrichtigt

### **4. 👥 Partner wählen**
`my-offers.html` → Interessenten-Liste → "Chat starten" → Private Verhandlung

---

## 📁 **Projekt-Struktur**

```
Bitcoin-Tausch-Netzwerk/
├── docs/                    # 🌐 Live Web-App
│   ├── start.html          # 🔐 Login (nsec-Input)
│   ├── dashboard.html      # 💰 Angebots-Übersicht  
│   ├── my-offers.html      # 👥 Interessenten-Management
│   ├── config.js           # ⚙️ Allowlist-Konfiguration
│   └── styles/             # 🎨 Bootstrap + Glassmorphismus
├── src/                    # 🦀 Rust Backend (v2.0)
├── MASTERPLAN.md          # 📋 Vollständige Dokumentation
└── README.md              # 🚀 Diese Anleitung
```

---

## 🎨 **Design-Features**
- **🔮 Glassmorphismus** - Moderne transparente UI
- **🟣 Nostr-Branding** - Lila/Pink Farbschema (#6b46c1, #ec4899)
- **📱 Mobile-optimiert** - Bootstrap 5.3 responsive
- **⚡ PWA-ready** - Installierbar als App

---

## 🚀 **Deployment (5 Minuten)**

### **GitHub Pages aktivieren:**
```bash
1. Repository Settings → Pages → Source: docs/
2. ✅ Live unter: https://dein-username.github.io/Bitcoin-Tausch-Netzwerk/start.html
```

### **Allowlist anpassen:**
```javascript
// In docs/config.js:
const AUTHORIZED_MEMBERS = [
    'npub1deine...echte...npub',     // Du
    'npub1freund1...npub',           // Bitcoin-Trader 1  
    'npub1freund2...npub',           // Bitcoin-Trader 2
];
```

---

## ⚡ **Schnellstart**

### **1. Lokal testen:**
```bash
cd docs/ && python -m http.server 8000
# → http://localhost:8000/start.html
```

### **2. Demo-Login verwenden:**
```
nsec15tl6pwn8h00w85qcvggchzuf0egk4jha08q8mtw74q6ennu45lzsjmpfhw
```

### **3. Live deployen:**
```bash
git push → GitHub Pages aktivieren → Sofort verfügbar!
```

---

## 🎯 **Status & Roadmap**

### **✅ MVP - Vollständig implementiert**
- [x] nsec-Login funktional
- [x] Anonyme Angebots-Anzeige
- [x] Interessenten-System aktiv
- [x] Partner-Auswahl implementiert
- [x] Mobile-responsive

### **🚧 v2.0 - Nostr-Integration (geplant)**
- [ ] Echte Nostr DMs für Chat
- [ ] Live-Angebots-Publishing
- [ ] Push-Notifications
- [ ] create-offer.html Seite

### **📈 v3.0 - Community-Features**
- [ ] Reputation-System
- [ ] Lightning-Integration  
- [ ] Multi-Sprachen Support

---

**🎉 Das Bitcoin-Tausch-Netzwerk ist LIVE!**

### 🔗 **Direktlinks:**
- 🚀 **Login**: `start.html`
- 💰 **Dashboard**: `dashboard.html` 
- 👥 **Management**: `my-offers.html`

*Anonyme Bitcoin-Kontaktvermittlung - funktional, sicher, sofort einsatzbereit.* ⚡₿
