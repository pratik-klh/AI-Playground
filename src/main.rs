use ai_playground::AIPlayground;
use anyhow::Result;
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting AI Playground...");
    
    // Create and run the playground
    let mut playground = AIPlayground::new();
    
    if let Err(e) = playground.run().await {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
    
    info!("AI Playground finished successfully");
    Ok(())
} 