use std::collections::HashMap;

use axum::{extract::{Path, Query}, response::Html, routing::get, Router};
use axum::http::HeaderMap;

async fn test() -> Html<&'static str> {
    Html("<h1>Hello world<h1>")
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
    let app = Router::new()
        .route("/", get(test))
        .route("/books/{id}", get(path_extract))
        .route("/books", get(query_extract))
        .route("/headers", get(header_extract));

    let addr = "127.0.0.1:3001";
    let listener = tokio::net::TcpListener::bind(addr)
        .await.unwrap();

    println!("Listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}
