use std::net::SocketAddr;

use axum::http::StatusCode;
use axum::{routing::get, Router};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};

mod handlers;
mod models;

const DATABASE_NAME: &str = "test";
const PRODUCT_COLLECTION_NAME: &str = "products";
const CATEGORIES_COLLECTION_NAME: &str = "categories";

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
        .nest("/products", products_route())
        .nest("/categories", categories_route())
        .with_state(db?)
        .fallback(|| async { StatusCode::NOT_FOUND });

    Ok(router)
}

fn products_route() -> Router<Client> {
    Router::new()
        .route(
            "/",
            get(handlers::products::get_all).post(handlers::products::add_one),
        )
        .route(
            "/:id",
            get(handlers::products::get_one)
                .put(handlers::products::update_one)
                .delete(handlers::products::delete_one),
        )
}

fn categories_route() -> Router<Client> {
    Router::new()
        .route(
            "/",
            get(handlers::categories::get_all).post(handlers::categories::add_one),
        )
        .route(
            "/:id",
            get(handlers::categories::get_one)
                .put(handlers::categories::update_one)
                .delete(handlers::categories::delete_one),
        )
}
