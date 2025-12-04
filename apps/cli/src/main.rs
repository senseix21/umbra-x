mod chat;
mod ui;

use anyhow::Result;
use chat::ChatSession;
use clap::{Parser, Subcommand};
use tracing::info;
use umbra_net::P2PNode;
use umbra_identity::{Identity, Prover, Storage};
use ui::UI;

#[derive(Parser)]
#[command(name = "umbra")]
#[command(about = "UMBRA.chat - Post-quantum private P2P chat", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Data directory for identity and keys (default: ~/.umbra)
    #[arg(long, global = true)]
    data_dir: Option<String>,
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
    
    /// Identity management
    Identity {
        #[command(subcommand)]
        command: IdentityCommands,
    },
    
    /// Show node info
    Info,
}

#[derive(Subcommand)]
enum IdentityCommands {
    /// Create a new identity
    Create {
        /// Password to derive identity from
        password: String,
    },
    
    /// Show current identity
    Show,
    
    /// Verify an identity proof
    Verify {
        /// Hex-encoded proof bytes
        proof: String,
        
        /// Hex-encoded identity ID
        identity_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    
    // Set data directory globally
    let data_dir = cli.data_dir.clone().unwrap_or_else(|| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.umbra", home)
    });

    match cli.command {
        Commands::Start { port, connect, topic, username } => {
            start_chat(port, connect, topic, username, &data_dir).await?;
        }
        Commands::Identity { command } => {
            handle_identity_command(command, &data_dir).await?;
        }
        Commands::Info => {
            show_info().await?;
        }
    }

    Ok(())
}

async fn start_chat(port: Option<u16>, connect: Option<String>, topic: String, username: String, data_dir: &str) -> Result<()> {
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
        node.dial(addr)?;
        UI::print_success("Connection initiated");
        
        // Give time for connection to establish
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
    UI::print_chat_ready();
    
    // Create and run chat session
    let session = ChatSession::new(node, username, topic, data_dir.to_string());
    session.run().await?;
    
    Ok(())
}

async fn show_info() -> Result<()> {
    UI::print_info();
    Ok(())
}

async fn handle_identity_command(command: IdentityCommands, data_dir: &str) -> Result<()> {
    let storage = Storage::new(data_dir)?;

    match command {
        IdentityCommands::Create { password } => {
            println!("🔐 Creating identity...");
            
            // Create identity
            let identity = Identity::create(&password)?;
            println!("✓ Identity created");
            println!("  ID: {}", hex::encode(&identity.id));
            
            // Save identity
            storage.save_identity(&identity)?;
            println!("✓ Identity saved to {}/umbra_identity.bin", data_dir);
            
            // Setup prover if not exists
            if !storage.has_keys() {
                println!("🔧 Setting up ZK prover (one-time, ~30s)...");
                let prover = Prover::setup()?;
                storage.save_keys(&prover)?;
                println!("✓ Prover keys saved");
            }
            
            println!("\n✅ Identity ready!");
        }
        
        IdentityCommands::Show => {
            if !storage.has_identity() {
                println!("❌ No identity found. Create one with: umbra identity create <password>");
                return Ok(());
            }
            
            let identity = storage.load_identity()?;
            println!("🆔 Current Identity:");
            println!("  ID: {}", hex::encode(&identity.id));
            println!("  Location: {}/umbra_identity.bin", data_dir);
            
            if storage.has_keys() {
                println!("  Prover: ✓ Ready");
            } else {
                println!("  Prover: ✗ Not setup (run create to initialize)");
            }
        }
        
        IdentityCommands::Verify { proof, identity_id } => {
            println!("🔍 Verifying identity proof...");
            
            // Decode inputs
            let proof_bytes = hex::decode(&proof)
                .map_err(|e| anyhow::anyhow!("Invalid proof hex: {}", e))?;
            let id_bytes = hex::decode(&identity_id)
                .map_err(|e| anyhow::anyhow!("Invalid identity_id hex: {}", e))?;
            
            if id_bytes.len() != 32 {
                return Err(anyhow::anyhow!("Identity ID must be 32 bytes"));
            }
            
            let mut id = [0u8; 32];
            id.copy_from_slice(&id_bytes);
            
            // Load or setup prover
            let prover = if storage.has_keys() {
                storage.load_keys()?
            } else {
                println!("⚠️  No prover keys found, setting up...");
                let p = Prover::setup()?;
                storage.save_keys(&p)?;
                p
            };
            
            // Verify
            let valid = umbra_identity::verify_identity_proof(&prover, &proof_bytes, &id)?;
            
            if valid {
                println!("✅ Proof VALID for identity {}", hex::encode(&id[..8]));
            } else {
                println!("❌ Proof INVALID");
            }
        }
    }
    
    Ok(())
}
