use axum::{Router, routing::get};
use std::net::SocketAddr;

// mod orm;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(routes::health_check))
        .route("/query", get(routes::database::query_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}