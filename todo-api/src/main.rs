use sqlx::PgPool;
use std::net::SocketAddr;
use todo_api::{AppState, create_app};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todo_api=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set")?;

    let pool = PgPool::connect(&database_url).await?;

    let state = AppState { pool };

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, create_app(state)).await?;

    Ok(())
}
