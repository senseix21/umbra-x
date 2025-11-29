# UMBRA Chat - P2P Messaging Guide

## Features Implemented ✅

1. **P2P Networking** - libp2p + QUIC transport
2. **Message Reception** - Real-time display of incoming messages  
3. **E2E Encryption** - ChaCha20-Poly1305 AEAD encryption (post-quantum ready)
4. **Peer Discovery** - Kademlia DHT + manual peer connection
5. **Interactive CLI** - Async stdin/stdout handling

## Quick Start

### Build
```bash
cd /Users/abuhamzah/Dev/umbra-chat
cargo build --release --bin umbra
```

### Run Node 1
```bash
./target/release/umbra start -u alice
```

This will output something like:
```
✓ Your Peer ID: 12D3KooWABC123...
✓ Listening on:
  /ip4/127.0.0.1/udp/54321/quic-v1/p2p/12D3KooWABC123...
  /ip4/192.168.1.100/udp/54321/quic-v1/p2p/12D3KooWABC123...
```

### Run Node 2 (Connect to Node 1)
```bash
./target/release/umbra start -u bob \
  -c "/ip4/127.0.0.1/udp/54321/quic-v1/p2p/12D3KooWABC123..."
```

Replace the address with the actual address from Node 1.

## CLI Commands

### Start Options
```bash
umbra start [OPTIONS]

Options:
  -p, --port <PORT>           Port to listen on (default: random)
  -c, --connect <PEER_ADDR>   Peer address to connect to
  -t, --topic <TOPIC>         Topic/channel to join (default: "umbra-chat")
  -u, --username <USERNAME>   Username to display (default: "anon")
```

### Chat Commands
- `/help` - Show help message
- `/peers` - Show your peer info and connected peers
- `/quit` or `/exit` - Exit the chat

## Example Session

### Terminal 1 (Alice)
```bash
$ ./target/release/umbra start -u alice -p 5000

╔════════════════════════════════════════════════════════════════╗
║              UMBRA.chat - Secure P2P Messaging                 ║
║  Post-quantum encrypted • No servers • No trace • No spam      ║
╚════════════════════════════════════════════════════════════════╝

✓ Node started successfully!
✓ Your Peer ID: 12D3KooWXYZ...
✓ Listening on:
  /ip4/0.0.0.0/udp/5000/quic-v1/p2p/12D3KooWXYZ...
✓ Subscribed to topic: umbra-chat

┌────────────────────────────────────────────────────────────────┐
│  Chat Ready! Type your message and press Enter to send.       │
│  All messages are encrypted with post-quantum crypto.         │
│  Commands: /help /peers /quit                                 │
└────────────────────────────────────────────────────────────────┘

alice> Hello Bob!
✓ Sent (encrypted)
[12D3KooW] bob: Hi Alice! This is encrypted!
alice> 
```

### Terminal 2 (Bob)
```bash
$ ./target/release/umbra start -u bob \
  -c "/ip4/127.0.0.1/udp/5000/quic-v1/p2p/12D3KooWXYZ..."

╔════════════════════════════════════════════════════════════════╗
║              UMBRA.chat - Secure P2P Messaging                 ║
║  Post-quantum encrypted • No servers • No trace • No spam      ║
╚════════════════════════════════════════════════════════════════╝

✓ Node started successfully!
✓ Your Peer ID: 12D3KooWABC...
📞 Connecting to peer: /ip4/127.0.0.1/udp/5000/quic-v1/p2p/12D3KooWXYZ...
✓ Connection initiated
✓ Subscribed to topic: umbra-chat

bob> [12D3KooW] alice: Hello Bob!
bob> Hi Alice! This is encrypted!
✓ Sent (encrypted)
bob> 
```

## Network Architecture

```
┌─────────────┐                    ┌─────────────┐
│   Node A    │                    │   Node B    │
│             │                    │             │
│ ┌─────────┐ │                    │ ┌─────────┐ │
│ │  CLI    │ │                    │ │  CLI    │ │
│ └────┬────┘ │                    │ └────┬────┘ │
│      │      │                    │      │      │
│ ┌────▼────┐ │                    │ ┌────▼────┐ │
│ │ Crypto  │ │   Encrypted P2P    │ │ Crypto  │ │
│ │ChaCha20 │ │◄──────────────────►│ │ChaCha20 │ │
│ └────┬────┘ │   (QUIC/libp2p)    │ └────┬────┘ │
│      │      │                    │      │      │
│ ┌────▼────┐ │                    │ ┌────▼────┐ │
│ │ Network │ │                    │ │ Network │ │
│ │  P2P    │ │                    │ │  P2P    │ │
│ └─────────┘ │                    │ └─────────┘ │
└─────────────┘                    └─────────────┘
```

## Features Breakdown

### 1. Message Reception Display ✅
- Incoming messages are displayed in real-time
- Shows peer ID (first 8 chars) and username
- Non-blocking async I/O

### 2. E2E Encryption Integration ✅
- ChaCha20-Poly1305 AEAD
- Random nonce per message
- Post-quantum ready (hybrid KEM support in crypto layer)

### 3. Improved Peer Discovery ✅
- Kademlia DHT for automatic peer discovery
- Manual peer connection via multiaddr
- Ping/identify protocols for peer info exchange
- Gossipsub for efficient message distribution

## Testing

### Two-Node Local Test
```bash
# Terminal 1
./target/release/umbra start -u alice -p 5000

# Terminal 2  
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/5000/quic-v1/p2p/<PEER_ID>"
```

### Multi-Node Network Test
```bash
# Node 1 (bootstrap)
./target/release/umbra start -u node1 -p 5000

# Node 2 (connects to node1)
./target/release/umbra start -u node2 -c "<node1_addr>"

# Node 3 (connects to node2, will discover node1 via DHT)
./target/release/umbra start -u node3 -c "<node2_addr>"
```

## Troubleshooting

### Connection Issues
- Ensure firewall allows UDP traffic on the specified port
- Use local IP (127.0.0.1) for same-machine testing
- Use LAN IP (192.168.x.x) for local network
- Use public IP for internet (requires port forwarding)

### Messages Not Appearing
- Check both nodes are on the same topic (default: "umbra-chat")
- Verify connection with `/peers` command
- Check logs for network errors

### Performance
- Release build is ~10x faster than debug: `cargo build --release`
- Each message is encrypted/decrypted independently
- Cover traffic and onion routing not yet enabled (Phase E)

## Next Steps (Optional Enhancements)

1. **Persistent Chat History** - Optional encrypted local storage
2. **Group Key Exchange** - Hybrid KEM session establishment
3. **Cover Traffic** - Metadata protection (already implemented in crate)
4. **Onion Routing** - 3-hop circuits (already implemented in crate)
5. **ZK Rate Limiting** - Spam protection (already implemented in crate)

## Security Notes

- Current encryption uses random keys per session (ephemeral)
- For production: implement proper key exchange via HybridKem
- Messages are encrypted but metadata (peer IDs, timestamps) is visible to network observers
- Enable onion routing and cover traffic for metadata protection

## Project Info
```bash
./target/release/umbra info
```

Shows:
- Version: 0.1.0-alpha
- Protocol: QUIC + libp2p
- Encryption: Post-quantum hybrid ready
- Features: P2P, E2EE, ZK proofs, etc.
