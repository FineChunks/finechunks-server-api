use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde_json::json;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async {
            Json(json!({ "message": "Hello, World!" }))
        }))
        .layer(cors);

    // run our app with hyper, listening globally on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}