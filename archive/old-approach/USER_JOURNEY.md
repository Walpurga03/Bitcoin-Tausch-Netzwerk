# 🚀 USER JOURNEY - Bitcoin-Tausch-Gruppe beitreten

## 📱 **Schritt-für-Schritt: Vom Link zur Gruppe**

---

### 🔗 **SCHRITT 1: Invite-Link erhalten**

**Du (Admin) sendest einem Freund:**
```
Hey! Tritt unserer privaten Bitcoin-Gruppe bei:
👉 https://walpurga03.github.io/Bitcoin-Tausch-Netzwerk/group.html#join/BTN_A7X3M9K2

Einfach anklicken und mitmachen! 🚀
```

---

### 📱 **SCHRITT 2: Link anklicken**

**User klickt Link → Automatische Weiterleitung:**

```
🌐 Browser öffnet GitHub Pages Web-App
📍 URL: .../group.html#join/BTN_A7X3M9K2
🎯 App erkennt Invite-Code automatisch
```

**Was der User sieht:**
```
┌─────────────────────────────────────────┐
│ 🔐 Du wurdest eingeladen!               │
│                                         │
│ 🏢 Bitcoin-Tausch-Gruppe               │
│                                         │
│ Tritt unserer privaten Community bei.  │
│ Hier lernst du vertrauensvolle         │
│ Handelspartner kennen.                  │
│                                         │
│ Spitzname: [BitcoinMike_____]           │
│                                         │
│ [🚀 Mit Wallet anmelden]               │
│                                         │
│ ℹ️ Nach dem Beitritt: Stelle dich vor  │
└─────────────────────────────────────────┘
```

---

### 🔐 **SCHRITT 3: Anmeldung mit Nostr-Wallet**

**User klickt "Mit Wallet anmelden":**

#### Option A: Browser-Extension (Alby, nos2x)
```javascript
// Automatische Erkennung
if (window.nostr) {
    // Alby/nos2x gefunden!
    const pubkey = await window.nostr.getPublicKey();
    // ✅ Sofort angemeldet
}
```

#### Option B: Mobile (Amber App)
```javascript
// QR-Code für Amber
nostrLogin.launch({
    methods: ['connect', 'extension', 'local']
});
```

#### Option C: Neue Identität erstellen
```javascript
// Automatische Schlüsselgenerierung
const newKeys = generatePrivateKey();
// Sicher im Browser gespeichert
```

---

### 👋 **SCHRITT 4: Gruppe beitreten**

**Nach erfolgreicher Anmeldung:**

```
User gibt Spitznamen ein: "BitcoinMike"
└─ Validierung: 3-30 Zeichen, nur Buchstaben/Zahlen
└─ [🚀 Gruppe beitreten] wird aktiv

Klick auf "Gruppe beitreten":
└─ Invite-Code wird validiert: BTN_A7X3M9K2
└─ User wird zur Mitgliederliste hinzugefügt
└─ Automatische Weiterleitung zur Gruppen-Oberfläche
```

---

### 🏠 **SCHRITT 5: Gruppen-Interface**

**User sieht sofort:**

```
┌─────────────────────────────────────────────────────────────┐
│ 🔒 Private Gruppe • Nur mit Einladung                      │
├─────────────────────────────────────────────────────────────┤
│ 🏢 Bitcoin-Tausch-Gruppe                                   │
│ 👥 15 Mitglieder • Kennenlernphase                         │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│ ⚠️ VORSTELLUNG ERFORDERLICH                                │
│ Um Bitcoin-Angebote erstellen zu können, musst du dich     │
│ zuerst vorstellen und vom Admin verifiziert werden.        │
│                                                             │
│ [Erzähle etwas über dich: Woher kommst du?           ]     │
│ [Wie lange handelst du mit Bitcoin?                  ]     │
│ [Was sind deine Ziele?                               ]     │
│                                                             │
│ [📤 Vorstellung senden]                                    │
├─────────────────────────────────────────────────────────────┤
│ 💬 GRUPPEN-CHAT                                            │
│                                                             │
│ 🎉 Admin: Willkommen in der Gruppe! Stellt euch vor.      │
│ 👋 SatoshiFan: Hi! Ich bin aus München...                  │
│ 💬 CryptoTrader: Hallo alle! Trade seit 2020...           │
│                                                             │
│ [Nachricht schreiben...              ] [📤]               │
├─────────────────────────────────────────────────────────────┤
│ 👥 MITGLIEDER                                              │
│ • 🟢 Admin ⭐                                              │
│ • 🟢 SatoshiFan ✅                                         │
│ • 🟡 CryptoTrader ⏳                                       │
│ • 🟢 BitcoinMike (Du) 🆕                                   │
└─────────────────────────────────────────────────────────────┘
```

---

### 📝 **SCHRITT 6: Vorstellung senden**

**User schreibt Vorstellung:**
```
"👋 Hallo! Ich bin Mike aus Berlin und trade seit 2 Jahren 
mit Bitcoin. Ich suche lokale Händler für regelmäßige 
Käufe. Freue mich auf den Austausch!"
```

**Was passiert:**
```javascript
// Vorstellung wird als Nostr-Event gesendet
const introEvent = {
    kind: 1,
    content: "👋 Hallo! Ich bin Mike...",
    tags: [
        ["t", "introduction"],
        ["p", groupId]
    ]
};

// An alle Gruppenmitglieder verteilt
publishToNostrRelays(introEvent);
```

---

### ✅ **SCHRITT 7: Admin-Verifikation (später)**

**Admin sieht neue Vorstellung:**
```
┌─────────────────────────────────────────┐
│ 🆕 Neue Vorstellung: BitcoinMike       │
│                                         │
│ "Hi! Ich bin Mike aus Berlin..."       │
│                                         │
│ [✅ Verifizieren] [❌ Ablehnen]        │
└─────────────────────────────────────────┘
```

**Nach Verifikation:**
```
✅ BitcoinMike wurde verifiziert!
└─ Kann jetzt Bitcoin-Angebote erstellen
└─ Bekommt Notification: "Du bist jetzt verifiziert!"
```

---

### 🚀 **SCHRITT 8: Bitcoin-Handel freigeschaltet**

**Verifizierte Mitglieder sehen:**
```
┌─────────────────────────────────────────┐
│ 🚀 BITCOIN-HANDEL                      │
│                                         │
│ [💰 Bitcoin-Angebot erstellen]         │
│ [📊 Aktive Angebote ansehen]           │
│                                         │
│ Status: ✅ Verifiziert                 │
│ Reputation: ⭐⭐⭐ (75/100)            │
└─────────────────────────────────────────┘
```

---

## 🔧 **Technische Implementation**

### 📱 **Frontend (Web-App)**
```javascript
// URL-Parsing für Invite-Code
const inviteCode = window.location.hash.substring(6); // #join/BTN_...

// Nostr-Wallet Integration
if (window.nostr) {
    const pubkey = await window.nostr.getPublicKey();
}

// Gruppe beitreten
await joinGroup(inviteCode, nickname, pubkey);
```

### 📡 **Nostr-Events**
```javascript
// Gruppen-Beitritt Event
{
    kind: 1,
    content: `${nickname} ist der Gruppe beigetreten`,
    tags: [
        ["t", "group-join"],
        ["g", groupId],
        ["invite", inviteCode]
    ]
}

// Chat-Nachrichten
{
    kind: 1,
    content: "Hallo alle!",
    tags: [
        ["t", "group-chat"],
        ["g", groupId]
    ]
}
```

### 🔒 **Sicherheit**
```javascript
// Invite-Code Validierung
const isValidInvite = (code) => {
    return code.match(/^BTN_[A-Z0-9]{8}$/);
};

// Gruppen-Berechtigung
const canCreateOffers = (user) => {
    return user.isVerified && user.hasIntroduced;
};
```

---

## 📊 **Timeline für einen neuen User**

```
⏰ 0 Min:    Link erhalten & geklickt
⏰ 1 Min:    Wallet-Anmeldung abgeschlossen  
⏰ 2 Min:    Gruppe beigetreten & Interface geladen
⏰ 5 Min:    Vorstellung geschrieben & gesendet
⏰ 10 Min:   Mit anderen Mitgliedern gechattet
⏰ 1-24h:    Admin-Verifikation erhalten
⏰ Danach:   Bitcoin-Angebote erstellen möglich
```

---

**🎯 So einfach ist es! User braucht nur:**
1. ✅ Link anklicken
2. ✅ Nickname eingeben  
3. ✅ Wallet verbinden
4. ✅ Sich vorstellen

**Keine App-Installation, kein Registrierungsformular, keine Komplexität!** 🚀
