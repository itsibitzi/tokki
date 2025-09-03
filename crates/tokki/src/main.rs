use std::{net::SocketAddr, sync::Arc};

use clap::Parser as _;
use metrics_exporter_prometheus::PrometheusBuilder;
use tracing_subscriber::EnvFilter;

use tokki::{
    app_state::AppState,
    cli::{Cli, CliMode, CliStorageEngine},
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
        CliMode::Leader { required_replicas } => AppState::builder()
            .leader()
            .with_profiling_enabled(cli.enable_profiling)
            .with_storage(storage)
            .with_token(token)
            .with_required_replicas(required_replicas)
            .build(),
        CliMode::Follower { leader } => AppState::builder()
            .follower()
            .with_profiling_enabled(cli.enable_profiling)
            .with_leader(leader)
            .with_socket_addr(addr)
            .with_storage(storage)
            .with_token(token)
            .build(),
    };

    let app = create_router(app_state);

    listen(app, addr).await?;

    Ok(())
}
