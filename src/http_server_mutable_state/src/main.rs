use std::{collections::HashMap, sync::atomic::AtomicUsize};

use axum::{extract::{Path, Query}, response::Html, routing::get, Router};
use axum::http::HeaderMap;
use axum::extract::State;
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;

struct MyConfig {
    counter: AtomicUsize,
}

async fn handler(State(config): State<Arc<MyConfig>>) -> Html<String> {
    config.counter.fetch_add(1, Relaxed);

    Html(format!("You are visitor {}", config.counter.load(Relaxed)))
}

async fn path_extract(Path(id): Path<u32>) -> Html<String> {
    Html(format!("Book ID {id}"))
}

async fn query_extract(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    Html(format!("{params:#?}"))
}

async fn header_extract(headers: HeaderMap) -> Html<String> {
    Html(format!("{headers:#?}"))
}



#[tokio::main]
async fn main() {
    let shared_config = Arc::new(
        MyConfig {
            counter: AtomicUsize::new(0),
        }
    );

    let app = Router::new()
        .route("/", get(handler))
        .route("/books/{id}", get(path_extract))
        .route("/books", get(query_extract))
        .route("/headers", get(header_extract))
        .with_state(shared_config);

    let addr = "127.0.0.1:3001";
    let listener = tokio::net::TcpListener::bind(addr)
        .await.unwrap();

    println!("Listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}
