use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Expose your local server to the internet.")]
pub struct Cli {
    #[arg(short, long)]
    pub port: u16,

    #[arg(short, long)]
    pub subdomain: Option<String>,

    #[arg(long, default_value = "https://loca.lt")]
    pub host: String,
}