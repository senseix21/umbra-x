// End-to-end test for encrypted message exchange

use umbra_net::MessageExchange;
use libp2p::PeerId;

#[test]
fn test_message_exchange_roundtrip() {
    let mut alice = MessageExchange::new().unwrap();
    let mut bob = MessageExchange::new().unwrap();
    
    let alice_peer = PeerId::random();
    let bob_peer = PeerId::random();
    
    // Alice encrypts a message
    let encrypted = alice.encrypt_message(
        bob_peer,
        "alice",
        "Hello Bob!",
    ).unwrap();
    
    // Bob decrypts (using same peer ID = same session key)
    let (username, content) = bob.decrypt_message(bob_peer, &encrypted).unwrap();
    
    assert_eq!(username, "alice");
    assert_eq!(content, "Hello Bob!");
}

#[test]
fn test_message_exchange_wrong_peer() {
    let mut alice = MessageExchange::new().unwrap();
    let mut eve = MessageExchange::new().unwrap();
    
    let alice_peer = PeerId::random();
    let eve_peer = PeerId::random();
    
    // Alice encrypts for alice_peer
    let encrypted = alice.encrypt_message(
        alice_peer,
        "alice",
        "Secret message",
    ).unwrap();
    
    // Eve tries with different peer ID
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

