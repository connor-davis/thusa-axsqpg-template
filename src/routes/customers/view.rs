use axum::{extract, http::StatusCode, response::IntoResponse, Json};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{data::models::user::User, AppState};

#[utoipa::path(
    get,
    path = "/customers",
    tag = "Customers",
    security(("bearer_auth" = [])),
)]
//TODO
#[allow(unused)]
pub async fn customers(
    extract::State(app_state): extract::State<AppState>,
    extract::Extension(authentication_user): extract::Extension<User>,
) -> Result<(StatusCode, impl IntoResponse), (StatusCode, Json<Value>)> {
    Ok((
        StatusCode::OK,
        Json(json!({
            "success": true
        })),
    ))
}

#[utoipa::path(
    get,
    path = "/customers/{customer_id}",
    params(("customer_id" = String, Path, description = "The customers id.")),
    tag = "Customers",
    security(("bearer_auth" = [])),
)]
// TODO
#[allow(unused)]
pub async fn customer(
    extract::State(app_state): extract::State<AppState>,
    extract::Path(customer_id): extract::Path<Uuid>,
    extract::Extension(authentication_user): extract::Extension<User>,
) -> Result<(StatusCode, impl IntoResponse), (StatusCode, Json<Value>)> {
    Ok((
        StatusCode::OK,
        Json(json!({
            "success": true,
            "customer_id": customer_id
        })),
    ))
}
