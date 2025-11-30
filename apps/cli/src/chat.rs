use anyhow::Result;
use libp2p::PeerId;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
use umbra_crypto::{ChatCrypto, SessionManager};
use umbra_net::P2PNode;

use crate::ui::UI;

pub struct ChatSession {
    node: P2PNode,
    session_mgr: SessionManager,
    username: String,
    topic: String,
}

impl ChatSession {
    pub fn new(node: P2PNode, username: String, topic: String) -> Self {
        let session_mgr = SessionManager::new().expect("Failed to create session manager");
        
        // Print our identity key (for peer verification in future)
        let pk = session_mgr.public_key();
        let pk_hex = hex::encode(pk.as_bytes());
        println!("Your identity: {}...", &pk_hex[..16]);
        
        Self {
            node,
            session_mgr,
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
        // Get per-peer session key
        let session = match self.session_mgr.get_session(peer_id) {
            Ok(s) => s,
            Err(_) => {
                UI::print_error("Failed to get session for peer");
                return;
            }
        };
        
        let crypto = ChatCrypto::from_key(session.key());
        match crypto.decrypt(&data) {
            Ok(plaintext) => {
                if let Ok(msg) = String::from_utf8(plaintext) {
                    let peer_short = peer_id.to_string().chars().take(8).collect::<String>();
                    UI::print_incoming_message(&peer_short, &msg, &self.username);
                    
                    // Track message count for rotation
                    session.increment();
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
        
        // For now, use topic-based encryption (broadcast to all)
        // TODO: Per-peer encryption when we have peer list
        let peer_id = *self.node.local_peer_id();
        let session = self.session_mgr.get_session(peer_id)?;
        let crypto = ChatCrypto::from_key(session.key());
        let encrypted = crypto.encrypt(formatted_msg.as_bytes());

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
