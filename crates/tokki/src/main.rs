use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use clap::Parser as _;
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio::time::sleep;
use tokki_api::{TokkiClient, clustering::ReplicateLogRequest};
use tracing_subscriber::EnvFilter;

use tokki::{
    app_state::{AppState, AppStateInner},
    cli::{Cli, CliMode, CliStorageEngine},
    replication::Replication,
    server::{create_router, listen},
    server_error::ServerError,
    storage::{InMemoryChannelStorage, InMemoryLockFree, InMemoryStorage, Storage},
};

#[tokio::main]
async fn main() -> Result<(), ServerError> {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    PrometheusBuilder::new()
        .with_http_listener(([0, 0, 0, 0], 8050))
        .install()
        .unwrap();

    let addr: SocketAddr = ([0, 0, 0, 0], cli.port).into();

    let storage: Arc<dyn Storage> = match cli.storage {
        CliStorageEngine::InMemoryMutex => Arc::new(InMemoryStorage::default()),
        CliStorageEngine::InMemoryChannel => Arc::new(InMemoryChannelStorage::new().await.unwrap()),
        CliStorageEngine::InMemoryLockFree => Arc::new(InMemoryLockFree::new()),
    };

    let token = cli.token;

    let app_state = match cli.mode {
        CliMode::Leader { required_replicas } => AppState {
            profiling_enabled: cli.enable_profiling,
            inner: Arc::new(AppStateInner::Leader {
                token,
                storage,
                required_replicas,
                replication: Arc::new(Mutex::new(Replication::new(required_replicas))),
            }),
        },
        CliMode::Follower { leader } => {
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
                        let req = ReplicateLogRequest::new(
                            follower_url.clone(),
                            storage.max_offset().await.expect("Get max offset"),
                        );

                        let res = leader_client
                            .replicate_records(req, &token)
                            .await
                            .expect("Send request")
                            .into_verified(&token)
                            .expect("good token");

                        if res.records.is_empty() {
                            if backoff_ms < 1000 {
                                backoff_ms += 10;
                            }
                        } else {
                            for r in res.records {
                                storage.put_record(r).await.expect("put record");
                            }
                            backoff_ms = 10;
                        }
                        sleep(Duration::from_millis(backoff_ms)).await;
                    }
                }
            });

            AppState {
                profiling_enabled: cli.enable_profiling,
                inner: Arc::new(AppStateInner::Follower {
                    storage,
                    leader_client,
                }),
            }
        }
    };

    let app = create_router(app_state);

    listen(app, addr).await?;

    Ok(())
}
