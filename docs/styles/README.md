# 🎨 NOSTR DESIGN SYSTEM
## Bitcoin-Tausch-Netzwerk Style Guide

### 🔮 **Farb-Palette (Nostr-inspiriert)**

```scss
// Primary Colors
$nostr-purple: #6b46c1;      // Hauptfarbe für Buttons, Links
$nostr-violet: #8b5cf6;      // Sekundär für Akzente  
$nostr-pink: #ec4899;        // Call-to-Action, Highlights
$nostr-lavender: #a78bfa;    // Subtile Akzente
$nostr-magenta: #d946ef;     // Vibrant für Erfolg/Aktionen

// Gradients
$gradient-primary: linear-gradient(135deg, #6b46c1 0%, #8b5cf6 50%, #ec4899 100%);
$gradient-secondary: linear-gradient(45deg, #a78bfa 0%, #d946ef 100%);
```

### 📱 **Responsive Breakpoints**

```scss
$mobile: 768px;     // Smartphones
$tablet: 1024px;    // Tablets  
$desktop: 1280px;   // Desktop
$xl: 1536px;        // Large screens
```

### 🎯 **Design-Prinzipien**

#### ✨ **Glass Morphism**
- Transparente Hintergründe mit Blur-Effekt
- Subtile Rahmen für moderne Optik
- `@include glass($opacity, $blur)`

#### 🌟 **Gradient Buttons**
- Nostr-Farbverlauf für Call-to-Actions
- Hover-Effekte mit Glow
- `@include gradient-button($gradient)`

#### 💫 **Neon Glow Effects**
- Leuchtende Rahmen für Fokus-Zustände
- Pulsing-Animation für Live-Status
- `@include neon-glow($color)`

### 🧱 **Komponenten-System**

#### 📦 **Cards**
```scss
.card {
  @include card($elevated: true);  // Mit Hover-Effekt
  @include card($elevated: false); // Statisch
}
```

#### 🎨 **Buttons**
```scss
.btn-primary    // Gradient-Button (Hauptaktion)
.btn-secondary  // Glass-Button (Sekundäraktion)
.btn-outline    // Transparenter Button mit Rahmen
```

#### 💬 **Chat Interface**
```scss
.chat-container     // Hauptcontainer mit Glass-Effekt
.message.sent       // Gesendete Nachrichten (rechts, Gradient)
.message.received   // Empfangene Nachrichten (links, Glass)
```

### ⚡ **Animationen**

#### 🌊 **Entrance Animations**
```scss
.fade-in     // Sanftes Einblenden
.slide-up    // Von unten einsliden
.floating    // Schwebender Effekt
```

#### 🔄 **Loading States**
```scss
.loading-overlay    // Vollbild-Overlay
.skeleton          // Skelett-Loading für Content
.loading-spinner   // Rotating Spinner
```

### 🎭 **Theme System**

#### 🌙 **Dark Mode (Standard)**
- Dunkler Hintergrund mit Nostr-Gradienten
- Hoher Kontrast für Lesbarkeit
- Subtile Glowing-Effekte

#### ☀️ **Light Mode (Optional)**
- Heller Hintergrund mit gedämpften Farben
- Behält Nostr-Farbakzente bei

### 📝 **Development Workflow**

#### 🔧 **SCSS Compilation**
```bash
npm run build-css    # Einmalige Kompilierung
npm run watch-css    # Auto-Kompilierung bei Änderungen
npm run dev         # Development mit Watch-Mode
```

#### 📁 **File Structure**
```
styles/
├── variables.scss    # Farben, Größen, Breakpoints
├── mixins.scss      # Wiederverwendbare Mixins & Animationen
├── main.scss        # Hauptstyles & Layout
├── components.scss  # Spezielle Komponenten
└── main.css        # Kompilierte CSS (automatisch)
```

#### 🔧 **Modern SCSS Syntax**
```scss
// Modern @use syntax instead of deprecated @import
@use 'variables' as *;  // Import alle Variablen
@use 'mixins' as *;     // Import alle Mixins
@use 'components';      // Import Komponenten-Styles
```

### 🎯 **Usage Examples**

#### 🔮 **Glass Card mit Gradient-Button**
```html
<div class="card glass">
  <h3 class="text-gradient">Bitcoin Angebot</h3>
  <p>Kaufe Bitcoin in Berlin</p>
  <button class="btn btn-primary">Interessiert</button>
</div>
```

#### 💬 **Chat Message**
```html
<div class="message sent">
  <div class="message-bubble">
    Hallo! Ich interessiere mich für dein Angebot.
  </div>
  <div class="message-time">14:23</div>
</div>
```

#### 🔔 **Notification**
```html
<div class="notification success">
  <div class="notification-content">
    <i class="notification-icon bi bi-check-circle"></i>
    <div class="notification-text">
      <div class="notification-title">Erfolgreich!</div>
      <div class="notification-message">Angebot wurde erstellt</div>
    </div>
  </div>
</div>
```

### 🚀 **Performance Optimizations**

- **Komprimierte CSS** (12KB gzipped)
- **CDN-basierte Fonts** (Inter & JetBrains Mono)
- **Optimierte Animationen** (GPU-accelerated)
- **Mobile-First** Responsive Design

### 🎨 **Visual Hierarchy**

1. **Primary Actions** → Gradient Buttons (Purple→Violet→Pink)
2. **Secondary Actions** → Glass Buttons with Border
3. **Content Cards** → Glass Morphism with subtle shadows
4. **Status Indicators** → Colored dots with pulsing animation
5. **Text Emphasis** → Gradient text for headings

### 🔒 **Accessibility Features**

- **High Contrast** ratios (WCAG 2.1 AA compliant)
- **Focus Rings** mit Nostr-Farben
- **Keyboard Navigation** Support
- **Screen Reader** friendly markup
- **Motion Preferences** berücksichtigt

---

**🌟 Das Design-System kombiniert moderne Web-Trends mit der Nostr-Farbpalette für eine einzigartige, professionelle Bitcoin-Trading-Erfahrung!**
