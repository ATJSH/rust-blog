mod extractors;
mod routers;

use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_tokio_postgres=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer());

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    let v = "hello world";

    let app = Router::new()
        .route("/", get(routers::root::get_hello_world))
        .route("/category", get(routers::category::get_categories))
        .route(
            "/category/:category_id/posts",
            get(routers::post::get_category_posts),
        )
        .with_state(pool)
        .with_state(v);

    lambda_http::run(app).await
}
