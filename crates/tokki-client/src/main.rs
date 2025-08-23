use clap::Parser;

use crate::{cli::Cli, commands::load_test};

mod cli;
mod commands;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::CliCommand::LoadTest { count, batch_size } => {
            load_test(cli.base_url, count, batch_size).await
        }
    }
}
