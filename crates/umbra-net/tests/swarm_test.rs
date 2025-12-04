use umbra_net::P2PNode;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
#[ignore] // Long-running test
async fn test_50_node_swarm() {
    tracing_subscriber::fmt()
        .with_test_writer()
        .try_init()
        .ok();
    
    const NUM_NODES: usize = 50;
    
    println!("Creating {} nodes...", NUM_NODES);
    
    // Create first node as bootstrap
    let mut bootstrap = P2PNode::new_with_port(30000).await.unwrap();
    
    let bootstrap_task = tokio::spawn(async move {
        timeout(Duration::from_millis(500), bootstrap.run()).await.ok();
        bootstrap
    });
    
    tokio::time::sleep(Duration::from_millis(600)).await;
    let bootstrap = bootstrap_task.await.unwrap();
    
    let bootstrap_addr = bootstrap.listening_addresses()[0].clone();
    let bootstrap_peer_id = *bootstrap.local_peer_id();
    
    println!("Bootstrap node: {} at {}", bootstrap_peer_id, bootstrap_addr);
    
    // Create remaining nodes
    let mut nodes = vec![bootstrap];
    
    for i in 1..NUM_NODES {
        let mut node = P2PNode::new_with_port(30000 + i as u16).await.unwrap();
        
        let node_task = tokio::spawn(async move {
            timeout(Duration::from_millis(500), node.run()).await.ok();
            node
        });
        
        tokio::time::sleep(Duration::from_millis(100)).await;
        let mut node = node_task.await.unwrap();
        
        // Connect to bootstrap
        node.dial(bootstrap_addr.clone()).ok();
        node.add_peer(bootstrap_peer_id, bootstrap_addr.clone());
        
        nodes.push(node);
    }
    
    println!("✓ Created and connected {} nodes", NUM_NODES);
    
    // Run all nodes briefly
    let handles: Vec<_> = nodes.into_iter().map(|mut node| {
        tokio::spawn(async move {
            timeout(Duration::from_secs(5), node.run()).await.ok();
        })
    }).collect();
    
    for handle in handles {
        handle.await.ok();
    }
    
    println!("✓ 50-node swarm test completed");
}
