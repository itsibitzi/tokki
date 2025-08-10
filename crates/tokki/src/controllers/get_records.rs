use api::{
    clustering::{ReplicateLogRequest, ReplicateLogResponse},
    get_records::{GetRecordsRequest, GetRecordsResponse},
};
use axum::{Json, extract::State};
use common::hmac::HmacForm;
use snafu::ResultExt as _;

use crate::{
    app_state::AppState,
    server_error::{HmacSnafu, ServerError},
};

pub async fn get_records(
    State(state): State<AppState>,
    Json(req): Json<GetRecordsRequest>,
) -> Json<GetRecordsResponse> {
    let records = state.storage().get_records(req.offset, req.max_records);
    let res = GetRecordsResponse::new(records.0, records.1);

    Json(res)
}

pub async fn get_records_for_replication(
    State(state): State<AppState>,
    Json(req): Json<HmacForm<ReplicateLogRequest>>,
) -> Result<Json<HmacForm<ReplicateLogResponse>>, ServerError> {
    match state {
        AppState::Leader {
            token,
            storage,
            replication,
        } => {
            let req = req.into_verified(&token).context(HmacSnafu)?;

            tracing::trace!(
                "{} replicated to {:?}",
                req.follower_url,
                req.max_acknowledged_offset
            );

            {
                let mut guard = replication.lock().expect("not poisoned");
                guard.update_follower_max_offset(req.follower_url, req.max_acknowledged_offset);
            }

            let next_batch_offset = req
                .max_acknowledged_offset
                .map(|offset| offset + 1)
                .unwrap_or_default();
            let records = storage.get_records(next_batch_offset, 10);
            let response = ReplicateLogResponse::new(records.0);

            let form = HmacForm::new(response, &token);

            Ok(Json(form))
        }
        AppState::Follower { leader_client, .. } => Err(ServerError::IsFollower {
            leader: leader_client.base_url().to_string(),
        }),
    }
}
