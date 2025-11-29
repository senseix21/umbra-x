# UMBRA CLI Chat - Quick Start Guide

## What is UMBRA CLI?

UMBRA CLI is a command-line P2P chat application that allows you to send encrypted messages directly to other peers without any servers.

## Prerequisites

- Rust 1.75+ installed
- Two terminals (or two computers on the same network)

## Quick Start: Two-Person Chat

### Terminal 1 (First Person)

```bash
cd /Users/abuhamzah/Dev/umbra-chat
./target/release/umbra start -u alice -p 9000
```

**Expected Output:**
```
╔════════════════════════════════════════════════════════════════╗
║              UMBRA.chat - Secure P2P Messaging                 ║
║  Post-quantum encrypted • No servers • No trace • No spam      ║
╚════════════════════════════════════════════════════════════════╝

✓ Node started successfully!
✓ Your Peer ID: 12D3KooWABC123...
✓ Listening on:
  /ip4/0.0.0.0/udp/9000/quic-v1
  /ip4/127.0.0.1/udp/9000/quic-v1/p2p/12D3KooWABC123...
✓ Subscribed to topic: umbra-chat

┌────────────────────────────────────────────────────────────────┐
│  Chat Ready! Type your message and press Enter to send.       │
│  Commands: /help /peers /quit                                 │
└────────────────────────────────────────────────────────────────┘

alice>
```

**Copy the full address** that looks like: `/ip4/127.0.0.1/udp/9000/quic-v1/p2p/12D3KooWABC123...`

### Terminal 2 (Second Person)

```bash
cd /Users/abuhamzah/Dev/umbra-chat
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/9000/quic-v1/p2p/12D3KooWABC123..."
```

Replace the address with the one you copied from Terminal 1.

**Expected Output:**
```
╔════════════════════════════════════════════════════════════════╗
║              UMBRA.chat - Secure P2P Messaging                 ║
║  Post-quantum encrypted • No servers • No trace • No spam      ║
╚════════════════════════════════════════════════════════════════╝

✓ Node started successfully!
✓ Your Peer ID: 12D3KooWXYZ456...
✓ Listening on:
  /ip4/0.0.0.0/udp/52847/quic-v1
  /ip4/127.0.0.1/udp/52847/quic-v1/p2p/12D3KooWXYZ456...
✓ Subscribed to topic: umbra-chat

📞 Connecting to peer: /ip4/127.0.0.1/udp/9000/quic-v1/p2p/12D3KooWABC123...
✓ Connection initiated

┌────────────────────────────────────────────────────────────────┐
│  Chat Ready! Type your message and press Enter to send.       │
│  Commands: /help /peers /quit                                 │
└────────────────────────────────────────────────────────────────┘

bob>
```

### Start Chatting!

**Terminal 1 (Alice):**
```
alice> Hello Bob!
✓ Sent: Hello Bob!
```

**Terminal 2 (Bob):**
```
bob> Hi Alice! How are you?
✓ Sent: Hi Alice! How are you?
```

## Command Reference

### Start Command

```bash
./target/release/umbra start [OPTIONS]
```

**Options:**
- `-u, --username <NAME>` - Your display name (default: "anon")
- `-p, --port <PORT>` - Port to listen on (default: random)
- `-c, --connect <ADDR>` - Peer address to connect to
- `-t, --topic <TOPIC>` - Channel name (default: "umbra-chat")

**Examples:**

```bash
# Start with username on specific port
./target/release/umbra start -u alice -p 9000

# Start and connect to a peer
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/9000/quic-v1/p2p/12D3..."

# Join a specific topic/channel
./target/release/umbra start -u charlie -t "secret-room"

# All options together
./target/release/umbra start -u dave -p 9001 -t "my-channel" -c "/ip4/..."
```

### Info Command

```bash
./target/release/umbra info
```

Shows information about UMBRA.chat features and capabilities.

### Chat Commands (while in chat)

- `/help` - Show help message
- `/peers` - Show your peer ID and listening addresses
- `/quit` or `/exit` - Exit the chat

## Network Scenarios

### Scenario 1: Same Computer (localhost)

Use `127.0.0.1` for the IP address:
```bash
# Terminal 1
./target/release/umbra start -u alice -p 9000

# Terminal 2
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/9000/quic-v1/p2p/..."
```

### Scenario 2: Same Local Network

Find your local IP address first:
```bash
# macOS/Linux
ifconfig | grep "inet " | grep -v 127.0.0.1

# You'll see something like: inet 192.168.1.100
```

Then use that IP:
```bash
# Computer 1 (192.168.1.100)
./target/release/umbra start -u alice -p 9000

# Computer 2 (any IP on same network)
./target/release/umbra start -u bob -c "/ip4/192.168.1.100/udp/9000/quic-v1/p2p/..."
```

### Scenario 3: Over Internet (requires port forwarding)

1. Forward UDP port 9000 on your router to your computer
2. Find your public IP: `curl ifconfig.me`
3. Share your public IP with the other person

```bash
# Your computer (behind router with port forwarding)
./target/release/umbra start -u alice -p 9000

# Other person (anywhere on internet)
./target/release/umbra start -u bob -c "/ip4/YOUR.PUBLIC.IP/udp/9000/quic-v1/p2p/..."
```

## Troubleshooting

### Problem: "Failed to send: ..."

**Cause:** Peers might not be connected yet.
**Solution:** Wait a few seconds after connection before sending messages.

### Problem: "No route to host"

**Cause:** Firewall is blocking UDP traffic.
**Solution:** 
```bash
# macOS - Allow incoming UDP on port 9000
# Go to System Preferences > Security & Privacy > Firewall > Firewall Options
# Or temporarily disable firewall for testing

# Linux
sudo ufw allow 9000/udp
```

### Problem: Can't find the binary

**Solution:**
```bash
cd /Users/abuhamzah/Dev/umbra-chat
cargo build --bin umbra --release
./target/release/umbra start
```

### Problem: "Address already in use"

**Cause:** Port is already taken.
**Solution:** Use a different port:
```bash
./target/release/umbra start -p 9001
```

## Advanced Usage

### Create an Alias

Add to your `~/.bashrc` or `~/.zshrc`:
```bash
alias umbra='/Users/abuhamzah/Dev/umbra-chat/target/release/umbra'
```

Then you can use:
```bash
umbra start -u yourname
```

### Multiple Channels

Run multiple instances on different ports for different channels:
```bash
# Terminal 1: Work chat
./target/release/umbra start -u alice -p 9000 -t "work"

# Terminal 2: Friends chat
./target/release/umbra start -u alice -p 9001 -t "friends"
```

### Group Chat

All peers need to subscribe to the same topic and be connected to at least one other peer:

```bash
# Peer 1
./target/release/umbra start -u alice -p 9000 -t "group"

# Peer 2 (connects to Peer 1)
./target/release/umbra start -u bob -p 9001 -t "group" -c "/ip4/.../p2p/PEER1_ID"

# Peer 3 (connects to Peer 1 or 2)
./target/release/umbra start -u charlie -p 9002 -t "group" -c "/ip4/.../p2p/PEER1_ID"
```

## What's Working

✅ **P2P Discovery** - Nodes find each other using libp2p  
✅ **QUIC Transport** - Fast, encrypted connections  
✅ **Gossipsub Messaging** - Pub/sub messaging protocol  
✅ **Topic-based Channels** - Join different chat rooms  
✅ **Direct Dialing** - Connect to specific peers  
✅ **CLI Interface** - Easy command-line usage  

## Coming Soon

🚧 **Message Reception UI** - Currently messages are published but not displayed from peers yet  
🚧 **Post-Quantum Encryption** - ML-KEM integration  
🚧 **ZK Proofs** - Rate limiting without identity  
🚧 **MLS Groups** - End-to-end encrypted group chat  
🚧 **Onion Routing** - Privacy-preserving message routing  

## Technical Details

- **Protocol:** QUIC (UDP-based)
- **P2P Library:** libp2p 0.53
- **Messaging:** Gossipsub (pub/sub)
- **Discovery:** Kademlia DHT
- **Language:** Rust

## Support

For issues or questions:
1. Check the [TESTING.md](TESTING.md) file
2. Review the [ROADMAP.md](ROADMAP.md) for current status
3. Open an issue on GitHub

## Privacy & Security Note

⚠️ **Current Status:** This is an alpha version. While the infrastructure for post-quantum encryption and zero-knowledge proofs is built, message encryption is not yet fully integrated into the CLI. Messages are currently transmitted over encrypted QUIC connections but are not end-to-end encrypted with PQ cryptography yet.

**Use for:** Testing, development, non-sensitive communications  
**Don't use for:** Confidential information (yet!)

## Quick Reference Card

```
┌─ UMBRA CLI Quick Reference ────────────────────────────────────┐
│                                                                 │
│  Start node:                                                    │
│    ./target/release/umbra start -u <name> -p <port>            │
│                                                                 │
│  Connect to peer:                                              │
│    ./target/release/umbra start -c "<full_address>"            │
│                                                                 │
│  In chat:                                                       │
│    Type message + Enter = Send                                 │
│    /help = Show help                                           │
│    /peers = Show connection info                               │
│    /quit = Exit                                                │
│                                                                 │
│  Address format:                                               │
│    /ip4/<IP>/udp/<PORT>/quic-v1/p2p/<PEER_ID>                 │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```
