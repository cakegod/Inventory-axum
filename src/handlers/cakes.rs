use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::{bson::oid::ObjectId, Client};

use crate::models::cake::Cake;

pub async fn get_all(State(db): State<Client>) -> Json<Vec<Cake>> {
    Json(Cake::get_all(&db).await.unwrap())
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
