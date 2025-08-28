use std::{sync::atomic::AtomicUsize};

use axum::{response::Html, routing::get, Extension, Router};
use std::sync::Arc;
use std::sync::atomic::Ordering::Relaxed;

struct MyCounter {
    counter: AtomicUsize,
}

struct MyConfig {
    text: String,
}

async fn handler(
    Extension(counter): Extension<Arc<MyCounter>>,
    Extension(config): Extension<Arc<MyConfig>>,
) -> Html<String> {
    let visitors_count = counter.counter.fetch_add(1, Relaxed);

    Html(format!("{} {}", config.text, visitors_count))
}

#[tokio::main]
async fn main() {
    let shared_counter= Arc::new(
        MyCounter {
            counter: AtomicUsize::new(0),
        }
    );
    let shared_text = Arc::new(
        MyConfig {
            text: "You are visitor #".to_string(),
        }
    );

    let app = Router::new()
        .route("/", get(handler))
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
