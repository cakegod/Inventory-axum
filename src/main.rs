use std::net::SocketAddr;

use axum::{routing::get, Router};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use tower_http::services::ServeDir;

mod handlers;
mod models;

const DATABASE_NAME: &str = "inventorydb";
const PRODUCT_COLLECTION_NAME: &str = "inventory";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app().await.into_make_service())
        .await
        .unwrap();
}

async fn setup_db() -> Client {
    let client_options =
        ClientOptions::parse(std::env::var("MONGODB_URL").expect("MONGODB_URL is not set"));
    Client::with_options(client_options.await.unwrap()).unwrap()
}

async fn app() -> Router {
    let db = setup_db().await;
    Router::new()
        .route("/products", get(handlers::cakes::get_all))
        .route("/products/:id", get(handlers::cakes::get_one))
        .nest_service("/styles.css", ServeDir::new("assets/styles.css").clone())
        .with_state(db)
}
