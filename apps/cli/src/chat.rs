use anyhow::Result;
use libp2p::PeerId;
use tokio::io::{AsyncBufReadExt, BufReader};
use umbra_crypto::ChatCrypto;
use umbra_net::P2PNode;
use umbra_identity::{Identity, Prover, Storage};
use std::collections::HashMap;

use crate::ui::UI;

pub struct ChatSession {
    node: P2PNode,
    username: String,
    topic: String,
    identity: Option<Identity>,
    #[allow(dead_code)]
    prover: Option<Prover>,
    #[allow(dead_code)]
    peer_identities: HashMap<PeerId, [u8; 32]>,
    data_dir: String,
}

impl ChatSession {
    pub fn new(node: P2PNode, username: String, topic: String, data_dir: String) -> Self {
        // Try to load identity from specified data directory
        let (identity, prover) = if let Ok(storage) = Storage::new(&data_dir) {
            let id = storage.load_identity().ok();
            let pr = storage.load_keys().ok();
            (id, pr)
        } else {
            (None, None)
        };
        
        if identity.is_some() {
            println!("🔐 Identity loaded");
        }
        
        Self {
            node,
            username,
            topic,
            identity,
            prover,
            peer_identities: HashMap::new(),
            data_dir,
        }
    }

    pub async fn run(mut self) -> Result<()> {
        // Set identity in node if available
        if self.identity.is_some() && self.prover.is_some() {
            // Take ownership to avoid clone
            let identity = self.identity.take().unwrap();
            let prover = self.prover.take().unwrap();
            self.node.set_identity(identity, prover);
            println!("✅ ZK identity verification enabled");
        }

        // Get message receiver
        let mut message_rx = self
            .node
            .take_message_receiver()
            .ok_or_else(|| anyhow::anyhow!("Failed to get message receiver"))?;

        // Async stdin reader
        let stdin = tokio::io::stdin();
        let mut reader = BufReader::new(stdin).lines();

        UI::print_prompt(&self.username);

        loop {
            tokio::select! {
                // Handle network events (non-blocking poll)
                event = self.node.poll_once() => {
                    if let Err(e) = event {
                        UI::print_error(&format!("Network error: {}", e));
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

    fn handle_incoming_message(&mut self, peer_id: PeerId, data: Vec<u8>) {
        // Try to decrypt with new message exchange protocol
        match self.node.decrypt_message(peer_id, &data) {
            Ok((username, content, verified_identity)) => {
                if let Some(id) = verified_identity {
                    // Store verified identity
                    self.peer_identities.insert(peer_id, id);
                    let id_hex = hex::encode(&id[..8]);
                    UI::print_verified_message(&username, &content, &id_hex);
                } else {
                    UI::print_incoming_message(&username, &content);
                }
            }
            Err(_) => {
                // Fall back to legacy topic-based encryption for backwards compatibility
                let topic_key = Self::derive_topic_key(&self.topic);
                let crypto = ChatCrypto::from_key(&topic_key);
                match crypto.decrypt(&data) {
                    Ok(plaintext) => {
                        if let Ok(msg) = String::from_utf8(plaintext) {
                            let peer_short = peer_id.to_string().chars().take(8).collect::<String>();
                            UI::print_incoming_message(&peer_short, &msg);
                        }
                    }
                    Err(_) => {
                        UI::print_decryption_error();
                    }
                }
            }
        }
    }
    
    // Temporary topic-based key derivation (fallback)
    fn derive_topic_key(topic: &str) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(b"umbra-topic-key-v0.2");
        hasher.update(topic.as_bytes());
        hasher.finalize().into()
    }

    async fn handle_user_input(&mut self, line: &str) -> Result<bool> {
        let message = line.trim();
        if message.is_empty() {
            UI::print_prompt(&self.username);
            return Ok(true);
        }

        // Handle commands
        if message == "/quit" || message == "/exit" {
            UI::print_goodbye();
            return Ok(false);
        }

        if message == "/help" {
            UI::print_help();
            UI::print_prompt(&self.username);
            return Ok(true);
        }

        if message == "/peers" {
            let peers = self.node.connected_peers();
            let peer_id = self.node.local_peer_id();
            let addrs = self.node.listening_addresses();
            UI::print_peers_info(peer_id, &addrs, peers);
            UI::print_prompt(&self.username);
            return Ok(true);
        }

        if message == "/clear" {
            UI::clear_screen();
            UI::print_prompt(&self.username);
            return Ok(true);
        }

        if message == "/whoami" {
            if let Some(ref identity) = self.identity {
                println!("🆔 Your identity: {}", hex::encode(&identity.id[..8]));
            } else {
                println!("⚠️  No identity loaded. Create one with: umbra identity create <password>");
            }
            UI::print_prompt(&self.username);
            return Ok(true);
        }

        // Send encrypted message
        // For group chat, we use topic-based encryption (shared key)
        // For 1-1 chat, we'd use peer-specific encryption with send_encrypted_message
        
        // Check if we have any peers
        let peers = self.node.connected_peers();
        if peers.is_empty() {
            UI::print_error("No peers connected yet. Message not sent.");
            UI::print_prompt(&self.username);
            return Ok(true);
        }

        // Try new encrypted message format first
        // For group chat, we send to all peers with the first peer's session
        // (This is temporary - proper group keys come in v0.5)
        let send_result = if let Some(&first_peer) = peers.first() {
            self.node.send_encrypted_message(
                &self.topic,
                first_peer,
                &self.username,
                message,
            )
        } else {
            // Fallback to legacy topic-based encryption
            let formatted_msg = format!("{}: {}", self.username, message);
            let topic_key = Self::derive_topic_key(&self.topic);
            let crypto = ChatCrypto::from_key(&topic_key);
            let encrypted = crypto.encrypt(formatted_msg.as_bytes());
            self.node.publish(&self.topic, encrypted)
        };

        match send_result {
            Ok(_) => {
                UI::print_message_sent();
            }
            Err(e) => {
                let error_msg = e.to_string();
                if error_msg.contains("InsufficientPeers") {
                    UI::print_error("No peers connected yet. Message not sent.");
                } else {
                    UI::print_message_failed(&error_msg);
                }
            }
        }

        UI::print_prompt(&self.username);
        Ok(true)
    }
}