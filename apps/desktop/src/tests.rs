#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_node_starts() {
        let state = AppState::default();
        let result = start_node(tauri::State::from(&state)).await;
        assert!(result.is_ok());
        
        let peer_id = result.unwrap();
        assert!(!peer_id.is_empty());
        println!("✅ Node started with PeerID: {}", peer_id);
    }
}
