/**
 * 🔐 NSEC-LOGIN CONFIGURATION
 * Bitcoin-Tausch-Netzwerk - Private Group Access Control
 */

// 👥 AUTHORIZED MEMBERS (npub format)
// Nur diese Nostr Public Keys haben Zugang zur Bitcoin-Handels-Gruppe
const AUTHORIZED_MEMBERS = [
    // 🎯 DEMO ACCOUNTS - echte gültige Nostr-Keys mit korrekten Checksums
    'npub1804w7hpzpwmpjgqjzsx3cmtdutpzsyrr32n5p3aelx3ae2dvnw5qlqt8hg',  // Demo User - nsec1dxdzmrddnhdwrwpgu8sn86mtwnakqjl2g92xq3feecge52medwcquqc7hs
    'npub1yc5axr7c428r78dez4unnd8dsmzpuhkah7z7ppxemlm2k9n49p9sk949em',  // Alice Bitcoin - nsec1ur30z3t2tryk79tx3c47zfllqhjvl7lydk0hwxlhkmq9p4czl9rqx58s5t
    'npub1yxy5ftxcj22ms7scdj233kqt470euzkqf9dm25ra6jah30d2aa0qhdfu5h',  // Bob Lightning - nsec10232akdev76tr5dt9c2a8mgce8dk9ct3q36wfwf6dytk8y2pwadq3vrhe7
    'npub10stvcy697dkhyyjt8at2faj8yxl0kzywsguzxk97xmk9976n85uq7gywpu',  // Charlie Hodler - nsec1kxzfchk7r47qpccju304t3808c6cwcezs4chsytsxpv3p4r2rthsf83ys7
    'npub16p6kwyq9sg7l2p9uks930tg82cyqs4dgt9wytu79t5070d3seuasqa4ckm',  // Diana Trader - nsec1atc9cggm6t54g3t5j3ewreyw4a7rqyx4ry97jealfrnczv4gak6shu4urd
    
    // 🔧 ADMINS  
    'npub1admin1...example',  // Admin 1 - Hauptadmin
    'npub1admin2...example',  // Admin 2 - Backup Admin
    
    // 👨‍💼 BITCOIN OGs  
    'npub1max...example1',    // Max - Bitcoin Maximalist
    'npub1lisa...example2',   // Lisa - Lightning Expert
    'npub1tom...example3',    // Tom - Hardware Hodler
    
    // 👩‍💻 TECH SAVVY
    'npub1sarah...example4',  // Sarah - DeFi Queen
    'npub1mike...example5',   // Mike - Mining Expert
    'npub1alex...example6',   // Alex - Trading Bot Developer
    
    // 🏪 TRADERS
    'npub1trader1...example', // Local Bitcoin Dealer
    'npub1trader2...example', // Cash Trading Specialist
    
    // 📍 REGIONAL REPS
    'npub1munich...example',  // München Representative  
    'npub1berlin...example',  // Berlin Representative
    'npub1hamburg...example', // Hamburg Representative
    
    // 🚀 ADD NEW MEMBERS HERE:
    // 'npub1newuser...xyz',  // New User Name - Description
];

// 🎛️ ADMIN CONFIGURATION
const ADMIN_CONFIG = {
    // Main admin npub (receives all access requests)
    mainAdmin: 'npub1admin1...example',
    
    // Backup admins (can also approve members)
    backupAdmins: [
        'npub1admin2...example'
    ],
    
    // Auto-approval settings
    autoApprove: false,  // Set to true for open registration
    maxMembers: 50,      // Maximum number of members
    
    // Notification settings
    notifications: {
        newRequests: true,     // Notify on new access requests
        newMembers: true,      // Notify when members join
        offlineActivity: true  // Notify of activity while offline
    }
};

// 🏷️ MEMBER TAGS (optional, for organization)
const MEMBER_TAGS = {
    'npub1admin1...example': ['admin', 'founder'],
    'npub1max...example1': ['og', 'maximalist', 'munich'],
    'npub1lisa...example2': ['lightning', 'tech', 'berlin'],
    'npub1tom...example3': ['hardware', 'security', 'hodler'],
    'npub1sarah...example4': ['defi', 'altcoins', 'trading'],
    'npub1mike...example5': ['mining', 'hardware', 'energy'],
    // Add tags for better member organization
};

// 🌍 REGIONAL GROUPS (optional, for local meetups)
const REGIONAL_GROUPS = {
    munich: [
        'npub1max...example1',
        'npub1munich...example'
    ],
    berlin: [
        'npub1lisa...example2', 
        'npub1berlin...example'
    ],
    hamburg: [
        'npub1hamburg...example'
    ],
    online: [
        'npub1sarah...example4',
        'npub1mike...example5',
        'npub1alex...example6'
    ]
};

// 📊 REPUTATION SYSTEM (future feature)
const REPUTATION_CONFIG = {
    enabled: false,  // Enable reputation tracking
    minTradesForRep: 3,     // Minimum trades before reputation visible
    maxReputation: 100,     // Maximum reputation score
    decayRate: 0.95,        // Monthly reputation decay (to keep active)
    
    // Reputation factors
    factors: {
        successfulTrades: 10,   // Points per successful trade
        quickResponse: 2,       // Points for fast responses
        fairPricing: 5,         // Points for competitive pricing
        reliability: 3,         // Points for showing up to trades
        disputes: -15,          // Penalty for trade disputes
        spam: -25              // Penalty for spam/inappropriate behavior
    }
};

// 💰 TRADING LIMITS (optional security feature)
const TRADING_LIMITS = {
    enabled: false,  // Enable trading limits
    
    // New member limits (first 30 days)
    newMemberLimits: {
        maxOfferValue: 1000,    // Max €1000 per offer
        maxDailyVolume: 2000,   // Max €2000 per day
        requireEscrow: true     // Require escrow for trades
    },
    
    // Trusted member limits
    trustedMemberLimits: {
        maxOfferValue: 10000,   // Max €10k per offer
        maxDailyVolume: 50000,  // Max €50k per day
        requireEscrow: false    // No escrow required
    },
    
    // VIP member limits (high reputation)
    vipMemberLimits: {
        maxOfferValue: 100000,  // Max €100k per offer
        maxDailyVolume: 500000, // Max €500k per day
        requireEscrow: false    // No escrow required
    }
};

// 🚀 EXPORT FOR USAGE
if (typeof module !== 'undefined' && module.exports) {
    // Node.js environment
    module.exports = {
        AUTHORIZED_MEMBERS,
        ADMIN_CONFIG,
        MEMBER_TAGS,
        REGIONAL_GROUPS,
        REPUTATION_CONFIG,
        TRADING_LIMITS
    };
} else {
    // Browser environment - variables are available globally
    console.log('🔐 Bitcoin-Tausch-Netzwerk Config Loaded');
    console.log(`👥 ${AUTHORIZED_MEMBERS.length} authorized members`);
    console.log(`🎛️ Admin: ${ADMIN_CONFIG.mainAdmin.substring(0, 15)}...`);
}
