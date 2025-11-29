use anyhow::Result;
use tracing::info;

/// Headless UMBRA node for relays/gateways
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    info!("Starting UMBRA headless node...");
    let mut node = umbra_sdk::Node::spawn().await?;
    
    info!("Node ID: {}", node.peer_id());
    info!("Running as relay/gateway...");
    
    node.run().await?;
    
    Ok(())
}
