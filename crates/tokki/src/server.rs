use std::net::SocketAddr;

use axum::{
    Router,
    routing::{get, put},
};
use snafu::ResultExt as _;

use crate::{
    app_state::AppState,
    controllers::{
        get_healthcheck, get_records, get_records_for_replication, get_shards, put_records,
        start_profiling,
    },
    server_error::{PortBindSnafu, ServeSnafu, ServerError},
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/healthcheck", get(get_healthcheck))
        .route("/shards", get(get_shards))
        .route("/records", get(get_records))
        .route("/records", put(put_records))
        .route("/replication", get(get_records_for_replication))
        .route("/profiling/start", get(start_profiling))
        .layer(axum_metrics::MetricLayer::default())
        .with_state(app_state)
}

pub async fn listen(app: Router, addr: SocketAddr) -> Result<(), ServerError> {
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context(PortBindSnafu { port: addr.port() })?;

    tracing::info!("Server running on {}", addr);

    axum::serve(listener, app).await.context(ServeSnafu)?;

    Ok(())
}
