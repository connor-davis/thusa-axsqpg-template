use axum::{routing::get, Router};

use crate::{
    routes::{fallback::get_fallback, index::get_index},
    AppState,
};

pub async fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(get_index))
        .fallback(get_fallback)
        .with_state(app_state)
}
