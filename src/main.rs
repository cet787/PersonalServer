use axum::{
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Status {
    status: String,
}

async fn health() -> Json<Status> {
    Json(Status {
        status: "ok".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health));

    let listener =
        TcpListener::bind("0.0.0.0:3000")
            .await
            .unwrap();

    println!("Listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

