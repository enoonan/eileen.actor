use axum::{routing::{get, get_service}, Router, response::Html};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
      .route("/.well-known/atproto-did", get(|| async { "did:plc:fgu6fltgbrfq3feill27dtyj" }))
      .nest_service("/", get_service(ServeDir::new("public")))
      .nest_service("/static", get_service(ServeDir::new("static")));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
