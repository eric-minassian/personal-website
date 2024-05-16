mod routes;

use axum::{routing::get, Router};
use routes::{hello::greet, index::index};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/hello/:name", get(greet))
        .nest_service("/resume", ServeDir::new("assets/resume"))
        .nest_service("/public", ServeDir::new("public"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
