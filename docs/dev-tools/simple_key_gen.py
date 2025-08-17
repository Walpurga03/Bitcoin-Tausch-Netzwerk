import secrets
import hashlib

def generate_demo_keys():
    """Generate demo nsec/npub pairs that look realistic"""
    
    names = ['Demo User', 'Alice Bitcoin', 'Bob Lightning', 'Charlie Hodler', 'Diana Trader']
    accounts = []
    
    print("🔑 Generating 5 demo accounts...\n")
    
    for i, name in enumerate(names):
        # Generate reproducible keys for demo purposes
        seed = f"demo-account-{i}-{name}".encode()
        private_hash = hashlib.sha256(seed).hexdigest()
        public_hash = hashlib.sha256(seed + b'public').hexdigest()
        
        # Create nsec and npub format (demo keys, not cryptographically secure)
        nsec = f"nsec1{private_hash[:50]}"
        npub = f"npub1{public_hash[:50]}"
        
        account = {
            'name': name,
            'nsec': nsec,
            'npub': npub
        }
        
        accounts.append(account)
        
        print(f"👤 {account['name']}:")
        print(f"   nsec: {account['nsec']}")
        print(f"   npub: {account['npub']}\n")
    
    print("📋 config.js AUTHORIZED_MEMBERS update:")
    print("    // 🎯 DEMO ACCOUNTS - Test-Accounts für Entwicklung")
    for acc in accounts:
        print(f"    '{acc['npub']}',  // {acc['name']}")
    
    print(f"\n🚀 Use any nsec above for login at http://localhost:8001/start.html")
    
    # Write to file
    try:
        with open('/home/tower/projekt/github/repo/Bitcoin-Tausch-Netzwerk/docs/demo-keys.txt', 'w') as f:
            f.write("🔑 DEMO ACCOUNTS FOR LOGIN\n\n")
            for acc in accounts:
                f.write(f"{acc['name']}:\n")
                f.write(f"nsec: {acc['nsec']}\n")
                f.write(f"npub: {acc['npub']}\n\n")
            
            f.write("Config.js AUTHORIZED_MEMBERS:\n")
            for acc in accounts:
                f.write(f"    '{acc['npub']}',  // {acc['name']}\n")
        
        print("📝 Keys saved to demo-keys.txt")
    except Exception as e:
        print(f"⚠️ Could not save file: {e}")
    
    return accounts

if __name__ == "__main__":
    accounts = generate_demo_keys()
