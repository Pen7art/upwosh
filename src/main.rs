use std::sync::Arc;

use data::model::app_state::AppState;
use routes::app_routes::app_routes;
use tracing::{debug, info};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;

mod test;
mod data;
mod infra;
mod middleware;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    info!("Starting server...");
    let _guard = init_tracing();

    let app_state = AppState {
        access_token: "test".to_string(),
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app_routes(Arc::new(app_state)))
        .await
        .unwrap();
}

fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::hourly("logs", "upwosh.log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_env_filter("server")
            .with_max_level(tracing::Level::INFO)
            .finish()
            .with(fmt::Layer::default().with_writer(file_writer)),
    )
    .expect("Unable to set global tracing subscriber");
    debug!("Tracing initialized.");
    guard
}
