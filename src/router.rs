use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_governor::{
    governor::GovernorConfigBuilder, key_extractor::GlobalKeyExtractor, GovernorLayer,
};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    authentication::jwt,
    documentation::api_documentation::ApiDoc,
    routes::{authentication, customers, fallback::get_fallback, index::get_index},
    AppState,
};

pub async fn create_router(app_state: AppState) -> Router {
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(100)
            .key_extractor(GlobalKeyExtractor)
            .finish()
            .unwrap(),
    );

    Router::new()
        // business routes
        .nest(
            "/customers",
            Router::new().nest(
                "/",
                Router::new()
                    .route("/", get(customers::view::customers))
                    .route("/:customer_id", get(customers::view::customer)),
            ),
        )
        .nest(
            "/authentication",
            Router::new().route("/check", get(authentication::check::index)),
        )
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt::jwt_guard,
        ))
        // authentication
        .nest(
            "/authentication",
            Router::new().route("/login", post(authentication::login::user)),
        )
        .route_layer(middleware::from_fn_with_state(
            app_state.clone(),
            jwt::jwt_guard_optional,
        ))
        // default routes
        .route("/", get(get_index))
        // logs
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .fallback(get_fallback)
        // documentation
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .layer(GovernorLayer {
            config: Box::leak(governor_conf),
        })
        .with_state(app_state.clone())
}
