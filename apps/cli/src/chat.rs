use anyhow::Result;
use libp2p::PeerId;
use sha2::{Digest, Sha256};
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
use umbra_crypto::ChatCrypto;
use umbra_net::P2PNode;

use crate::ui::UI;

pub struct ChatSession {
    node: P2PNode,
    crypto: ChatCrypto,
    username: String,
    topic: String,
}

impl ChatSession {
    pub fn new(node: P2PNode, username: String, topic: String) -> Self {
        // Derive deterministic key from topic so all peers in same topic can decrypt
        // NOTE: This is for development only - not secure for production!
        let session_key = Self::derive_topic_key(&topic);
        
        Self {
            node,
            crypto: ChatCrypto::from_key(&session_key),
            username,
            topic,
        }
    }
    
    /// Derive deterministic 32-byte key from topic name
    /// All peers joining the same topic will derive the same key
    /// 
    /// WARNING: Development/testing only! In production, use:
    /// - Hybrid KEM for peer-to-peer encryption
    /// - MLS for group encryption with forward secrecy
    fn derive_topic_key(topic: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"umbra-topic-key-v1"); // Salt/domain separator
        hasher.update(topic.as_bytes());
        hasher.finalize().into()
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

    fn handle_incoming_message(&self, peer_id: PeerId, data: Vec<u8>) {
        match self.crypto.decrypt(&data) {
            Ok(plaintext) => {
                if let Ok(msg) = String::from_utf8(plaintext) {
                    let peer_short = peer_id.to_string().chars().take(8).collect::<String>();
                    UI::print_incoming_message(&peer_short, &msg, &self.username);
                }
            }
            Err(_) => {
                UI::print_decryption_error();
                UI::print_prompt(&self.username);
            }
        }
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

        // Send encrypted message
        let formatted_msg = format!("{}: {}", self.username, message);
        let encrypted = self.crypto.encrypt(formatted_msg.as_bytes());

        match self.node.publish(&self.topic, encrypted) {
            Ok(_) => {
                UI::print_message_sent();
            }
            Err(e) => {
                UI::print_message_failed(&e.to_string());
            }
        }

        UI::print_prompt(&self.username);
        Ok(true)
    }
}
