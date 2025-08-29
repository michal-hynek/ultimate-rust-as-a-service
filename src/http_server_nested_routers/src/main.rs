use axum::{response::Html, routing::get, Router};

fn service_one() -> Router {
    Router::new().route("/", get(|| async {
        Html("Service one")
    }))
}

fn service_two() -> Router {
    Router::new().route("/", get(|| async {
        Html("Service two")
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest("/1", service_one())
        .nest("/2", service_two());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
