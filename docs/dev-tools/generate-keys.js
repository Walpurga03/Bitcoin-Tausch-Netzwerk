const { generatePrivateKey, getPublicKey, nip19 } = require('nostr-tools');

console.log('🔑 Generating 5 valid Nostr demo accounts...\n');

const names = ['Demo User', 'Alice Bitcoin', 'Bob Lightning', 'Charlie Hodler', 'Diana Trader'];
const accounts = [];

for (let i = 0; i < 5; i++) {
    try {
        // Generate private key and derive public key
        const privateKey = generatePrivateKey();
        const publicKey = getPublicKey(privateKey);
        
        // Encode to nsec and npub format
        const nsec = nip19.nsecEncode(privateKey);
        const npub = nip19.npubEncode(publicKey);
        
        // Verify by decoding (throws if invalid)
        nip19.decode(nsec);
        nip19.decode(npub);
        
        const account = {
            name: names[i],
            nsec: nsec,
            npub: npub
        };
        
        accounts.push(account);
        
        console.log(`👤 ${account.name}:`);
        console.log(`   nsec: ${account.nsec}`);
        console.log(`   npub: ${account.npub}\n`);
        
    } catch (error) {
        console.error(`❌ Error generating ${names[i]}:`, error.message);
    }
}

console.log('📋 config.js AUTHORIZED_MEMBERS update:');
console.log('// 🎯 DEMO ACCOUNTS - echte gültige Test-Accounts');
accounts.forEach(acc => {
    console.log(`    '${acc.npub}',  // ${acc.name}`);
});

console.log('\n🚀 Use any nsec above for login at http://localhost:8001/start.html');
