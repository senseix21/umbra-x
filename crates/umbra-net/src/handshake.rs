use libp2p::swarm::{ConnectionHandler, NetworkBehaviour};
use libp2p::PeerId;
use std::collections::{HashMap, VecDeque};
use umbra_crypto::handshake::Handshake;
use umbra_wire::handshake::{
    HandshakeInit as WireHandshakeInit,
    HandshakeResp as WireHandshakeResp,
};
use umbra_crypto::handshake::{
    HandshakeInit as CryptoHandshakeInit,
    HandshakeResp as CryptoHandshakeResp,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use tracing::{debug, info};

/// Events emitted by the handshake protocol
#[derive(Debug)]
pub enum HandshakeEvent {
    /// Handshake completed successfully
    Completed {
        peer_id: PeerId,
        session_key: [u8; 32],
        verify_key: VerifyingKey,
    },
    /// Handshake failed
    Failed {
        peer_id: PeerId,
        error: String,
    },
}

/// Handshake protocol behaviour
pub struct HandshakeBehaviour {
    identity: SigningKey,
    peer_keys: HashMap<PeerId, VerifyingKey>,
    pending_events: VecDeque<HandshakeEvent>,
    session_keys: HashMap<PeerId, [u8; 32]>,
}

impl HandshakeBehaviour {
    pub fn new(identity: SigningKey) -> Self {
        Self {
            identity,
            peer_keys: HashMap::new(),
            pending_events: VecDeque::new(),
            session_keys: HashMap::new(),
        }
    }

    /// Register a peer's verify key (must be done before handshake)
    pub fn register_peer(&mut self, peer_id: PeerId, verify_key: VerifyingKey) {
        self.peer_keys.insert(peer_id, verify_key);
    }

    /// Get session key for a peer (if handshake completed)
    pub fn get_session_key(&self, peer_id: &PeerId) -> Option<&[u8; 32]> {
        self.session_keys.get(peer_id)
    }

    /// Initiate handshake with a peer
    pub fn initiate_handshake(&mut self, peer_id: PeerId) -> Result<(), String> {
        if self.session_keys.contains_key(&peer_id) {
            return Ok(()); // Already have session key
        }

        debug!("Initiating handshake with {}", peer_id);
        
        // Create handshake init message
        let hs = Handshake::new(self.identity.clone())
            .map_err(|e| format!("Failed to create handshake: {:?}", e))?;
        
        let _init = hs.initiate(peer_id)
            .map_err(|e| format!("Failed to initiate handshake: {:?}", e))?;
        
        // TODO: Send via connection handler in Week 2
        
        Ok(())
    }

    // Helper methods for future use (Week 2)
    #[allow(dead_code)]
    fn handle_init(&mut self, peer_id: PeerId, init: &WireHandshakeInit) -> Result<WireHandshakeResp, String> {
        // Convert from wire format
        let crypto_init = CryptoHandshakeInit::try_from(init)
            .map_err(|e| format!("Invalid init message: {}", e))?;

        // Extract and register peer's verify key
        let peer_key = ed25519_dalek::VerifyingKey::from_bytes(&crypto_init.verify_key)
            .map_err(|e| format!("Invalid verify key: {}", e))?;
        self.peer_keys.insert(peer_id, peer_key);

        // Create handshake and respond
        let hs = Handshake::new(self.identity.clone())
            .map_err(|e| format!("Failed to create handshake: {:?}", e))?;

        let (crypto_resp, session_key) = hs.respond(peer_id, &crypto_init, &peer_key)
            .map_err(|e| format!("Failed to respond to handshake: {:?}", e))?;

        // Store session key
        self.session_keys.insert(peer_id, session_key);
        info!("Handshake completed with {} (responder)", peer_id);

        // Emit event with verify key
        self.pending_events.push_back(HandshakeEvent::Completed {
            peer_id,
            session_key,
            verify_key: peer_key,
        });

        // Convert to wire format
        Ok(WireHandshakeResp::from(&crypto_resp))
    }

    #[allow(dead_code)]
    fn handle_resp(&mut self, peer_id: PeerId, resp: &WireHandshakeResp) -> Result<(), String> {
        // Convert from wire format
        let crypto_resp = CryptoHandshakeResp::try_from(resp)
            .map_err(|e| format!("Invalid resp message: {}", e))?;

        // Extract and register peer's verify key
        let peer_key = ed25519_dalek::VerifyingKey::from_bytes(&crypto_resp.verify_key)
            .map_err(|e| format!("Invalid verify key: {}", e))?;
        self.peer_keys.insert(peer_id, peer_key);

        // Complete handshake
        let hs = Handshake::new(self.identity.clone())
            .map_err(|e| format!("Failed to create handshake: {:?}", e))?;

        let session_key = hs.complete(&crypto_resp, &peer_key)
            .map_err(|e| format!("Failed to complete handshake: {:?}", e))?;

        // Store session key
        self.session_keys.insert(peer_id, session_key);
        info!("Handshake completed with {} (initiator)", peer_id);

        // Emit event with verify key
        self.pending_events.push_back(HandshakeEvent::Completed {
            peer_id,
            session_key,
            verify_key: peer_key,
        });

        Ok(())
    }
}

// Simplified NetworkBehaviour implementation (stub for now)
impl NetworkBehaviour for HandshakeBehaviour {
    type ConnectionHandler = libp2p::swarm::dummy::ConnectionHandler;
    type ToSwarm = HandshakeEvent;

    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        _peer: PeerId,
        _local_addr: &libp2p::Multiaddr,
        _remote_addr: &libp2p::Multiaddr,
    ) -> Result<Self::ConnectionHandler, libp2p::swarm::ConnectionDenied> {
        Ok(libp2p::swarm::dummy::ConnectionHandler)
    }

    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        _peer: PeerId,
        _addr: &libp2p::Multiaddr,
        _role_override: libp2p::core::Endpoint,
    ) -> Result<Self::ConnectionHandler, libp2p::swarm::ConnectionDenied> {
        Ok(libp2p::swarm::dummy::ConnectionHandler)
    }

    fn on_swarm_event(&mut self, _event: libp2p::swarm::FromSwarm) {}

    fn on_connection_handler_event(
        &mut self,
        _peer_id: PeerId,
        _connection_id: libp2p::swarm::ConnectionId,
        _event: <Self::ConnectionHandler as ConnectionHandler>::ToBehaviour,
    ) {
    }

    fn poll(
        &mut self,
        _cx: &mut std::task::Context,
    ) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::ToSwarm, <Self::ConnectionHandler as ConnectionHandler>::FromBehaviour>> {
        // Return pending events
        if let Some(event) = self.pending_events.pop_front() {
            return std::task::Poll::Ready(libp2p::swarm::ToSwarm::GenerateEvent(event));
        }
        
        std::task::Poll::Pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_keypair() -> SigningKey {
        SigningKey::from_bytes(&rand::random())
    }

    #[test]
    fn test_handshake_behaviour_creation() {
        let identity = gen_keypair();
        let behaviour = HandshakeBehaviour::new(identity);
        
        assert_eq!(behaviour.session_keys.len(), 0);
        assert_eq!(behaviour.peer_keys.len(), 0);
    }

    #[test]
    fn test_register_peer() {
        let identity = gen_keypair();
        let mut behaviour = HandshakeBehaviour::new(identity);
        
        let peer_id = PeerId::random();
        let peer_key = gen_keypair().verifying_key();
        
        behaviour.register_peer(peer_id, peer_key);
        assert!(behaviour.peer_keys.contains_key(&peer_id));
    }
}
