use axum::{
    extract::{Path, State},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::{
    model::{HealthResponse, SynthesizeRequest, SynthesizeResponse, VoiceModel},
    service::{AppState, SynthesizeError},
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/v1/health", get(health))
        .route("/v1/models", get(models))
        .route("/v1/synthesize", post(synthesize))
        .route("/v1/audio/:request_id", get(audio))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(state.health())
}

async fn models(State(state): State<AppState>) -> Json<Vec<VoiceModel>> {
    Json((*state.models).clone())
}

async fn synthesize(
    State(state): State<AppState>,
    Json(req): Json<SynthesizeRequest>,
) -> Result<Json<SynthesizeResponse>, (StatusCode, String)> {
    if req.text.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "text cannot be empty".into()));
    }

    state
        .synthesize(req)
        .await
        .map(Json)
        .map_err(map_synthesize_error)
}

async fn audio(State(state): State<AppState>, Path(request_id): Path<String>) -> Response {
    let store = state.audio_store.read().await;
    if let Some(audio) = store.get(&request_id) {
        let content_type = HeaderValue::from_str(&audio.content_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream"));

        (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, content_type)],
            audio.data.clone(),
        )
            .into_response()
    } else {
        (StatusCode::NOT_FOUND, "Audio not found").into_response()
    }
}

fn map_synthesize_error(error: SynthesizeError) -> (StatusCode, String) {
    match error {
        SynthesizeError::InvalidRequest(message) => (StatusCode::BAD_REQUEST, message),
        SynthesizeError::UnknownModel(message) => (StatusCode::NOT_FOUND, message),
        SynthesizeError::MissingConfiguration(message) => {
            (StatusCode::SERVICE_UNAVAILABLE, message)
        }
        SynthesizeError::Upstream(message) => (StatusCode::BAD_GATEWAY, message),
        SynthesizeError::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use super::map_synthesize_error;
    use crate::service::SynthesizeError;

    #[test]
    fn map_synthesize_error_returns_expected_status_codes() {
        let (status, _) = map_synthesize_error(SynthesizeError::InvalidRequest("bad input".into()));
        assert_eq!(status, StatusCode::BAD_REQUEST);

        let (status, _) = map_synthesize_error(SynthesizeError::UnknownModel("missing".into()));
        assert_eq!(status, StatusCode::NOT_FOUND);

        let (status, _) = map_synthesize_error(SynthesizeError::MissingConfiguration(
            "misconfigured".into(),
        ));
        assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);

        let (status, _) = map_synthesize_error(SynthesizeError::Upstream("upstream".into()));
        assert_eq!(status, StatusCode::BAD_GATEWAY);

        let (status, _) = map_synthesize_error(SynthesizeError::Internal("boom".into()));
        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
