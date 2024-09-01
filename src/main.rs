use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use config::Config;

mod config;
mod form;
mod model;
mod route;
mod template;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Arc::new(Config::new()?);
    let app = Router::new()
        .route("/", get(route::user::list))
        .route("/users/:username", get(route::user::edit))
        .route("/users", post(route::user::save))
        .route("/users/delete/:username", get(route::user::delete))
        .with_state(config.clone());
    let bind = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
