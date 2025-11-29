use anyhow::Result;
use tracing::info;
use std::env;

/// Two-node mesh example demonstrating QUIC P2P discovery
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("node1");
    
    match mode {
        "node1" => run_node1().await,
        "node2" => {
            let peer_addr = args.get(2)
                .ok_or_else(|| anyhow::anyhow!("Node2 requires peer address as argument"))?;
            run_node2(peer_addr).await
        }
        _ => {
            println!("Usage:");
            println!("  Terminal 1: cargo run --example hello_mesh node1");
            println!("  Terminal 2: cargo run --example hello_mesh node2 <addr_from_node1>");
            Ok(())
        }
    }
}

async fn run_node1() -> Result<()> {
    info!("🚀 Starting UMBRA Node 1...");
    
    let mut node = umbra_sdk::Node::spawn().await?;
    info!("✓ Node 1 started");
    info!("✓ Peer ID: {}", node.peer_id());
    
    // Wait for listening to be established
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    let addrs = node.listening_addresses();
    info!("");
    info!("📡 Node 1 is ready and listening on:");
    for addr in &addrs {
        info!("   {}", addr);
    }
    info!("");
    info!("📋 In terminal 2, run:");
    if let Some(addr) = addrs.first() {
        info!("   cargo run --example hello_mesh node2 {}", addr);
    }
    info!("");
    
    // Run the node
    node.run().await?;
    
    Ok(())
}

async fn run_node2(peer_addr: &str) -> Result<()> {
    info!("🚀 Starting UMBRA Node 2...");
    
    let mut node = umbra_sdk::Node::spawn().await?;
    info!("✓ Node 2 started");
    info!("✓ Peer ID: {}", node.peer_id());
    
    // Dial Node 1
    info!("📞 Dialing Node 1 at: {}", peer_addr);
    node.dial(peer_addr).await?;
    info!("✓ Dialed Node 1 successfully");
    
    info!("");
    info!("📡 Node 2 connected! Ping messages will be exchanged automatically.");
    info!("");
    
    // Run the node
    node.run().await?;
    
    Ok(())
}


