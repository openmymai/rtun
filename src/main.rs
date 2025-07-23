// src/main.rs

mod cli;
mod tunnel;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false) // Hide module paths for cleaner output
        .compact()
        .init();

    let cli = Cli::parse();

    if let Err(e) = tunnel::start_tunnel(&cli).await {
        tracing::error!("An error occurred: {}", e);
        std::process::exit(1);
    }

    Ok(())
}