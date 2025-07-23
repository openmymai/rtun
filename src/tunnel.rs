use crate::cli::Cli;
use anyhow::{Context, Result};
use colored::*;
use serde::Deserialize;
use std::time::Duration;
use tokio::io::{self}; 
use tokio::net::TcpStream;
use tracing::{error, info, warn};
use url::Url;

#[derive(Deserialize, Debug)]
struct TunnelInfo {
    id: String,
    port: u16,
    url: String,
    max_conn_count: u16,
}

pub async fn start_tunnel(args: &Cli) -> Result<()> {
    info!("Requesting tunnel for localhost:{}...", args.port);

    let client = reqwest::Client::new();
    
    let server_url = match &args.subdomain {
        Some(sd) => format!("{}/?new={}", args.host, sd),
        None => format!("{}/?new", args.host),
    };

    let res = client
        .get(&server_url)
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?;

    if !res.status().is_success() {
        anyhow::bail!(
            "Server returned an error: {} - {}",
            res.status(),
            res.text().await.unwrap_or_default()
        );
    }

    let response_text = res.text().await?;
    let info: TunnelInfo = serde_json::from_str(&response_text)
        .with_context(|| {
            error!("Failed to parse the following server response: <<<{}>>>", response_text);
            "Failed to parse server response"
        })?;

    println!();
    println!("{}", " Tunnel Details ".on_blue().white().bold());
    println!("{}: {}", "Public URL".bold(), info.url.cyan());
    println!("{}: {}", "Local Port".bold(), args.port);
    println!("{}: {}", "Status".bold(), "Live".green());
    println!();
    info!("Waiting for incoming requests...");

    let remote_host = Url::parse(&info.url)?.host_str().unwrap_or("").to_string();
    let base_host = Url::parse(&args.host)?.host_str().unwrap_or("server.loca.lt").to_string();

    let local_addr = format!("127.0.0.1:{}", args.port);

    for _ in 0..info.max_conn_count {
        let remote_addr = format!("{}:{}", base_host, info.port);
        let local_addr_clone = local_addr.clone();
        tokio::spawn(async move {
            loop {
                if let Err(e) = proxy_connection(&remote_addr, &local_addr_clone).await {
                    error!("Proxy connection failed: {}. Retrying in 5s...", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });
    }

    tokio::signal::ctrl_c().await?;
    println!("\n{}", "Shutting down tunnel...".yellow());

    Ok(())
}


async fn proxy_connection(remote_addr: &str, local_addr: &str) -> Result<()> {
    let remote_stream = TcpStream::connect(remote_addr)
        .await
        .with_context(|| format!("Failed to connect to remote server at {}", remote_addr))?;

    log_request(&remote_stream).await;

    let local_stream = TcpStream::connect(local_addr)
        .await
        .with_context(|| format!("Failed to connect to local server at {}", local_addr))?;

    let (mut remote_reader, mut remote_writer) = remote_stream.into_split();
    let (mut local_reader, mut local_writer) = local_stream.into_split();

    tokio::select! {
        res = io::copy(&mut remote_reader, &mut local_writer) => {
            if let Err(e) = res { warn!("Error copying from remote to local: {}", e); }
        },
        res = io::copy(&mut local_reader, &mut remote_writer) => {
            if let Err(e) = res { warn!("Error copying from local to remote: {}", e); }
        }
    }

    Ok(())
}

async fn log_request(remote_stream: &TcpStream) {
    let peer_addr = remote_stream.peer_addr().map(|a| a.to_string()).unwrap_or_else(|_| "unknown".to_string());
    info!(
        "{} from {}",
        "Incoming request".green(),
        peer_addr.yellow()
    );
}