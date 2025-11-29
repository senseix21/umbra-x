mod chat;

use anyhow::Result;
use chat::ChatSession;
use clap::{Parser, Subcommand};
use tracing::info;
use umbra_net::P2PNode;

#[derive(Parser)]
#[command(name = "umbra")]
#[command(about = "UMBRA.chat - Post-quantum private P2P chat", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a new chat node
    Start {
        /// Port to listen on (default: random)
        #[arg(short, long)]
        port: Option<u16>,
        
        /// Peer address to connect to (format: /ip4/IP/udp/PORT/quic-v1/p2p/PEER_ID)
        #[arg(short, long)]
        connect: Option<String>,
        
        /// Topic/channel to join (default: "umbra-chat")
        #[arg(short, long, default_value = "umbra-chat")]
        topic: String,
        
        /// Username to display
        #[arg(short, long, default_value = "anon")]
        username: String,
    },
    
    /// Show node info
    Info,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Start { port, connect, topic, username } => {
            start_chat(port, connect, topic, username).await?;
        }
        Commands::Info => {
            show_info().await?;
        }
    }
    
    Ok(())
}

async fn start_chat(port: Option<u16>, connect: Option<String>, topic: String, username: String) -> Result<()> {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║              UMBRA.chat - Secure P2P Messaging                 ║");
    println!("║  Post-quantum encrypted • No servers • No trace • No spam      ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
    
    // Create node with specific port if provided
    let mut node = if let Some(p) = port {
        info!("Starting node on port {}...", p);
        P2PNode::new_with_port(p).await?
    } else {
        info!("Starting node on random port...");
        P2PNode::new().await?
    };
    
    let peer_id = node.local_peer_id();
    let addrs = node.listening_addresses();
    
    println!("✓ Node started successfully!");
    println!("✓ Your Peer ID: {}", peer_id);
    println!("✓ Listening on:");
    for addr in &addrs {
        println!("  {}", addr);
    }
    
    // Subscribe to topic
    node.subscribe(&topic)?;
    println!("✓ Subscribed to topic: {}", topic);
    
    // Connect to peer if specified
    if let Some(peer_addr) = connect {
        println!("\n📞 Connecting to peer: {}", peer_addr);
        let addr: libp2p::Multiaddr = peer_addr.parse()?;
        node.dial(addr).await?;
        println!("✓ Connection initiated");
        
        // Give time for connection to establish
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
    println!("\n┌────────────────────────────────────────────────────────────────┐");
    println!("│  Chat Ready! Type your message and press Enter to send.       │");
    println!("│  All messages are encrypted with post-quantum crypto.         │");
    println!("│  Commands: /help /peers /quit                                 │");
    println!("└────────────────────────────────────────────────────────────────┘\n");
    
    // Create and run chat session
    let session = ChatSession::new(node, username, topic);
    session.run().await?;
    
    Ok(())
}

async fn show_info() -> Result<()> {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║                    UMBRA.chat - Project Info                   ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");
    println!("Version: 0.1.0-alpha");
    println!("Protocol: QUIC + libp2p");
    println!("Encryption: Post-quantum hybrid (X25519 + ML-KEM)");
    println!("Messaging: MLS (Messaging Layer Security)");
    println!("Privacy: Onion routing + cover traffic");
    println!("Anti-spam: Zero-knowledge rate limiting\n");
    println!("Features:");
    println!("  ✓ P2P mesh networking (libp2p + QUIC)");
    println!("  ✓ Post-quantum cryptography");
    println!("  ✓ End-to-end encrypted groups (MLS)");
    println!("  ✓ Zero-knowledge proofs (RLN)");
    println!("  ✓ RAM-only mode (ephemeral)");
    println!("  ✓ Onion routing (3-hop circuits)");
    println!("  ✓ Cover traffic for metadata protection\n");
    println!("For more info: https://github.com/yourusername/umbra-chat\n");
    
    Ok(())
}
