// Bitcoin-Tausch-Netzwerk Web Interface
// Nostr Integration mit nostr-tools

class BitcoinTauschApp {
    constructor() {
        this.relay = null;
        this.privateKey = null;
        this.publicKey = null;
        this.relayUrl = 'wss://nostr-relay.online';
        this.offers = new Map();
        this.conversations = new Map();
        this.unreadCount = 0;
        
        this.initializeApp();
    }

    async initializeApp() {
        console.log('🚀 Initializing Bitcoin-Tausch-Netzwerk...');
        
        // Initialize nostr-login
        await this.initNostrLogin();
        
        // Bind event listeners
        this.bindEventListeners();
        
        // Check for saved session
        this.checkSavedSession();
    }

    async initNostrLogin() {
        try {
            // Initialize nostr-login with options
            if (window.NostrLogin) {
                this.nostrLogin = new window.NostrLogin({
                    // Optional: customize the login methods
                    methods: ['extension', 'connect', 'readOnly'],
                    // Optional: theme
                    theme: 'default'
                });
                console.log('🔐 nostr-login initialized');
            }
        } catch (error) {
            console.error('❌ Failed to initialize nostr-login:', error);
        }
    }

    checkAmberSupport() {
        if (window.nostr && window.nostr.getPublicKey) {
            document.getElementById('amber-btn').style.display = 'block';
            console.log('📱 Amber support detected');
        }
    }

    bindEventListeners() {
        // Login buttons
        document.getElementById('nostr-login-btn').onclick = () => this.loginWithNostrLogin();
        document.getElementById('generate-key-btn').onclick = () => this.generateNewKey();
        document.getElementById('login-btn').onclick = () => this.loginWithKey();
        document.getElementById('logout-btn').onclick = () => this.logout();

        // Offer creation
        document.getElementById('create-offer-btn').onclick = () => this.createOffer();
        document.getElementById('refresh-offers-btn').onclick = () => this.loadOffers();

        // Auto-calculate EUR/BTC conversion
        const btcInput = document.getElementById('btc-amount');
        const eurInput = document.getElementById('eur-amount');
        
        btcInput.oninput = () => {
            const btc = parseFloat(btcInput.value);
            const eur = parseFloat(eurInput.value);
            if (btc && eur) {
                const price = (eur / btc).toFixed(0);
                console.log(`💰 BTC Price: ${price}€/BTC`);
            }
        };
    }

    checkSavedSession() {
        const savedKey = localStorage.getItem('btc-tausch-nsec');
        if (savedKey) {
            try {
                this.privateKey = savedKey;
                this.publicKey = NostrTools.getPublicKey(savedKey);
                this.showMainInterface();
                this.connectToRelay();
            } catch (error) {
                console.error('❌ Invalid saved key:', error);
                localStorage.removeItem('btc-tausch-nsec');
            }
        }
    }

    async generateNewKey() {
        try {
            console.log('🔑 Generating new keypair...');
            this.privateKey = NostrTools.generatePrivateKey();
            this.publicKey = NostrTools.getPublicKey(this.privateKey);
            
            // Save to localStorage
            localStorage.setItem('btc-tausch-nsec', this.privateKey);
            
            this.showMainInterface();
            await this.connectToRelay();
            
            // Show success message
            this.showStatus('✅ Neue Identität erstellt! Sie sind jetzt verbunden.', 'success');
        } catch (error) {
            console.error('❌ Key generation failed:', error);
            this.showStatus('❌ Fehler bei der Schlüsselgenerierung', 'error');
        }
    }

    async loginWithKey() {
        const nsecInput = document.getElementById('nsec-input');
        const nsec = nsecInput.value.trim();
        
        if (!nsec) {
            this.showStatus('⚠️ Bitte geben Sie einen nsec-Schlüssel ein', 'warning');
            return;
        }

        try {
            // Convert nsec to hex if needed
            let privateKey;
            if (nsec.startsWith('nsec1')) {
                privateKey = NostrTools.nip19.decode(nsec).data;
            } else {
                privateKey = nsec;
            }

            this.privateKey = privateKey;
            this.publicKey = NostrTools.getPublicKey(privateKey);
            
            // Save to localStorage
            localStorage.setItem('btc-tausch-nsec', this.privateKey);
            
            this.showMainInterface();
            await this.connectToRelay();
            
            this.showStatus('✅ Erfolgreich angemeldet!', 'success');
        } catch (error) {
            console.error('❌ Login failed:', error);
            this.showStatus('❌ Ungültiger Schlüssel', 'error');
        }
    }

    async loginWithNostrLogin() {
        try {
            if (!this.nostrLogin && window.NostrLogin) {
                this.nostrLogin = new window.NostrLogin();
            }

            if (!this.nostrLogin) {
                // Fallback to manual NIP-07 detection
                await this.loginWithNip07();
                return;
            }

            console.log('� Starting nostr-login authentication...');
            
            // Launch the nostr-login modal
            const loginResult = await this.nostrLogin.launch();
            
            if (loginResult && loginResult.pubkey) {
                this.publicKey = loginResult.pubkey;
                this.privateKey = 'nip07'; // Special marker for NIP-07 signing
                this.nip07Signer = loginResult; // Store the signer
                
                this.showMainInterface();
                await this.connectToRelay();
                
                this.showStatus('✅ Erfolgreich mit Nostr-Wallet angemeldet!', 'success');
            } else {
                throw new Error('Login cancelled or failed');
            }
        } catch (error) {
            console.error('❌ nostr-login failed:', error);
            // Fallback to NIP-07
            await this.loginWithNip07();
        }
    }

    async loginWithNip07() {
        try {
            // Direct NIP-07 check
            if (!window.nostr) {
                this.showStatus('❌ Keine Nostr-Wallet gefunden. Bitte installieren Sie Alby, nos2x oder eine andere NIP-07 kompatible Wallet.', 'error');
                return;
            }

            console.log('🔐 Connecting with NIP-07 wallet...');
            this.publicKey = await window.nostr.getPublicKey();
            this.privateKey = 'nip07'; // Special marker
            
            this.showMainInterface();
            await this.connectToRelay();
            
            this.showStatus('✅ Mit Nostr-Wallet verbunden!', 'success');
        } catch (error) {
            console.error('❌ NIP-07 connection failed:', error);
            this.showStatus('❌ Wallet-Verbindung fehlgeschlagen. Bitte erlauben Sie den Zugriff in Ihrer Wallet.', 'error');
        }
    }

    logout() {
        localStorage.removeItem('btc-tausch-nsec');
        this.privateKey = null;
        this.publicKey = null;
        
        if (this.relay) {
            this.relay.close();
            this.relay = null;
        }
        
        // Show login interface
        document.getElementById('login-section').style.display = 'block';
        document.getElementById('main-interface').style.display = 'none';
        document.getElementById('connection-status').style.display = 'none';
        
        this.showStatus('👋 Erfolgreich abgemeldet', 'info');
    }

    showMainInterface() {
        document.getElementById('login-section').style.display = 'none';
        document.getElementById('main-interface').style.display = 'block';
        
        // Show user info
        const npub = NostrTools.nip19.npubEncode(this.publicKey);
        document.getElementById('user-npub').textContent = npub;
    }

    async connectToRelay() {
        try {
            console.log('🔗 Connecting to relay:', this.relayUrl);
            document.getElementById('connection-status').style.display = 'block';
            document.getElementById('connection-status').textContent = '🔄 Verbinde zu Relay...';
            
            this.relay = NostrTools.relayInit(this.relayUrl);
            
            this.relay.on('connect', () => {
                console.log('✅ Connected to relay');
                document.getElementById('connection-status').style.display = 'none';
                this.startListening();
                this.loadOffers();
            });

            this.relay.on('error', () => {
                console.error('❌ Relay connection error');
                document.getElementById('connection-status').textContent = '❌ Relay-Verbindung fehlgeschlagen';
                document.getElementById('connection-status').className = 'bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4';
            });

            await this.relay.connect();
        } catch (error) {
            console.error('❌ Connection failed:', error);
            this.showStatus('❌ Relay-Verbindung fehlgeschlagen', 'error');
        }
    }

    startListening() {
        if (!this.relay) return;

        // Listen for offers (kind 1 with #BitcoinTausch tag)
        const offerSub = this.relay.sub([
            {
                kinds: [1],
                '#t': ['BitcoinTausch'],
                limit: 50
            }
        ]);

        offerSub.on('event', (event) => {
            this.processOfferEvent(event);
        });

        // Listen for private messages to us
        const dmSub = this.relay.sub([
            {
                kinds: [4],
                '#p': [this.publicKey],
                limit: 50
            }
        ]);

        dmSub.on('event', (event) => {
            this.processDirectMessage(event);
        });

        console.log('👂 Started listening for events');
    }

    processOfferEvent(event) {
        try {
            // Parse offer from event content
            const offer = JSON.parse(event.content);
            offer.eventId = event.id;
            offer.pubkey = event.pubkey;
            offer.created_at = event.created_at;
            
            this.offers.set(event.id, offer);
            this.updateOffersDisplay();
        } catch (error) {
            console.error('❌ Failed to process offer:', error);
        }
    }

    async processDirectMessage(event) {
        try {
            // Decrypt the message
            let decryptedContent;
            if (this.privateKey === 'nip07') {
                // Use NIP-07 wallet for decryption
                decryptedContent = await window.nostr.nip04.decrypt(event.pubkey, event.content);
            } else {
                // Use local private key
                decryptedContent = await NostrTools.nip04.decrypt(this.privateKey, event.pubkey, event.content);
            }

            const message = JSON.parse(decryptedContent);
            message.eventId = event.id;
            message.timestamp = event.created_at;
            
            // Add to conversations
            const conversationId = this.getConversationId(event.pubkey);
            if (!this.conversations.has(conversationId)) {
                this.conversations.set(conversationId, []);
            }
            this.conversations.get(conversationId).push(message);
            
            this.unreadCount++;
            this.updateMessagesDisplay();
        } catch (error) {
            console.error('❌ Failed to process DM:', error);
        }
    }

    async createOffer() {
        const type = document.getElementById('offer-type').value;
        const btcAmount = parseFloat(document.getElementById('btc-amount').value);
        const eurAmount = parseFloat(document.getElementById('eur-amount').value);
        const validityHours = parseInt(document.getElementById('validity-hours').value);

        if (!btcAmount || !eurAmount || !validityHours) {
            this.showStatus('⚠️ Bitte füllen Sie alle Felder aus', 'warning');
            return;
        }

        try {
            const offer = {
                offer_id: this.generateId(),
                pseudo_id: `anon_${this.generateId().substring(0, 8)}`,
                amount_btc: btcAmount,
                amount_eur: eurAmount,
                offer_type: type,
                status: 'Active',
                created_at: Math.floor(Date.now() / 1000),
                expires_at: Math.floor(Date.now() / 1000) + (validityHours * 3600),
                interested_users: []
            };

            const event = {
                kind: 1,
                pubkey: this.publicKey,
                created_at: Math.floor(Date.now() / 1000),
                tags: [['t', 'BitcoinTausch']],
                content: JSON.stringify(offer)
            };

            let signedEvent;
            if (this.privateKey === 'nip07') {
                signedEvent = await window.nostr.signEvent(event);
            } else {
                signedEvent = NostrTools.finishEvent(event, this.privateKey);
            }

            await this.relay.publish(signedEvent);
            
            this.showStatus('✅ Angebot erfolgreich veröffentlicht!', 'success');
            
            // Clear form
            document.getElementById('btc-amount').value = '';
            document.getElementById('eur-amount').value = '';
            
            // Refresh offers
            setTimeout(() => this.loadOffers(), 1000);
        } catch (error) {
            console.error('❌ Failed to create offer:', error);
            this.showStatus('❌ Fehler beim Erstellen des Angebots', 'error');
        }
    }

    async sendInterest(offerId, offerPubkey) {
        try {
            const message = {
                message_id: this.generateId(),
                sender_pubkey: this.publicKey,
                recipient_pubkey: offerPubkey,
                content: `🎯 Interesse an Angebot ${offerId}. Können wir die Details besprechen?`,
                offer_id: offerId,
                message_type: 'Interest',
                timestamp: Math.floor(Date.now() / 1000),
                read: false
            };

            const messageJson = JSON.stringify(message);
            
            let encryptedContent;
            if (this.privateKey === 'nip07') {
                encryptedContent = await window.nostr.nip04.encrypt(offerPubkey, messageJson);
            } else {
                encryptedContent = await NostrTools.nip04.encrypt(this.privateKey, offerPubkey, messageJson);
            }

            const dmEvent = {
                kind: 4,
                pubkey: this.publicKey,
                created_at: Math.floor(Date.now() / 1000),
                tags: [['p', offerPubkey]],
                content: encryptedContent
            };

            let signedEvent;
            if (this.privateKey === 'nip07') {
                signedEvent = await window.nostr.signEvent(dmEvent);
            } else {
                signedEvent = NostrTools.finishEvent(dmEvent, this.privateKey);
            }

            await this.relay.publish(signedEvent);
            
            this.showStatus('✅ Interesse gesendet!', 'success');
        } catch (error) {
            console.error('❌ Failed to send interest:', error);
            this.showStatus('❌ Fehler beim Senden', 'error');
        }
    }

    loadOffers() {
        // The offers are automatically loaded through the subscription
        this.updateOffersDisplay();
    }

    updateOffersDisplay() {
        const offersList = document.getElementById('offers-list');
        
        if (this.offers.size === 0) {
            offersList.innerHTML = '<p class="text-gray-500 text-center py-8">Keine Angebote gefunden</p>';
            return;
        }

        const offersArray = Array.from(this.offers.values())
            .sort((a, b) => b.created_at - a.created_at);

        offersList.innerHTML = offersArray.map(offer => {
            const isOwnOffer = offer.pubkey === this.publicKey;
            const price = (offer.amount_eur / offer.amount_btc).toFixed(0);
            const timeLeft = this.getTimeLeft(offer.expires_at);
            
            return `
                <div class="offer-card border border-gray-200 rounded-lg p-4 ${isOwnOffer ? 'bg-blue-50' : 'bg-white'}">
                    <div class="flex justify-between items-start mb-2">
                        <div class="flex items-center space-x-2">
                            <span class="text-lg font-semibold ${offer.offer_type === 'BUY' ? 'text-green-600' : 'text-red-600'}">
                                ${offer.offer_type === 'BUY' ? '🟢 KAUFE' : '🔴 VERKAUFE'}
                            </span>
                            <span class="font-mono text-sm text-gray-600">${offer.pseudo_id}</span>
                        </div>
                        <span class="text-xs text-gray-500">${timeLeft}</span>
                    </div>
                    
                    <div class="mb-3">
                        <p class="text-lg font-semibold">${offer.amount_btc} BTC für ${offer.amount_eur}€</p>
                        <p class="text-sm text-gray-600">Preis: ${price}€/BTC</p>
                    </div>
                    
                    ${!isOwnOffer ? `
                        <button onclick="app.sendInterest('${offer.offer_id}', '${offer.pubkey}')" 
                                class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 rounded transition-colors">
                            💬 Interesse zeigen
                        </button>
                    ` : `
                        <div class="text-center text-sm text-blue-600 font-semibold">📝 Ihr Angebot</div>
                    `}
                </div>
            `;
        }).join('');
    }

    updateMessagesDisplay() {
        const messagesList = document.getElementById('messages-list');
        const unreadCountEl = document.getElementById('unread-count');
        
        if (this.conversations.size === 0) {
            messagesList.innerHTML = '<p class="text-gray-500 text-center py-8">Keine Nachrichten</p>';
            unreadCountEl.style.display = 'none';
            return;
        }

        // Update unread count
        if (this.unreadCount > 0) {
            unreadCountEl.textContent = this.unreadCount;
            unreadCountEl.style.display = 'inline';
        } else {
            unreadCountEl.style.display = 'none';
        }

        // Display conversations
        const conversationsHtml = Array.from(this.conversations.entries()).map(([conversationId, messages]) => {
            const lastMessage = messages[messages.length - 1];
            const unreadInConv = messages.filter(m => !m.read).length;
            
            return `
                <div class="border border-gray-200 rounded-lg p-4 ${unreadInConv > 0 ? 'message-unread' : ''}">
                    <div class="flex justify-between items-start mb-2">
                        <span class="font-semibold">Conversation ${conversationId.substring(0, 8)}</span>
                        ${unreadInConv > 0 ? `<span class="bg-red-500 text-white px-2 py-1 rounded-full text-xs">${unreadInConv}</span>` : ''}
                    </div>
                    <p class="text-sm text-gray-600 mb-2">${lastMessage.content.substring(0, 100)}...</p>
                    <div class="flex justify-between items-center text-xs text-gray-500">
                        <span>${new Date(lastMessage.timestamp * 1000).toLocaleString()}</span>
                        <span>${messages.length} Nachrichten</span>
                    </div>
                </div>
            `;
        }).join('');

        messagesList.innerHTML = conversationsHtml;
    }

    // Utility functions
    generateId() {
        return Array.from(crypto.getRandomValues(new Uint8Array(16)))
            .map(b => b.toString(16).padStart(2, '0'))
            .join('');
    }

    getConversationId(pubkey) {
        const keys = [this.publicKey, pubkey].sort();
        return keys[0].substring(0, 8) + '_' + keys[1].substring(0, 8);
    }

    getTimeLeft(expiresAt) {
        const now = Math.floor(Date.now() / 1000);
        const diff = expiresAt - now;
        
        if (diff <= 0) return 'Abgelaufen';
        
        const hours = Math.floor(diff / 3600);
        const minutes = Math.floor((diff % 3600) / 60);
        
        if (hours > 0) {
            return `${hours}h ${minutes}m`;
        } else {
            return `${minutes}m`;
        }
    }

    showStatus(message, type = 'info') {
        const statusEl = document.getElementById('connection-status');
        statusEl.textContent = message;
        statusEl.style.display = 'block';
        
        // Set colors based on type
        switch (type) {
            case 'success':
                statusEl.className = 'bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded mb-4';
                break;
            case 'error':
                statusEl.className = 'bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4';
                break;
            case 'warning':
                statusEl.className = 'bg-yellow-100 border border-yellow-400 text-yellow-700 px-4 py-3 rounded mb-4';
                break;
            default:
                statusEl.className = 'bg-blue-100 border border-blue-400 text-blue-700 px-4 py-3 rounded mb-4';
        }
        
        // Auto-hide after 5 seconds
        setTimeout(() => {
            statusEl.style.display = 'none';
        }, 5000);
    }
}

// Initialize app when page loads
let app;
document.addEventListener('DOMContentLoaded', () => {
    app = new BitcoinTauschApp();
});
