// Debug Script - läuft im Browser
const nsec = 'nsec15tl6pwn8h00w85qcvggchzuf0egk4jha08q8mtw74q6ennu45lzsjmpfhw';

console.log('Starting conversion...');
console.log('nsec:', nsec);

try {
    // Direkte Konvertierung mit nostr-tools
    const decoded = window.NostrTools.nip19.decode(nsec);
    const publicKey = decoded.data;
    const npub = window.NostrTools.nip19.npubEncode(publicKey);
    
    console.log('✅ Successful conversion:');
    console.log('Public Key (hex):', publicKey);
    console.log('npub:', npub);
    
    // Copy to clipboard if available
    if (navigator.clipboard) {
        navigator.clipboard.writeText(npub).then(() => {
            console.log('✅ npub copied to clipboard!');
        });
    }
    
    alert(`✅ Conversion successful!\n\nnpub: ${npub}\n\n(Copied to clipboard)`);
    
} catch (error) {
    console.error('❌ Conversion failed:', error);
    alert('❌ Error: ' + error.message);
}
