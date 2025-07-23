mod cli;
mod tunnel;

use anyhow::Result;
use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_target(false) 
        .compact()
        .init();

    let cli = Cli::parse();

    if let Err(e) = tunnel::start_tunnel(&cli).await {
        tracing::error!("An error occurred: {}", e);

        if let Some(source_err) = e.source() {
            if let Some(reqwest_err) = source_err.downcast_ref::<reqwest::Error>() {
                if reqwest_err.is_connect() {
                    tracing::error!("Hint: This looks like a connection error. Please check your internet connection, firewall settings, or if the server '{}' is online.", cli.host);
                } else if reqwest_err.is_timeout() {
                    tracing::error!("Hint: The connection timed out. The server might be slow or overloaded.");
                } else if reqwest_err.is_request() {
                     tracing::error!("Hint: There was an error building the request itself.");
                }
            }
        }
        std::process::exit(1);
    }

    Ok(())
}