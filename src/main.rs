use std::net::SocketAddr;

use anyhow::Error;
use axum::{
    extract::DefaultBodyLimit,
    http::{header, HeaderValue, Method},
    Router,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{config::Config, router::create_router};

mod config;
mod router;
mod routes;
mod utils;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub config: Config,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!(
        r#"

'########:'##::::'##:'##::::'##::'######:::::'###:::::::'########:'########::'######::'##::::'##:
... ##..:: ##:::: ##: ##:::: ##:'##... ##:::'## ##::::::... ##..:: ##.....::'##... ##: ##:::: ##:
::: ##:::: ##:::: ##: ##:::: ##: ##:::..:::'##:. ##:::::::: ##:::: ##::::::: ##:::..:: ##:::: ##:
::: ##:::: #########: ##:::: ##:. ######::'##:::. ##::::::: ##:::: ######::: ##::::::: #########:
::: ##:::: ##.... ##: ##:::: ##::..... ##: #########::::::: ##:::: ##...:::: ##::::::: ##.... ##:
::: ##:::: ##:::: ##: ##:::: ##:'##::: ##: ##.... ##::::::: ##:::: ##::::::: ##::: ##: ##:::: ##:
::: ##:::: ##:::: ##:. #######::. ######:: ##:::: ##::::::: ##:::: ########:. ######:: ##:::: ##:
:::..:::::..:::::..:::.......::::......:::..:::::..::::::::..:::::........:::......:::..:::::..::
                                                                  
    "#
    );

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(32)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connected to database.");
            pool
        }
        Err(error) => {
            println!("🔥 Failed to connect to database: {}", error);
            std::process::exit(1);
        }
    };

    match sqlx::migrate!().run(&pool).await {
        Ok(_) => {
            println!("✅ Database migrated.");
        }
        Err(error) => {
            println!("🔥 Failed to run database migrations: {}", error);
            std::process::exit(1);
        }
    };

    let app_state: AppState = AppState { db: pool, config };

    let router: Router = create_router(app_state.clone()).await;

    let cors: CorsLayer = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT])
        .allow_credentials(true);

    let router: Router = router
        .layer(DefaultBodyLimit::max(100_000_000))
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let address: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let listener: TcpListener = TcpListener::bind(address).await?;

    println!("🚀 Server listening on: {}", address);

    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
