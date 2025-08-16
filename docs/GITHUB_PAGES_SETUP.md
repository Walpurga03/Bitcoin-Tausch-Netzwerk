# 🚀 GitHub Pages Setup Anleitung

## ✅ **Dateien bereits erstellt:**

- ✅ `.github/workflows/pages.yml` - GitHub Actions Workflow
- ✅ `docs/_config.yml` - Jekyll Konfiguration  
- ✅ `docs/index.html` - Haupt-Web-Interface
- ✅ `docs/index.md` - Markdown Fallback
- ✅ `docs/script.js` - JavaScript Funktionalität
- ✅ `docs/login-demo.html` - Login-Test-Seite

## 🔧 **GitHub Pages aktivieren:**

### Schritt 1: Repository Settings
1. Gehe zu deinem GitHub Repository: https://github.com/Walpurga03/Bitcoin-Tausch-Netzwerk
2. Klicke auf **"Settings"** (rechts oben)
3. Scrolle zu **"Pages"** (linke Sidebar)

### Schritt 2: Source konfigurieren
4. Bei **"Source"** wähle **"GitHub Actions"**
5. Das war's! GitHub erkennt automatisch den Workflow in `.github/workflows/pages.yml`

### Schritt 3: Workflow starten
6. Der Workflow startet automatisch beim nächsten Push
7. Oder gehe zu **"Actions"** Tab und starte manuell

## 🌐 **Deine Website wird verfügbar sein unter:**

```
https://walpurga03.github.io/Bitcoin-Tausch-Netzwerk/
```

## 📋 **Was passiert automatisch:**

1. **Bei jedem Push** auf `main` branch
2. **GitHub Actions** startet den Workflow
3. **Dateien aus `docs/`** werden deployed  
4. **Website ist live** in ~2-5 Minuten

## 🧪 **Nach dem Deployment testen:**

- **Haupt-Interface:** https://walpurga03.github.io/Bitcoin-Tausch-Netzwerk/
- **Login-Demo:** https://walpurga03.github.io/Bitcoin-Tausch-Netzwerk/login-demo.html
- **Markdown-Seite:** https://walpurga03.github.io/Bitcoin-Tausch-Netzwerk/index.md

## ⚡ **Troubleshooting:**

### Workflow fails?
- Gehe zu **"Actions"** Tab
- Klicke auf den fehlgeschlagenen Workflow
- Überprüfe die Logs

### Pages nicht aktiviert?
- Settings → Pages → Source: "GitHub Actions" auswählen

### 404 Error?
- Warten bis Deployment complete (GitHub Actions)
- Cache leeren (Ctrl+F5)

## 🔄 **Updates deployen:**

```bash
# Änderungen machen
git add .
git commit -m "Update website"
git push origin main

# GitHub Pages updated automatisch!
```

## 🎉 **Features der Website:**

- **🔐 Universal Nostr Login** (Alby, Amber, nos2x)
- **📱 Responsive Design** für alle Geräte
- **💬 Live Bitcoin-Angebote** vom Nostr-Relay
- **🔒 Client-side Verschlüsselung**
- **⚡ Keine Server-Kosten** - alles statisch!

Das ist die **einfachste Lösung** für User-Onboarding in das Bitcoin-Tausch-Netzwerk! 🚀
