use axum::{Json, extract::State};
use snafu::ResultExt;
use tokki_api::profiling::FinishProfilingResponse;

use crate::{
    app_state::AppState,
    controller_error::{
        ControllerError, FlamegraphSnafu, ProfilingReportSnafu, ProfilingStartFailedSnafu,
    },
};

pub async fn start_profiling(
    State(state): State<AppState>,
) -> Result<Json<FinishProfilingResponse>, ControllerError> {
    if !state.profiling_enabled {
        return Err(ControllerError::ProfilingDisabled);
    }

    tracing::info!("Starting profiling");

    // // Run profiling in a blocking task since it's CPU-intensive and interacts with OS
    // let flamegraph = tokio::task::spawn_blocking(|| {
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(1000)
        .build()
        .context(ProfilingStartFailedSnafu)?;

    // TODO make customizable
    std::thread::sleep(std::time::Duration::from_secs(5));

    // Generate report
    let report = guard.report().build().context(ProfilingReportSnafu)?;
    let mut flamegraph = Vec::new();
    report
        .flamegraph(&mut flamegraph)
        .context(ProfilingReportSnafu)?;

    let flamegraph = String::from_utf8(flamegraph).context(FlamegraphSnafu)?;
    // })
    // .await
    // .unwrap()?;

    tracing::info!("Profiling completed");

    let res = FinishProfilingResponse { flamegraph };

    Ok(Json(res))
}
