const { generateSecretKey, getPublicKey, nip19 } = require('nostr-tools');

console.log('🔑 Generating REAL working Nostr keys with nostr-tools...\n');

const names = ['Demo User', 'Alice Bitcoin', 'Bob Lightning', 'Charlie Hodler', 'Diana Trader'];
const accounts = [];

for (let i = 0; i < names.length; i++) {
    try {
        console.log(`Generating ${names[i]}...`);
        
        // Generate proper private key
        const privateKey = generateSecretKey();
        const publicKey = getPublicKey(privateKey);
        
        // Encode to nsec and npub
        const nsec = nip19.nsecEncode(privateKey);
        const npub = nip19.npubEncode(publicKey);
        
        // Verify by decoding (this will throw if invalid)
        nip19.decode(nsec);
        nip19.decode(npub);
        
        console.log(`✅ Generated valid keys for ${names[i]}`);
        console.log(`   nsec: ${nsec}`);
        console.log(`   npub: ${npub}`);
        console.log(`   Length: nsec=${nsec.length}, npub=${npub.length}`);
        console.log();
        
        accounts.push({
            name: names[i],
            nsec: nsec,
            npub: npub
        });
        
    } catch (error) {
        console.error(`❌ Error generating ${names[i]}:`, error.message);
    }
}

console.log('📋 config.js AUTHORIZED_MEMBERS update:');
accounts.forEach(acc => {
    console.log(`    '${acc.npub}',  // ${acc.name}`);
});

console.log('\n📝 DEMO-ACCOUNTS.md format:');
accounts.forEach(acc => {
    console.log(`## 👤 ${acc.name}`);
    console.log(`- **nsec:** \`${acc.nsec}\``);
    console.log(`- **npub:** \`${acc.npub}\``);
    console.log();
});

console.log('🎯 These are REAL Nostr keys with valid checksums!');
