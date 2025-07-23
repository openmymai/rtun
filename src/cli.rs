use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Expose your local server to the internet.")]
pub struct Cli {
    /// The local port you want to expose (e.g., 3000, 8080)
    #[arg(short, long)]
    pub port: u16,

    /// Request a specific subdomain. (e.g., my-cool-app)
    #[arg(short, long)]
    pub subdomain: Option<String>,

    /// The host of the tunneling server.
    #[arg(long, default_value = "https://localtunnel.me")]
    pub host: String,
}