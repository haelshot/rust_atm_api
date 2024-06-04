use std::net::SocketAddr;

use deadpool_diesel::postgres::{Manager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::settings::config;
use crate::app::app_router;

mod settings;
mod app;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[derive(Clone)]
pub struct AppState {
    pool: Pool,
}

#[tokio::main]
async fn main() {
    // init_tracing();

    // Load settings
    let config = config().await;

    // Create connection pool to psql db
    let manager = Manager::new(
        config.db_url().to_string(),
        deadpool_diesel::Runtime::Tokio1,
    );

    let pool = Pool::builder(manager).build().unwrap();
    run_migrations(&pool).await;


    let state = AppState {
        pool
    };


    let app = app_router(state.clone()).with_state(state);

    let host = config.server_host();
    let port = config.server_port();

    let address = format!("{}:{}", host, port);

    let socket_addr: SocketAddr = address.parse().unwrap();

    tracing::info!("listening on http://{}", socket_addr);

    axum::Server::bind(&socket_addr)
    .serve(app.into_make_service())
    .await
    .unwrap()
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}


async fn run_migrations(pool: &Pool) {
    let conn = pool.get().await.unwrap();
    conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
    .await
    .unwrap();
}