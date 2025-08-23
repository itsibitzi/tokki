use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use axum::{
    Router,
    routing::{get, put},
};
use clap::Parser as _;
use snafu::ResultExt as _;
use tokio::time::sleep;
use tokki_api::{TokkiClient, clustering::ReplicateLogRequest};

use crate::{
    app_state::AppState,
    cli::{Cli, Mode},
    controllers::{get_records, get_records_for_replication, get_shards, put_records},
    error::{Error, PortBindSnafu},
    replication::Replication,
    storage::{InMemoryStorage, Storage},
};

mod app_state;
mod cli;
mod controllers;
mod error;
mod replication;
mod server_error;
mod storage;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    tracing_subscriber::fmt().init();

    let addr: SocketAddr = ([127, 0, 0, 1], cli.port).into();
    let storage = InMemoryStorage::default();

    let token = cli.token;

    let app_state = match cli.mode {
        Mode::Leader { required_replicas } => AppState::Leader {
            token,
            storage,
            required_replicas,
            replication: Arc::new(Mutex::new(Replication::new(required_replicas))),
        },
        Mode::Follower { leader } => {
            let leader_client = TokkiClient::new(leader);

            // Spin off replication background task
            tokio::task::spawn({
                let follower_url = format!("http://{}", addr);
                let token = token.clone();
                let storage = storage.clone();
                let leader_client = leader_client.clone();

                let mut backoff_ms = 100;
                async move {
                    loop {
                        let req =
                            ReplicateLogRequest::new(follower_url.clone(), storage.max_offset());
                        let res = leader_client
                            .replicate_records(req, &token)
                            .await
                            .expect("Send request");
                        let res = res.into_verified(&token).expect("good token");
                        for r in &res.records {
                            storage.put_record(r);
                        }
                        if res.records.is_empty() {
                            if backoff_ms < 1000 {
                                backoff_ms += 10;
                            }
                        } else {
                            backoff_ms = 10;
                        }
                        sleep(Duration::from_millis(backoff_ms)).await;
                    }
                }
            });

            AppState::Follower {
                storage,
                leader_client,
            }
        }
    };

    let app = Router::new()
        .route("/shards", get(get_shards))
        .route("/records", get(get_records))
        .route("/records", put(put_records))
        .route("/replication", get(get_records_for_replication))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context(PortBindSnafu { port: addr.port() })?;

    println!("Server running on {}", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
