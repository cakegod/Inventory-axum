use std::net::SocketAddr;

use askama_axum::IntoResponse;
use axum::http::StatusCode;
use axum::{routing::get, Router};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use tower_http::services::ServeDir;

use crate::templates::Template_404;

mod handlers;
mod models;
mod templates;

const DATABASE_NAME: &str = "inventorydb";
const PRODUCT_COLLECTION_NAME: &str = "inventory";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::debug!("listening on {}", addr);

    let app = app().await.fallback(handler_404);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
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
        .route(
            "/products",
            get(handlers::cakes::get_all).post(handlers::cakes::add_one),
        )
        .route(
            "/products/:id",
            get(handlers::cakes::get_one).put(handlers::cakes::update_one),
        )
        .nest_service("/styles.css", ServeDir::new("assets/styles.css").clone())
        .with_state(db)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Template_404 {
            title: "404".to_string(),
        },
    )
}
