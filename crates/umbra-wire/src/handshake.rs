// Generated protobuf types
pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/umbra.handshake.rs"));
}

use crate::error::{WireError, Result};

// Re-export for convenience
pub use proto::{HandshakeInit, HandshakeResp, HandshakeMessage, handshake_message};

impl HandshakeMessage {
    pub fn encode_to_vec(&self) -> Vec<u8> {
        use prost::Message;
        let mut buf = Vec::new();
        self.encode(&mut buf).expect("encoding should not fail");
        buf
    }
    
    pub fn decode_from_bytes(bytes: &[u8]) -> Result<Self> {
        use prost::Message;
        Self::decode(bytes).map_err(WireError::Decode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;
    
    #[test]
    fn test_handshake_init_roundtrip() {
        let init = HandshakeInit {
            peer_id: vec![1, 2, 3, 4],
            x25519_pk: vec![5u8; 32],
            pq_pk: vec![],
            signature: vec![9u8; 64],
            verify_key: vec![7u8; 32],
        };
        
        let msg = HandshakeMessage {
            message: Some(handshake_message::Message::Init(init.clone())),
        };
        
        let bytes = msg.encode_to_vec();
        let decoded = HandshakeMessage::decode_from_bytes(&bytes).unwrap();
        
        if let Some(handshake_message::Message::Init(decoded_init)) = decoded.message {
            assert_eq!(decoded_init.peer_id, init.peer_id);
            assert_eq!(decoded_init.x25519_pk, init.x25519_pk);
            assert_eq!(decoded_init.signature, init.signature);
            assert_eq!(decoded_init.verify_key, init.verify_key);
        } else {
            panic!("Expected Init message");
        }
    }
    
    #[test]
    fn test_handshake_resp_roundtrip() {
        let resp = HandshakeResp {
            peer_id: vec![1, 2, 3, 4],
            x25519_pk: vec![5u8; 32],
            pq_ct: vec![],
            signature: vec![9u8; 64],
            verify_key: vec![7u8; 32],
        };
        
        let msg = HandshakeMessage {
            message: Some(handshake_message::Message::Resp(resp.clone())),
        };
        
        let bytes = msg.encode_to_vec();
        let decoded = HandshakeMessage::decode_from_bytes(&bytes).unwrap();
        
        if let Some(handshake_message::Message::Resp(decoded_resp)) = decoded.message {
            assert_eq!(decoded_resp.peer_id, resp.peer_id);
            assert_eq!(decoded_resp.x25519_pk, resp.x25519_pk);
            assert_eq!(decoded_resp.signature, resp.signature);
            assert_eq!(decoded_resp.verify_key, resp.verify_key);
        } else {
            panic!("Expected Resp message");
        }
    }
}
