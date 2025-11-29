use anyhow::Result;
use tracing::info;

/// Simple chat example (expanded in later phases)
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("UMBRA Simple Chat Example");
    info!("=========================");
    
    let mut node = umbra_sdk::Node::spawn().await?;
    info!("✓ Node started");
    info!("✓ Peer ID: {}", node.peer_id());
    
    // TODO (W3-W6): Join topic, send/receive messages
    // TODO (W7-W9): MLS group creation
    // TODO (W10-W13): ZK proof generation
    
    info!("\nNode is running. Features coming in upcoming phases:");
    info!("  - W3-W6: Message sending/receiving with onion routing");
    info!("  - W7-W9: End-to-end encrypted groups (MLS)");
    info!("  - W10-W13: Zero-knowledge proofs for anti-spam");
    
    node.run().await?;
    
    Ok(())
}
