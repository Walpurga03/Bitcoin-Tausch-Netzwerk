# Phase 1 – Basis-Setup & verschlüsselter Gruppen-Chat

## ✅ Was wurde implementiert

### 1. UI-Grundgerüst
- **Login-Seite** (`src/routes/+page.svelte`): Glassmorphic Oberfläche für Gruppenbeitritt
- **Chat-Interface** (`src/routes/(app)/group/+page.svelte`): Vollständige Chat-Ansicht
- **Responsive Design**: Mobile-first mit modernen Gradients

### 2. Nostr-Integration
- **NostrClient** (`src/lib/nostr/client.ts`): Relay-Verbindung & Event-Handling
- **nostr-tools**: Aktuelle API-Version integriert
- **Event-Handling**: Senden/Empfangen von Nostr-Events (NIP-28)

### 3. Schlüssel-Management
- **Validierung**: Hex- und nsec-Format für private Schlüssel
- **Sicherheit**: Private Schlüssel werden nur lokal verwendet

### 4. Verschlüsselung
- **AES-256-GCM**: In `src/lib/nostr/crypto.ts`
- **PBKDF2**: Schlüsselableitung aus Gruppen-Secret
- **End-to-End**: Nachrichten werden clientseitig ver-/entschlüsselt

### 5. Svelte Stores
- **userStore**: Nutzer-Authentifizierung & Profil
- **groupStore**: Gruppenkonfiguration & Nachrichten
- **offerStore**: Vorbereitung für Angebots-Funktionalität (Phase 2)

---

## 🛡️ Whitelist-Zugangskontrolle – Schritt für Schritt

### 1. Whitelist-Datei anlegen & pflegen
- Die Datei `src/whitelist.json` enthält alle erlaubten npub-Pubkeys und Namen:

```json
{
  "allowed_pubkeys": [
    { "pubkey": "npub1...", "name": "Alice" },
    { "pubkey": "npub1...", "name": "Bob" }
  ]
}
```
- Jeder Eintrag: `pubkey` (npub) und `name`.
- **Wichtig:** `src/whitelist.json` muss in `.gitignore` stehen!

### 2. Zugangskontrolle im Code
- Beim Login wird der private Schlüssel (hex oder nsec) eingegeben.
- Die App leitet daraus den npub ab und prüft, ob dieser in der Whitelist steht:

```typescript
import { nip19, getPublicKey } from 'nostr-tools';
const privHex = ... // aus Eingabe (hex oder nsec)
const privBytes = Uint8Array.from(Buffer.from(privHex, 'hex'));
const npub = nip19.npubEncode(getPublicKey(privBytes));
if (!allowedPubkeys.includes(npub)) {
  throw new Error('Du bist nicht auf der Whitelist!');
}
```

### 3. Ablauf für neue Nutzer
1. Admin trägt neuen npub und Namen in die Whitelist ein.
2. Nutzer erhält Einladungslink und gibt seinen privaten Schlüssel (hex oder nsec) ein.
3. Die App prüft automatisch, ob der abgeleitete npub in der Whitelist steht.
4. Bei Erfolg: Zugang zur Gruppe. Bei Fehler: Hinweis, dass kein Zugang besteht.

### 4. Vorteile
- Übersichtliche Verwaltung der Gruppenmitglieder
- Schnelles Hinzufügen/Entfernen von Nutzern
- Keine versehentliche Veröffentlichung sensibler Daten

---

## 🧪 Testing-Anleitung

### 1. Anwendung starten
```bash
cd bitcoinSwap
pnpm run dev
# → http://localhost:5174/
```

### 2. Test-Einladungslink
```
http://localhost:5174?relay=wss://nostr-relay.online&secret=test123456
```

### 3. Ablauf testen
1. ✅ **Link eingeben**: Test-Link von oben
2. ✅ **Privaten Schlüssel eingeben**: Hex oder nsec
3. ✅ **Gruppe beitreten**: Weiterleitung zu `/group`
4. 🔄 **Relay-Verbindung**: Automatisch beim Laden der Chat-Seite
5. 💬 **Chat testen**: Nachrichten senden/empfangen

---

## 📋 Zusammenfassung für Admins & Nutzer
- **Whitelist pflegen:** `src/whitelist.json` bearbeiten
- **Zugang testen:** Mit Einladungslink und privatem Schlüssel anmelden
- **Fehlermeldung:** Nur Whitelist-Mitglieder können beitreten
- **Sicherheit:** Whitelist niemals ins öffentliche Repository pushen

---

## 🎯 Was funktioniert bereits
- ✅ **UI/UX**: Login und Chat-Interface vollständig
- ✅ **Verschlüsselung**: AES-256-GCM funktioniert
- ✅ **Nostr-Events**: Können erstellt und signiert werden
- ✅ **Navigation**: Zwischen Seiten funktioniert
- ✅ **Stores**: Daten-Persistierung im localStorage

## 🔧 Was noch zu testen ist
- 🧪 **Relay-Verbindung**: Echte Relay-Tests mit mehreren Clients
- 🧪 **Nachrichten-Sync**: Verschiedene Browser/Tabs
- 🧪 **Verschlüsselung**: End-to-End zwischen verschiedenen Nutzern
- 🧪 **Fehlerbehandlung**: Was passiert bei Relay-Ausfall?

---

## 📋 Bereit für Phase 2
Mit Phase 1 abgeschlossen können wir jetzt zu **Phase 2 – Angebots-Funktionalität** wechseln:
- 🏷️ **Angebots-Erstellung**: Bitcoin-Handelangebote mit 3 Zahlungsoptionen
- 👀 **Interesse zeigen**: Reaktionen auf Angebote (NIP-25)
- 🎭 **Anonymität**: Temporäre Schlüssel für anonyme Angebote
- 📱 **UI-Erweiterung**: Angebots-Raum neben Chat-Raum

Die Grundlage (verschlüsselte Kommunikation, Nostr-Integration, UI-Framework) steht solide!