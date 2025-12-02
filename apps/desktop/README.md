# UMBRA Desktop

Quantum-safe P2P messaging desktop app built with Tauri + Rust.

## Quick Start

```bash
# Build
cd apps/desktop
cargo build

# Run
cargo run

# Or with Tauri CLI (once installed)
cargo tauri dev
```

## Architecture

```
apps/desktop/
├── src/
│   └── main.rs          # Rust backend (Tauri commands)
├── ui/public/
│   ├── index.html       # UI
│   ├── style.css        # Dark theme CSS
│   └── app.js           # Frontend logic
└── tauri.conf.json      # Tauri config
```

## Tauri Commands

- `start_node()` - Start P2P node
- `connect_peer(multiaddr)` - Connect to peer
- `subscribe_topic(topic)` - Subscribe to topic
- `send_message(topic, content)` - Send message

## Events (Backend → Frontend)

- `peer_connected` - New peer connected
- `message_received` - New message
- `handshake_completed` - Quantum handshake done

## Development

Keep it simple. No overengineering. Ship fast. Test continuously.
