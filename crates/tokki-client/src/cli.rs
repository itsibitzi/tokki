use clap::{Parser, Subcommand};
use url::Url;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long)]
    pub base_url: Url,
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    LoadTest {
        /// The number of messages to send
        #[arg(short, long)]
        count: usize,
        /// The number of messges per batch
        #[arg(short, long)]
        batch_size: usize,
    },
}
