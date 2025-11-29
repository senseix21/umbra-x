# 🎯 UMBRA.chat - How to Use (Simple Guide)

**Version:** 0.1.0-alpha  
**Updated:** November 29, 2024

---

## 🚀 Start Here (3 Steps)

### Step 1: Build the App (2 minutes)
```bash
cd /Users/abuhamzah/Dev/umbra-chat
cargo build --bin umbra --release
```

Wait for compilation to complete. You'll see:
```
   Compiling umbra-cli v0.1.0
    Finished release [optimized] target(s) in 2m 30s
```

### Step 2: Verify It Works (30 seconds)
```bash
./target/release/umbra info
```

You should see UMBRA.chat project information.

### Step 3: Start Chatting! (2 people needed)

**Person 1 (Terminal 1):**
```bash
./target/release/umbra start -u alice -p 9000
```

**Copy the address** that looks like:
```
/ip4/127.0.0.1/udp/9000/quic-v1/p2p/12D3KooWABC123...
```

**Person 2 (Terminal 2):**
```bash
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/9000/quic-v1/p2p/12D3KooWABC123..."
```

**Both terminals:** Type messages and press Enter!

---

## 📖 Commands

### Start Command
```bash
./target/release/umbra start [OPTIONS]
```

**Options:**
- `-u <name>` or `--username <name>` - Your display name
- `-p <port>` or `--port <port>` - Port number to listen on
- `-c <address>` or `--connect <address>` - Connect to peer
- `-t <topic>` or `--topic <topic>` - Channel name (default: "umbra-chat")

### Examples
```bash
# Basic (random port, username "anon")
./target/release/umbra start

# With username
./target/release/umbra start -u alice

# With username and port
./target/release/umbra start -u alice -p 9000

# Connect to someone
./target/release/umbra start -u bob -c "<their_address>"

# Different channel
./target/release/umbra start -u charlie -t "secret-room"
```

### In-Chat Commands
While chatting, type these commands:

- `/help` - Show help message
- `/peers` - Show your connection info
- `/quit` or `/exit` - Leave the chat

---

## 🏠 Network Scenarios

### Scenario 1: Same Computer (Testing)
Perfect for testing alone or with one friend on same computer.

**Terminal 1:**
```bash
./target/release/umbra start -u alice -p 9000
```

**Terminal 2:**
```bash
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/9000/quic-v1/p2p/..."
```

### Scenario 2: Same WiFi Network
Chat with friends on the same WiFi.

**Step 1:** Find your local IP:
```bash
# macOS/Linux
ifconfig | grep "inet " | grep -v 127.0.0.1
# Example output: inet 192.168.1.100
```

**Person 1 (192.168.1.100):**
```bash
./target/release/umbra start -u alice -p 9000
```

**Person 2 (any computer on same WiFi):**
```bash
./target/release/umbra start -u bob -c "/ip4/192.168.1.100/udp/9000/quic-v1/p2p/..."
```

### Scenario 3: Over Internet
Requires port forwarding on router.

1. Forward UDP port 9000 to your computer in router settings
2. Find your public IP: `curl ifconfig.me`
3. Share that IP with your friend

**You:**
```bash
./target/release/umbra start -u alice -p 9000
```

**Friend (anywhere):**
```bash
./target/release/umbra start -u bob -c "/ip4/YOUR.PUBLIC.IP/udp/9000/quic-v1/p2p/..."
```

---

## 🔧 Troubleshooting

### Problem: Can't find the binary
```bash
cd /Users/abuhamzah/Dev/umbra-chat
ls -lh target/release/umbra
# If not found:
cargo build --bin umbra --release
```

### Problem: Port already in use
Use a different port:
```bash
./target/release/umbra start -p 9001
```

### Problem: Connection failed
- Make sure peer is running
- Check firewall settings
- Verify you copied the full address
- Try pinging the IP: `ping 192.168.1.100`

### Problem: No messages appear
**Current status:** Message reception display is coming soon!  
**Workaround:** Messages are being sent, just not displayed yet.

---

## ✅ What Works

- ✅ Starting nodes
- ✅ Connecting to peers
- ✅ Sending messages
- ✅ Topic channels
- ✅ Multiple channels
- ✅ Commands (/help, /peers, /quit)

## 🚧 Coming Soon

- 🚧 Receiving messages (display in CLI)
- 🚧 End-to-end encryption
- 🚧 Group discovery
- 🚧 File sharing

---

## 📚 More Information

| Document | What It's For |
|----------|---------------|
| [CLI_GUIDE.md](CLI_GUIDE.md) | Complete CLI reference |
| [STATUS.md](STATUS.md) | Current project status |
| [SUMMARY.md](SUMMARY.md) | Project overview |
| [COMPLETE_TESTING_GUIDE.md](COMPLETE_TESTING_GUIDE.md) | Testing instructions |

---

## 💡 Tips

### Tip 1: Create an Alias
Add to `~/.zshrc` or `~/.bashrc`:
```bash
alias umbra='/Users/abuhamzah/Dev/umbra-chat/target/release/umbra'
```

Then you can just run:
```bash
umbra start -u yourname
```

### Tip 2: Multiple Channels
Run multiple instances for different topics:
```bash
# Terminal 1: Work chat
./target/release/umbra start -u alice -p 9000 -t "work"

# Terminal 2: Friends chat  
./target/release/umbra start -u alice -p 9001 -t "friends"
```

### Tip 3: Remember Your Port
If you always use the same port, others can save your address:
```bash
# Always use port 9000
./target/release/umbra start -u alice -p 9000
```

---

## 🎯 Quick Reference

```
┌─ UMBRA CLI Cheat Sheet ────────────────────────────────┐
│                                                         │
│  Build:                                                 │
│    cargo build --bin umbra --release                   │
│                                                         │
│  Start:                                                 │
│    ./target/release/umbra start -u <name>              │
│                                                         │
│  Connect:                                              │
│    ./target/release/umbra start -c "<address>"         │
│                                                         │
│  Options:                                              │
│    -u  Username                                        │
│    -p  Port (default: random)                          │
│    -c  Connect to peer address                         │
│    -t  Topic/channel (default: "umbra-chat")           │
│                                                         │
│  In Chat:                                              │
│    /help   - Show help                                 │
│    /peers  - Connection info                           │
│    /quit   - Exit                                      │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## ❓ FAQ

**Q: Do I need servers?**  
A: No! UMBRA is peer-to-peer. You connect directly to other users.

**Q: Is it encrypted?**  
A: Yes! QUIC provides transport encryption. Full E2E encryption coming soon.

**Q: Can I chat with multiple people?**  
A: Yes! Everyone connects to the same topic/channel.

**Q: Does it work offline?**  
A: You need internet to connect to peers, but no central server.

**Q: How do I share files?**  
A: File sharing is planned for a future update.

**Q: Can I use this on my phone?**  
A: Mobile support is planned but not yet available.

---

## 🆘 Need Help?

1. Read [CLI_GUIDE.md](CLI_GUIDE.md)
2. Check [STATUS.md](STATUS.md) for known issues
3. Run `./target/release/umbra info`
4. Try the examples: `cargo run --example hello_mesh node1`

---

**Ready to start? Run this now:**
```bash
./target/release/umbra start -u yourname
```

**Happy chatting! 🎉**
