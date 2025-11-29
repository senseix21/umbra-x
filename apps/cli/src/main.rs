mod chat;
mod ui;

use anyhow::Result;
use chat::ChatSession;
use clap::{Parser, Subcommand};
use tracing::info;
use umbra_net::P2PNode;
use ui::UI;

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
    UI::print_banner();
    
    // Create node with specific port if provided
    let mut node = if let Some(p) = port {
        info!("Starting node on port {}...", p);
        UI::print_spinner("Initializing P2P node...");
        P2PNode::new_with_port(p).await?
    } else {
        info!("Starting node on random port...");
        UI::print_spinner("Initializing P2P node...");
        P2PNode::new().await?
    };
    
    let peer_id = node.local_peer_id();
    let addrs = node.listening_addresses();
    
    UI::print_success("Node started successfully!");
    UI::print_node_info(peer_id, &addrs);
    
    // Subscribe to topic
    node.subscribe(&topic)?;
    UI::print_success(&format!("Subscribed to topic: {}", topic));
    
    // Connect to peer if specified
    if let Some(peer_addr) = connect {
        UI::print_connecting(&peer_addr);
        let addr: libp2p::Multiaddr = peer_addr.parse()?;
        node.dial(addr).await?;
        UI::print_success("Connection initiated");
        
        // Give time for connection to establish
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
    UI::print_chat_ready();
    
    // Create and run chat session
    let session = ChatSession::new(node, username, topic);
    session.run().await?;
    
    Ok(())
}

async fn show_info() -> Result<()> {
    UI::print_info();
    Ok(())
}
