use axum::{Router, routing::get};

use backend::api::wishlist::create_wishlist;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/wishlist", get(create_wishlist));
    let listner = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listner, app).await.unwrap();
}
