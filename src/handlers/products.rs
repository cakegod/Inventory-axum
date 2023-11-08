use axum::{extract::State, http::StatusCode, Form, Json, RequestPartsExt};
use mongodb::Client;

use crate::models::product::Product;
use crate::models::{Id, CRUD};

pub async fn get_all(State(db): State<Client>) -> Result<Json<Vec<Product>>, StatusCode> {
    match Product::get_all(&db).await {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

pub async fn get_one(State(db): State<Client>, Id(id): Id) -> Result<Json<Product>, StatusCode> {
    match Product::get_one(&db, id).await {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

pub async fn update_one(
    State(db): State<Client>,
    Id(id): Id,
    Form(updated): Form<Product>,
) -> StatusCode {
    match Product::update_one(&db, id, updated).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn add_one(State(db): State<Client>, Form(product): Form<Product>) -> StatusCode {
    match Product::add_one(&db, product).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
pub async fn delete_one(State(db): State<Client>, Id(id): Id) -> StatusCode {
    match Product::delete_one(&db, id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
