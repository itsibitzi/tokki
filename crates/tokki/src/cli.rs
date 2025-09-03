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
    #[arg(long)]
    pub storage: CliStorageEngine,
    /// Should profiling endpoints be enabled?
    #[arg(long)]
    pub enable_profiling: bool,
    #[command(subcommand)]
    pub mode: CliMode,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum CliStorageEngine {
    InMemoryMutex,
    InMemoryChannel,
    InMemoryLockFree,
}

#[derive(Debug, Subcommand)]
pub enum CliMode {
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
