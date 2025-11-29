use umbra_net::P2PNode;
use anyhow::Result;

pub struct Node {
    p2p: P2PNode,
}

impl Node {
    pub async fn spawn() -> Result<Self> {
        let p2p = P2PNode::new().await?;
        Ok(Self { p2p })
    }
    
    pub fn peer_id(&self) -> String {
        self.p2p.local_peer_id().to_string()
    }
    
    pub fn listening_addresses(&self) -> Vec<String> {
        self.p2p.listening_addresses()
            .iter()
            .map(|addr| addr.to_string())
            .collect()
    }
    
    pub async fn dial(&mut self, addr: &str) -> Result<()> {
        let multiaddr: libp2p::Multiaddr = addr.parse()?;
        self.p2p.dial(multiaddr).await?;
        Ok(())
    }
    
    pub fn subscribe(&mut self, topic: &str) -> Result<()> {
        self.p2p.subscribe(topic)?;
        Ok(())
    }
    
    pub fn publish(&mut self, topic: &str, data: &[u8]) -> Result<()> {
        self.p2p.publish(topic, data.to_vec())?;
        Ok(())
    }
    
    pub fn add_peer(&mut self, peer_id: &str, addr: &str) -> Result<()> {
        let peer_id: libp2p::PeerId = peer_id.parse()?;
        let multiaddr: libp2p::Multiaddr = addr.parse()?;
        self.p2p.add_peer(peer_id, multiaddr);
        Ok(())
    }
    
    pub fn bootstrap(&mut self) -> Result<()> {
        self.p2p.bootstrap()?;
        Ok(())
    }
    
    pub async fn run(&mut self) -> Result<()> {
        self.p2p.run().await?;
        Ok(())
    }
}


