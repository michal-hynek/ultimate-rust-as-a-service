use std::time::UNIX_EPOCH;

use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};

async fn handler() -> Result<impl IntoResponse, (StatusCode, String)> {
    let start = std::time::SystemTime::now();
    let seconds_wrapped = start.duration_since(UNIX_EPOCH)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "bad clock".to_string()))?
        .as_secs() % 3;

    let divided = 100u64.checked_div(seconds_wrapped)
        .ok_or((StatusCode::INTERNAL_SERVER_ERROR, "division by 0".to_string()))?;

    Ok(Json(divided))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}