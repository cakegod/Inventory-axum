use std::net::SocketAddr;

use axum::http::StatusCode;
use axum::{routing::get, Router};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use tower_http::services::ServeDir;

mod handlers;
mod models;

const DATABASE_NAME: &str = "test";
const PRODUCT_COLLECTION_NAME: &str = "products";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    tracing::debug!("listening on {}", addr);

    let app = app().await?;

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn setup_db() -> anyhow::Result<Client> {
    let client_options =
        ClientOptions::parse(std::env::var("MONGODB_URL").expect("MONGODB_URL is not set"));
    let client = Client::with_options(client_options.await?)?;
    Ok(client)
}

async fn app() -> anyhow::Result<Router> {
    let db = setup_db().await;
    let router = Router::new()
        .route(
            "/products",
            get(handlers::cakes::get_all).post(handlers::cakes::add_one),
        )
        .route(
            "/products/:id",
            get(handlers::cakes::get_one)
                .put(handlers::cakes::update_one)
                .delete(handlers::cakes::delete_one),
        )
        .nest_service("/styles.css", ServeDir::new("assets/styles.css").clone())
        .with_state(db?)
        .fallback(|| async { StatusCode::NOT_FOUND });

    Ok(router)
}
