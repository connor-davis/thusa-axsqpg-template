use axum::{extract, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::AppState;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomerPayload {}

// TODO
#[allow(unused)]
pub async fn customer(
    extract::State(app_state): extract::State<AppState>,
    extract::Json(payload): extract::Json<CustomerPayload>,
) -> Result<(StatusCode, impl IntoResponse), (StatusCode, Json<Value>)> {
    Ok((
        StatusCode::OK,
        Json(json!({
            "success": true
        })),
    ))
}
