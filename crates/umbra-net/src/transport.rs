use libp2p::swarm::NetworkBehaviour;
use libp2p::{
    gossipsub, identify, kad, ping,
    swarm::SwarmEvent,
    Multiaddr, PeerId, Swarm,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tracing::{debug, info, warn};
use crate::handshake::{HandshakeBehaviour, HandshakeEvent};

/// Combined network behaviour for UMBRA P2P
#[derive(NetworkBehaviour)]
#[behaviour(to_swarm = "UmbraEvent")]
pub struct UmbraBehaviour {
    ping: ping::Behaviour,
    identify: identify::Behaviour,
    kad: kad::Behaviour<kad::store::MemoryStore>,
    gossipsub: gossipsub::Behaviour,
    handshake: HandshakeBehaviour,
}

#[derive(Debug)]
pub enum UmbraEvent {
    Ping(ping::Event),
    Identify(identify::Event),
    Kad(kad::Event),
    Gossipsub(gossipsub::Event),
    Handshake(HandshakeEvent),
}

impl From<ping::Event> for UmbraEvent {
    fn from(event: ping::Event) -> Self {
        UmbraEvent::Ping(event)
    }
}

impl From<identify::Event> for UmbraEvent {
    fn from(event: identify::Event) -> Self {
        UmbraEvent::Identify(event)
    }
}

impl From<kad::Event> for UmbraEvent {
    fn from(event: kad::Event) -> Self {
        UmbraEvent::Kad(event)
    }
}

impl From<gossipsub::Event> for UmbraEvent {
    fn from(event: gossipsub::Event) -> Self {
        UmbraEvent::Gossipsub(event)
    }
}

impl From<HandshakeEvent> for UmbraEvent {
    fn from(event: HandshakeEvent) -> Self {
        UmbraEvent::Handshake(event)
    }
}

pub struct P2PNode {
    swarm: Swarm<UmbraBehaviour>,
    local_peer_id: PeerId,
    message_rx: Option<tokio::sync::mpsc::UnboundedReceiver<(PeerId, Vec<u8>)>>,
    message_tx: tokio::sync::mpsc::UnboundedSender<(PeerId, Vec<u8>)>,
    message_exchange: crate::message::MessageExchange,
}

impl P2PNode {
    pub async fn new() -> crate::error::Result<Self> {
        Self::new_with_port(0).await
    }

    pub async fn new_with_port(port: u16) -> crate::error::Result<Self> {
        let local_key = libp2p::identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());
        
        info!("Local peer id: {}", local_peer_id);
        
        // Configure gossipsub with message deduplication
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(1))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .message_id_fn(|message| {
                let mut hasher = DefaultHasher::new();
                message.data.hash(&mut hasher);
                gossipsub::MessageId::from(hasher.finish().to_string())
            })
            .build()
            .map_err(|e| crate::error::NetError::Transport(format!("Gossipsub config: {}", e)))?;

        let gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config,
        )
        .map_err(|e| crate::error::NetError::Transport(format!("Gossipsub init: {}", e)))?;
        
        let behaviour = UmbraBehaviour {
            ping: ping::Behaviour::new(ping::Config::new()),
            identify: identify::Behaviour::new(identify::Config::new(
                "/umbra/0.1.0".to_string(),
                local_key.public(),
            )),
            kad: kad::Behaviour::new(
                local_peer_id,
                kad::store::MemoryStore::new(local_peer_id),
            ),
            gossipsub,
            handshake: {
                // Generate signing key for handshake
                let signing_key = ed25519_dalek::SigningKey::from_bytes(&rand::random());
                HandshakeBehaviour::new(signing_key)
            },
        };
        
        // Create swarm with QUIC transport (libp2p 0.53 API)
        let mut swarm = libp2p::SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_quic()
            .with_behaviour(|_| behaviour)
            .map_err(|e| crate::error::NetError::Transport(format!("Swarm build failed: {:?}", e)))?
            .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
            .build();
        
        // Listen on QUIC with specified port
        let listen_addr = format!("/ip4/0.0.0.0/udp/{}/quic-v1", port);
        swarm.listen_on(listen_addr.parse().unwrap())
            .map_err(|e| crate::error::NetError::Transport(format!("Listen failed: {:?}", e)))?;
        
        let (message_tx, message_rx) = tokio::sync::mpsc::unbounded_channel();
        
        let message_exchange = crate::message::MessageExchange::new(local_peer_id)
            .map_err(|e| crate::error::NetError::Transport(format!("MessageExchange init: {}", e)))?;
        
        Ok(Self {
            swarm,
            local_peer_id,
            message_rx: Some(message_rx),
            message_tx,
            message_exchange,
        })
    }
    
    pub fn local_peer_id(&self) -> &PeerId {
        &self.local_peer_id
    }
    
    pub fn listening_addresses(&self) -> Vec<Multiaddr> {
        self.swarm.listeners().cloned().collect()
    }
    
    pub async fn dial(&mut self, addr: Multiaddr) -> crate::error::Result<()> {
        self.swarm.dial(addr)
            .map_err(|e| crate::error::NetError::Transport(format!("Dial failed: {:?}", e)))?;
        Ok(())
    }
    
    /// Subscribe to a gossipsub topic
    pub fn subscribe(&mut self, topic: &str) -> crate::error::Result<()> {
        let topic = gossipsub::IdentTopic::new(topic);
        self.swarm.behaviour_mut().gossipsub.subscribe(&topic)
            .map_err(|e| crate::error::NetError::Transport(format!("Subscribe failed: {}", e)))?;
        Ok(())
    }
    
    /// Publish message to gossipsub topic
    pub fn publish(&mut self, topic: &str, data: Vec<u8>) -> crate::error::Result<()> {
        let topic = gossipsub::IdentTopic::new(topic);
        self.swarm.behaviour_mut().gossipsub.publish(topic, data)
            .map_err(|e| crate::error::NetError::Transport(format!("Publish failed: {}", e)))?;
        Ok(())
    }

    /// Send encrypted message to a topic
    pub fn send_encrypted_message(
        &mut self,
        topic: &str,
        peer: PeerId,
        username: &str,
        content: &str,
    ) -> crate::error::Result<()> {
        // Encrypt message for peer
        let encrypted_data = self.message_exchange.encrypt_message(peer, username, content)?;
        
        // Publish to gossipsub
        self.publish(topic, encrypted_data)?;
        
        Ok(())
    }

    /// Decrypt received message
    pub fn decrypt_message(&mut self, peer: PeerId, data: &[u8]) -> crate::error::Result<(String, String)> {
        self.message_exchange.decrypt_message(peer, data)
    }
    
    /// Add a peer to the routing table
    pub fn add_peer(&mut self, peer_id: PeerId, addr: Multiaddr) {
        self.swarm.behaviour_mut().kad.add_address(&peer_id, addr);
    }
    
    /// Bootstrap the Kademlia DHT
    pub fn bootstrap(&mut self) -> crate::error::Result<()> {
        self.swarm.behaviour_mut().kad.bootstrap()
            .map_err(|e| crate::error::NetError::Discovery(format!("Bootstrap failed: {:?}", e)))?;
        Ok(())
    }
    
    /// Take message receiver for application use
    pub fn take_message_receiver(&mut self) -> Option<tokio::sync::mpsc::UnboundedReceiver<(PeerId, Vec<u8>)>> {
        self.message_rx.take()
    }
    
    /// Get connected peers
    pub fn connected_peers(&self) -> Vec<PeerId> {
        self.swarm.connected_peers().copied().collect()
    }
    
    /// Run one iteration of the event loop (non-blocking)
    pub async fn poll_once(&mut self) -> crate::error::Result<()> {
        use futures::StreamExt;
        
        match self.swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                debug!("Listening on {:?}", address);
            }
            SwarmEvent::Behaviour(event) => {
                match event {
                    UmbraEvent::Ping(ping::Event { peer, result, .. }) => {
                        match result {
                            Ok(_) => {} // Silent ping success
                            Err(e) => warn!("Ping to {} failed: {}", peer, e),
                        }
                    }
                    UmbraEvent::Identify(identify::Event::Received { peer_id, info }) => {
                        debug!("Identified peer {}: {}", peer_id, info.protocol_version);
                        for addr in info.listen_addrs {
                            self.swarm.behaviour_mut().kad.add_address(&peer_id, addr);
                        }
                    }
                    UmbraEvent::Kad(kad::Event::RoutingUpdated { peer, .. }) => {
                        debug!("Routing table updated: {}", peer);
                    }
                    UmbraEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source,
                        message,
                        ..
                    }) => {
                        info!(
                            "Received message from {}: {} bytes",
                            propagation_source,
                            message.data.len()
                        );
                        // Forward message to application layer
                        let _ = self.message_tx.send((propagation_source, message.data));
                    }
                    UmbraEvent::Handshake(event) => {
                        use crate::handshake::HandshakeEvent;
                        match event {
                            HandshakeEvent::Completed { peer_id, session_key: _, verify_key } => {
                                info!("✅ Quantum-safe handshake completed with {}", peer_id);
                                // Register peer's verify key for message signature verification
                                self.message_exchange.session_manager_mut().register_peer(peer_id, verify_key);
                                info!("🔑 Registered verify key for {}", peer_id);
                            }
                            HandshakeEvent::Failed { peer_id, error } => {
                                warn!("❌ Handshake with {} failed: {}", peer_id, error);
                            }
                        }
                    }
                    _ => {}
                }
            }
            SwarmEvent::ConnectionEstablished { peer_id, endpoint, .. } => {
                info!("✓ Connected to {}", peer_id);
                
                // Auto-register peer keys for signature verification (mock for now)
                let our_verify_key = self.message_exchange.session_manager().public_key();
                self.message_exchange.session_manager_mut().register_peer(peer_id, our_verify_key);
            }
            SwarmEvent::ConnectionClosed { peer_id, cause, .. } => {
                debug!("Connection to {} closed: {:?}", peer_id, cause);
            }
            SwarmEvent::IncomingConnection { .. } => {
                debug!("Incoming connection");
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error, .. } => {
                warn!("Outgoing connection error to {:?}: {}", peer_id, error);
            }
            _ => {}
        }
        
        Ok(())
    }
    
    /// Run the event loop
    pub async fn run(&mut self) -> crate::error::Result<()> {
        loop {
            self.poll_once().await?;
        }
    }

    // Handshake methods
    
    /// Register a peer's verify key (must be done before handshake)
    pub fn register_peer_key(&mut self, peer_id: PeerId, verify_key: ed25519_dalek::VerifyingKey) {
        self.swarm.behaviour_mut().handshake.register_peer(peer_id, verify_key);
    }
    
    /// Get session key for a peer (if handshake completed)
    pub fn get_session_key(&self, peer_id: &PeerId) -> Option<&[u8; 32]> {
        self.swarm.behaviour().handshake.get_session_key(peer_id)
    }
    
    /// Initiate handshake with a peer (for manual testing)
    pub fn initiate_handshake(&mut self, peer_id: PeerId) -> Result<(), String> {
        self.swarm.behaviour_mut().handshake.initiate_handshake(peer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_node_creation() {
        let node = P2PNode::new().await.unwrap();
        assert!(node.local_peer_id().to_base58().len() > 0);
    }
    
    #[tokio::test]
    async fn test_node_listening() {
        let _node = P2PNode::new_with_port(0).await.unwrap();
        
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    
    #[tokio::test]
    async fn test_gossipsub_subscribe() {
        let mut node = P2PNode::new().await.unwrap();
        let result = node.subscribe("test-topic");
        assert!(result.is_ok());
    }
}





