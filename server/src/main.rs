mod extractors;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{http::StatusCode, routing::get, Router};
use chrono::NaiveDateTime;
use extractors::DatabaseConnection;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[derive(Deserialize, Serialize)]
struct GetPostRow {
    id: i64,
    title: String,
    content: String,
    created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
struct GetCategoryRow {
    id: i64,
    name: String,
    created_at: NaiveDateTime,
}

async fn root() -> &'static str {
    "Hello, world!"
}

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
        .route("/", get(root))
        .route("/category", get(get_categories))
        .route("/category/:category_id/posts", get(get_category_posts))
        .with_state(pool)
        .with_state(v);

    lambda_http::run(app).await
}

async fn get_categories(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query_as!(GetCategoryRow, "select id, name, created_at from category")
        .fetch_all(&mut *conn)
        .await
        .unwrap();

    Ok(axum::Json(result))
}

async fn get_category_posts(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(category_id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query_as!(
        GetPostRow,
        "select id, title, content, created_at from post where category_id = $1",
        category_id
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap();

    Ok(axum::Json(result))
}
