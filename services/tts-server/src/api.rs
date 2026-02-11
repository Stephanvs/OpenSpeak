use axum::{
    extract::{Path, State},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::{model::SynthesizeRequest, service::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/v1/health", get(health))
        .route("/v1/models", get(models))
        .route("/v1/synthesize", post(synthesize))
        .route("/v1/audio/:request_id", get(audio))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::to_value(state.health()).unwrap_or_else(|_| serde_json::json!({"status":"ok","models_loaded":0})))
}

async fn models(State(state): State<AppState>) -> Json<Vec<crate::model::VoiceModel>> {
    Json((*state.models).clone())
}

async fn synthesize(
    State(state): State<AppState>,
    Json(req): Json<SynthesizeRequest>,
) -> Result<Json<crate::model::SynthesizeResponse>, (StatusCode, String)> {
    if req.text.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "text cannot be empty".into()));
    }

    state
        .synthesize(req)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::NOT_FOUND, e))
}

async fn audio(State(state): State<AppState>, Path(request_id): Path<String>) -> Response {
    let store = state.audio_store.read().await;
    if let Some(data) = store.get(&request_id) {
        (
            StatusCode::OK,
            [(axum::http::header::CONTENT_TYPE, HeaderValue::from_static("audio/wav"))],
            data.clone(),
        )
            .into_response()
    } else {
        (StatusCode::NOT_FOUND, "Audio not found").into_response()
    }
}
