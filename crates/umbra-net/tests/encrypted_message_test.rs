// End-to-end test for encrypted message exchange

use umbra_net::MessageExchange;
use libp2p::PeerId;

#[test]
fn test_message_exchange_roundtrip() {
    let mut alice = MessageExchange::new().unwrap();
    let mut bob = MessageExchange::new().unwrap();
    
    let peer_id = PeerId::random(); // Use same peer ID for same session
    
    // Register each other's public keys for signature verification
    let alice_pubkey = alice.session_manager().public_key();
    let bob_pubkey = bob.session_manager().public_key();
    alice.session_manager_mut().register_peer(peer_id, bob_pubkey);
    bob.session_manager_mut().register_peer(peer_id, alice_pubkey);
    
    // Alice encrypts a message
    let encrypted = alice.encrypt_message(
        peer_id,
        "alice",
        "Hello Bob!",
    ).unwrap();
    
    // Bob decrypts and verifies signature
    let (username, content) = bob.decrypt_message(peer_id, &encrypted).unwrap();
    
    assert_eq!(username, "alice");
    assert_eq!(content, "Hello Bob!");
}

#[test]
fn test_message_exchange_wrong_peer() {
    let mut alice = MessageExchange::new().unwrap();
    let mut eve = MessageExchange::new().unwrap();
    
    let alice_peer = PeerId::random();
    let eve_peer = PeerId::random();
    
    // Register keys
    let alice_pubkey = alice.session_manager().public_key();
    eve.session_manager_mut().register_peer(alice_peer, alice_pubkey);
    
    // Alice encrypts for alice_peer
    let encrypted = alice.encrypt_message(
        alice_peer,
        "alice",
        "Secret message",
    ).unwrap();
    
    // Eve tries with different peer ID (wrong decryption key)
    let result = eve.decrypt_message(eve_peer, &encrypted);
    assert!(result.is_err(), "Should fail with wrong peer");
}

#[test]
fn test_message_exchange_multiple_messages() {
    let mut alice = MessageExchange::new().unwrap();
    let peer = PeerId::random();
    
    // Send 5 messages
    for i in 0..5 {
        alice.encrypt_message(
            peer,
            "alice",
            &format!("Message {}", i),
        ).unwrap();
    }
    
    // Check session counter
    assert_eq!(alice.session_count(), 1);
}

#[test]
fn test_signature_verification_success() {
    let mut alice = MessageExchange::new().unwrap();
    let mut bob = MessageExchange::new().unwrap();
    
    let alice_peer = PeerId::random();
    
    // Bob registers Alice's public key
    let alice_pubkey = alice.session_manager().public_key();
    bob.session_manager_mut().register_peer(alice_peer, alice_pubkey);
    
    // Alice sends message
    let encrypted = alice.encrypt_message(alice_peer, "alice", "signed message").unwrap();
    
    // Bob decrypts and verifies (should succeed)
    let result = bob.decrypt_message(alice_peer, &encrypted);
    assert!(result.is_ok(), "Valid signature should verify");
}

#[test]
fn test_signature_verification_fails_wrong_key() {
    let mut alice = MessageExchange::new().unwrap();
    let mut bob = MessageExchange::new().unwrap();
    let mut eve = MessageExchange::new().unwrap();
    
    let alice_peer = PeerId::random();
    
    // Bob registers EVE's public key instead of Alice's (wrong key!)
    let eve_pubkey = eve.session_manager().public_key();
    bob.session_manager_mut().register_peer(alice_peer, eve_pubkey);
    
    // Alice sends message
    let encrypted = alice.encrypt_message(alice_peer, "alice", "signed message").unwrap();
    
    // Bob tries to decrypt but signature verification should fail
    let result = bob.decrypt_message(alice_peer, &encrypted);
    assert!(result.is_err(), "Wrong public key should fail verification");
    
    // Check it's a signature error
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Signature") || err_msg.contains("signature"), 
            "Error should mention signature: {}", err_msg);
}

#[test]
fn test_message_tampering_detected() {
    let mut alice = MessageExchange::new().unwrap();
    let mut bob = MessageExchange::new().unwrap();
    
    let peer_id = PeerId::random();
    
    // Register Alice's key with Bob
    let alice_pubkey = alice.session_manager().public_key();
    bob.session_manager_mut().register_peer(peer_id, alice_pubkey);
    
    // Alice sends message
    let mut encrypted = alice.encrypt_message(peer_id, "alice", "original message").unwrap();
    
    // Eve tampers with the ciphertext (flip some bits in the middle)
    let len = encrypted.len();
    if len > 40 {
        encrypted[len / 2] ^= 0xFF; // Flip bits in middle of message
    }
    
    // Bob tries to decrypt - should fail (either decryption or signature verification)
    let result = bob.decrypt_message(peer_id, &encrypted);
    assert!(result.is_err(), "Tampered message should fail");
}
