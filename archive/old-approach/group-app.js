// =====================================
// 🏢 BITCOIN-TAUSCH-GRUPPE APP
// Private NIP-29 Group Management
// =====================================

class BitcoinTauschGroup {
    constructor() {
        this.relays = [
            'wss://relay.damus.io',
            'wss://nostr-relay.online',
            'wss://relay.snort.social'
        ];
        this.currentUser = null;
        this.currentGroup = null;
        this.groupMembers = new Map();
        this.groupMessages = [];
        this.inviteCode = null;
        
        this.initializeApp();
    }

    async initializeApp() {
        console.log('🏢 Initializing Bitcoin-Tausch-Gruppe...');
        
        // Check for invite code in URL
        this.checkForInviteCode();
        
        // Initialize UI
        this.bindEventListeners();
        this.updateConnectionStatus('Verbinde mit Nostr-Netzwerk...', 'connecting');
        
        // Try to restore session
        await this.restoreSession();
    }

    // 🔗 Check for invite code in URL
    checkForInviteCode() {
        const urlParams = new URLSearchParams(window.location.search);
        const hashParams = window.location.hash.substring(1);
        
        // Check for invite code in URL: #join/BTN_PRIV_ABC123
        if (hashParams.startsWith('join/')) {
            this.inviteCode = hashParams.substring(5);
        } else if (urlParams.has('invite')) {
            this.inviteCode = urlParams.get('invite');
        } else {
            // Demo: Use default invite code
            this.inviteCode = 'BTN_DEMO123';
        }

        // Unterscheidung: Private vs. Öffentliche Gruppe
        this.isPrivateGroup = this.inviteCode.includes('_PRIV_');
        
        if (this.isPrivateGroup) {
            this.showPrivateInviteScreen();
        } else {
            this.showPublicInviteScreen();
        }
    }

    // 🔐 Show private invite screen (kein Nickname nötig)
    showPrivateInviteScreen() {
        document.getElementById('inviteScreen').style.display = 'block';
        document.getElementById('groupInterface').style.display = 'none';
        
        // Für private Gruppen: Nickname-Input verstecken
        const nicknameSection = document.querySelector('#inviteScreen .mb-4');
        if (nicknameSection) {
            nicknameSection.style.display = 'none';
        }
        
        // Text anpassen
        const title = document.querySelector('#inviteScreen .h3');
        const subtitle = document.querySelector('#inviteScreen .text-muted');
        if (title) title.textContent = 'Private Bitcoin-Handels-Gruppe';
        if (subtitle) {
            subtitle.innerHTML = `
                Du wurdest zu einer privaten Gruppe eingeladen. 
                <br>Da wir uns bereits kennen, kannst du sofort loslegen! 🚀
            `;
        }
        
        // Button-Text ändern
        const joinBtn = document.getElementById('joinGroupBtn');
        if (joinBtn) {
            joinBtn.innerHTML = '<i class="bi bi-shield-lock"></i> Mit Nostr-Wallet beitreten';
        }
    }

    // 👥 Show public invite screen (mit Nickname)
    showPublicInviteScreen() {
        document.getElementById('inviteScreen').style.display = 'block';
        document.getElementById('groupInterface').style.display = 'none';
        
        // Set focus on nickname input for public groups
        setTimeout(() => {
            const nicknameInput = document.getElementById('nicknameInput');
            if (nicknameInput) nicknameInput.focus();
        }, 100);
    }

        // Update UI with current invite code
        this.updateInviteCodeDisplay();
        
        // Check if user is already member
        const savedGroup = localStorage.getItem('bitcoinGroup');
        if (savedGroup && this.inviteCode !== 'BTN_DEMO123') {
            this.showGroupInterface();
        } else {
            this.showInviteScreen();
        }
    }

    // 📱 Update invite code display
    updateInviteCodeDisplay() {
        const currentInviteCode = document.getElementById('currentInviteCode');
        const displayInviteCode = document.getElementById('displayInviteCode');
        const fullInviteLink = document.getElementById('fullInviteLink');
        
        if (currentInviteCode) currentInviteCode.textContent = this.inviteCode;
        if (displayInviteCode) displayInviteCode.textContent = this.inviteCode;
        if (fullInviteLink) {
            const baseUrl = window.location.origin + window.location.pathname;
            fullInviteLink.textContent = `${baseUrl}#join/${this.inviteCode}`;
        }
    }

    // 📱 Show invite screen
    showInviteScreen() {
        document.getElementById('inviteScreen').style.display = 'block';
        document.getElementById('groupInterface').style.display = 'none';
        
        // Set focus on nickname input
        setTimeout(() => {
            document.getElementById('nicknameInput').focus();
        }, 100);
    }

    // 🏠 Show group interface
    showGroupInterface() {
        document.getElementById('inviteScreen').style.display = 'none';
        document.getElementById('groupInterface').style.display = 'block';
        
        this.loadGroupData();
    }

    // 🔐 Bind event listeners
    bindEventListeners() {
        // Copy demo link
        const copyDemoLink = document.getElementById('copyDemoLink');
        if (copyDemoLink) {
            copyDemoLink.onclick = () => this.copyDemoLink();
        }

        // Wallet login options
        const walletLoginBtn = document.getElementById('walletLoginBtn');
        const amberLoginBtn = document.getElementById('amberLoginBtn');
        const newKeyBtn = document.getElementById('newKeyBtn');
        
        if (walletLoginBtn) {
            walletLoginBtn.onclick = () => this.loginWithExtension();
        }
        if (amberLoginBtn) {
            amberLoginBtn.onclick = () => this.loginWithAmber();
        }
        if (newKeyBtn) {
            newKeyBtn.onclick = () => this.createNewIdentity();
        }

        // Join group
        const joinGroupBtn = document.getElementById('joinGroupBtn');
        if (joinGroupBtn) {
            joinGroupBtn.onclick = () => this.joinGroup();
        }

        // Send introduction
        const sendIntroBtn = document.getElementById('sendIntroductionBtn');
        if (sendIntroBtn) {
            sendIntroBtn.onclick = () => this.sendIntroduction();
        }

        // Send message
        const sendMessageBtn = document.getElementById('sendMessageBtn');
        const messageInput = document.getElementById('messageInput');
        if (sendMessageBtn) {
            sendMessageBtn.onclick = () => this.sendMessage();
        }
        if (messageInput) {
            messageInput.onkeypress = (e) => {
                if (e.key === 'Enter') this.sendMessage();
            };
        }

        // Create invite link
        const createInviteBtn = document.getElementById('createInviteBtn');
        if (createInviteBtn) {
            createInviteBtn.onclick = () => this.createInviteLink();
        }

        // Copy invite link
        const copyInviteLinkBtn = document.getElementById('copyInviteLinkBtn');
        if (copyInviteLinkBtn) {
            copyInviteLinkBtn.onclick = () => this.copyInviteLink();
        }

        // Nickname input validation
        const nicknameInput = document.getElementById('nicknameInput');
        if (nicknameInput) {
            nicknameInput.oninput = () => this.validateNickname();
        }
    }

    // 🚀 Join group with invite code
    async joinGroup() {
        const nicknameInput = document.getElementById('nicknameInput');
        const nickname = nicknameInput.value.trim();

        if (!nickname) {
            this.showNotification('Bitte gib einen Spitznamen ein', 'error');
            return;
        }

        if (!this.currentUser) {
            this.showNotification('Bitte melde dich zuerst an', 'error');
            return;
        }

        if (!this.inviteCode) {
            this.showNotification('Kein gültiger Invite-Code gefunden', 'error');
            return;
        }

        try {
            // Simulate joining group (in real implementation, this would call Rust backend)
            const groupId = await this.simulateGroupJoin(this.inviteCode, nickname);
            
            this.currentGroup = {
                id: groupId,
                name: 'Bitcoin-Tausch-Gruppe',
                memberCount: 12,
                tradingEnabled: false,
                userRole: 'member',
                userVerified: false,
                userIntroduced: false
            };

            // Save to localStorage
            localStorage.setItem('bitcoinGroup', JSON.stringify(this.currentGroup));
            localStorage.setItem('userNickname', nickname);

            this.showGroupInterface();
            this.showNotification('Erfolgreich der Gruppe beigetreten!', 'success');
            
        } catch (error) {
            console.error('Error joining group:', error);
            this.showNotification('Fehler beim Beitreten der Gruppe', 'error');
        }
    }

    // 📤 Send introduction
    async sendIntroduction() {
        const introText = document.getElementById('introductionText').value.trim();
        
        if (!introText) {
            this.showNotification('Bitte schreibe eine Vorstellung', 'error');
            return;
        }

        if (introText.length < 50) {
            this.showNotification('Deine Vorstellung sollte mindestens 50 Zeichen haben', 'error');
            return;
        }

        try {
            // Add introduction message to chat
            this.addMessageToChat({
                id: Date.now().toString(),
                sender: this.currentUser.pubkey,
                senderName: localStorage.getItem('userNickname'),
                content: `👋 ${introText}`,
                type: 'introduction',
                timestamp: Date.now()
            });

            // Hide introduction panel
            document.getElementById('introductionPanel').style.display = 'none';
            
            // Update user status
            this.currentGroup.userIntroduced = true;
            localStorage.setItem('bitcoinGroup', JSON.stringify(this.currentGroup));

            this.showNotification('Vorstellung gesendet! Warte auf Admin-Verifikation.', 'success');
            
        } catch (error) {
            console.error('Error sending introduction:', error);
            this.showNotification('Fehler beim Senden der Vorstellung', 'error');
        }
    }

    // 💬 Send group message
    async sendMessage() {
        const messageInput = document.getElementById('messageInput');
        const content = messageInput.value.trim();

        if (!content) return;

        try {
            const message = {
                id: Date.now().toString(),
                sender: this.currentUser.pubkey,
                senderName: localStorage.getItem('userNickname'),
                content: content,
                type: 'general',
                timestamp: Date.now()
            };

            this.addMessageToChat(message);
            messageInput.value = '';
            
        } catch (error) {
            console.error('Error sending message:', error);
            this.showNotification('Fehler beim Senden der Nachricht', 'error');
        }
    }

    // 💬 Add message to chat display
    addMessageToChat(message) {
        const messagesContainer = document.getElementById('groupMessages');
        const messageEl = document.createElement('div');
        messageEl.className = 'message mb-3';

        const isOwnMessage = message.sender === this.currentUser?.pubkey;
        const messageClass = isOwnMessage ? 'own-message' : 'other-message';

        let typeIcon = '';
        switch (message.type) {
            case 'introduction':
                typeIcon = '<i class="bi bi-person-waves text-primary"></i> ';
                break;
            case 'verification':
                typeIcon = '<i class="bi bi-patch-check text-success"></i> ';
                break;
            case 'announcement':
                typeIcon = '<i class="bi bi-megaphone text-warning"></i> ';
                break;
            default:
                typeIcon = '';
        }

        messageEl.innerHTML = `
            <div class="${messageClass}">
                <div class="message-header d-flex justify-content-between align-items-center mb-1">
                    <div class="d-flex align-items-center gap-2">
                        ${typeIcon}
                        <strong class="text-sm">${message.senderName || 'Unbekannt'}</strong>
                        ${this.getMemberBadge(message.sender)}
                    </div>
                    <span class="text-muted text-xs">${this.formatTime(message.timestamp)}</span>
                </div>
                <div class="message-content">
                    ${this.formatMessageContent(message.content)}
                </div>
            </div>
        `;

        messagesContainer.appendChild(messageEl);
        messagesContainer.scrollTop = messagesContainer.scrollHeight;

        this.groupMessages.push(message);
    }

    // 🏷️ Get member badge
    getMemberBadge(pubkey) {
        if (pubkey === 'admin') {
            return '<span class="badge bg-danger text-xs">Admin</span>';
        }
        // In real implementation, check if member is verified
        const isVerified = Math.random() > 0.5; // Simulate
        return isVerified ? '<span class="badge bg-success text-xs">✓</span>' : '<span class="badge bg-secondary text-xs">Neu</span>';
    }

    // 📝 Format message content
    formatMessageContent(content) {
        // Basic markdown-like formatting
        return content
            .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
            .replace(/\*(.*?)\*/g, '<em>$1</em>')
            .replace(/\n/g, '<br>');
    }

    // 🕐 Format timestamp
    formatTime(timestamp) {
        const date = new Date(timestamp);
        const now = new Date();
        const diffMinutes = Math.floor((now - date) / 60000);

        if (diffMinutes < 1) return 'gerade eben';
        if (diffMinutes < 60) return `${diffMinutes}m`;
        if (diffMinutes < 1440) return `${Math.floor(diffMinutes / 60)}h`;
        return date.toLocaleDateString('de-DE');
    }

    // 🔗 Create invite link
    async createInviteLink() {
        try {
            // Generate invite code (in real implementation, call backend)
            const inviteCode = 'BTN_' + Math.random().toString(36).substring(2, 10).toUpperCase();
            const inviteLink = `${window.location.origin}${window.location.pathname}#join/${inviteCode}`;

            document.getElementById('inviteLinkInput').value = inviteLink;
            document.getElementById('inviteLinkCard').style.display = 'block';

            this.showNotification('Invite-Link erstellt!', 'success');

        } catch (error) {
            console.error('Error creating invite link:', error);
            this.showNotification('Fehler beim Erstellen des Invite-Links', 'error');
        }
    }

    // 📋 Copy invite link
    async copyInviteLink() {
        const inviteLinkInput = document.getElementById('inviteLinkInput');
        
        try {
            await navigator.clipboard.writeText(inviteLinkInput.value);
            this.showNotification('Link kopiert!', 'success');
        } catch (error) {
            // Fallback for older browsers
            inviteLinkInput.select();
            document.execCommand('copy');
            this.showNotification('Link kopiert!', 'success');
        }
    }

    // ✅ Validate nickname
    validateNickname() {
        const nicknameInput = document.getElementById('nicknameInput');
        const joinBtn = document.getElementById('joinGroupBtn');
        const nickname = nicknameInput.value.trim();

        const isValid = nickname.length >= 3 && nickname.length <= 30 && /^[a-zA-Z0-9_-]+$/.test(nickname);
        
        if (joinBtn) {
            joinBtn.disabled = !isValid || !this.currentUser;
        }

        if (nickname.length > 0 && !isValid) {
            nicknameInput.classList.add('is-invalid');
        } else {
            nicknameInput.classList.remove('is-invalid');
        }
    }

    // 🔐 Show login options
    async showLoginOptions() {
        if (window.nostr) {
            await this.loginWithExtension();
        } else {
            // Show nostr-login modal
            this.showNotification('Bitte installiere eine Nostr-Wallet (z.B. Alby)', 'info');
        }
    }

    // ⚡ Login with browser extension
    async loginWithExtension() {
        try {
            if (window.nostr) {
                const pubkey = await window.nostr.getPublicKey();
                this.currentUser = { pubkey };
                this.showLoginSuccess('Browser-Extension');
            } else {
                this.showNotification('Keine Nostr-Extension gefunden. Bitte installiere Alby oder nos2x.', 'error');
            }
        } catch (error) {
            console.error('Extension login error:', error);
            this.showNotification('Fehler beim Login mit Extension', 'error');
        }
    }

    // 📱 Login with Amber (mobile)
    async loginWithAmber() {
        try {
            // Simulate Amber login (in real implementation, use nostr-login)
            this.showNotification('Amber-Login wird simuliert...', 'info');
            
            setTimeout(() => {
                // Simulate successful login
                this.currentUser = { 
                    pubkey: 'amber_' + Math.random().toString(36).substring(2, 9),
                    source: 'amber'
                };
                this.showLoginSuccess('Amber (Mobile)');
            }, 2000);
            
        } catch (error) {
            console.error('Amber login error:', error);
            this.showNotification('Fehler beim Amber-Login', 'error');
        }
    }

    // 🔑 Create new identity
    async createNewIdentity() {
        try {
            // Generate new keys (in real implementation, use nostr-tools)
            const newPubkey = 'new_' + Math.random().toString(36).substring(2, 15);
            this.currentUser = { 
                pubkey: newPubkey,
                source: 'generated',
                isNew: true
            };
            
            this.showLoginSuccess('Neue Identität');
            this.showNotification('Neue Nostr-Identität erstellt! Schlüssel sicher gespeichert.', 'success');
            
        } catch (error) {
            console.error('Key generation error:', error);
            this.showNotification('Fehler beim Erstellen der Identität', 'error');
        }
    }

    // ✅ Show login success and advance to next step
    showLoginSuccess(method) {
        // Hide wallet step, show nickname step
        document.getElementById('walletStep').style.display = 'none';
        document.getElementById('nicknameStep').style.display = 'block';
        document.getElementById('joinStep').style.display = 'block';
        
        // Update connection status
        this.updateConnectionStatus(`Angemeldet via ${method}`, 'online');
        
        // Focus nickname input
        setTimeout(() => {
            document.getElementById('nicknameInput').focus();
        }, 100);
        
        this.showNotification(`Erfolgreich angemeldet via ${method}!`, 'success');
    }

    // 📋 Copy demo link
    async copyDemoLink() {
        const fullInviteLink = document.getElementById('fullInviteLink');
        const linkText = fullInviteLink.textContent;
        
        try {
            await navigator.clipboard.writeText(linkText);
            this.showNotification('Demo-Link kopiert!', 'success');
        } catch (error) {
            // Fallback
            console.log('Link to copy:', linkText);
            this.showNotification('Link in Console geloggt', 'info');
        }
    }

    // 🔄 Update UI after login
    updateUIAfterLogin() {
        const loginBtn = document.getElementById('loginBtn');
        if (loginBtn) {
            loginBtn.innerHTML = '<i class="bi bi-person-check"></i> Angemeldet';
            loginBtn.disabled = true;
            loginBtn.classList.remove('btn-primary');
            loginBtn.classList.add('btn-success');
        }

        this.validateNickname(); // Re-validate to enable join button
        this.updateConnectionStatus('Verbunden', 'online');
    }

    // 📊 Load group data
    async loadGroupData() {
        this.updateGroupStats();
        this.loadMembers();
        this.loadMessages();
        this.checkIntroductionStatus();
    }

    // 📈 Update group statistics
    updateGroupStats() {
        if (!this.currentGroup) return;

        document.getElementById('groupName').textContent = this.currentGroup.name;
        document.getElementById('memberCount').textContent = this.currentGroup.memberCount;
        document.getElementById('tradingStatus').textContent = 
            this.currentGroup.tradingEnabled ? 'Handel aktiv' : 'Kennenlernphase';

        const groupStats = document.getElementById('groupStats');
        if (groupStats) {
            groupStats.innerHTML = `
                ${this.currentGroup.memberCount} Mitglieder • 
                ${this.currentGroup.tradingEnabled ? 'Handel aktiv' : 'Kennenlernphase'}
            `;
        }
    }

    // 👥 Load group members
    loadMembers() {
        const membersList = document.getElementById('membersList');
        if (!membersList) return;

        // Simulate members data
        const members = [
            { name: 'Admin', verified: true, isAdmin: true, online: true },
            { name: 'BitcoinMike', verified: true, isAdmin: false, online: true },
            { name: 'SatoshiFan', verified: true, isAdmin: false, online: false },
            { name: 'NewUser123', verified: false, isAdmin: false, online: true },
        ];

        membersList.innerHTML = members.map(member => `
            <div class="d-flex align-items-center justify-content-between py-2">
                <div class="d-flex align-items-center gap-2">
                    <div class="status-dot ${member.online ? 'online' : 'offline'}"></div>
                    <span class="text-sm">${member.name}</span>
                    ${member.isAdmin ? '<span class="badge bg-danger text-xs">Admin</span>' : ''}
                    ${member.verified ? '<span class="badge bg-success text-xs">✓</span>' : '<span class="badge bg-secondary text-xs">Neu</span>'}
                </div>
            </div>
        `).join('');
    }

    // 💬 Load group messages
    loadMessages() {
        // Load some demo messages
        const demoMessages = [
            {
                id: '1',
                sender: 'admin',
                senderName: 'Admin',
                content: '🎉 Willkommen in der Bitcoin-Tausch-Gruppe! Bitte stellt euch alle vor.',
                type: 'announcement',
                timestamp: Date.now() - 86400000 // 1 day ago
            },
            {
                id: '2',
                sender: 'user1',
                senderName: 'BitcoinMike',
                content: '👋 Hallo! Ich bin Mike aus München und handle seit 3 Jahren mit Bitcoin. Freue mich auf den Austausch!',
                type: 'introduction',
                timestamp: Date.now() - 43200000 // 12 hours ago
            }
        ];

        demoMessages.forEach(message => this.addMessageToChat(message));
    }

    // 📝 Check introduction status
    checkIntroductionStatus() {
        if (!this.currentGroup) return;

        const introPanel = document.getElementById('introductionPanel');
        if (introPanel && !this.currentGroup.userIntroduced) {
            introPanel.style.display = 'block';
        }

        const tradingPanel = document.getElementById('tradingPanel');
        if (tradingPanel) {
            tradingPanel.style.display = 'block';
            
            if (this.currentGroup.tradingEnabled && this.currentGroup.userVerified) {
                document.getElementById('tradingDisabled').style.display = 'none';
                document.getElementById('tradingEnabled').style.display = 'block';
            }
        }
    }

    // 💾 Restore session
    async restoreSession() {
        const savedGroup = localStorage.getItem('bitcoinGroup');
        if (savedGroup) {
            this.currentGroup = JSON.parse(savedGroup);
            this.showGroupInterface();
        }
    }

    // 🔄 Check existing membership
    checkExistingMembership() {
        // If no invite code and no saved group, redirect to main site
        const savedGroup = localStorage.getItem('bitcoinGroup');
        if (!savedGroup) {
            window.location.href = 'index.html';
        }
    }

    // 📊 Update connection status
    updateConnectionStatus(message, status) {
        const statusEl = document.getElementById('connectionStatus');
        if (statusEl) {
            const statusDot = statusEl.querySelector('.status-dot');
            const statusText = statusEl.querySelector('span');
            
            if (statusDot && statusText) {
                statusText.textContent = message;
                statusDot.className = `status-dot ${status}`;
            }
        }
    }

    // 🔔 Show notification
    showNotification(message, type = 'info') {
        const container = document.getElementById('notificationsContainer');
        if (!container) return;

        const notification = document.createElement('div');
        notification.className = `alert alert-${type === 'error' ? 'danger' : type} alert-dismissible fade show`;
        notification.style.position = 'fixed';
        notification.style.top = '20px';
        notification.style.right = '20px';
        notification.style.zIndex = '9999';
        notification.style.maxWidth = '400px';

        notification.innerHTML = `
            ${message}
            <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
        `;

        container.appendChild(notification);

        // Auto-remove after 5 seconds
        setTimeout(() => {
            if (notification.parentNode) {
                notification.remove();
            }
        }, 5000);
    }

    // 🎲 Simulate group join (for demo)
    async simulateGroupJoin(inviteCode, nickname) {
        // Simulate API call delay
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        // Validate invite code format
        if (!inviteCode.startsWith('BTN_')) {
            throw new Error('Ungültiger Invite-Code');
        }

        return 'group_' + Math.random().toString(36).substring(2, 9);
    }
}

// 🚀 Initialize app when page loads
let groupApp;
document.addEventListener('DOMContentLoaded', () => {
    groupApp = new BitcoinTauschGroup();
});
