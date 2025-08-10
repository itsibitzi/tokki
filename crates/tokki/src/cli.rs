use clap::{Parser, Subcommand};
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Used for authenticating requests within the cluster
    #[arg(short, long)]
    pub token: String,
    /// Port to listen on
    #[arg(short, long, default_value_t = 9999)]
    pub port: u16,

    #[command(subcommand)]
    pub mode: Mode,
}

#[derive(Debug, Subcommand)]
pub enum Mode {
    /// Start this node as a leader, follows will copy the log
    Leader {
        /// The number of replicas required for a record before it is considered committed
        #[arg(short, long)]
        required_replicas: usize,
    },
    /// Start this node as a follower, copies the leaders log
    Follower {
        /// The URL of the leader this node will replicate from
        #[arg(short, long)]
        leader: Url,
    },
}
