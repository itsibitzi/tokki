use api::put_record::{PutRecordsRequest, PutRecordsResponse};
use axum::{Json, extract::State};
use common::Offset;
use snafu::ResultExt as _;
use tokio::sync::oneshot;

use crate::{
    app_state::AppState,
    server_error::{LeaderForwardingSnafu, ServerError},
};

pub async fn put_records(
    State(state): State<AppState>,
    Json(req): Json<PutRecordsRequest>,
) -> Result<Json<PutRecordsResponse>, ServerError> {
    match state {
        AppState::Leader {
            replication,
            storage,
            ..
        } => {
            let mut max_offset = 0;
            let mut len = 0;

            for record in req.records {
                let offset = storage.put_record(&record);
                max_offset = offset.0;
                len += 1;
            }

            let wake_rx = {
                let mut guard = replication.lock().expect("not poisoned");
                let (wake_tx, wake_rx) = oneshot::channel();
                guard.register_wait(Offset(max_offset), wake_tx);
                wake_rx
            };

            match tokio::time::timeout(std::time::Duration::from_secs(5), wake_rx).await {
                Ok(_) => {
                    let initial_offset = max_offset - (len - 1);
                    let response = PutRecordsResponse::new(Offset(initial_offset), len);

                    Ok(Json(response))
                }
                Err(_) => {
                    tracing::error!("Timeout waiting for {}", max_offset);
                    return Err(ServerError::Replication { timeout_s: 5 });
                }
            }
        }
        AppState::Follower { leader_client, .. } => leader_client
            .put_record(req)
            .await
            .with_context(|_| LeaderForwardingSnafu {
                leader: leader_client.base_url().to_string(),
            })
            .map(|r| Json(r)),
    }
}
