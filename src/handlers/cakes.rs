use std::process::id;
use std::str::FromStr;

use axum::extract::Path;
use axum::{extract::State, http::StatusCode, Form, Json};
use mongodb::{bson::oid::ObjectId, Client};

use crate::models::cake::Cake;

pub async fn get_all(State(db): State<Client>) -> Result<Json<Vec<Cake>>, StatusCode> {
    Cake::get_all(&db)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_one(
    State(db): State<Client>,
    Path(id): Path<String>,
) -> Result<Json<Cake>, StatusCode> {
    let Ok(id) = ObjectId::from_str(&id) else {
        Err(StatusCode::UNPROCESSABLE_ENTITY)?
    };
    Cake::get_one(&db, id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn update_one(State(db): State<Client>, Path(id): Path<String>) -> StatusCode {
    let Ok(id) = ObjectId::from_str(&id) else {
        return StatusCode::UNPROCESSABLE_ENTITY;
    };
    match Cake::update_one(&db, id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn add_one(State(db): State<Client>, Form(product): Form<Cake>) -> StatusCode {
    match Cake::add_one(&db, product).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
