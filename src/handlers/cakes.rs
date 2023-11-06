use axum::{extract::State, Form, http::StatusCode, Json, RequestPartsExt};
use mongodb::Client;

use crate::{models, PRODUCT_COLLECTION_NAME};
use crate::models::cake::Cake;
use crate::models::Id;

pub async fn get_all(State(db): State<Client>) -> Result<Json<Vec<Cake>>, StatusCode> {
    match models::get_all(&db, PRODUCT_COLLECTION_NAME).await {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

pub async fn get_one(State(db): State<Client>, Id(id): Id) -> Result<Json<Cake>, StatusCode> {
    match models::get_one(&db, id, PRODUCT_COLLECTION_NAME).await {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

pub async fn update_one(
    State(db): State<Client>,
    Id(id): Id,
    Form(updated): Form<Cake>,
) -> StatusCode {
    match models::update_one(&db, id, updated, PRODUCT_COLLECTION_NAME).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn add_one(State(db): State<Client>, Form(product): Form<Cake>) -> StatusCode {
    match models::add_one(&db, product, PRODUCT_COLLECTION_NAME).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
pub async fn delete_one(State(db): State<Client>, Id(id): Id) -> StatusCode {
    match models::delete_one::<Cake>(&db, id, PRODUCT_COLLECTION_NAME).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
