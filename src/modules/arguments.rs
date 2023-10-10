use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long)]
    pub site_domain: String,

    #[arg(short, long)]
    pub whois: bool
}