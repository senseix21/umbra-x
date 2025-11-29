use anyhow::Result;

/// Desktop app (Tauri UI to be added in W7-W9)
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("UMBRA Desktop (UI coming in Phase C - W7-W9)");
    println!("For now, use: cargo run --example simple_chat");
    
    Ok(())
}
