use axum::{response::Html, routing::get, Router};

async fn test() -> Html<&'static str> {
    Html("<h1>Hello world<h1>")
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(test));

    let addr = "127.0.0.1:3001";
    let listener = tokio::net::TcpListener::bind(addr)
        .await.unwrap();

    println!("Listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}
