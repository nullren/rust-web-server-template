use axum::{response::Json, routing::get, Router};
use clap::Parser;
use rust_web_server_template::config::Config;
use serde_json::{json, Value};

// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
async fn plain_text() -> &'static str {
    "foo"
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[tokio::main]
async fn main() {
    let config = Config::parse();

    let api = Router::new().route("/json", get(json));

    let app = Router::new()
        .nest("/api/v1", api)
        .route("/health", get(plain_text))
        .route("/", get(|| async { "Hello, World!" }));

    let addr = format!("0.0.0.0:{}", config.port).parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
