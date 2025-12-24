use std::{net::SocketAddr, sync::Arc};

use tracing::info;

mod models;
mod routes;
mod settings;
mod utils;

use crate::settings::Settings;
use tower_http::cors::{Any, CorsLayer};
use http::{Method, HeaderValue};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = sqlx::SqlitePool::connect("sqlite://data.db?mode=rwc")
        .await
        .unwrap();

    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("https://hamrofund.org/*"))
        .allow_origin(HeaderValue::from_static("https://cdn.hamrofund.org/*"))
        .allow_origin(HeaderValue::from_static("http://localhost:3000/*"))
        .allow_methods(Any)
        .allow_headers(Any);

    let schema = include_str!("../schema.sql");
    sqlx::query(schema).execute(&pool).await.unwrap();

    let config = Settings::new().unwrap();
    let app = routes::create(Arc::new(pool), &config).layer(cors);

    let addr = SocketAddr::from((config.bind_addr, config.bind_port));
    info!("listening on http://{}/", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
