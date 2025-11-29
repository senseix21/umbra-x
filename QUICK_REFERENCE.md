# UMBRA.chat - Quick Reference

## Build & Run

```bash
# Build (release mode)
cargo build --release --bin umbra

# Start node
./target/release/umbra start -u <username> [-p <port>] [-c <peer_address>]

# Show info
./target/release/umbra info
```

## Examples

```bash
# Node 1: Alice on port 5000
./target/release/umbra start -u alice -p 5000

# Node 2: Bob connects to Alice
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/5000/quic-v1/p2p/<PEER_ID>"

# Node 3: Custom topic
./target/release/umbra start -u charlie -t "secret-channel"
```

## Chat Commands

| Command | Description |
|---------|-------------|
| `/help` | Show help message |
| `/peers` | Show peer info and connections |
| `/quit` or `/exit` | Exit chat |

## Features

✅ P2P messaging (libp2p + QUIC)  
✅ E2E encryption (ChaCha20-Poly1305)  
✅ Peer discovery (Kademlia DHT)  
✅ Real-time message display  
✅ Async I/O (tokio)  

## Security

- **Encryption**: ChaCha20-Poly1305 AEAD
- **Transport**: QUIC with TLS 1.3
- **Authentication**: Gossipsub message signing
- **Nonce**: Random 96-bit per message
- **Key**: Random 256-bit per session

## Architecture

```
User → CLI → Encrypt → P2P Network → Decrypt → Display
```

## Repository

```
https://github.com/senseix21/umbra-x
```

## Docs

- [CLI_USER_GUIDE.md](./CLI_USER_GUIDE.md) - Full usage guide
- [IMPLEMENTATION_SUMMARY_FINAL.md](./IMPLEMENTATION_SUMMARY_FINAL.md) - Technical details
- [FINAL_COMPLETION.md](./FINAL_COMPLETION.md) - Completion summary
- [ROADMAP.md](./ROADMAP.md) - Project roadmap

## Status

**✅ COMPLETE & READY TO USE**

All core features implemented and tested.
