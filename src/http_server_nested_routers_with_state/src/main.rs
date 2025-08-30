use std::sync::{atomic::AtomicUsize, Arc};

use axum::{extract::State, response::Html, routing::get, Extension, Router};

struct MyCounter {
    n: AtomicUsize,
}

struct MyConfig {
    text: String,
}

async fn sv1_handler(
   Extension(counter): Extension<Arc<MyCounter>>,
   State(config): State<Arc<MyConfig>>,
) -> Html<String> {
    counter.n.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    Html(format!("{} - {}",
        config.text,
        counter.n.load(std::sync::atomic::Ordering::Relaxed),
    ))
}

fn service_one() -> Router {
    let state = Arc::new(MyConfig{ text: "Service 1".to_string() });

    Router::new()
        .route("/", get(sv1_handler))
        .with_state(state)
}

async fn sv2_handler(
   Extension(counter): Extension<Arc<MyCounter>>,
    State(config): State<Arc<MyConfig>>
) -> Html<String> {
    counter.n.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    Html(format!("{} - {}", config.text, counter.n.load(std::sync::atomic::Ordering::Relaxed)))
}

fn service_two() -> Router {
    let state = Arc::new(MyConfig{ text: "Service 2".to_string() });

    Router::new()
        .route("/", get(sv2_handler))
        .with_state(state)
}

#[tokio::main]
async fn main() {
    let shared_counter = Arc::new(MyCounter {
        n: AtomicUsize::new(0),
    });
    let shared_text = Arc::new(MyConfig {
        text: "Shared config".to_string(),
    });

    let app = Router::new()
        .nest("/1", service_one())
        .nest("/2", service_two())
        .layer(Extension(shared_counter))
        .layer(Extension(shared_text));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await.unwrap();

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
