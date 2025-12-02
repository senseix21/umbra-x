use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{Emitter, State};
use tokio::sync::Mutex;
use umbra_net::P2PNode;

#[derive(Default)]
struct AppState {
    node: Arc<Mutex<Option<P2PNode>>>,
    dial_tx: Arc<Mutex<Option<tokio::sync::mpsc::UnboundedSender<libp2p::Multiaddr>>>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct PeerInfo {
    id: String,
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct MessagePayload {
    sender: String,
    content: String,
    timestamp: u64,
}

#[tauri::command]
async fn start_node(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let mut node_lock = state.node.lock().await;
    
    if node_lock.is_some() {
        return Err("Node already running".into());
    }
    
    let mut node = P2PNode::new()
        .await
        .map_err(|e| e.to_string())?;
    
    let peer_id = node.local_peer_id().to_string();
    println!("🚀 Node started with PeerID: {}", peer_id);
    
    // Subscribe to a default topic for discovery
    let default_topic = "umbra-global";
    println!("📡 Subscribing to discovery topic: {}", default_topic);
    node.subscribe(default_topic)
        .map_err(|e| format!("Failed to subscribe: {}", e))?;
    
    // Try to bootstrap DHT (optional - won't fail if no peers)
    println!("🌐 Attempting DHT bootstrap...");
    match node.bootstrap() {
        Ok(_) => println!("✅ DHT bootstrap initiated"),
        Err(e) => println!("⚠️  DHT bootstrap skipped: {} (this is OK for local networks)", e),
    }
    
    // Take message receiver before storing node
    let mut message_rx = node.take_message_receiver()
        .ok_or("Failed to get message receiver")?;
    
    // Take connection receiver
    let mut connection_rx = node.take_connection_receiver()
        .ok_or("Failed to get connection receiver")?;
    
    // Create dial channel
    let (dial_tx, mut dial_rx) = tokio::sync::mpsc::unbounded_channel::<libp2p::Multiaddr>();
    
    // Test the channel
    println!("📡 Dial channel created");
    
    // Store node and dial sender
    *node_lock = Some(node);
    let mut tx_lock = state.dial_tx.lock().await;
    *tx_lock = Some(dial_tx);
    drop(tx_lock);
    drop(node_lock);
    
    // Clone the Arc for the background task
    let node_arc = state.node.clone();
    
    // Spawn message receiver task
    let app_clone = app.clone();
    tokio::spawn(async move {
        while let Some((peer_id, data)) = message_rx.recv().await {
            let payload = MessagePayload {
                sender: peer_id.to_string(),
                content: String::from_utf8_lossy(&data).to_string(),
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            };
            let _ = app_clone.emit("message_received", payload);
        }
    });
    
    // Spawn connection receiver task
    let app_clone2 = app.clone();
    tokio::spawn(async move {
        while let Some(peer_id) = connection_rx.recv().await {
            println!("🎉 Connection established with {}", peer_id);
            let _ = app_clone2.emit("peer_connected", PeerInfo {
                id: peer_id.to_string(),
                name: None,
            });
        }
    });
    
    // Spawn network poll task with dial handling
    tokio::spawn(async move {
        println!("🔄 Poll loop started");
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
            
            // Process any pending dial requests
            while let Ok(addr) = dial_rx.try_recv() {
                println!("📞 Processing dial request: {}", addr);
                let mut node_guard = node_arc.lock().await;
                if let Some(ref mut node) = *node_guard {
                    match node.dial(addr.clone()) {
                        Ok(_) => println!("✅ Dial executed for {}", addr),
                        Err(e) => eprintln!("❌ Dial failed: {}", e),
                    }
                }
                drop(node_guard);
            }
            
            // Short-lived lock for polling
            let poll_result = {
                let mut node_guard = node_arc.lock().await;
                if let Some(ref mut node) = *node_guard {
                    node.poll_once().await
                } else {
                    return; // Node gone, exit
                }
            }; // Lock dropped
            
            if let Err(e) = poll_result {
                eprintln!("❌ Poll error: {}", e);
            }
        }
    });
    
    Ok(peer_id)
}

#[tauri::command]
async fn get_listen_addrs(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let node_lock = state.node.lock().await;
    let node = node_lock.as_ref().ok_or("Node not started")?;
    
    Ok(node.listening_addresses()
        .iter()
        .map(|a| a.to_string())
        .collect())
}

#[tauri::command]
async fn connect_peer(
    peer_input: String,
    state: State<'_, AppState>,
    _app: tauri::AppHandle,
) -> Result<(), String> {
    println!("📥 connect_peer called with: '{}'", peer_input);
    
    let peer_input = peer_input.trim();
    println!("📝 Trimmed input: '{}'", peer_input);
    
    // Build multiaddr BEFORE locking
    let multiaddr = if peer_input.starts_with("/ip4/") || peer_input.starts_with("/ip6/") {
        println!("✅ Full multiaddr detected");
        peer_input.to_string()
    } else {
        println!("🔧 Parsing simple format...");
        if let Some((addr_part, peer_id)) = peer_input.split_once('/') {
            println!("  addr_part: '{}', peer_id: '{}'", addr_part, peer_id);
            if let Some((ip, port_str)) = addr_part.split_once(':') {
                // Validate port range
                let port: u16 = port_str.parse()
                    .map_err(|_| {
                        let err = format!("Invalid port '{}'. Must be 0-65535", port_str);
                        println!("❌ {}", err);
                        err
                    })?;
                
                let addr = format!("/ip4/{}/udp/{}/quic-v1/p2p/{}", ip, port, peer_id);
                println!("  Built multiaddr: '{}'", addr);
                addr
            } else {
                let err = "Format: IP:PORT/PEER_ID or full multiaddr".to_string();
                println!("❌ {}", err);
                return Err(err);
            }
        } else {
            let err = "Format: IP:PORT/PEER_ID or full multiaddr".to_string();
            println!("❌ {}", err);
            return Err(err);
        }
    };
    
    println!("🔌 Dialing: {}", multiaddr);
    
    let addr: libp2p::Multiaddr = multiaddr.parse()
        .map_err(|e| {
            let err = format!("Invalid address: {}", e);
            println!("❌ Parse error: {}", err);
            err
        })?;
    
    println!("📡 Parsed multiaddr OK, sending dial request...");
    
    // Send dial request via channel - non-blocking
    let tx_lock = state.dial_tx.lock().await;
    if let Some(ref tx) = *tx_lock {
        println!("📤 Sending to dial channel...");
        tx.send(addr).map_err(|e| {
            let err = format!("Failed to send dial request: {}", e);
            println!("❌ {}", err);
            err
        })?;
        println!("📬 Sent successfully to channel");
    } else {
        return Err("Node not started".to_string());
    }
    drop(tx_lock);
    
    println!("✅ Dial request queued!");
    
    let peer_id = multiaddr.split("/p2p/").last().unwrap_or(peer_input).to_string();
    println!("👤 Peer ID: {}", peer_id);
    println!("⏳ Waiting for connection to establish...");
    
    Ok(())
}

#[tauri::command]
async fn send_message(
    topic: String,
    content: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut node_lock = state.node.lock().await;
    let node = node_lock.as_mut().ok_or("Node not started")?;
    
    node.publish(&topic, content.as_bytes().to_vec())
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn subscribe_topic(
    topic: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut node_lock = state.node.lock().await;
    let node = node_lock.as_mut().ok_or("Node not started")?;
    
    node.subscribe(&topic)
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();
    
    tauri::Builder::default()
        .manage(AppState::default())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            start_node,
            get_listen_addrs,
            connect_peer,
            send_message,
            subscribe_topic,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    run()
}
