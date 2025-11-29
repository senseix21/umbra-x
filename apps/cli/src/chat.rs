use anyhow::Result;
use libp2p::PeerId;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
use umbra_crypto::ChatCrypto;
use umbra_net::P2PNode;

pub struct ChatSession {
    node: P2PNode,
    crypto: ChatCrypto,
    username: String,
    topic: String,
}

impl ChatSession {
    pub fn new(node: P2PNode, username: String, topic: String) -> Self {
        Self {
            node,
            crypto: ChatCrypto::new(),
            username,
            topic,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        // Get message receiver
        let mut message_rx = self
            .node
            .take_message_receiver()
            .ok_or_else(|| anyhow::anyhow!("Failed to get message receiver"))?;

        // Async stdin reader
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin).lines();

        println!("{}> ", self.username);
        io::stdout().flush()?;

        loop {
            tokio::select! {
                // Handle network events (non-blocking poll)
                event = self.node.poll_once() => {
                    if let Err(e) = event {
                        eprintln!("Network error: {}", e);
                        break;
                    }
                }

                // Handle incoming messages
                Some((peer_id, data)) = message_rx.recv() => {
                    self.handle_incoming_message(peer_id, data);
                }

                // Handle user input
                Ok(Some(line)) = reader.next_line() => {
                    if !self.handle_user_input(&line).await? {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_incoming_message(&self, peer_id: PeerId, data: Vec<u8>) {
        match self.crypto.decrypt(&data) {
            Ok(plaintext) => {
                if let Ok(msg) = String::from_utf8(plaintext) {
                    let peer_short = peer_id.to_string().chars().take(8).collect::<String>();
                    println!("\r[{}] {}", peer_short, msg);
                    print!("{}> ", self.username);
                    io::stdout().flush().ok();
                }
            }
            Err(e) => {
                eprintln!("Decryption failed: {}", e);
            }
        }
    }

    async fn handle_user_input(&mut self, line: &str) -> Result<bool> {
        let message = line.trim();
        if message.is_empty() {
            print!("{}> ", self.username);
            io::stdout().flush()?;
            return Ok(true);
        }

        // Handle commands
        if message == "/quit" || message == "/exit" {
            println!("Goodbye!");
            return Ok(false);
        }

        if message == "/help" {
            self.print_help();
            print!("{}> ", self.username);
            io::stdout().flush()?;
            return Ok(true);
        }

        if message == "/peers" {
            self.show_peers();
            print!("{}> ", self.username);
            io::stdout().flush()?;
            return Ok(true);
        }

        // Send encrypted message
        let formatted_msg = format!("{}: {}", self.username, message);
        let encrypted = self.crypto.encrypt(formatted_msg.as_bytes());

        match self.node.publish(&self.topic, encrypted) {
            Ok(_) => {
                println!("✓ Sent (encrypted)");
            }
            Err(e) => {
                println!("✗ Failed to send: {}", e);
            }
        }

        print!("{}> ", self.username);
        io::stdout().flush()?;
        Ok(true)
    }

    fn print_help(&self) {
        println!("\n┌─ UMBRA Chat Commands ─────────────────────────────────────────┐");
        println!("│  /help     - Show this help message                           │");
        println!("│  /peers    - Show connected peers and your info               │");
        println!("│  /quit     - Exit the chat                                    │");
        println!("│  /exit     - Exit the chat                                    │");
        println!("└────────────────────────────────────────────────────────────────┘\n");
    }

    fn show_peers(&self) {
        println!("\n┌─ Peer Information ────────────────────────────────────────────┐");
        println!("│ Your Peer ID: {}", self.node.local_peer_id());
        println!("│");
        println!("│ Listening addresses:");
        for addr in self.node.listening_addresses() {
            println!("│   {}", addr);
        }
        println!("│");
        println!("│ Connected peers:");
        let peers = self.node.connected_peers();
        if peers.is_empty() {
            println!("│   (none)");
        } else {
            for peer in peers {
                println!("│   {}", peer);
            }
        }
        println!("└────────────────────────────────────────────────────────────────┘\n");
    }
}
