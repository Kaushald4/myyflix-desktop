use axum::{routing::post, Json, Router};
use serde_json::json;

use crate::models::extract::ExtractRequest;
use crate::services::extract_service::extract_stream_link;

pub fn router() -> Router {
    Router::new().route("/extract", post(handler))
}

async fn handler(Json(payload): Json<ExtractRequest>) -> Json<serde_json::Value> {
    match extract_stream_link(
        &payload.id,
        &payload.content_type,
        payload.season,
        payload.episode,
        "https://localhost:4000",
    )
    .await
    {
        Ok(Some(data)) => Json(json!({ "ok": true, "data": data })),
        Ok(None) => Json(json!({ "ok": false, "error": "Not found" })),
        Err(e) => Json(json!({ "ok": false, "error": e.to_string() })),
    }
}
