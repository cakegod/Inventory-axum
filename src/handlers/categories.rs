use axum::{extract::State, http::StatusCode, Form, Json};
use mongodb::Client;

use crate::models::category::Category;
use crate::models::{Id, CRUD};

pub async fn get_all(State(db): State<Client>) -> Result<Json<Vec<Category>>, StatusCode> {
    match Category::get_all(&db).await {
        Ok(products) => Ok(Json(products)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

pub async fn get_one(State(db): State<Client>, Id(id): Id) -> Result<Json<Category>, StatusCode> {
    match Category::get_one(&db, id).await {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(StatusCode::SERVICE_UNAVAILABLE),
    }
}

pub async fn update_one(
    State(db): State<Client>,
    Id(id): Id,
    Form(updated): Form<Category>,
) -> StatusCode {
    match Category::update_one(&db, id, updated).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn add_one(State(db): State<Client>, Form(product): Form<Category>) -> StatusCode {
    match Category::add_one(&db, product).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
pub async fn delete_one(State(db): State<Client>, Id(id): Id) -> StatusCode {
    match Category::delete_one(&db, id).await {
        Ok(_) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
